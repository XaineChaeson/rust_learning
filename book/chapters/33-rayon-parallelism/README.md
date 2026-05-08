# 第 33 章：Rayon 式数据并行

前面的第 22 章已经讲过线程、chunk、partial result 和 merge。本章进入 Rust 生态中的主流数据并行工具：Rayon。

本章示例仍然使用标准库线程，但会训练 Rayon 背后的核心思想。真正使用 Rayon 时，你会把很多手写调度交给成熟库；但如果你不理解任务切分和确定性合并，就会把并行写成不可控的黑箱。

## 本章解决什么问题

量化计算经常天然并行：

- 多资产独立计算。
- 多参数组合独立回测。
- Monte Carlo 多路径独立模拟。
- 多天或多分区批处理。

但是并行不自动等于高性能。

常见问题：

- chunk 太小，调度成本超过计算收益。
- chunk 太大，负载不均衡。
- 多线程共享一个全局 Vec，锁竞争严重。
- 输出顺序不稳定，实验不可复现。
- 浮点归约顺序变化，结果略有差异。

Rayon 的价值是提供成熟的 work-stealing 数据并行模型，让你用 `par_iter`、`par_chunks` 等方式表达并行计算。

但专业开发者必须先知道：数据如何切、结果如何合并、顺序是否重要。

## 学习前提

你需要理解：

- 第 16 章：线程、channel、`Arc<Mutex<T>>`。
- 第 22 章：并行计算基础。
- 第 26 章：参数搜索和 Monte Carlo。
- 第 32 章：并行优化也必须 benchmark。

如果你还认为 `Arc<Mutex<Vec<_>>>` 是并行计算默认写法，请先回到第 16 章。

## Python 对照

Python 中常见并行方式包括：

- multiprocessing。
- joblib。
- Dask。
- Ray。
- PyTorch DataLoader workers。

Python 的问题通常是：

- GIL 限制 CPU-bound 多线程。
- 进程间序列化成本高。
- NumPy 内部可能已经多线程。
- 多层并行容易 oversubscription。

Rust 的 Rayon 更适合 CPU-bound 数据并行：

- 没有 GIL。
- 类型系统保证线程安全边界。
- `Send` 和 `Sync` 约束跨线程移动和共享。
- work-stealing runtime 减少手写线程管理。

但 Rust 也不会替你解决算法切分问题。

## Rayon 的真实位置

真实项目中，你可能会写：

```rust
use rayon::prelude::*;

let results = assets
    .par_iter()
    .map(|asset| compute_factor(asset))
    .collect::<Vec<_>>();
```

或者：

```rust
values.par_chunks(chunk_size)
```

本章不直接引入 Rayon，是为了先训练三个基本能力：

1. 会设计 partition。
2. 会做 partial result。
3. 会恢复 deterministic order。

这些能力掌握后，Rayon 只是把调度实现交给库。

## 核心概念一：partition 必须覆盖且不重叠

如果输入长度是 10，切成 3 份，一种合理结果是：

```text
0..4
4..7
7..10
```

要求：

- 覆盖所有元素。
- 不重复。
- 不遗漏。
- 起点终点可测试。

本章示例的 `partition_ranges` 就是为了训练这个边界。

专业并行代码不能只靠“看起来差不多”。

## 核心概念二：partial result 比共享锁更适合计算

错误设计：

```text
多个 worker -> lock global Vec -> push result
```

这会导致：

- 锁竞争。
- 输出顺序不稳定。
- 测试困难。
- 性能收益下降。

更好的设计：

```text
worker -> local partial result -> merge
```

在 Rayon 中，很多 collect/reduce 操作会帮你做这件事。但你仍要知道它的语义。

## 核心概念三：顺序和可复现性

在量化研究里，结果顺序经常有业务意义：

- 第几个资产。
- 第几个参数组合。
- 第几个时间分区。

并行执行完成顺序不稳定。

所以结果必须携带 index 或 stable id。

本章示例中的 `restore_order` 展示：

```text
[(2, c), (0, a), (1, b)] -> [a, b, c]
```

这就是分布式和并行任务中最常见的模式。

## 示例代码走读

示例位置：

```text
book/chapters/33-rayon-parallelism/example/src/lib.rs
```

运行：

```bash
cargo test -p ch33-rayon-parallelism
```

关键函数：

- `partition_ranges`
- `sum_single_thread`
- `sum_partitioned`
- `sum_threaded`
- `restore_order`

这不是为了替代 Rayon，而是为了让你看清 Rayon 背后的计算形状。

## 代码走读：partition_ranges

函数签名：

```rust
pub fn partition_ranges(len: usize, partitions: usize) -> Result<Vec<(usize, usize)>, ParallelPlanError>
```

它没有接收数据本身，只接收长度和分区数量。

这是一种好设计：

- partition planning 和计算分离。
- partition 可以独立测试。
- 后续可以复用于不同数据类型。

错误处理：

- `len == 0` 返回 `EmptyInput`。
- `partitions == 0` 返回 `InvalidPartitions`。

不要让无效并行计划进入执行阶段。

## 代码走读：sum_threaded

`sum_threaded` 使用 `thread::scope`。

这允许线程借用当前栈上的 `values`，而不是强迫你把数据 clone 到每个线程。

这和 Rust 所有权模型高度相关：

- 线程不能悬垂引用。
- scope 结束前所有 worker 必须完成。
- 编译器能验证 borrow 不会逃出 scope。

真实 Rayon 代码通常更短，但底层安全问题相同。

## 动手操作

1. 跑测试：

```bash
cargo test -p ch33-rayon-parallelism
```

2. 给 `partition_ranges(11, 4)` 写预期测试。

3. 实现 `mean_threaded(values, partitions)`。

4. 给 `restore_order` 增加重复 index 的处理策略。

5. 在 `projects/02-quant-lab-engine` 中思考：哪些 experiment 可以并行？

## 常见错误

错误 1：小数据过度并行。

线程调度和同步有成本，小数据可能更慢。

错误 2：共享全局可变状态。

优先 partial result + merge。

错误 3：忽略浮点归约顺序。

并行 sum 可能和单线程最后几位不同。

错误 4：忘记输出顺序。

没有 stable id 的并行结果很难复现。

错误 5：多层并行。

如果 BLAS、NumPy、DataFusion 或 Rayon 都在并行，可能出现 oversubscription。

## 量化/HPC 连接

Rayon 最适合这些场景：

- 每个资产独立。
- 每个参数组合独立。
- 每条 Monte Carlo path 独立。
- 每个日期分区可独立预处理。

不适合：

- 强顺序依赖的事件流。
- 共享状态频繁修改。
- 单个任务太小。
- IO-bound pipeline。

专业判断不是“能不能 par_iter”，而是“这个计算形状是否适合数据并行”。

## 本章验收

你通过本章时，应该能做到：

1. 解释 Rayon 解决什么问题。
2. 手写 partition 并证明覆盖不重叠。
3. 解释 partial result + merge。
4. 解释为什么输出要带 stable id。
5. 判断一个量化任务是否适合 Rayon。
6. 知道并行优化必须回到第 32 章做 benchmark。

## 进入下一章前

确认你能说清：Rayon 是调度工具，不是算法设计工具。下一章进入 Python 扩展边界，把 Rust kernel 接回 Python 研究流。
