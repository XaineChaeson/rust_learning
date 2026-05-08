# 第 37 章：分布式 scheduler 的生产硬化

第 29 章讲过 task graph、partition、retry 和 idempotency。本章进入生产硬化：lease、attempt、result store、重复完成、冲突结果、失败封顶。

分布式调度不是“把任务放进队列让 worker 取”。真正困难的是失败和重复。

## 本章解决什么问题

在高性能量化平台中，分布式任务很常见：

- 多参数回测。
- 多资产因子计算。
- 多日期分区批处理。
- Monte Carlo 多路径模拟。
- 大规模数据转换。

worker 会失败：

- 进程崩溃。
- 机器重启。
- 网络中断。
- 任务超时。
- 结果写入一半。

系统也会重复：

- worker 完成后 ack 丢失。
- scheduler 重试同一任务。
- 用户重复提交。
- 下游接口超时但实际写入成功。

专业 scheduler 必须处理：

- lease。
- attempt。
- max retry。
- idempotent result。
- conflict detection。
- deterministic task id。

## 学习前提

你需要理解：

- 第 16 章：并发基础。
- 第 26 章：experiment id 和 seed。
- 第 29 章：distributed scheduling。
- 第 30 章：metrics 和 production。
- 第 36 章：observability。

如果你还不能解释 idempotency，先回到第 29 章。

## Python 对照

Python 生态中常见：

- Celery。
- Ray。
- Dask。
- Airflow。
- Prefect。

这些工具能帮你调度任务，但它们不能替你设计业务幂等性。

无论语言是什么，都要回答：

- task id 如何生成？
- retry 后结果会不会重复？
- worker 死亡如何发现？
- 结果冲突如何处理？
- attempt 是否记录？
- max retry 后如何失败？

Rust 的优势是可以用类型明确表达状态机和错误。

## 核心概念一：lease 不是 ownership

scheduler 把任务 lease 给 worker，表示：

```text
worker 在 expires_at 前有执行权
```

但 lease 不是永久 ownership。

如果 worker 没按时完成，lease 过期，任务可以重新变成 pending。

这解决 worker 崩溃问题。

本章示例：

```rust
TaskStatus::Leased { worker, attempt, expires_at }
```

## 核心概念二：attempt 是事实记录

retry 不是简单“再跑一次”。

每次执行必须有 attempt number：

- 第一次 attempt=1。
- lease 过期后 attempt=2。
- 达到 max_attempts 后任务失败。

attempt 能帮助你：

- 诊断不稳定任务。
- 防止旧 worker 用过期 attempt 覆盖结果。
- 统计 retry rate。

## 核心概念三：result store 必须幂等

假设 worker 完成任务并写入结果，但 ack 丢了。

scheduler 可能重试。

如果第二次写入同样结果，系统应该能识别 duplicate，并安全忽略。

但如果同一个 task id 写入不同结果，系统必须报冲突。

本章示例：

- 相同 result -> `DuplicateIgnored`
- 不同 result -> `ResultConflict`

这就是生产系统必须具备的防线。

## 核心概念四：deterministic task id

分布式任务 id 不能随便生成。

对于量化实验，task id 通常应该来自：

- strategy version。
- parameter set。
- symbol partition。
- date range。
- data version。
- seed。

这样重复提交同一个任务时，系统能识别它是同一个工作单元。

随机 id 会让幂等性变困难。

## 示例代码走读

示例位置：

```text
book/chapters/37-scheduler-hardening/example/src/lib.rs
```

运行：

```bash
cargo test -p ch37-scheduler-hardening
```

关键类型：

- `Task`
- `TaskStatus`
- `Scheduler`
- `CompletionOutcome`
- `SchedulerError`

这是一套最小生产化 scheduler 状态机。

## 代码走读：add_task

`add_task` 拒绝：

- 空 task id。
- `max_attempts == 0`。

任务初始状态：

```rust
TaskStatus::Pending
```

并记录：

```rust
attempts_used = 0
```

这让 scheduler 能区分“从未执行”和“执行失败后重试”。

## 代码走读：lease_next

`lease_next` 查找 pending task。

然后：

- attempt 加一。
- 写入 worker。
- 写入 expires_at。
- 返回 task id 和 attempt。

worker 完成时必须带回 attempt。

这能防止旧 lease 的结果污染新 attempt。

## 代码走读：advance_to 和 expire_leases

示例用手动时间：

```rust
advance_to(now)
```

真实系统会使用 clock。

当 lease 过期：

- 如果 attempt 没到 max，回到 pending。
- 如果 attempt 达到 max，变成 failed。

这就是 retry 上限。

## 代码走读：complete

`complete` 先检查 result store。

如果已有相同结果：

```rust
DuplicateIgnored
```

如果已有不同结果：

```rust
ResultConflict
```

如果没有结果，再检查 task 和 attempt。

只有 leased attempt 匹配，才能完成任务。

这体现了生产 scheduler 的核心安全逻辑。

## 动手操作

1. 跑测试：

```bash
cargo test -p ch37-scheduler-hardening
```

2. 增加测试：lease 过期两次后任务进入 `Failed`。

3. 增加 `worker_heartbeat` 字段。

4. 增加 metric：`task_retry_total`。

5. 设计 task id：`strategy:version:symbol:start:end:seed`。

## 常见错误

错误 1：没有 lease，只把任务标记为 running。

worker 死后任务永远卡住。

错误 2：没有 attempt。

旧 worker 可能覆盖新结果。

错误 3：结果写入不幂等。

retry 会产生重复结果。

错误 4：冲突结果沉默覆盖。

同一个 task id 产生不同结果必须报警。

错误 5：task id 随机。

随机 id 让重复提交无法识别。

## 量化/HPC 连接

分布式量化任务通常是 expensive 和 long-running。

失败不可避免。

你要保护：

- 结果不重复。
- 结果不丢失。
- 失败可重试。
- 重试有上限。
- 每次 attempt 可追踪。
- task id 可复现。

这比单机函数更接近真实基础设施。

## 本章验收

你通过本章时，应该能做到：

1. 解释 lease 和 running status 的区别。
2. 解释 attempt 的作用。
3. 设计 idempotent result store。
4. 处理 duplicate completion。
5. 处理 conflicting result。
6. 为参数搜索设计 deterministic task id。

## 课程总验收

到本章为止，本项目已经覆盖：

- Rust 核心语言。
- 高性能计算。
- Python 边界。
- 数据并行。
- 列式批处理。
- 生产 runtime。
- 分布式任务硬化。
- 最终 capstone。

从这里开始，你的进步主要来自持续扩展 `projects/02-quant-lab-engine`，并把每个扩展都写成可测试、可 benchmark、可解释的工程模块。
