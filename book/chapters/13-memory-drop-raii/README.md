# 第 13 章：memory、Copy、Clone、Drop 和 RAII

Rust 没有 GC，但也不是手动 `free`。Rust 的核心模型是：每个值有唯一 owner，当 owner 离开作用域时，资源自动释放。这叫 RAII。

## move、Copy、Clone

默认情况下，赋值会 move：

```rust
let a = String::from("AAPL");
let b = a;
```

此后 `a` 不可用，因为 `String` 拥有堆内存，不能无声复制。

但 `f64`、`usize`、小的纯值结构可以实现 `Copy`：

```rust
#[derive(Clone, Copy)]
pub struct Quote {
    bid: f64,
    ask: f64,
}
```

`Copy` 表示按位复制就是语义复制。不要给拥有资源的类型实现 `Copy`。

`Clone` 是显式复制，可能分配。看到 `.clone()`，应该问：这里复制成本是多少？是否真的需要所有权？

## Drop

`Drop` 在值离开作用域时运行。文件句柄、锁、网络连接、临时缓冲都可以依赖 Drop 清理。

在高性能系统里，Drop 的意义不仅是安全释放，也包括让资源生命周期清楚。比如 scratch buffer 应该由调用者持有并复用，而不是在每次 kernel 调用里创建又释放。

## RAII 和锁

`Mutex::lock()` 返回 guard。guard 离开作用域时自动释放锁。这能避免忘记 unlock，但也会引出另一个问题：如果 guard 作用域太长，会扩大锁竞争。

专业代码会刻意缩短资源 guard 的作用域。

## 分配是性能事件

`Vec::new()` 很便宜，但 push 到超过 capacity 会重新分配。热路径里反复创建 `Vec` 会制造 allocator 压力和 cache 抖动。

常见策略：

- `Vec::with_capacity`
- 复用 scratch buffer
- 输出由调用者传入 `&mut Vec<f64>`
- 批量处理而不是单行处理

## 本章示例

```bash
cargo test -p ch13-memory-drop-raii
```

重点看：

- `Quote` 为什么可以 `Copy`。
- `ScratchBuffer` 如何复用 allocation。
- `DropCounter` 如何证明 RAII。

## 本章练习

1. 给 `ScratchBuffer` 增加 `capacity()`。
2. 写一个函数接收 `&mut ScratchBuffer`，避免内部创建 `Vec`。
3. 故意在循环里分配 `Vec`，写出为什么这可能慢。
4. 写复盘：什么时候 clone 是正确工程选择？

## 本章验收

你应该能解释：

- move、Copy、Clone 的区别。
- Drop 什么时候运行。
- RAII 如何管理资源和锁。
- 为什么 allocation 是高性能代码必须测量的事件。

## 教材化补充：内存不是抽象掉就不存在

Python 让你很少关心对象什么时候释放。Rust 要求你面对资源生命周期。这个要求一开始更难，但它让高性能系统更可控。

`Vec<f64>` 本身包含三部分元数据：

- 指向堆内存的指针。
- 当前长度。
- 当前容量。

move 一个 Vec 通常只是移动这三部分元数据，不复制所有元素。clone 一个 Vec 才会复制底层元素。

## Copy 类型和 Clone 类型

`Copy` 适合很小、按位复制就正确的值，比如数字、小结构体。`Clone` 是显式复制，可能分配。

专业代码里看到 `clone()`，不应该立刻删除，也不应该无视。你要判断它是否在热路径，复制规模多大，是否能用借用替代。

## Drop 和资源释放

Rust 在 owner 离开作用域时自动调用 `Drop`。这让文件、锁、buffer、临时目录等资源管理变得确定。

高性能系统喜欢确定性释放，因为你能知道内存峰值、锁持有时间和资源生命周期。

## scratch buffer 的意义

如果一个函数每次调用都创建输出 `Vec`，在大循环里会产生大量分配。更高阶的 API 可以让调用者传入 buffer：

```rust
fn compute_into(input: &[f64], output: &mut Vec<f64>)
```

这种 API 更麻烦，但在热路径里很常见。

## 示例走读

```text
book/chapters/13-memory-drop-raii/example/src/lib.rs
```

`ScratchBuffer` 展示了复用分配的基本模式。`DropCounter` 展示值离开作用域时会自动执行清理逻辑。

## 常见错误

错误 1：以为 move 很贵。

move 拥有型容器通常不复制底层数据。

错误 2：为了绕过借用检查到处 clone。

这会让代码看似通过编译，但隐藏严重性能成本。

错误 3：忽视作用域。

锁、临时 buffer、引用的作用域越短，代码越容易推理，也越少阻塞。

## 代码走读与操作清单

看 `ScratchBuffer`：

```rust
pub struct ScratchBuffer {
    data: Vec<f64>,
}
```

它拥有一个 `Vec<f64>`。调用 `clear_and_extend` 时，Vec 的长度清空，但容量通常保留。下一次写入时可以复用已有分配。

这就是很多高性能库会提供 workspace、scratch、buffer 参数的原因。

看 `DropCounter`：

```rust
impl Drop for DropCounter {
    fn drop(&mut self) { ... }
}
```

测试中 `_counter` 离开作用域后，drop count 增加。这个例子证明 Rust 不需要你手动调用 cleanup。

操作清单：

1. 给 `ScratchBuffer` 增加 `capacity()`。
2. 在测试里多次调用 `clear_and_extend`。
3. 观察 capacity 是否复用。
4. 写一个不复用 buffer 的版本，对比 API 语义。

专业思考：复用 buffer 会让 API 多一个 mutable 参数，牺牲一点易用性换取性能。是否值得，需要 benchmark 和场景判断。

## 自测与复盘问题

1. move 一个 `Vec<f64>` 时，底层元素是否被复制？
2. `Copy` 和 `Clone` 的语义差异是什么？
3. `Drop` 为什么能帮助资源管理？
4. scratch buffer 为什么能减少 allocation？
5. 如果一个函数需要最高易用性和最高性能，你会设计两个 API 吗？

如果这些问题回答不出来，重新阅读本章的 `ScratchBuffer` 和 `DropCounter` 示例。

## 进入下一章前

确认你能不用背诵术语，而是用自己的话说明：一个值何时被 move，何时被 clone，何时运行 Drop。再确认你能指出一个函数里哪些地方可能分配内存。做到这些，再进入集合和字节边界。
