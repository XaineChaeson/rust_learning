# 第 32 章：Criterion、profiling 和性能证据

本章进入进阶阶段。前面的第 19 章已经讲过性能工程的基本纪律：先 baseline，再 candidate，再用 release benchmark 证明。现在要进一步学习专业 Rust 项目如何把性能实验制度化。

## 本章解决什么问题

在量化研究中，性能优化非常容易被误判：

- 一次运行更快，可能只是噪声。
- debug 模式更快或更慢，不能代表 release。
- 算法复杂度更低，不代表目标规模上一定更快。
- unsafe、SIMD、并行都可能让代码更复杂，却没有实际收益。

专业开发者不能说“我感觉这个更快”。你必须能拿出证据：

- 输入规模是什么。
- baseline 是什么。
- candidate 是什么。
- 是否 release build。
- 重复次数是多少。
- 噪声是否可控。
- 结果是否保持正确。
- 性能收益是否值得保留复杂度。

Rust 生态中常用 Criterion 做严肃 benchmark，用 `perf`、flamegraph、系统 profiler 做 profiling。本章不直接引入依赖，而是先用 std-only 示例训练你理解 Criterion 背后的判断逻辑。

## 学习前提

你需要已经理解：

- 第 19 章：baseline、candidate、release benchmark。
- 第 20 章：cache、allocation、访问模式。
- `projects/01-factor-core` 中的 `rolling_mean` 和 `rolling_mean_incremental`。
- `book/performance-lab.md` 的记录模板。

如果你还不能解释 debug 和 release 的区别，先回到第 19 章。

## Python 对照

Python 数据科学里常见做法是：

```python
%timeit f(x)
```

这很方便，但它经常隐藏几个问题：

- 输入数据是否每次重新创建？
- 是否包含 IO 或数据转换？
- 是否测到了 JIT warmup 或缓存状态？
- 是否记录了机器环境？
- 是否和正确 baseline 对照？

Rust 的性能实验通常更工程化。你要明确函数边界、编译 profile、样本数量、输入规模和输出校验。

Python 的优势是快速试验；Rust 的优势是把性能实验纳入工程流程。

## Criterion 是什么

Criterion 是 Rust 生态中常用的 benchmark 框架。它通常用于：

- 多次运行被测函数。
- 统计运行时间。
- 估计噪声。
- 比较历史结果。
- 生成报告。

真实项目中你会在 `benches/` 下写 benchmark，并用：

```bash
cargo bench
```

或对单个 benchmark 运行。正式引入时通常作为 dev-dependency。

本章示例不直接使用 Criterion，是因为初学者更需要先理解：

- 为什么要多样本。
- 为什么要看 median/mean/min/max。
- 为什么噪声过高时不能下结论。
- 为什么 speedup 需要阈值。

理解这些后，再使用 Criterion 才不是“复制模板”。

## profiling 是什么

benchmark 回答：“candidate 是否更快？”

profiling 回答：“时间花在哪里？”

两者不同：

- benchmark 是比较。
- profiling 是定位。

例子：

如果 `rolling_mean_incremental` 比 baseline 快，你已经知道 candidate 更快。

但如果一个复杂 pipeline 很慢，你还不知道瓶颈在：

- CSV parsing。
- data validation。
- rolling factor。
- memory allocation。
- backtest state update。
- report serialization。

这时需要 profiling。

常见工具包括：

```bash
perf record -- cargo run -p factor-core --release --bin bench
perf report
```

或者：

```bash
cargo flamegraph -p factor-core --bin bench
```

这些工具依赖系统环境。本课程不强制安装，但你必须知道它们解决的问题。

## 核心概念一：样本不是装饰

一次运行没有统计意义。

性能数字会受很多因素影响：

- CPU frequency scaling。
- 其他进程抢占。
- cache 状态。
- 内存分配器状态。
- 操作系统调度。

所以你要多次采样。

本章示例中的 `summarize(samples)` 会计算：

- sample count。
- min。
- median。
- max。
- mean。

这些不是为了好看，而是为了判断：

- 是否有异常慢样本。
- 平均值是否被极端值拉偏。
- 样本数量是否足够。

## 核心概念二：噪声过高时不要下结论

如果 baseline 的最小值是 100ns，最大值是 5000ns，这说明实验环境不稳定。

这时即使 candidate 看起来快，也可能只是噪声。

专业判断：

- 噪声高，先改善实验环境。
- 输入太小，扩大输入规模。
- benchmark 包含 IO，拆开测。
- 样本太少，增加 repeat。

不要把不稳定数字写进结论。

## 核心概念三：speedup 要有阈值

candidate 快 1.01x 是否值得保留？

不一定。

如果代码变复杂、API 变难用、测试成本增加，1% 收益通常不值得。

但如果一个 kernel 是全系统瓶颈，1% 也可能有价值。

所以你需要 `minimum_speedup`。

示例中的 `decide` 要求：

- 噪声低于阈值。
- speedup 超过阈值。

否则结论就是 keep baseline 或 inconclusive。

## 示例代码走读

示例位置：

```text
book/examples/ch32-criterion-profiling/src/lib.rs
```

运行：

```bash
cargo test -p ch32-criterion-profiling
```

关键类型：

```rust
BenchmarkSummary
BenchmarkDecision
```

`BenchmarkSummary` 不是 benchmark 工具本身，而是性能判断所需要的最小报告。

`BenchmarkDecision` 有三种结果：

- `CandidateWins`
- `KeepBaseline`
- `Inconclusive`

这很重要。专业工程里，“无法下结论”是合法输出。

## 代码走读：summarize

`summarize` 先拒绝空样本：

```rust
if samples.is_empty() {
    return None;
}
```

空样本不能产生性能结论。

然后复制并排序：

```rust
let mut sorted = samples.to_vec();
sorted.sort_unstable();
```

这里复制是合理的，因为排序会修改数据，而函数不应该改变调用方传入的样本。

这也是 API 设计判断：

- 输入用 `&[u128]`。
- 内部需要排序，所以内部复制。
- 调用方保留原始数据。

## 代码走读：decide

`decide` 先计算噪声比例：

```rust
max / min
```

如果比例过高，返回 `Inconclusive`。

然后计算 speedup：

```rust
baseline.mean / candidate.mean
```

只有 speedup 足够高，才返回 `CandidateWins`。

这个流程体现了一个原则：性能结论要先过质量门，再比较输赢。

## 动手操作

1. 跑本章测试：

```bash
cargo test -p ch32-criterion-profiling
```

2. 增加一个测试：candidate 只快 1.05x，但 `minimum_speedup` 是 1.2，应该返回 `KeepBaseline`。

3. 修改 `BenchmarkSummary`，增加 `range_nanos` 字段。

4. 在 `decide` 中加入样本数量要求：少于 5 个样本返回 `Inconclusive`。

5. 回到 `projects/01-factor-core` 跑：

```bash
cargo run -p factor-core --release --bin bench
```

把输出写进 `book/performance-lab.md` 的模板中。

## 常见错误

错误 1：用 debug 数字做结论。

debug 只用于开发，不用于性能判断。

错误 2：只看最快一次。

最快一次通常代表理想情况，不代表稳定吞吐。

错误 3：把 IO 和计算混在一起测。

如果包含数据读取，你不知道优化 kernel 是否有意义。

错误 4：没有正确性对照。

更快但结果错，是 bug。

错误 5：忽略噪声。

噪声高时，结论应该是 `Inconclusive`。

## 量化/HPC 连接

量化系统的性能实验通常要分层：

- kernel benchmark：rolling、dot、reduction。
- batch benchmark：多资产、多字段、多窗口。
- pipeline benchmark：data -> factor -> backtest -> report。
- system benchmark：调度、并行、存储、网络。

不要用 system benchmark 证明 kernel 优化，也不要用 kernel benchmark 代表系统吞吐。

专业开发者要能把问题拆开测。

## 本章验收

你通过本章时，应该能做到：

1. 解释 Criterion 解决什么问题。
2. 解释 benchmark 和 profiling 的区别。
3. 解释 mean、median、min、max 的意义。
4. 写出噪声过高时为什么不能下结论。
5. 运行 `factor-core` benchmark 并记录结论。
6. 在优化收益不足时主动保留 baseline。

## 进入下一章前

确认你已经理解：性能工程不是追求漂亮数字，而是建立可重复、可审查、能支持决策的证据链。下一章会进入 Rayon 式数据并行。
