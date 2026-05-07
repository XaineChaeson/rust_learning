# 第 20 章：内存布局、cache 和 allocation

高性能数值计算很多时候不是 CPU 不够强，而是数据没有按 CPU 喜欢的方式摆放。Rust 让你显式控制数据布局，这是它适合写计算核心的重要原因。

## 连续内存

`Vec<f64>` 是连续内存。CPU 顺序读取时，cache line 会提前加载后续元素。

`Vec<Vec<f64>>` 不是一个连续矩阵。外层 `Vec` 连续保存的是每一行的指针、长度、capacity；每一行的数据各自分配。遍历时会产生更多间接访问。

高性能矩阵通常用扁平布局：

```rust
data[row * cols + col]
```

## row-major 和 column-major

row-major 中，同一行连续。同一列相邻元素间隔 `cols`。

如果你按行遍历 row-major，内存访问连续；按列遍历时会跳跃。跳跃访问会降低 cache locality。

这不只是理论。因子计算中按资产、按时间、按字段的布局选择，会影响整个系统吞吐。

## SoA 和 AoS

AoS：array of structs

```rust
Vec<Bar { open, high, low, close }>
```

SoA：struct of arrays

```rust
struct Bars {
    open: Vec<f64>,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
}
```

如果你经常只处理 `close`，SoA 更适合列式分析。Arrow、Parquet、Polars 都偏向列式。

## allocation 策略

分配不是免费的。热路径策略：

- 预分配 capacity。
- 输出 buffer 由调用者传入。
- 大对象复用。
- 避免中间 `collect`。
- 批量处理减少调用开销。

## 本章示例

```bash
cargo test -p ch20-memory-layout-cache
```

重点看：

- `RowMajorMatrix` 的索引公式。
- row sum 和 col sum 的访问模式。
- `BarsSoa` 如何表达列式数据。

## 本章练习

1. 给 `RowMajorMatrix` 增加 `get(row, col)`。
2. 写 `all_row_sums()` 和 `all_col_sums()`。
3. 给 `BarsSoa` 增加 `returns_from_close()`。
4. 写复盘：如果因子只用 close，为什么 SoA 更自然？

## 本章验收

你应该能解释：

- `Vec<Vec<f64>>` 为什么不是高性能矩阵。
- row-major 的索引公式。
- SoA/AoS 的取舍。
- allocation 为什么会成为性能瓶颈。

## 教材化补充：CPU 等数据比算数据更常见

现代 CPU 很快，但内存访问相对慢。很多数值程序不是“乘法太慢”，而是 CPU 一直在等数据从内存过来。

cache line 会一次加载一小段连续内存。如果你的访问模式是连续的，CPU 能提前加载后面的数据。如果访问模式跳来跳去，cache 命中率会下降。

这就是为什么同样的矩阵，按行遍历和按列遍历可能性能差很多。

## Python/NumPy 对照

NumPy array 有 dtype、shape、stride。你平时可能不看 stride，但它决定视图如何映射到底层内存。

Rust 里我们先手写 row-major matrix，是为了看清楚：

```text
index = row * cols + col
```

理解这个公式后，你再看 ndarray/Arrow/Polars 会容易得多。

## SoA 对因子计算的意义

如果你要计算 close-to-close return，只需要 close 列。AoS 结构会把 open/high/low/close 混在一起；SoA 让 close 单独连续存放。

这减少了无用字段读取，也更适合 SIMD 和列式执行。

## 示例走读

```text
book/examples/ch20-memory-layout-cache/src/lib.rs
```

`RowMajorMatrix` 展示扁平矩阵布局。`BarsSoa` 展示按列保存 OHLC 数据。

## 常见错误

错误 1：以为 `Vec<Vec<f64>>` 就是矩阵。

它是很多独立 Vec 的集合，不是单块连续二维数组。

错误 2：忽略 capacity。

反复 push 超过 capacity 会重新分配和复制。

错误 3：只关注算法复杂度。

两个 `O(n)` 算法可能因为内存布局不同，性能差很多。

## 代码走读与实验设计

看矩阵构造：

```rust
RowMajorMatrix::new(rows, cols, data)
```

`data` 是扁平 `Vec<f64>`。构造时必须检查 `rows * cols == data.len()`，否则 shape 和内存不一致。

看 row sum：

```rust
let start = row * self.cols;
self.data[start..start + self.cols].iter().sum()
```

这是连续访问，cache 友好。

看 col sum：

```rust
(0..self.rows).map(|row| self.data[row * self.cols + col]).sum()
```

每次跳过 `cols` 个元素，可能更不 cache friendly。

实验设计：

1. 构造大矩阵，例如 `10_000 x 100`。
2. 分别计算所有 row sum 和所有 col sum。
3. 用 release build 测时间。
4. 写下访问模式如何影响结果。

这个实验的目标不是记住哪个更快，而是建立“访问顺序也是性能设计”的直觉。

## 自测与复盘问题

1. `Vec<Vec<f64>>` 和扁平 `Vec<f64>` 的内存差异是什么？
2. row-major 中 `row * cols + col` 为什么成立？
3. 按列遍历 row-major 为什么可能慢？
4. SoA 为什么适合只读取 close 列的因子？
5. allocation 为什么会影响热路径延迟？

如果这些问题回答不出来，手画一个 `2 x 3` 矩阵到底层数组的映射。

## 进入下一章前

确认你能解释一段连续 `Vec<f64>` 如何表达矩阵，以及为什么访问顺序会影响 cache。你还应该能说明 SoA 为什么适合列式因子计算。做到这些，再进入数值 kernel。
