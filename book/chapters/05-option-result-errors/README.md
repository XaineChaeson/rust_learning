# 第 5 章：`Option`、`Result` 和错误处理

量化数据经常有异常：空输入、非法窗口、非正价格、NaN、长度不一致。专业系统必须明确处理这些情况。

Rust 不鼓励把可恢复错误藏在 panic 里，也不鼓励用魔法值表示错误。它提供两个核心类型：

- `Option<T>`：可能有值，也可能没有。
- `Result<T, E>`：可能成功，也可能失败。

## `Option`

空数组的平均值不存在。这个场景适合 `Option`：

```rust
pub fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    Some(values.iter().sum::<f64>() / values.len() as f64)
}
```

调用方必须处理：

```rust
match mean(&values) {
    Some(value) => println!("mean: {value}"),
    None => println!("empty input"),
}
```

Python 中你可能返回 `None`，但调用方不一定检查。Rust 的类型会提醒你：这里不是总有值。

## `Result`

收益率计算中，价格必须为正：

```rust
pub fn returns(prices: &[f64]) -> Result<Vec<f64>, NumericError>
```

这表示：

- 成功：返回 `Vec<f64>`。
- 失败：返回 `NumericError`。

错误类型：

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum NumericError {
    EmptyInput,
    InvalidWindow,
    NonPositivePrice,
}
```

这比返回字符串更好，因为调用方可以根据错误类型做不同处理。

## `match`

处理 `Result`：

```rust
match returns(&prices) {
    Ok(values) => println!("{values:?}"),
    Err(error) => println!("failed: {error:?}"),
}
```

`Ok` 是成功分支，`Err` 是失败分支。

## `?`

当一个函数也返回 `Result` 时，可以用 `?` 传播错误：

```rust
fn compute_pipeline(prices: &[f64]) -> Result<Vec<f64>, NumericError> {
    let returns = returns(prices)?;
    rolling_mean(&returns, 20)
}
```

如果 `returns(prices)` 失败，整个 `compute_pipeline` 直接返回同样的错误。如果成功，就取出里面的值。

## 不要滥用 `unwrap`

`unwrap` 的意思是：如果失败就 panic。

```rust
let values = returns(&prices).unwrap();
```

学习阶段可以偶尔在示例里用，但库代码不要这样处理可恢复错误。量化系统遇到脏数据不应该直接崩溃，而应该把错误表达出来。

## 什么时候用 `Option`，什么时候用 `Result`

用 `Option`：

- 没有值是正常情况。
- 不需要解释太多原因。

例子：空数组没有均值。

用 `Result`：

- 失败有原因。
- 调用方需要知道为什么失败。
- 失败可能影响后续流程。

例子：窗口为 0、价格非正、长度不一致。

## Python 对照

Python 中你可能写：

```python
def rolling_mean(values, window):
    if window == 0:
        raise ValueError("window must be positive")
```

Rust 中错误是签名的一部分：

```rust
pub fn rolling_mean(values: &[f64], window: usize) -> Result<Vec<f64>, NumericError>
```

调用方一眼就知道它可能失败。

## 本章练习

在 `book/chapters/05-option-result-errors/example/src/lib.rs` 中：

1. 给 `returns` 增加非有限数检查。
2. 给 `rolling_mean` 增加空输入测试。
3. 写 `fn first(values: &[f64]) -> Option<f64>`。
4. 写 `fn checked_window(values: &[f64], window: usize) -> Result<(), NumericError>`。
5. 不使用 `unwrap` 处理一个 `Result`。

## 本章验收

你可以进入下一章，如果你能回答：

- `Option` 和 `Result` 的区别是什么？
- 为什么空数组 mean 用 `Option` 合理？
- 为什么非法窗口用 `Result` 合理？
- `?` 做了什么？
- 为什么库代码不应该随便 `unwrap`？

## 教材化补充：失败也是 API 的一部分

Python 里你可能习惯直接抛异常，或者返回 `None`，或者返回 `np.nan`。这些方式在探索性分析里方便，但在基础设施里容易让错误流向很远的地方才暴露。

Rust 要求你在函数签名里表达失败：

```rust
fn mean(values: &[f64]) -> Option<f64>
fn rolling_mean(values: &[f64], window: usize) -> Result<Vec<f64>, RollingError>
```

`Option` 表示“可能没有值，但不是错误细节”。`Result` 表示“可能失败，并且失败有原因”。

## 什么时候用 Option

适合 `Option`：

- 空数组没有 mean。
- 查找某个 key 可能不存在。
- slice 最后一个元素可能不存在。

这些情况通常不需要错误消息。调用者只需要知道“有”或“没有”。

## 什么时候用 Result

适合 `Result`：

- CSV 字段解析失败。
- 窗口大小非法。
- 输入包含 NaN。
- 两个数组长度不一致。

这些情况需要告诉调用者原因。尤其是量化系统中，数据质量问题必须能定位。

## `?` 的意义

`?` 不是“忽略错误”。它的含义是：如果成功，取出值；如果失败，立刻把错误返回给调用者。

这让错误传播保持简洁，同时不丢失类型。

## 动手操作

运行：

```bash
cargo test -p ch05-option-result-errors
```

打开：

```text
book/chapters/05-option-result-errors/example/src/lib.rs
```

重点看：

- `QuantError` 的每个 variant 表达什么。
- `validate_finite` 如何返回第一个非法值的位置。
- 测试为什么不能直接比较 `NaN == NaN`。

## 常见错误

错误 1：库代码里 `unwrap()`。

`unwrap()` 适合测试或原型，不适合库热路径。库应该把错误交给调用者处理。

错误 2：所有错误都变成字符串。

字符串给人看，不适合程序匹配。第 15 章会讲完整错误架构。

错误 3：用 `NaN` 表示所有失败。

NaN 会传播，且不同 NaN 的语义不清楚。数据缺失、窗口非法、计算无定义应该分开表达。
