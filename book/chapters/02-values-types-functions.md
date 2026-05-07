# 第 2 章：值、类型和函数

本章开始写真正的 Rust 代码。目标是理解 Rust 如何表达数值、变量、函数和返回值。

量化计算最常见的数据是数字。你会大量使用：

- `f64`：浮点数，价格、收益率、因子值。
- `usize`：长度、索引、窗口大小。
- `Vec<f64>`：一组浮点数。
- `&[f64]`：一组浮点数的只读视图。

本章先关注 `f64`、`usize` 和函数。

## 变量默认不可变

Rust 中：

```rust
let score = 78.5;
```

这个变量默认不能修改。如果要修改，必须写：

```rust
let mut score = 78.5;
score += 5.0;
```

Python 中变量名随时可以重新绑定：

```python
score = 78.5
score += 5.0
```

Rust 的默认不可变不是为了麻烦你，而是为了让数据流更清楚。高性能量化代码里，一个数组到底是原始价格、收益率，还是标准化后的因子，必须清楚。随意修改会让测试和并行变难。

## 类型不是负担，是契约

Rust 可以推断类型：

```rust
let price = 101.5;
```

但函数签名必须写清楚：

```rust
fn add_one(value: f64) -> f64 {
    value + 1.0
}
```

这表示：

- 输入是一个 `f64`。
- 输出也是一个 `f64`。
- 函数不会返回字符串、空值或任意对象。

在量化系统里，这种明确性很有价值。你希望 `rolling_mean` 永远返回数值序列或明确错误，而不是有时返回 list、有时返回 None、有时抛异常。

## 表达式和返回值

Rust 函数最后一行如果没有分号，就是返回值：

```rust
fn square(x: f64) -> f64 {
    x * x
}
```

下面这样也可以，但更啰嗦：

```rust
fn square(x: f64) -> f64 {
    return x * x;
}
```

新手常犯错误是多写分号：

```rust
fn square(x: f64) -> f64 {
    x * x;
}
```

这会返回 `()`，也就是 unit 类型，而不是 `f64`。编译器会报类型不匹配。

## `usize` 为什么重要

窗口大小、数组长度、索引通常用 `usize`：

```rust
fn has_enough_data(len: usize, window: usize) -> bool {
    len >= window
}
```

Python 中索引一般就是 int。Rust 区分整数类型，是为了让不同用途更明确。`usize` 的大小和机器指针宽度相关，适合表示内存中对象的长度和索引。

## 第一个数值函数

平均值函数在 Python 中可能这样写：

```python
def mean(values):
    return sum(values) / len(values)
```

但空数组怎么办？

```python
mean([])
```

Python 会在运行时报 `ZeroDivisionError`。Rust 中我们更愿意把“可能没有结果”写进类型：

```rust
pub fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    Some(values.iter().sum::<f64>() / values.len() as f64)
}
```

先不用急着理解 `&[f64]` 和 `Option` 的全部细节。你先记住：

- `values` 是一组 `f64` 的只读输入。
- 空输入没有平均值，所以返回 `None`。
- 非空输入返回 `Some(mean)`。

这个函数在：

```text
book/examples/ch02-values-types-functions/src/lib.rs
```

运行：

```bash
cargo test -p ch02-values-types-functions
```

## 为什么不是返回 0.0

很多新手会写：

```rust
fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}
```

这在量化计算中很危险。空数组的均值不是 0。返回 0 会制造假数据。Rust 的 `Option` 让调用方必须面对“没有结果”这个事实。

## 本章练习

在 `book/examples/ch02-numeric-functions` 中完成或修改：

1. `mean(&[1.0, 2.0, 3.0])` 应该返回 `Some(2.0)`。
2. `mean(&[])` 应该返回 `None`。
3. 新增 `min(values: &[f64]) -> Option<f64>`。
4. 新增 `max(values: &[f64]) -> Option<f64>`。
5. 给 `min` 和 `max` 写测试。

## 本章验收

你可以进入下一章，如果你能回答：

- 为什么 Rust 变量默认不可变？
- 函数最后一行没有分号是什么意思？
- 为什么空数组的 mean 不应该返回 0？
- `usize` 适合表达什么？

## 教材化补充：类型是计算合同

Python 里函数签名经常只是名字：

```python
def mean(values):
    return sum(values) / len(values)
```

这段代码没有告诉调用者 `values` 应该是什么，也没有说明空输入怎么办。Rust 的函数签名更像合同：

```rust
pub fn mean(values: &[f64]) -> Option<f64>
```

这句话包含很多信息：

- `values` 是一段借来的 `f64` 数据。
- 函数不拥有输入。
- 函数不会修改输入。
- 输出可能没有值，所以是 `Option<f64>`。

一个专业 Rust 开发者会先看签名，再看实现。签名设计错了，后面代码再漂亮也会难用。

## 为什么变量默认不可变

数据科学代码里，经常会反复覆盖变量名。Rust 默认不可变，是为了减少“我以为这个值没变，但其实中途被改了”的问题。

在计算代码中，不可变有两个好处：

- 更容易推理结果。
- 编译器更容易优化。

需要修改时显式写 `mut`：

```rust
let mut total = 0.0;
```

这等于告诉读者：这个变量后面会变化。

## 函数返回值和分号

Rust 中没有分号的最后一行是表达式返回值：

```rust
fn add_one(x: f64) -> f64 {
    x + 1.0
}
```

如果写成：

```rust
fn add_one(x: f64) -> f64 {
    x + 1.0;
}
```

最后一行变成语句，返回 `()`，编译器会报错。这个规则一开始别扭，但它让函数体更像表达式组合。

## 动手操作

运行：

```bash
cargo test -p ch02-values-types-functions
```

打开：

```text
book/examples/ch02-values-types-functions/src/lib.rs
```

按顺序读：

1. `NumericError` 表达哪些失败。
2. `mean` 为什么返回 `Option`。
3. `returns` 为什么检查非正价格。
4. `rolling_mean` 为什么窗口为 0 是错误。

## 常见错误

错误 1：空输入返回 `0.0`。

这会混淆“真实均值是 0”和“没有均值”。专业数值库不能这样设计。

错误 2：所有数字都用 `f64`。

长度、索引、窗口大小应该用 `usize`，因为它们描述内存中的位置和数量，不是小数。

错误 3：函数签名只图方便。

如果输入用 `Vec<f64>`，调用者必须把数据所有权交给函数。对库 API 来说，这通常过于强势。
