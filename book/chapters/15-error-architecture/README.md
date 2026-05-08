# 第 15 章：error architecture 和库 API 设计

入门时会写 `Result<T, String>`，专业库不能这样。错误类型是 API 的一部分，它决定调用者能否匹配错误、能否保留上下文、能否稳定演进。

## 错误不是日志

日志给人看，错误类型给程序处理。一个好的错误类型应该能回答：

- 哪个输入错了？
- 错误属于解析、数据质量、计算还是系统资源？
- 调用者能否恢复？
- 是否应该重试？

```rust
pub enum ParsePriceError {
    EmptyField,
    InvalidFloat(ParseFloatError),
    NonPositive(f64),
}
```

这比 `"bad price"` 有用得多。

## `From` 和 `?`

`?` 会尝试把下层错误转换成当前函数的错误类型。转换来自 `From`：

```rust
impl From<ParseFloatError> for ParsePriceError
```

这允许你保留简洁控制流，同时不丢失错误结构。

## 库错误和应用错误

库应该返回具体错误：

```rust
Result<Vec<f64>, ComputeError>
```

应用层可以把多个库错误合并成 `AppError`，或者在 CLI 里最终转成 `Box<dyn Error>` 打印。不要让底层 kernel 直接依赖应用层错误。

## 错误稳定性

公开 enum 的 variant 也是 API。生产库如果要允许未来加 variant，可以使用 `#[non_exhaustive]`。学习阶段先不使用，但要知道：错误类型一旦公开，就会被调用方 match。

## 本章示例

```bash
cargo test -p ch15-error-architecture
```

重点看：

- `ParsePriceError` 如何保留错误种类。
- `From<ParseFloatError>` 如何支持 `?`。
- `type Result<T>` 如何减少重复。

## 本章练习

1. 增加 `NonFinite(f64)` 错误。
2. 增加 `parse_prices(lines: &[&str])`。
3. 给每类错误写测试。
4. 写一个上层 `DataError`，用 `From<ParsePriceError>` 包装下层错误。

## 本章验收

你应该能解释：

- 为什么 `String` 不是专业库的好错误类型。
- `From` 和 `?` 如何协作。
- 库错误和应用错误应该如何分层。
- 错误 enum 的公开 variant 为什么是 API 承诺。

## 教材化补充：错误类型要服务调用者

错误类型不是为了让当前函数写起来方便，而是为了让调用者能做正确决策。

例如价格解析失败，调用者可能需要区分：

- 字段为空：数据缺失。
- 无法解析浮点数：格式错误。
- 非正价格：数据质量错误。

如果你只返回 `"bad price"`，上层无法判断是否跳过、报警、重试或停止任务。

## 错误分层

底层函数返回具体错误：

```rust
ParsePriceError
```

数据读取模块可以包装成：

```rust
DataError::InvalidPrice(ParsePriceError)
```

应用入口最终可以把错误打印出来。不要让底层数值 kernel 直接依赖 CLI 或日志系统。

## `From` 的专业用途

`From` 让下层错误自然进入上层错误：

```rust
let value = text.parse::<f64>()?;
```

如果当前函数返回 `ParsePriceError`，而 `ParseFloatError` 可以转成 `ParsePriceError`，`?` 就能工作。

## 示例走读

```text
book/chapters/15-error-architecture/example/src/lib.rs
```

重点看 `ParsePriceError::InvalidFloat(ParseFloatError)`。它没有丢掉底层错误，而是把底层错误作为上下文保存起来。

## 常见错误

错误 1：`Result<T, String>`。

可以作为临时原型，但不适合专业库。调用者只能读字符串，不能稳定 match。

错误 2：过早使用 `Box<dyn Error>`。

应用入口可以这样做，库 API 不建议这样做。库应该保留具体错误类型。

错误 3：错误信息没有上下文。

处理文件或数据行时，错误应该尽量包含字段、行号、symbol、timestamp 等定位信息。

## 代码走读与操作清单

看错误类型：

```rust
pub enum ParsePriceError {
    EmptyField,
    InvalidFloat(ParseFloatError),
    NonPositive(f64),
}
```

这三个 variant 分别对应三种处理方式。空字段可能是数据缺失，InvalidFloat 可能是文件格式问题，NonPositive 可能是市场数据质量问题。

看 Display：

```rust
impl fmt::Display for ParsePriceError
```

Display 面向人类可读信息。Error trait 让它能进入 Rust 标准错误生态。

看 From：

```rust
impl From<ParseFloatError> for ParsePriceError
```

这让 `text.parse::<f64>()?` 可以自动把底层解析错误转成当前函数错误。

操作清单：

1. 增加 `NonFinite(f64)`。
2. 在 `parse_price` 中拒绝 NaN 和 infinity。
3. 为每个错误 variant 写测试。
4. 写一个上层 `DataError`，包装 `ParsePriceError` 并附加行号。

专业目标不是“错误能打印”，而是“错误能指导系统决策”。

## 自测与复盘问题

1. 为什么 `Result<T, String>` 不适合长期维护的库 API？
2. `Display` 和 `Error` trait 分别服务什么？
3. `From<LowerError>` 如何让 `?` 自动工作？
4. 库错误和应用错误为什么要分层？
5. 一个市场数据错误至少应该包含哪些定位上下文？

如果这些问题回答不出来，重新设计一次 `ParsePriceError`，并为每个 variant 写测试。
