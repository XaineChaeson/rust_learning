# 第 16 章：concurrency primitives、Send、Sync、Arc、Mutex 和 channel

并发不是并行性能的同义词。并发关注多个任务如何安全推进；并行关注 CPU 如何同时计算。Rust 的强项是让数据竞争在编译期暴露，但你仍然需要设计正确的共享边界。

## Send 和 Sync

`Send`：值可以安全移动到另一个线程。

`Sync`：多个线程可以安全共享同一个类型的引用。更准确地说，如果 `&T` 是 `Send`，那么 `T` 是 `Sync`。

大多数普通拥有型数据如 `Vec<f64>` 是 `Send`。但不是所有引用、裸指针、内部可变结构都能自动跨线程。

你通常不手写实现 `Send`/`Sync`。如果你需要 unsafe impl，说明你在写很底层的并发抽象，必须有严格不变量。

## Arc

`Arc<T>` 是原子引用计数，允许多个线程共享同一个 owner。它解决“谁拥有这份数据”的问题，不解决“如何修改这份数据”的问题。

共享只读大数据时，`Arc<[f64]>` 或 `Arc<Vec<f64>>` 很常见。

## Mutex

`Mutex<T>` 提供互斥修改。`Arc<Mutex<T>>` 很容易写，但不是高性能并行计算默认答案，因为锁会串行化热路径。

更好的模式通常是：

1. 每个线程计算自己的 partial result。
2. 最后 merge。

锁适合保护小的共享状态，不适合每个元素更新都加锁。

## channel

channel 用于在线程之间传递 ownership。它常用于 worker 把结果发回 scheduler。

相较共享可变状态，message passing 更容易推理，但也有队列、背压和内存增长问题。

## 本章示例

```bash
cargo test -p ch16-concurrency-primitives
```

重点看：

- `Arc<Mutex<f64>>` 如何共享总和。
- channel 如何把 partial result 发回主线程。
- 为什么测试需要排序 channel 输出。

## 本章练习

1. 把 `sum_on_threads` 改成先收集 partial，再单线程求和。
2. 比较锁版本和 partial merge 版本的可读性。
3. 写一个 `worker_results_with_index`，让输出顺序确定。
4. 写复盘：为什么 `Arc<Mutex<Vec<_>>>` 不是默认并行方案？

## 本章验收

你应该能解释：

- `Send` 和 `Sync` 的直觉。
- `Arc` 和 `Mutex` 分别解决什么问题。
- channel 是传 ownership，而不是共享引用。
- 为什么高性能并行更偏好分片和归约。

## 教材化补充：并发安全不是“加锁就完了”

Python 中 CPU 并行经常受 GIL 限制，很多数据科学任务会转向 multiprocessing。Rust 允许你在一个进程内安全使用多线程，但它不会自动让你的算法变快。

最容易写出的并发代码是：

```rust
Arc<Mutex<Vec<f64>>>
```

每个线程抢锁、写结果。这个模式容易理解，但在高性能计算里常常慢，因为锁把并行工作重新串行化。

## 更好的并行模式

更常见的模式是：

```text
split input -> local compute -> partial result -> merge
```

每个线程拥有自己的输入 chunk 和本地输出。只有最后合并时才同步。

这也更容易测试，因为每个 partial result 都可以单独验证。

## Send/Sync 的直觉

你不需要一开始背完整定义。先记住：

- `Send`：值能不能移动到另一个线程。
- `Sync`：引用能不能被多个线程共享。

Rust 会自动为很多类型推导这些 trait。你通常不应该手动实现它们。

## 示例走读

```text
book/chapters/16-concurrency-primitives/example/src/lib.rs
```

`sum_on_threads` 用 `Arc<Mutex<f64>>` 展示共享状态。`worker_results` 用 channel 展示结果 ownership 从 worker 发送回主线程。

## 常见错误

错误 1：把 `Arc` 当成可变共享。

`Arc` 只解决多 owner，不解决修改。修改需要 `Mutex`、`RwLock` 或其他同步结构。

错误 2：在热循环里锁。

这会让多线程收益消失。

错误 3：测试并行输出时假设顺序固定。

线程完成顺序不稳定。需要 index 或排序。

## 代码走读与操作清单

看 `sum_on_threads`：

```rust
let total = Arc::new(Mutex::new(0.0));
```

`Arc` 让多个线程共享同一个 `Mutex<f64>` owner。`Mutex` 保护内部 `f64` 的修改。

在线程里：

```rust
let mut guard = total.lock().expect("mutex is not poisoned");
*guard += partial;
```

`guard` 存在期间锁被持有。guard 离开作用域时锁释放。这是 RAII。

看 `worker_results`：

```rust
let (sender, receiver) = mpsc::channel();
```

每个 worker 把 partial sum 发送给 receiver。发送的是值的 ownership，不是共享引用。

操作清单：

1. 把 `sum_on_threads` 改成每个线程返回 partial。
2. 主线程 join 后求和。
3. 比较这个版本和 Mutex 版本。
4. 解释哪个更适合高性能 rolling 多资产计算。

你应该得出结论：锁是正确性工具，不是默认性能工具。

## 自测与复盘问题

1. `Arc` 解决的是共享 owner，还是共享可变性？
2. `Mutex` guard 什么时候释放锁？
3. channel 发送的是引用还是 ownership？
4. 为什么并行计算更偏好 partial result + merge？
5. 什么时候 `Arc<Mutex<T>>` 是合理选择？

如果这些问题回答不出来，先不要进入并行性能优化。并发正确性比速度更基础。

## 进入下一章前

确认你能画出 `Arc<Mutex<T>>` 的所有权关系，也能画出 channel 的 ownership 流动。再确认你知道锁版本为什么只是教学示例，不是高性能默认方案。做到这些，再学习 async 边界。

## 额外复盘

把本章内容映射到多资产计算：每个资产可以成为一个独立任务，每个任务产生本地结果，主线程只负责收集和合并。你应该能说明这个设计为什么比多个线程共享一个可变 Vec 更容易验证。

## 专业判断清单

设计并发代码前先回答：

1. 数据是共享读取、共享修改，还是 ownership 可以直接转移？
2. 每个 worker 的输出能不能先存在本地，最后统一合并？
3. 锁是否出现在热循环里？
4. 结果顺序是否有业务含义？
5. panic、poisoned mutex、worker 失败时系统如何暴露错误？

专业 Rust 并发不是把 `Arc<Mutex<T>>` 套到所有地方，而是尽量让数据所有权沿着任务边界移动，让共享可变状态成为少数、可解释、可测试的例外。
