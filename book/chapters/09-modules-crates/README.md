# 第 9 章：modules、crates、visibility 和 workspace

写小脚本时，文件结构不重要；写可长期维护的 Rust 库时，模块边界就是架构边界。专业 Rust 开发者必须能回答：哪些类型公开？哪些字段隐藏？哪些函数属于稳定 API？哪些只是内部实现？

## package、crate、module

Rust 有三个容易混淆的层次：

- **package**：由一个 `Cargo.toml` 描述的发布和构建单元。
- **crate**：一次编译单元，可以是 library crate，也可以是 binary crate。
- **module**：crate 内部的命名空间和可见性边界。

一个 package 可以同时包含一个 library crate 和多个 binary crate。比如第 1 章的 `ch01-rust-program-shape` 有多个 `src/bin/*.rs`，每个文件都是一个 binary target。

量化计算框架通常至少会拆成这些 crate：

```text
factor-core      # 核心计算，少依赖，高测试密度
factor-python    # PyO3 绑定，处理 Python 边界
backtest-core    # 回测状态机
experiment-runner
online-engine
```

学习阶段不急着创建很多 crate，但必须从一开始理解边界。

## visibility 是 API 设计工具

`pub` 不是“让代码能用”的开关，而是稳定性承诺。公开字段意味着外部调用者可以直接依赖字段名字和表示方式；以后你想改数据布局就会破坏调用方。

示例中 `Bar` 的字段是私有的：

```rust
pub struct Bar {
    symbol: String,
    close: f64,
}
```

调用者只能通过 `symbol()` 和 `close()` 访问。这样未来可以把 `symbol` 改成 interned symbol id，把 `close` 改成定点数或压缩列存，而不必改变外部 API。

## 模块依赖方向

模块依赖要从稳定概念指向具体实现，不能反过来。

好边界：

- `factors` 依赖 `market::Bar`
- `backtest` 依赖 `orders` 和 `portfolio`
- `python` 依赖 `factor-core`

坏边界：

- `factor-core` 依赖 Python 类型
- 数值 kernel 依赖 CLI 参数解析
- 数据结构层依赖日志输出和文件路径

高性能系统里，核心计算模块应该尽量少依赖。依赖越少，越容易 benchmark、测试、嵌入 Python、嵌入在线系统。

## workspace 的意义

workspace 允许多个 package 共用一个 `Cargo.lock`、一个 target 目录和统一命令：

```bash
cargo test
cargo test -p factor-core
cargo clippy --all-targets -- -D warnings
```

本仓库把每章 example 拆成独立 package，但仍放在同一个 workspace。这样你可以单独练一章，也可以全仓库验证。

## 本章示例

```bash
cargo test -p ch09-modules-crates
```

阅读：

```text
book/chapters/09-modules-crates/example/src/lib.rs
```

重点看：

- `market` 模块如何隐藏字段。
- `factors` 模块如何只依赖公开方法。
- 测试如何从外部视角使用 API。

## 本章练习

1. 给 `Bar` 增加 `volume` 字段，但不要公开字段。
2. 增加 `typical_price()` 方法。
3. 新增 `risk` 模块，实现 `price_change_abs(previous, current)`。
4. 故意把字段改成 `pub`，写一段说明为什么这会扩大 API 承诺。

## 本章验收

你应该能解释：

- package、crate、module 的区别。
- 为什么公开字段会限制未来重构。
- 为什么核心计算 crate 不应该依赖 Python 绑定层。
- 为什么 workspace 能帮助课程和生产项目同时保持一致质量门。

## 教材化补充：模块边界就是维护边界

初学者常把模块理解成“把文件拆开”。这只说对了一半。模块真正重要的地方是控制依赖和可见性。

当你写：

```rust
pub struct Bar {
    symbol: String,
    close: f64,
}
```

你创建了一个公开类型，但没有公开字段。调用者知道有 `Bar`，但不知道内部字段如何存储。它只能通过方法访问：

```rust
bar.symbol()
bar.close()
```

这让你未来可以改变内部结构。比如把 `symbol: String` 改成 `symbol_id: u32`，只要 `symbol()` 的行为保持，外部调用者就不需要改。

## Python 对照

Python 中模块边界通常比较软：

```python
bar.symbol = "AAPL"
bar.close = 100.0
```

任何人都能改对象字段。灵活，但长期系统里容易出现“字段被谁改了”的问题。Rust 的 visibility 让你从编译期开始控制访问权限。

## Python subpackage 到 Rust module tree

如果你有 Python package 开发经验，最容易产生的误解是：看到业务逻辑变复杂，就想在 Rust 里继续创建“子 package”。Rust 通常不这样做。

Python 常见结构可能是：

```text
quant/
  __init__.py
  market/
    __init__.py
    bar.py
  factors/
    __init__.py
    returns.py
    volatility.py
  backtest/
    __init__.py
    engine.py
```

在 Rust 里，如果这些逻辑仍属于同一个稳定编译和发布单元，通常先放进一个 crate 内，用 module tree 表达：

```text
factor-core/
  Cargo.toml
  src/
    lib.rs
    market.rs
    factors/
      mod.rs
      returns.rs
      volatility.rs
    backtest/
      mod.rs
      engine.rs
```

`src/lib.rs` 是这个 library crate 的入口。它决定哪些模块存在，哪些模块对外公开：

```rust
pub mod market;
pub mod factors;
pub mod backtest;
```

`src/factors/mod.rs` 是 `factors` 这个模块的入口。它继续声明子模块：

```rust
pub mod returns;
mod volatility;
```

这里的差异很重要：

- `pub mod returns;` 表示外部可以通过 `factor_core::factors::returns` 访问。
- `mod volatility;` 表示 `volatility` 只是当前 crate 内部模块，不直接公开给外部调用者。

如果 `returns.rs` 里有函数：

```rust
pub fn close_to_close(previous: f64, current: f64) -> Option<f64> {
    if previous <= 0.0 {
        return None;
    }

    Some(current / previous - 1.0)
}
```

外部用户可以这样用：

```rust
use factor_core::factors::returns::close_to_close;
```

这和 Python 的 import 很像，但 Rust 多了两个约束：

1. 文件存在不等于模块存在。你必须用 `mod` 或 `pub mod` 把它挂进 module tree。
2. 模块存在不等于外部可访问。你必须用 `pub` 明确公开。

## `mod`、`pub mod`、`use` 分别做什么

这三个词要分开理解：

| 语法 | 作用 | 类比 |
| --- | --- | --- |
| `mod market;` | 声明模块存在，把文件纳入当前 crate 编译 | 告诉 Rust “这里有一个子模块” |
| `pub mod market;` | 声明模块存在，并把模块名公开给外部 | 公开这个命名空间 |
| `use crate::market::Bar;` | 把已有路径引入当前作用域，方便书写 | Python 的 `from ... import ...` |

`use` 不会创建模块，也不会改变公开性。它只是让当前文件少写长路径。

例如：

```rust
use crate::market::Bar;
```

之后可以写：

```rust
fn compute(bar: &Bar) {}
```

而不是每次写：

```rust
fn compute(bar: &crate::market::Bar) {}
```

新手常见错误是以为 `use` 能让别人访问你的类型。不能。别人能不能访问，取决于模块和类型本身有没有 `pub`。

## `pub` 的层级：公开模块不等于公开字段

Rust 的公开性是逐层控制的。

```rust
pub mod market {
    pub struct Bar {
        symbol: String,
        close: f64,
    }
}
```

这段代码表示：

- `market` 模块公开。
- `Bar` 类型公开。
- `symbol` 和 `close` 字段不公开。

所以外部可以写：

```rust
use factor_core::market::Bar;
```

但不能写：

```rust
let close = bar.close;
```

除非你把字段也改成：

```rust
pub close: f64
```

不要因为“方便测试”就公开字段。字段一旦公开，外部就可以绕过构造函数和校验，未来你想换存储方式也会变成破坏性改动。

如果只想让同一个 crate 内部可见，可以用更窄的公开性：

```rust
pub(crate) fn validate_close(close: f64) -> bool {
    close.is_finite() && close > 0.0
}
```

`pub(crate)` 表示整个 crate 内可见，但 crate 外不可见。这适合内部工具函数。

## 什么时候用 module，什么时候拆 crate

不要把 Python subpackage 的直觉直接翻译成多个 Rust package。先问边界是否真的稳定。

继续放在同一个 crate 的信号：

- 只是为了组织文件。
- 模块之间经常一起修改。
- 依赖集合基本一样。
- 不需要单独发布或单独版本。
- 公开 API 还不稳定。

应该考虑拆成独立 crate 的信号：

- 这个部分会被多个应用复用。
- 它需要不同依赖，比如 Python binding 需要 PyO3，但核心计算不应该依赖 PyO3。
- 它有稳定 API，可以作为独立边界测试和 benchmark。
- 它的编译成本或 feature 开关应该和核心逻辑隔离。
- 它属于不同变化速度，比如 `factor-core` 很稳定，`factor-python` 会跟 Python packaging 变化。

量化系统里常见的判断是：

```text
factor-core      # 拆 crate：纯计算核心，少依赖，稳定 API
factor-python    # 拆 crate：Python 绑定层，依赖 PyO3
backtest-core    # 可以拆 crate：独立状态机和测试边界
market::bar      # 通常先做 module：市场数据类型内部层次
factors::returns # 通常先做 module：因子计算内部层次
```

也就是说：

```text
module = 组织代码和可见性
crate   = 编译、依赖、发布、复用边界
workspace = 多个 crate 的统一管理边界
```

## 常见误区

误区 1：文件存在，Rust 就会自动发现。

不会。你创建 `src/factors/returns.rs` 后，还需要在 `src/factors/mod.rs` 或对应父模块里写：

```rust
pub mod returns;
```

误区 2：`use` 等于 Python import，会自动加载模块。

不会。`use` 只能引用已经存在于 module tree 的路径。

误区 3：所有模块都 `pub`，开发更快。

短期更快，长期更难维护。`pub` 是 API 承诺。先保持私有，需要外部使用时再公开。

误区 4：目录越多越专业。

不对。Rust 更看重边界是否清楚。一个小 crate 里有清晰的 `market`、`factors`、`risk` 模块，比过早拆成许多 crate 更容易学习和维护。

误区 5：crate 越多越像真实工程。

也不对。拆 crate 会带来依赖管理、版本、feature、编译边界、公开 API 设计成本。没有稳定边界前，先用 module。

## 逐行理解本章示例

示例路径：

```text
book/chapters/09-modules-crates/example/src/lib.rs
```

`market` 模块定义市场数据，`factors` 模块定义因子计算。`factors` 使用：

```rust
use crate::market::Bar;
```

这表示它依赖同一个 crate 内的 `market::Bar`。它不能直接访问 `Bar` 的私有字段，只能调用公开方法。这正是 API 边界在发挥作用。

## 操作实验

尝试把 `Bar` 的字段改成：

```rust
pub symbol: String,
pub close: f64,
```

代码会更“方便”，但你要写下代价：外部调用者现在可以直接构造非法 `Bar`，也可以在任何地方修改字段。以后你想加入校验、压缩 symbol、调整布局都会更困难。

## 专业判断

核心计算 crate 应该尽量：

- 少依赖。
- 少公开。
- 类型稳定。
- 错误清楚。
- 测试充分。

绑定层、CLI、配置解析、日志输出都应该在外层。这样核心才能被 benchmark、Python、回测、在线系统复用。

## 代码走读与操作清单

本章代码很小，但已经包含专业 crate 设计的基本习惯。

第一步，看 `Bar::new`：

```rust
pub fn new(symbol: impl Into<String>, close: f64) -> Self
```

内部需要拥有 `String`，但调用者可以传 `&str` 或 `String`。这让 API 更友好，同时仍然保持内部所有权清楚。

第二步，看私有字段：

```rust
symbol: String,
close: f64,
```

字段没有 `pub`，外部只能通过方法访问。未来如果要校验 `close > 0.0`，可以放进构造函数，而不需要追踪所有外部构造点。

第三步，看因子函数：

```rust
pub fn close_to_close_return(previous: &Bar, current: &Bar) -> Option<f64>
```

输入是 borrowed `Bar`，函数不拥有数据，也不修改数据。返回 `Option<f64>`，因为 symbol 不同或 previous price 非法时没有合法收益率。

操作清单：

1. 把 `close` 字段临时改成 `pub`。
2. 在测试中直接构造 `Bar`。
3. 再把字段改回私有。
4. 写下两种 API 对未来重构的影响。

这类练习不是语法训练，而是在训练“公开什么、隐藏什么”的工程判断。
