# 第 4 章：所有权和借用

Rust 最重要的概念是所有权。很多人学 Rust 卡住，不是因为语法，而是因为还在用 Python 的引用模型理解 Rust。

本章目标：你能解释一个值归谁所有，函数是拿走数据、只读借用，还是可变借用。

## Python 的变量名和 Rust 的所有者

Python 中：

```python
a = [1, 2, 3]
b = a
b.append(4)
print(a)
```

`a` 和 `b` 指向同一个 list。你改 `b`，`a` 也看到变化。

Rust 中：

```rust
let a = vec![1, 2, 3];
let b = a;
println!("{a:?}");
```

这不能编译。`a` 的所有权移动给了 `b`，之后不能再使用 `a`。

这不是限制，而是规则明确：同一段堆内存不会有两个默认所有者。这样 Rust 不需要垃圾回收，也能知道什么时候释放内存。

## move

把 `String` 或 `Vec` 赋给另一个变量，通常会 move：

```rust
let prices = vec![100.0, 101.0, 102.0];
let moved_prices = prices;
```

之后 `prices` 不能再用。

为什么？因为 `Vec` 在堆上拥有一段内存。如果两个变量都以为自己拥有这段内存，释放时就会出问题。Rust 干脆规定：所有权只能有一个。

## borrow

如果函数只是读取数据，不要 move，借用即可：

```rust
fn print_len(values: &[f64]) {
    println!("{}", values.len());
}

let prices = vec![100.0, 101.0, 102.0];
print_len(&prices);
println!("{prices:?}");
```

`&prices` 表示借用。函数没有拿走所有权，所以调用后 `prices` 仍然可用。

## mutable borrow

如果函数需要原地修改数据，用可变借用：

```rust
fn normalize_in_place(values: &mut [f64]) {
    let mean = values.iter().sum::<f64>() / values.len() as f64;

    for value in values {
        *value -= mean;
    }
}

let mut values = vec![1.0, 2.0, 3.0];
normalize_in_place(&mut values);
```

注意两点：

1. 变量本身必须是 `mut`。
2. 调用时要写 `&mut values`。

## 借用规则

Rust 的核心借用规则：

1. 可以同时有多个不可变借用。
2. 或者只能有一个可变借用。
3. 可变借用和不可变借用不能同时存在。

这和高性能并发有关。如果很多地方都能随便改同一段数据，多线程计算会非常危险。Rust 在编译期阻止这类问题。

## `clone` 不是默认解法

新手看到所有权错误，常常加 `.clone()`：

```rust
let avg = mean(prices.clone());
```

这可能会复制几百万个浮点数。代码能跑，但性能已经坏了。

正确思路是先问：

- 函数真的需要拥有数据吗？
- 只是读取的话，能不能传 `&[f64]`？
- 只是修改的话，能不能传 `&mut [f64]`？
- 输出是否必须新分配？

## API 设计中的所有权

对量化计算函数，常见签名应该像这样：

```rust
pub fn mean(values: &[f64]) -> Option<f64>
```

只读输入，不拥有。

```rust
pub fn demean_in_place(values: &mut [f64]) -> Result<(), NumericError>
```

原地修改，不创建新数组。

```rust
pub fn returns(prices: &[f64]) -> Result<Vec<f64>, NumericError>
```

只读输入，创建新输出。

这些签名本身就是设计文档。它们告诉调用方数据会不会被拿走，会不会被修改，会不会分配新内存。

## 常见编译器错误

### moved value

你可能看到：

```text
borrow of moved value
```

含义：值已经被移动，你又想用旧变量。

解决顺序：

1. 如果函数只是读取，改成借用。
2. 如果确实需要两个独立副本，再 clone。
3. 如果数据流设计错了，重新设计所有权。

### cannot borrow as mutable

含义：你想可变借用一个不可变变量。

解决：

```rust
let mut values = vec![1.0, 2.0, 3.0];
```

### cannot borrow because already borrowed

含义：你同时持有不兼容的借用。通常是你一边读取，一边想修改。

解决：缩短借用范围，或拆分逻辑。

## 本章练习

1. 运行：

```bash
cargo run -p ch01-rust-program-shape --bin ownership
```

2. 写一个 `fn sum(values: &[f64]) -> f64`。
3. 写一个 `fn add_in_place(values: &mut [f64], delta: f64)`。
4. 故意把 `sum` 写成接收 `Vec<f64>`，观察调用后原变量是否还能使用。
5. 解释什么时候 clone 是合理的。

## 本章验收

你可以进入下一章，如果你能回答：

- move 和 borrow 的区别是什么？
- 为什么 `&[f64]` 对高性能计算重要？
- 为什么不能把 clone 当成默认修复？
- 可变借用为什么必须受限制？

## 教材化补充：所有权是性能和安全的共同基础

Python 变量名像标签，多个变量名可以指向同一个对象。Rust 变量默认是 owner。owner 离开作用域，值就被释放。

这不是为了折磨初学者，而是为了同时做到两件事：

- 没有 GC，也能确定资源何时释放。
- 没有数据竞争，也能写多线程。

高性能计算最怕两类隐性成本：隐藏复制和隐藏共享修改。Rust 让这两件事都变得显式。

## move 的直觉

```rust
let a = vec![1.0, 2.0];
let b = a;
```

这里不是复制整个 Vec，而是把 Vec 的所有权从 `a` 移到 `b`。之后 `a` 不能再用。

这避免了 Python 中常见的“我以为复制了，其实两个名字指向同一个对象”的问题。

## borrow 的直觉

```rust
fn sum(values: &[f64]) -> f64
```

函数只是借用数据。调用者仍然拥有数据。借用像去图书馆看书，不是把书买走。

可变借用：

```rust
fn add_in_place(values: &mut [f64], delta: f64)
```

函数可以改数据，但同一时间不能有其他读写借用。这条规则避免了很多并发和别名问题。

## clone 应该被审查

`.clone()` 不是错，但必须知道成本。clone 一个 `f64` 没什么，clone 一个几百万行的 `Vec<f64>` 就是性能事件。

专业代码 review 时，看到 clone 会问：

- 是否真的需要拥有数据？
- 是否可以借用？
- clone 是否在热路径？
- clone 的数据规模多大？

## 动手操作

运行：

```bash
cargo test -p ch04-ownership-borrowing
```

打开：

```text
book/chapters/04-ownership-borrowing/example/src/lib.rs
```

观察：

- `demean` 返回新 `Vec`，不改输入。
- `demean_in_place` 修改输入，不分配输出。
- 两种 API 都合理，但表达的语义不同。

## 常见错误

错误 1：编译器说 moved value，就立刻 clone。

先问函数是否应该借用。如果只是读取，改成 `&T` 或 `&[T]`。

错误 2：同时拿不可变和可变引用。

这通常说明你需要缩短借用作用域，或者把计算拆成两个阶段。

错误 3：把所有 API 都设计成 in-place。

in-place 性能好，但调用者可能需要保留原数据。专业库通常同时提供分配版和 in-place 版。
