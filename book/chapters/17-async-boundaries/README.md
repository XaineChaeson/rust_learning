# 第 17 章：async 边界、IO 并发和 CPU kernel

Rust async 是强大的 IO 并发模型，但它不是 CPU 加速器。量化系统同时有在线行情、网络 IO、磁盘 IO 和 CPU-heavy 因子计算。专业开发者必须知道 async 应该放在哪里，不应该放在哪里。

## async 解决什么

async 适合：

- 网络行情接入。
- 异步读写文件或对象存储。
- RPC 请求。
- 等待数据库或消息队列。
- 大量连接同时存在但多数时间在等待。

async 不适合直接解决：

- rolling kernel 更快。
- Monte Carlo 跑满 CPU。
- 矩阵乘法。
- SIMD。

CPU 计算应该进入线程池、Rayon、专门 worker 或同步 kernel。不要把长 CPU loop 放在 async runtime 的核心调度线程里。

## runtime independent core

核心计算库应该尽量不依赖 Tokio。比如：

```rust
pub fn rolling_mean(values: &[f64], window: usize) -> Result<Vec<f64>, Error>
```

这个函数可以被 CLI、Python、async service、batch job 调用。它不需要知道调用方是否 async。

系统边界可以 async：

```rust
async fn ingest_loop(...)
```

但 ingest loop 收到数据后，应该调用独立的同步核心或把 CPU 任务派发出去。

## 顺序、乱序和背压

在线特征系统不只是“收到事件就算”。它必须处理：

- duplicate event
- out-of-order event
- missing event
- burst traffic
- slow downstream

这些规则最好放在普通状态机里，而不是散在 async callback 中。这样更容易测试。

## 本章示例

```bash
cargo test -p ch17-async-boundaries
```

重点看：

- `IngestState` 是同步状态机。
- `normalize_event` 是 async 边界示意。
- 排序规则不依赖 runtime，因此容易单元测试。

## 本章练习

1. 给 `IngestState` 增加 duplicate 计数。
2. 增加 `max_gap`，超过 gap 时返回 `GapDetected`。
3. 写一个同步函数批量处理 `Vec<MarketEvent>`。
4. 写复盘：为什么 CPU kernel 不应该直接写成 async？

## 本章验收

你应该能解释：

- async 解决 IO 等待，不自动提升 CPU throughput。
- 为什么核心计算库应该 runtime independent。
- 背压和乱序为什么是在线系统核心问题。
- 哪些模块应该 async，哪些模块应该保持同步。

## 教材化补充：async 是调度模型，不是加速魔法

async 的价值在于等待 IO 时不阻塞线程。它适合行情接入、网络请求、磁盘等待、消息队列。但 rolling mean、矩阵运算、Monte Carlo path 这些 CPU-heavy 任务不会因为写成 `async fn` 就变快。

如果把长时间 CPU 循环放进 async runtime 的工作线程，反而可能阻塞其他 IO 任务。

## 核心计算为什么应该同步

核心 kernel 应该像这样：

```rust
fn compute(values: &[f64]) -> Vec<f64>
```

而不是：

```rust
async fn compute(values: &[f64]) -> Vec<f64>
```

同步函数可以被任何环境调用：CLI、Python、Tokio service、batch job、unit test。async 只应该出现在系统边界。

## 在线系统的真正难点

在线特征不是“来一条算一条”这么简单。它要处理：

- 事件重复。
- 事件乱序。
- 数据缺失。
- 下游变慢。
- 队列堆积。

这些规则应该先写成同步状态机并测试，再接入 async ingest loop。

## 示例走读

```text
book/chapters/17-async-boundaries/example/src/lib.rs
```

`IngestState` 是同步状态机，它不依赖任何 runtime。`normalize_event` 是 async 边界示意，但排序和去重规则不放在 async 细节里。

## 常见错误

错误 1：看到在线系统就立刻引入 Tokio。

先把状态机和核心计算写清楚，再决定 IO runtime。

错误 2：在 async 任务里做大量 CPU 计算。

这可能阻塞 runtime。应派发到专门线程池或同步 worker。

错误 3：把背压当成以后再说。

高吞吐系统一定会遇到下游变慢。背压策略必须早设计。

## 代码走读与操作清单

看 `MarketEvent`：

```rust
pub struct MarketEvent {
    pub sequence: u64,
    pub symbol: String,
    pub price: f64,
}
```

`sequence` 是在线系统判断乱序和重复的关键。没有 sequence，你很难知道事件是新数据还是旧数据。

看 `IngestState`：

```rust
last_sequence: Option<u64>
```

第一次没有 last sequence，所以是 `None`。收到第一条事件后，状态变成 `Some(sequence)`。

`accept` 是同步函数。它不依赖 async runtime，因此可以用普通单元测试覆盖所有排序规则。

操作清单：

1. 给 `IngestDecision` 增加 `GapDetected`。
2. 如果新 sequence 比旧 sequence 大超过 1，返回 gap。
3. 增加 duplicate 计数。
4. 写测试覆盖正常、重复、乱序、缺口。

专业判断：先把状态机写成同步纯逻辑，再把它接到 async 输入流上。

## 自测与复盘问题

1. async 解决 IO 等待还是 CPU 计算？
2. 为什么核心 rolling kernel 不应该依赖 Tokio？
3. 在线事件为什么需要 sequence？
4. duplicate、out-of-order、gap 三类事件应该如何区分？
5. 背压策略不明确会造成什么系统风险？

如果这些问题回答不出来，先把 `IngestState` 的状态转移画成表格。

## 进入下一章前

确认你能把在线系统拆成同步状态机和异步 IO 边界。你应该能说明哪些代码可以普通单元测试，哪些代码需要 runtime 或集成测试。做到这些，再进入 crate 工程质量。
