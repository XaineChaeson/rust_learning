# 第 19 章：性能工程不是猜速度

专业高性能开发的第一条纪律：没有测量，就没有优化。Rust 很快，但“用 Rust 写”不等于“代码已经快”。你必须能证明瓶颈在哪里，证明优化为什么有效。

## debug build 不能看性能

`cargo run` 默认是 debug build，优化级别低，保留调试信息，许多优化不会发生。性能判断至少要用：

```bash
cargo run --release
cargo test --release
```

benchmark 也必须明确输入规模、机器环境、编译配置和重复次数。

## baseline

baseline 是正确且容易理解的第一版。没有 baseline，你无法知道优化是否改变结果。

rolling sum 的 baseline：

```rust
values.windows(window).map(|slice| slice.iter().sum::<f64>())
```

优化版可能维护增量 sum。你必须证明两者结果一致，然后再比较速度。

## 性能问题分类

常见瓶颈：

- 算法复杂度：`O(n * window)` vs `O(n)`。
- allocation：热路径里反复创建 `Vec`。
- cache locality：访问顺序跳跃。
- branch：分支不可预测。
- bounds check：索引检查无法消除。
- synchronization：锁竞争。
- IO：把读文件时间算进 kernel。

不同瓶颈需要不同工具，不要把所有问题都归因于“Rust 还不够底层”。

## benchmark 的陷阱

- 数据太小，测到的是噪声。
- 只跑一次，受 CPU 频率和系统调度影响。
- debug build。
- 编译器把未使用结果优化掉。
- 把数据生成、IO、打印混入 kernel benchmark。
- 只比较平均值，不看方差。

## 本章示例

```bash
cargo test -p ch19-performance-engineering
```

重点看：

- 朴素 rolling sum 和增量 rolling sum 结果一致。
- 测性能之前先锁定正确性。

## 本章练习

1. 给 `rolling_sum_incremental` 增加 window 为 1 的测试。
2. 增加一个输出参数版本，复用 `Vec`。
3. 写 benchmark 计划：输入规模、重复次数、对照组。
4. 写复盘：当前最可能的瓶颈是算法、内存还是分配？

## 本章验收

你应该能解释：

- 为什么 debug build 不能做性能判断。
- baseline 为什么先于优化。
- 常见性能瓶颈如何分类。
- 一个合格 benchmark 应该记录哪些信息。

## 教材化补充：性能结论必须能复现

高性能开发最危险的句子是“我感觉这个更快”。专业团队不会接受这种说法。你需要给出：

- baseline 是什么。
- candidate 是什么。
- 输入规模是多少。
- 命令是什么。
- 编译 profile 是什么。
- 运行了多少次。
- 结果方差如何。

如果没有这些信息，别人无法复现，也无法判断优化是否真实。

## Python 对照

Python 数据科学里常见的性能流程是先写 pandas/numpy，再用 `%timeit` 看一下。Rust 也需要类似反馈，但要更严格：

- debug/release 差异巨大。
- 编译器可能优化掉未使用结果。
- 微基准容易被噪声影响。
- IO 和计算必须分开测。

所以 Rust 中严肃 benchmark 通常使用 Criterion，或至少写清楚 release 命令和输入数据。

## 初学者优化顺序

正确顺序：

1. 写清楚正确 baseline。
2. 用测试锁定结果。
3. 找复杂度瓶颈。
4. 看内存布局和分配。
5. 再看并行。
6. 最后才考虑 unsafe/SIMD。

不要从第 6 步开始。

## 示例走读

```text
book/chapters/19-performance-engineering/example/src/lib.rs
```

`rolling_sum_naive` 是 baseline，`rolling_sum_incremental` 是复杂度优化。测试不是在测速度，而是在证明优化版和 baseline 行为一致。

## 常见错误

错误 1：把打印时间算进 benchmark。

打印是 IO，会完全污染计算时间。

错误 2：数据规模太小。

小数据测到的可能是函数调用、调度或噪声，不是算法真实性能。

错误 3：优化后只看速度不看正确性。

高频优化经常引入边界 bug。性能测试永远不能替代正确性测试。

## 代码走读与实验设计

看 baseline：

```rust
pub fn rolling_sum_naive(values: &[f64], window: usize) -> Vec<f64>
```

这个函数每个窗口重新求和。它容易理解，因此适合作为 correctness baseline。

看 candidate：

```rust
pub fn rolling_sum_incremental(values: &[f64], window: usize) -> Vec<f64>
```

它复用上一个窗口的 sum。优化点是算法复杂度，不是语法技巧。

测试只证明结果一致，不证明速度更快。要证明速度，需要单独 benchmark。

实验设计：

1. 输入规模：`10_000`、`1_000_000`、`10_000_000`。
2. 窗口：`5`、`20`、`252`。
3. 对照：naive vs incremental。
4. 命令：release build。
5. 输出：吞吐 rows/sec 和耗时。

写 benchmark 结论时，不要只写“快了很多”。要写“在 n=1_000_000, window=252 时，incremental 相比 naive 减少了什么成本”。

## 自测与复盘问题

1. 为什么 debug build 的性能数据没有决策价值？
2. baseline 应该优先追求正确清晰，还是极致速度？
3. 如何把 IO 时间和 kernel 时间分开？
4. 输入规模为什么会影响 benchmark 结论？
5. 如果优化后速度变快但测试失败，应该保留吗？

如果这些问题回答不出来，不要进入 unsafe/SIMD。性能工程先是证据纪律。

## 进入下一章前

确认你能写出一份 benchmark 记录，而不是只说“更快”。你应该能区分正确性测试、benchmark、profiling 三件事。做到这些，再进入内存布局。
