# 第 36 章：Tokio、tracing、metrics 和生产可观测性

第 17 章讲过 async 边界，第 30 章讲过 observability 和 config。本章进入真实生产生态：Tokio、tracing、metrics。

高性能系统不是只要算得快。它必须能运行、能诊断、能限流、能暴露状态、能在出错时定位原因。

## 本章解决什么问题

一个量化系统上线后会遇到：

- 数据源延迟。
- 事件乱序。
- 任务堆积。
- 下游写入失败。
- 配置错误。
- CPU 飙高。
- 内存增长。
- 某个 symbol 结果异常。

如果系统没有观测能力，你只能猜。

专业 Rust 服务需要：

- structured logs。
- spans。
- metrics。
- config validation。
- backpressure。
- error context。

Tokio 解决 async IO runtime。

tracing 解决结构化事件和 span。

metrics 解决可聚合指标。

本章示例用 std-only 模型训练这些概念。

## 学习前提

你需要理解：

- 第 15 章：错误架构。
- 第 17 章：async 适合 IO，不适合直接加速 CPU kernel。
- 第 27 章：在线特征和 backpressure。
- 第 30 章：config、metrics、production。

如果你还不能区分 CPU parallelism 和 async IO concurrency，先回到第 17 章。

## Python 对照

Python 服务中常见：

- asyncio。
- logging。
- Prometheus client。
- FastAPI middleware。
- structlog。

Rust 对应生态：

- Tokio：async runtime。
- tracing：structured instrumentation。
- metrics 或 opentelemetry：指标和导出。
- tower：middleware/service 抽象。

但语言差异很重要：

- Rust 的 async 是编译期状态机。
- CPU-bound kernel 不应该堵在 async executor 中。
- 错误类型和 config 类型更适合静态校验。

## Tokio 是什么

Tokio 是 Rust 最主流的 async runtime。

它适合：

- 网络 IO。
- 文件或 socket 异步。
- 定时任务。
- channel。
- 服务编排。

它不适合直接让 CPU kernel 更快。

如果你把一个长时间 rolling computation 放进 Tokio task，但不做 blocking 隔离，可能拖慢 runtime。

专业设计：

```text
async IO ingest
  -> bounded queue
  -> CPU worker pool or Rayon
  -> async output
```

## tracing 是什么

`tracing` 不只是 logging。

它可以表达：

- span：一次操作的上下文。
- event：span 中发生的事件。
- fields：结构化字段。

例子：

```text
span: ingest_event
fields: symbol=BTC, sequence=1001, bytes=512
event: accepted
```

这比纯字符串日志更适合生产诊断。

## metrics 是什么

metrics 是可聚合数字。

常见类型：

- counter：累计次数。
- gauge：当前值。
- histogram：分布。

量化系统常见指标：

- events_accepted_total。
- events_dropped_total。
- bytes_accepted_total。
- factor_latency_seconds。
- queue_depth。
- task_retry_total。

指标名称要稳定，因为 dashboard 和 alert 依赖它。

## 核心概念一：config 是生产接口

配置不是随便的 struct。

它会影响系统行为：

- 最大并发。
- queue size。
- retry 次数。
- timeout。
- feature 开关。

配置必须验证。

本章示例的 `RuntimeConfig::validate` 拒绝：

- 空 service name。
- `max_in_flight == 0`。

真实系统中还要检查路径、端口、凭据、阈值关系。

## 核心概念二：span 要带上下文

坏日志：

```text
failed
```

好事件：

```text
span=ingest_event event_id=100 bytes=512 decision=accept
```

上下文让你能回答：

- 哪个事件失败？
- 哪个 symbol 失败？
- 哪个参数组合失败？
- 哪个 worker 失败？

本章示例用 `SpanRecord` 模拟 tracing span。

## 核心概念三：metrics 用于系统级判断

单条日志适合定位个案。

metrics 适合看整体趋势：

- drop rate 是否上升。
- queue 是否积压。
- latency 是否恶化。
- retry 是否异常。

本章示例的 `MetricRegistry` 记录 counters。

真实系统会把这些指标导出给 Prometheus、OpenTelemetry 或内部监控系统。

## 示例代码走读

示例位置：

```text
book/examples/ch36-runtime-observability/src/lib.rs
```

运行：

```bash
cargo test -p ch36-runtime-observability
```

关键类型：

- `RuntimeConfig`
- `RuntimeEvent`
- `MetricRegistry`
- `SpanRecord`
- `RuntimeReport`

这些类型组成一个最小在线处理模型。

## 代码走读：process_events

`process_events` 做四件事：

1. 验证 config。
2. 遍历事件。
3. 对每个事件生成 span。
4. 记录 accepted/dropped metrics。

它拒绝：

- bytes 为 0 的事件。
- 超过 `max_in_flight` 的事件。

这模拟 backpressure。

## 动手操作

1. 跑测试：

```bash
cargo test -p ch36-runtime-observability
```

2. 增加 `queue_depth` gauge。

3. 增加 `events_seen_total` counter。

4. 给 `SpanRecord` 增加 `service_name` 字段。

5. 设计 `factor_latency_seconds` histogram 的记录点。

## 常见错误

错误 1：只有 println。

生产系统需要结构化上下文。

错误 2：metrics 名称随意改。

dashboard 和 alert 会被破坏。

错误 3：config 不验证。

错误配置应该启动前失败，而不是运行中产生隐蔽错误。

错误 4：CPU kernel 堵住 async runtime。

CPU-bound 工作应该隔离到专用线程池或数据并行执行。

错误 5：日志包含结果却没有输入 id。

没有 id 就无法追溯。

## 量化/HPC 连接

在线量化系统必须知道：

- 数据是否延迟。
- 是否丢事件。
- 因子计算耗时。
- 队列是否积压。
- 每个任务 retry 次数。
- 每次实验配置是什么。

这些不是附属功能，而是系统能否进入生产的条件。

## 本章验收

你通过本章时，应该能做到：

1. 解释 Tokio 解决 IO concurrency，不直接加速 CPU kernel。
2. 解释 tracing span 和普通 log 的区别。
3. 设计稳定 metric 名称。
4. 写 config validation。
5. 用 metrics 判断系统是否积压。
6. 说明 CPU worker 和 async ingest 的边界。

## 进入下一章前

确认你能为一个在线 feature service 设计 config、metrics 和 span 字段。下一章进入分布式 scheduler 的生产硬化。
