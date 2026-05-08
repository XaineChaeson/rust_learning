# 第 18 章：macros、features、docs 和 release 工程

专业 Rust 项目不只是代码能跑。它还要能被别人使用、测试、裁剪、发布和诊断。宏、feature、doc test、release 配置都是 crate 工程能力的一部分。

## macro_rules! 的位置

宏适合消除重复的语法模式，不适合隐藏复杂业务逻辑。数值测试中常见：

```rust
assert_close!(left, right, tolerance)
```

这类宏能把错误信息标准化。不要用宏生成难以调试的大量代码，除非普通函数或泛型已经不够。

## feature flags

feature 用来控制可选能力：

- `simd`
- `python`
- `serde`
- `rayon`

生产 crate 常把 heavy dependency 放到 feature 后面，避免所有用户都付出编译和依赖成本。

但 feature 会增加组合复杂度。每个 feature 都应该有测试策略。

## docs 和 doc tests

Rust 文档可以直接包含可运行示例：

```rust
/// ```
/// let mean = my_crate::mean(&[1.0, 2.0]);
/// ```
```

这不是装饰。doc test 能防止 README 和 API 脱节。

## release 工程

高性能结论必须基于 release build：

```bash
cargo test --release
cargo bench
```

常见 profile 选项包括：

- `opt-level`
- `lto`
- `codegen-units`
- `panic`

学习阶段不急着调 profile，但要知道它会影响性能和二进制体积。

## 本章示例

```bash
cargo test -p ch18-macros-features-docs
cargo test -p ch18-macros-features-docs --features simd
```

重点看：

- `assert_close!` 如何标准化测试。
- `backend_name()` 如何读取 feature 状态。
- `mean` 的 doc test 如何参与测试。

## 本章练习

1. 给 `assert_close!` 增加可选错误消息。
2. 增加 `parallel` feature，并让 `backend_name()` 返回不同值。
3. 给一个公开函数写 doc test。
4. 写复盘：feature 太多为什么会变成维护成本？

## 本章验收

你应该能解释：

- 宏适合解决什么问题。
- feature flag 如何影响依赖和测试矩阵。
- doc test 为什么是 API 合同的一部分。
- release profile 为什么会影响性能判断。

## 教材化补充：工程质量也是专业能力

Rust 专业开发不只是能写通过编译的函数。一个 crate 还需要：

- 文档能让别人用。
- feature 能控制可选能力。
- 测试能覆盖配置组合。
- release profile 能支持性能目标。

这些内容看起来不像“算法”，但决定项目能不能长期维护。

## 宏应该少而清楚

宏适合消除重复语法。例如数值测试里经常写：

```rust
assert!((left - right).abs() <= tolerance)
```

把它变成 `assert_close!` 可以统一错误信息。但宏不应该隐藏复杂业务逻辑。能用函数解决，就先用函数。

## feature flag 的成本

feature 让 crate 支持可选能力，比如：

- `simd`
- `python`
- `parallel`
- `serde`

但每加一个 feature，测试矩阵就变大。`default features`、组合 features、禁用 features 都可能暴露问题。

## doc test 的价值

文档里的代码如果能被测试，就不容易过期。对库 API 来说，doc test 是最小使用示例，也是合同。

## 示例走读

```text
book/chapters/18-macros-features-docs/example/src/lib.rs
```

`assert_close!` 是宏示例，`mean` 的文档注释包含可运行代码，`backend_name()` 展示 feature flag 如何影响编译结果。

## 常见错误

错误 1：用宏隐藏普通函数能做的事。

宏更难调试，应该谨慎使用。

错误 2：feature 没有测试。

如果有 `--features simd`，就应该至少有命令验证它能编译。

错误 3：文档只写概念，不给可运行例子。

库文档应该让调用者快速复制最小示例。

## 代码走读与操作清单

看宏：

```rust
macro_rules! assert_close
```

宏内部把输入表达式绑定到本地变量。这避免表达式被求值多次，也让错误信息更清楚。

看文档测试：

```rust
/// ```
/// let mean = ch18_macros_features_docs::mean(&[1.0, 2.0, 3.0]);
/// ```
```

这段代码会被 `cargo test` 编译和运行。文档不再只是文字，而是测试的一部分。

看 feature：

```rust
if cfg!(feature = "simd")
```

这是编译期配置检查。运行：

```bash
cargo test -p ch18-macros-features-docs --features simd
```

可以验证 feature 组合能编译。

操作清单：

1. 给 `assert_close!` 增加自定义消息参数。
2. 增加 `parallel` feature。
3. 写一个 doc test 展示空输入返回 `None`。
4. 跑默认测试和 feature 测试。

专业目标：API 文档、feature 和测试要同步演进。
