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
