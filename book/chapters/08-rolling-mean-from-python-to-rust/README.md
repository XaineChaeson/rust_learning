# 第 8 章：从 Python 到 Rust 实现 rolling mean

这一章把前面的知识合并起来。目标是完整理解这个函数：

```rust
pub fn rolling_mean(values: &[f64], window: usize) -> Result<Vec<f64>, NumericError>
```

它是后续 rolling 因子引擎的第一块砖。

## Python 写法

你可能熟悉 pandas：

```python
series.rolling(window=3).mean()
```

或者 numpy 手写：

```python
def rolling_mean(values, window):
    out = []
    for i in range(len(values) - window + 1):
        out.append(sum(values[i:i + window]) / window)
    return out
```

这个写法隐藏了几个问题：

- `window == 0` 怎么办？
- 空输入怎么办？
- 输出如何对齐？
- 每次 `values[i:i+window]` 是否拷贝？
- 时间复杂度是多少？

Rust 版本要求你显式回答。

## 设计函数签名

我们选择：

```rust
pub fn rolling_mean(values: &[f64], window: usize) -> Result<Vec<f64>, NumericError>
```

逐项解释：

`values: &[f64]`：只读输入，不拷贝价格序列。

`window: usize`：窗口是长度和索引概念，用 `usize`。

`Result<_, NumericError>`：窗口为 0 或空输入需要明确表达。

`Vec<f64>`：输出是新创建的均值序列。

## 错误规则

我们先定义规则：

1. `window == 0` 是错误。
2. `values.is_empty()` 是错误。
3. `window > values.len()` 返回空输出。

第三条是设计选择。也可以把它当成错误，但本教材第一版选择返回空输出，因为没有任何完整窗口。

## 实现

代码在：

```text
book/chapters/08-rolling-mean-from-python-to-rust/example/src/lib.rs
```

核心实现：

```rust
pub fn rolling_mean(values: &[f64], window: usize) -> Result<Vec<f64>, NumericError> {
    if window == 0 {
        return Err(NumericError::InvalidWindow);
    }

    if values.is_empty() {
        return Err(NumericError::EmptyInput);
    }

    if window > values.len() {
        return Ok(Vec::new());
    }

    Ok(values
        .windows(window)
        .map(|slice| slice.iter().sum::<f64>() / window as f64)
        .collect())
}
```

## 一行一行理解

```rust
if window == 0 {
    return Err(NumericError::InvalidWindow);
}
```

窗口为 0 没有数学意义。这里不用 panic，因为这是调用方可能传错参数的可恢复错误。

```rust
if values.is_empty() {
    return Err(NumericError::EmptyInput);
}
```

空输入没有任何窗口。

```rust
if window > values.len() {
    return Ok(Vec::new());
}
```

输入不够形成窗口，返回空结果。

```rust
values.windows(window)
```

产生每个连续窗口，每个窗口都是 `&[f64]`，不是新 `Vec`。

```rust
.map(|slice| slice.iter().sum::<f64>() / window as f64)
```

对每个窗口求均值。

```rust
.collect()
```

把迭代器收集成 `Vec<f64>`。

最外面的 `Ok(...)` 表示成功。

## 时间复杂度

这个实现对每个窗口都重新求和。长度为 `n`、窗口为 `w`，复杂度是 `O(n * w)`。

这不是最终高性能版本，但它是正确、清楚、适合学习的 baseline。

后续你会写增量版本：

```text
新窗口 sum = 旧窗口 sum - 移出的值 + 进入的值
```

这样复杂度可以降到 `O(n)`。

## 测试

核心测试：

```rust
#[test]
fn rolling_mean_computes_right_aligned_windows() {
    let output = rolling_mean(&[1.0, 2.0, 3.0, 4.0], 2).expect("window is valid");

    assert_eq!(output, vec![1.5, 2.5, 3.5]);
}
```

手算：

```text
[1, 2] -> 1.5
[2, 3] -> 2.5
[3, 4] -> 3.5
```

这个测试说明了输出规则。

## 常见错误

### 错误 1：返回 0 代表错误

不要这样：

```rust
if window == 0 {
    return Ok(vec![0.0]);
}
```

这是制造假数据。

### 错误 2：输入接收 `Vec<f64>`

不要这样：

```rust
fn rolling_mean(values: Vec<f64>, window: usize) -> Result<Vec<f64>, NumericError>
```

这会拿走输入所有权。只读计算不应该这样设计。

### 错误 3：过早优化

第一版不要急着写 unsafe、SIMD、并行。你需要先有正确 baseline 和测试。

## 本章练习

1. 跑测试：

```bash
cargo test -p ch08-rolling-mean
```

2. 给 `rolling_mean` 增加 `window > values.len()` 的测试。
3. 实现 `rolling_sum`。
4. 用 `rolling_sum` 帮助理解增量 rolling 的思想。
5. 写一段复盘：当前实现为什么不是最高性能，但为什么仍然有价值？

## 本章验收

你可以进入 rolling 因子项目准备阶段，如果你能回答：

- 为什么输入是 `&[f64]`？
- 为什么输出是 `Vec<f64>`？
- 为什么返回 `Result`？
- 当前实现复杂度是多少？
- 增量版本如何降低复杂度？

## 教材化补充：这是第一条完整的 Rust 数值路径

第 8 章是前面知识的汇合点。你不再只是学语法，而是在设计一个真正的数值函数：

```rust
pub fn rolling_mean_incremental(values: &[f64], window: usize) -> Result<Vec<f64>, RollingError>
```

这一个签名同时用到了：

- slice：借用输入，不复制。
- `usize`：窗口大小是索引/长度概念。
- `Result`：非法输入有明确错误。
- `Vec<f64>`：输出是新生成的一列结果。
- 自定义错误：调用者能知道失败原因。

## Python 对照

Python/pandas 中你可能写：

```python
series.rolling(window).mean()
```

这很方便，但隐藏了很多决定：

- 窗口对齐在左边、右边还是居中？
- 窗口不完整时输出什么？
- NaN 如何处理？
- 输入为空怎么办？
- 输出长度是多少？

Rust 版本要求你把这些规则写进函数、错误类型和测试里。

## 复杂度直觉

朴素 rolling mean 每个窗口都求和。如果输入长度是 `n`，窗口是 `w`，复杂度约为 `O(n * w)`。

增量版本维护一个 running sum：

```text
新 sum = 旧 sum + 新进入窗口的值 - 离开窗口的值
```

这样每一步是 `O(1)`，整体是 `O(n)`。

这就是高性能计算最重要的优化类型：先改变算法复杂度，而不是先碰 unsafe。

## 动手操作

运行：

```bash
cargo test -p ch08-rolling-mean
```

打开：

```text
book/chapters/08-rolling-mean-from-python-to-rust/example/src/lib.rs
```

按这个顺序理解代码：

1. `RollingError` 定义失败类型。
2. `validate` 把输入检查集中在一个地方。
3. `window > values.len()` 返回空输出。
4. 第一窗口先完整求和。
5. 后续窗口用增量更新。

## 常见错误

错误 1：窗口为 0 时返回空 Vec。

窗口为 0 是调用错误，不是合法空结果。

错误 2：窗口大于长度时报错。

这可以设计成错误，也可以设计成空结果。本课程选择空结果，因为“没有完整窗口”是正常业务情况。

错误 3：没有写对齐测试。

rolling 最常见 bug 是输出和输入时间戳错位。测试必须明确第一个输出对应哪个窗口。

## 和后续章节的关系

后面所有专业内容都会回到这个函数：

- 第 10 章用 trait 抽象不同 factor。
- 第 19 章 benchmark 朴素版和增量版。
- 第 20 章分析内存布局。
- 第 22 章按资产并行。
- 第 23 章讨论是否值得 unsafe/SIMD。
- 第 24 章把它暴露给 Python。
