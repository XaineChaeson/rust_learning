# 第 11 章：lifetime 深入和 borrowed API 设计

很多 Rust 初学者把 lifetime 当成语法噪音。专业开发者必须反过来理解：lifetime 是 API 合同。它告诉调用者返回的引用依赖哪个输入，不能活得比谁更久。

## lifetime 不创造生命周期

`'a` 不会让数据活得更久。它只是给已有的借用关系命名。

```rust
pub struct Window<'a> {
    values: &'a [f64],
}
```

这表示 `Window` 不是数据所有者，只是某段 `f64` 的视图。只要原始 slice 不存在，`Window` 也不能存在。

这正是高性能计算需要的能力：创建窗口、视图、切片时不复制数据。

## 返回借用的函数

```rust
pub fn choose_longer<'a>(left: &'a [f64], right: &'a [f64]) -> &'a [f64]
```

这表示返回值可能来自 `left`，也可能来自 `right`，所以返回值必须不能超过两者共同允许的生命周期。

如果函数只返回 `left`，可以写得更精确：

```rust
pub fn first_input<'a>(left: &'a [f64], _right: &[f64]) -> &'a [f64]
```

专业 API 要尽量表达真实关系。过度宽泛会让调用方受不必要约束；过度承诺会无法编译。

## view API 是高性能库的核心

NumPy、Arrow、ndarray 都大量使用 view 思想。Rust 中 view 的基本形式就是带 lifetime 的结构体：

```rust
pub struct MatrixView<'a> {
    data: &'a [f64],
    rows: usize,
    cols: usize,
    stride: usize,
}
```

这类结构不拥有数据，因此创建很便宜。它的难点不是性能，而是保证不会返回悬垂引用，不会让调用者以为自己拥有数据。

## lifetime elision

很多函数不需要显式写 lifetime：

```rust
fn last(values: &[f64]) -> Option<&f64>
```

编译器能推断返回引用来自唯一输入引用。但只要有多个输入引用，或者结构体保存引用，你就经常需要显式 lifetime。

规则不是背诵用的，核心问题只有一个：返回引用到底借自哪里？

## 本章示例

```bash
cargo test -p ch11-lifetimes-api-design
```

重点看：

- `Window<'a>` 如何保存 borrowed slice。
- `rolling_windows` 为什么返回 `Vec<Window<'a>>`。
- `choose_longer` 为什么两个输入共享同一个 `'a`。

## 本章练习

1. 给 `Window<'a>` 增加 `mean()`。
2. 写 `first_window<'a>(values: &'a [f64], window: usize) -> Option<Window<'a>>`。
3. 写一个错误版本，让 `Window` 引用函数内部创建的 `Vec<f64>`，观察编译器错误。
4. 设计 `SymbolView<'a>`，保存 `&'a str` 和 `&'a [f64]`。

## 本章验收

你应该能解释：

- lifetime 标注不是延长生命周期。
- 返回 borrowed view 为什么比返回新 `Vec` 更高效。
- 多输入引用时，返回引用为什么需要明确来源。
- 为什么高性能数组库离不开 view。

## 教材化补充：lifetime 是“这段借用从哪里来”

不要把 lifetime 理解成“让变量活久一点”。它做不到。lifetime 标注只是告诉编译器和读者：这个返回值借用了哪个输入。

```rust
pub struct Window<'a> {
    values: &'a [f64],
}
```

`Window<'a>` 的含义是：这个窗口不能比它借用的 `values` 活得更久。

这和 NumPy view 很像。NumPy 中 view 不拥有数据，如果原数组不存在，view 就没有意义。Rust 把这种关系放进类型系统。

## 初学者为什么会卡住

你可能会写出这样的想法：

```rust
fn make_window() -> Window {
    let values = vec![1.0, 2.0, 3.0];
    Window::new(&values)
}
```

这是不允许的。`values` 是函数内部创建的，函数结束就释放。返回的 `Window` 会指向已经释放的内存。Python 可能用引用计数或 GC 延长对象生命，Rust 不这样隐式处理。

## API 设计判断

当你设计高性能 API 时，要问：

- 调用者是否需要拥有输出？
- 能不能返回 view？
- view 的生命周期来自哪个输入？
- 返回 view 会不会让 API 太难用？

例如 `rolling_windows(&values, 3)` 返回 borrowed windows 很高效；但 `rolling_mean` 返回 `Vec<f64>` 更合理，因为均值结果不是原数据中的一段视图，而是新计算出来的序列。

## 示例走读

```text
book/chapters/11-lifetimes-api-design/example/src/lib.rs
```

`rolling_windows<'a>` 接收 `&'a [f64]`，返回 `Vec<Window<'a>>`。每个 `Window` 都借用原始输入的一段。测试中 `values` 在 `windows` 使用期间仍然存在，所以合法。

## 常见错误

错误 1：到处加 `'static`。

`'static` 不是解决 lifetime 的万能药。它表示数据能活到程序结束，通常不适合普通借用。

错误 2：为了躲 lifetime，全部返回 `Vec`。

这会复制数据。有时合理，但 view 能明显减少分配和内存带宽。

错误 3：结构体里保存引用，却没有理解 owner 在哪里。

保存引用的结构体必须明确数据由谁拥有。否则 API 会很难用或无法编译。

## 代码走读与操作清单

看 `Window<'a>`：

```rust
pub struct Window<'a> {
    values: &'a [f64],
}
```

这不是拥有数据的窗口，而是视图。它像 NumPy view，但 Rust 在类型里写出 view 依赖原始数据。

看构造函数：

```rust
pub fn new(values: &'a [f64]) -> Self
```

输入和结构体内部保存的引用使用同一个 `'a`。这告诉编译器：`Window` 不能比 `values` 活得更久。

看 `rolling_windows`：

```rust
pub fn rolling_windows<'a>(values: &'a [f64], window: usize) -> Vec<Window<'a>>
```

返回的每个窗口都借用 `values`。输出 `Vec` 拥有的是窗口描述对象，不拥有底层数据。

操作清单：

1. 给 `Window` 增加 `mean()`。
2. 在测试里创建 `values`，再创建 `windows`。
3. 尝试在 `windows` 使用前 drop 掉 `values`，观察编译器不允许。
4. 写下为什么这是防止悬垂引用。

进阶思考：`rolling_mean` 为什么不返回 `Vec<Window>`？因为 rolling mean 的输出不是原始数据视图，而是新计算出来的数值序列。
