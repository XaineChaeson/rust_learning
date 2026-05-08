# 第 21 章：小型 ndarray 和 numerical kernel

理解 ndarray 的目标不是重新发明 NumPy，而是掌握数值库的基本抽象：shape、stride、view、contiguous layout、reduction。

## shape 和 stride

shape 描述逻辑维度，stride 描述沿某个维度移动一步时，底层 index 增加多少。

二维 row-major 矩阵：

```text
index = row * cols + col
```

此时 row stride 是 `cols`，col stride 是 `1`。

转置 view 不一定需要复制数据。它可以交换 shape 和 stride。但这要求 view 保存借用和 stride 信息。

## view

view 是 borrowed API：

```rust
struct MatrixView<'a> {
    data: &'a [f64],
    rows: usize,
    cols: usize,
    row_stride: usize,
    col_stride: usize,
}
```

这类 API 同时依赖第 11 章 lifetime 和第 20 章内存布局。专业数值库设计基本都绕不开它。

## reduction

sum、mean、min、max、dot 都是 reduction。高性能 reduction 的关键问题：

- 是否连续访问？
- 是否可以分片并行？
- 是否需要处理 NaN？
- 浮点加法是否允许重排？
- 是否需要稳定求和算法？

并行 reduction 改变加法顺序，结果可能有极小浮点差异。这不是 bug，但必须在测试容差里考虑。

## 本章示例

```bash
cargo test -p ch21-numerical-kernel
```

重点看：

- `Matrix` 如何用扁平 `Vec<f64>` 表达二维矩阵。
- `get(row, col)` 如何检查边界。

## 本章练习

1. 增加 `rows()` 和 `cols()`。
2. 增加 `row(row) -> Result<&[f64], MatrixError>`。
3. 增加 `transpose_copy()`。
4. 写复盘：view 和 copy 的 API 应该如何命名，避免误导调用者？

## 本章验收

你应该能解释：

- shape 和 stride 的区别。
- view 为什么需要 lifetime。
- reduction 为什么会遇到浮点误差。
- 什么时候应该使用成熟 `ndarray`，什么时候只写专用 kernel。

## 教材化补充：数值 kernel 是最小、最硬的计算边界

系统里有很多代码：配置、日志、调度、IO、Python 绑定。但真正需要极致性能的通常是少数 kernel：

- rolling sum/std/corr。
- dot product。
- matrix-vector multiply。
- reduction。
- filter/scatter/gather。

kernel 应该小、纯、可测试、可 benchmark。它不应该知道 CLI、文件路径、Python object 或 scheduler。

## shape/stride 的直觉

shape 是你眼中的形状，stride 是内存中的步长。

一个转置矩阵可以不复制数据，只改变 stride 解释。但这需要更复杂的 view 类型。

学习阶段先实现 copy 版本也可以，但你必须知道 copy 和 view 的语义差别。

## reduction 的数值问题

浮点加法不满足严格结合律：

```text
(a + b) + c 可能不等于 a + (b + c)
```

并行 reduction 改变加法顺序，所以结果可能有微小差异。测试应该用容差，而不是强行 `assert_eq`。

## 示例走读

```text
book/chapters/21-numerical-kernel/example/src/lib.rs
```

`Matrix::new` 做 shape 检查。`get` 做边界检查。这个版本很小，但已经包含专业数值库的基本边界：形状合法、索引合法、底层连续存储。

## 常见错误

错误 1：把矩阵逻辑和业务逻辑混在一起。

matrix kernel 不应该知道 symbol、日期或回测。

错误 2：view/copy 命名不清。

如果函数复制数据，名字要让调用者知道；如果返回 view，要让 lifetime 表达借用关系。

错误 3：过早实现通用 ndarray。

生产中通常优先使用成熟库；自己写的是为了理解或实现专用高性能 kernel。

## 代码走读与扩展路径

看 `Matrix`：

```rust
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}
```

它拥有数据。未来如果要做 view，需要另一个结构保存 `&[f64]`，而不是复用这个 owned matrix。

看 `Matrix::new`：

```rust
if rows * cols != data.len() {
    return Err(MatrixError::ShapeMismatch);
}
```

这是 shape contract。没有这个检查，后续索引公式可能读错位置或越界。

看 `get`：

```rust
Ok(self.data[row * self.cols + col])
```

这是 row-major index。你要能在纸上画出二维坐标如何映射到底层一维数组。

扩展路径：

1. `row(row) -> Result<&[f64], MatrixError>`：返回 borrowed row view。
2. `col_sum(col)`：练习跳跃访问。
3. `transpose_copy()`：先做复制版本。
4. `MatrixView<'a>`：再做 borrowed view 版本。

如果你能清楚区分 owned matrix、borrowed row、transpose copy、transpose view，说明你真正理解了数值库的基础。

## 自测与复盘问题

1. shape 描述逻辑形状，stride 描述什么？
2. `Matrix` 拥有数据，`MatrixView<'a>` 应该拥有什么？
3. 为什么 `row()` 可以返回 borrowed slice？
4. 并行 reduction 为什么可能产生微小浮点差异？
5. 什么时候应该手写专用 kernel，而不是写通用 ndarray？

如果这些问题回答不出来，先不要进入 SIMD。数值 kernel 的数据模型必须先清楚。

## 进入下一章前

确认你能区分 owned matrix、borrowed view、copy transpose 和 view transpose。你还应该能说明 reduction 为什么需要容差测试。做到这些，再进入并行计算。

## 额外复盘

把本章内容映射到因子矩阵：行可以代表时间，列可以代表资产，也可以反过来。不同布局会改变访问模式。你应该能解释自己的选择服务哪一种计算。
