# 第 3 章：`Vec`、slice、`String` 和 `&str`

本章是 Rust 数值计算的关键基础。你要理解拥有数据和借用数据的差异。

在 Python 中，你经常写：

```python
prices = [100.0, 101.0, 99.0]
```

函数可以直接接收这个 list：

```python
def mean(values):
    return sum(values) / len(values)
```

Rust 里你需要更明确地表达：函数是否拥有这组数据，是否只读，是否会修改。

## `Vec<f64>` 是拥有数据的动态数组

```rust
let prices = vec![100.0, 101.0, 99.0];
```

`prices` 是一个 `Vec<f64>`。它拥有这组三个浮点数。所谓拥有，意思是当 `prices` 离开作用域时，这段内存会被释放。

`Vec` 适合表达“我创建并拥有这组数据”。比如函数计算收益率，需要创建一个新的输出：

```rust
pub fn returns(prices: &[f64]) -> Result<Vec<f64>, NumericError> {
    // create and return a new Vec
}
```

输入是借用，输出是新拥有的数据。

## `&[f64]` 是只读视图

`&[f64]` 叫 slice。你可以把它理解成“一段连续 `f64` 数据的只读视图”。

为什么数值函数常用它？

```rust
pub fn mean(values: &[f64]) -> Option<f64>
```

因为这个函数只是读取输入，不需要拥有输入。这样调用方可以继续使用原始数据。

```rust
let prices = vec![100.0, 101.0, 99.0];
let avg = mean(&prices);
println!("{prices:?}");
```

`&prices` 表示借用。函数用完后，`prices` 仍然可用。

## 为什么不要随便接收 `Vec<f64>`

下面这个签名不好：

```rust
fn mean(values: Vec<f64>) -> Option<f64>
```

它表示函数拿走整个向量的所有权。调用后，原始变量不能再使用，除非你 clone。

```rust
let prices = vec![100.0, 101.0, 99.0];
let avg = mean(prices);
println!("{prices:?}"); // 这里会报错
```

很多新手看到报错后会写：

```rust
let avg = mean(prices.clone());
```

这能编译，但会拷贝整段数据。对几百万行价格数据来说，这就是性能问题。

高性能 Rust API 的基本原则：

- 只读输入用 `&[T]`。
- 需要修改输入用 `&mut [T]`。
- 需要创建输出才返回 `Vec<T>`。

## `String` 和 `&str`

数值计算里也会有字符串，例如资产代码、字段名、错误信息。

`String` 是拥有文本数据的类型：

```rust
let symbol = String::from("AAPL");
```

`&str` 是字符串切片，通常表示借用的文本：

```rust
fn print_symbol(symbol: &str) {
    println!("{symbol}");
}
```

函数如果只是读取字符串，优先接收 `&str`，这样既能接收 `String`，也能接收字符串字面量。

```rust
let symbol = String::from("AAPL");
print_symbol(&symbol);
print_symbol("MSFT");
```

## `windows`

Rust slice 提供一个非常适合 rolling 计算的方法：

```rust
for window in values.windows(3) {
    println!("{window:?}");
}
```

如果输入是 `[1.0, 2.0, 3.0, 4.0]`，窗口大小是 3，会产生：

```text
[1.0, 2.0, 3.0]
[2.0, 3.0, 4.0]
```

朴素 rolling mean 可以这样写：

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

这个版本不是最快的，但非常适合学习：

- 输入不拷贝。
- 每个窗口是 slice。
- 输出是新 `Vec<f64>`。
- 错误用 `Result`。

## 本章练习

1. 解释 `Vec<f64>` 和 `&[f64]` 的区别。
2. 修改 `returns`，让输入价格少于 2 个时返回空 `Vec`。
3. 用 `windows(2)` 实现简单收益率。
4. 写一个函数 `last(values: &[f64]) -> Option<f64>`。
5. 写测试证明调用函数后原始 `Vec` 仍然能使用。

## 本章验收

你可以进入下一章，如果你能回答：

- 为什么 `mean(values: &[f64])` 比 `mean(values: Vec<f64>)` 更适合作为库 API？
- `String` 和 `&str` 的关系是什么？
- rolling window 为什么可以用 slice 表达？

## 教材化补充：拥有数据和看一眼数据

`Vec<f64>` 拥有一段连续的 `f64` 数据。`&[f64]` 不拥有数据，只是看一段连续数据。这个区别是 Rust 高性能 API 的核心。

Python/NumPy 中，你经常传一个 array 给函数，函数通常不会把 array “吃掉”。Rust 也应该这样设计计算函数：

```rust
fn mean(values: &[f64]) -> Option<f64>
```

这表示函数只是借用数据。调用结束后，原始数据仍然能继续使用。

## slice 为什么适合 rolling

rolling window 本质上是原始序列中的连续视图。例如：

```text
[1, 2, 3, 4], window = 3
窗口 1: [1, 2, 3]
窗口 2: [2, 3, 4]
```

这些窗口不需要复制成新的 `Vec`。Rust 的 `windows` 会产生 slice：

```rust
values.windows(window)
```

每个窗口都是 `&[f64]`，只是借用原始数据的一段。

## `String` 和 `&str`

`String` 拥有文本，`&str` 借用文本。

量化系统里 symbol、field name、factor name 到处出现。如果函数只是读取名字，应该接收 `&str`：

```rust
fn symbol_label(symbol: &str, field: &str) -> String
```

输出是新的字符串，因为 `format!` 会创建新文本。

## 动手操作

运行：

```bash
cargo test -p ch03-vec-slice-string
```

打开：

```text
book/examples/ch03-vec-slice-string/src/lib.rs
```

重点改两个函数：

- 给 `window` 增加 `len == 0` 的行为测试。
- 给 `returns` 增加价格长度小于 2 的测试。

## 常见错误

错误 1：为了“方便”到处 `.to_vec()`。

这会复制数据。小数据没问题，大规模因子矩阵会浪费内存和时间。

错误 2：函数接收 `String` 而不是 `&str`。

如果函数只读文本，接收 `String` 会强迫调用者交出所有权。

错误 3：返回局部 `Vec` 的 slice。

不能返回指向函数内部临时 `Vec` 的引用，因为函数结束后 `Vec` 被释放。第 11 章会系统讲 lifetime。
