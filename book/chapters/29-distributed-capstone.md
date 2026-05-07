# 第 29 章：分布式调度、task graph 和容错

当单机多线程不够时，你需要任务调度。学习阶段不做生产级集群，但必须掌握分布式计算的核心语义：任务如何拆、如何重试、如何保证结果不丢不重。

## task、worker、scheduler

最小模型：

- scheduler：生成任务、分配任务、记录状态。
- worker：执行任务、上报结果。
- result store：保存输出。

任务应该有稳定 ID。没有 ID，就无法判断重试任务是否已经完成。

## partition

常见分区方式：

- 按资产。
- 按日期。
- 按参数组合。
- 按 Monte Carlo seed range。

分区必须考虑数据依赖。rolling 按日期切分需要窗口 overlap；按资产切分通常更简单。

## retry 和 idempotency

worker 失败后，scheduler 会重试。重试要求任务是 idempotent：同一个 task id 重跑不会产生重复结果。

策略：

- 结果以 task id 写入。
- 写入先到临时位置，完成后原子提交。
- 已完成任务跳过。
- 失败任务保留错误上下文。

## shuffle

有些计算需要跨分区聚合，比如全市场排名。它会产生 shuffle：数据从一个分区重新分发到另一个分区。shuffle 是分布式系统最贵的阶段之一。

设计系统时要尽量减少不必要 shuffle。

## 本章示例

```bash
cargo test -p ch29-distributed-capstone
```

重点看：

- `TaskStatus` 如何表达任务生命周期。
- `run_tasks` 如何从 pending 到 succeeded。

## 本章练习

1. 给 `Task` 增加 `attempts`。
2. 模拟某个任务失败一次后重试。
3. 给结果加 task id，避免顺序依赖。
4. 写复盘：为什么 idempotency 比“跑快”更基础？

## 本章验收

你应该能解释：

- task graph 和 worker pool 的基本结构。
- partition 如何影响 correctness。
- retry 为什么要求 idempotency。
- shuffle 为什么昂贵。

## 教材化补充：分布式首先是正确性问题

很多人以为分布式的核心是“更多机器更快”。实际上，分布式首先带来更多失败模式：

- worker 进程崩溃。
- 网络中断。
- 任务重复执行。
- 部分结果写入失败。
- scheduler 状态和实际结果不一致。

如果不能处理这些问题，多机器只会让错误更难排查。

## task id 的重要性

每个任务必须有稳定 ID。ID 用来判断：

- 这个任务是否已经完成。
- 重试是否会覆盖旧结果。
- 结果属于哪个参数和分区。
- 日志和 metrics 如何关联。

没有 task id，就没有可靠 retry。

## partition 的正确性

按资产分区通常简单，因为资产独立。按时间分区需要小心 rolling window overlap。按参数分区适合实验搜索。按 seed range 分区适合 Monte Carlo。

分区方式必须由数据依赖决定，不是随便平均切。

## 示例走读

```text
book/examples/ch29-distributed-capstone/src/lib.rs
```

`TaskStatus` 是任务生命周期的最小状态机。真实系统会增加 attempts、error、started_at、finished_at、result_uri 等字段。

## 常见错误

错误 1：失败后简单重跑，但结果重复写入。

需要 idempotent result key。

错误 2：没有记录失败原因。

重试不是把错误藏起来。错误上下文必须保留。

错误 3：过早引入网络。

先在本机多进程或单进程模拟 task graph，把语义做对。

## 代码走读与容错路径

看任务状态：

```rust
pub enum TaskStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}
```

这是任务生命周期的最小模型。真实系统还需要 attempts、error message、timestamps。

看 task：

```rust
pub struct Task {
    pub id: String,
    pub payload: usize,
    pub status: TaskStatus,
}
```

`id` 是最重要字段。payload 可以变化，执行方式可以变化，但 id 决定结果如何去重和恢复。

看 `run_tasks`：

```rust
task.status = TaskStatus::Running;
results.push(...);
task.status = TaskStatus::Succeeded;
```

这个顺序表达了 scheduler 对任务状态的推进。

操作清单：

1. 增加 `attempts` 字段。
2. 模拟 payload 为 0 时失败。
3. 失败任务 attempts +1。
4. 再次运行时只重试 failed task。

专业判断：分布式调度首先是状态管理问题，其次才是网络问题。

## 自测与复盘问题

1. task id 为什么是调度系统的核心字段？
2. partition 方式如何影响 rolling correctness？
3. retry 为什么要求 idempotency？
4. shuffle 为什么通常昂贵？
5. worker 失败后 scheduler 应该保留哪些信息？

如果这些问题回答不出来，先在单进程里模拟 task graph，不要急着上多机器。

## 进入下一章前

确认你能解释 task id、attempts、status、result key 如何共同支持 retry 和 idempotency。做到这些，再进入生产化配置和观测。

## 额外复盘

把本章内容映射到大规模实验：分布式不是为了炫技，而是为了让大量任务可拆分、可恢复、可审计。你应该能说明每个任务失败时系统如何继续前进。
