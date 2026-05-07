# 第 30 章：observability、config 和 production readiness

能在自己机器上跑通，不等于能放进研究平台或生产设施。专业基础设施必须可配置、可观测、可诊断、可回滚。

## config

配置不是散落的常量。配置应该有：

- 明确类型。
- 默认值。
- 校验。
- 环境覆盖策略。
- 文档。

比如 worker 数不能为 0，batch size 不能为 0。这类错误应该启动时失败，而不是运行半小时后卡死。

## metrics

高性能系统需要稳定指标名：

- rows_per_second
- batch_latency_ms
- queue_depth
- dropped_events
- retry_count
- nan_rejected_total

指标名一旦被 dashboard 或 alert 依赖，也是一种 API。

## logging 和 tracing

日志回答“发生了什么”。trace 回答“请求经过哪些阶段”。量化批处理也需要 trace：一个实验包含哪些任务、每个任务耗时、失败原因是什么。

热路径日志要谨慎。逐行打印会毁掉性能。

## failure mode

生产系统设计要提前回答：

- 输入有 NaN 怎么办？
- worker panic 怎么办？
- 输出目录已有结果怎么办？
- config 错误怎么办？
- benchmark 变慢怎么办？

## 本章示例

```bash
cargo test -p ch30-observability-config
```

重点看：

- `EngineConfig::validate` 如何把错误前置。
- `Metric` 如何保持稳定名称。

## 本章练习

1. 给 `EngineConfig` 增加 `max_retries` 并校验。
2. 增加 `nan_rejected_total` metric。
3. 设计一份运行报告模板。
4. 写复盘：为什么 metrics 名称也是系统合同？

## 本章验收

你应该能解释：

- config 校验为什么要在启动时完成。
- metrics、log、trace 的区别。
- 热路径日志为什么危险。
- production readiness 不只是代码正确。

## 教材化补充：能跑不等于能运维

研究代码在自己电脑上跑通，只说明逻辑可能正确。基础设施还需要回答：

- 出错时能不能定位？
- 变慢时能不能证明哪里慢？
- 配置错了能不能提前失败？
- 任务失败能不能恢复？
- API 改了会不会影响调用者？

这些问题属于 production readiness。

## config 应该类型化

不要让配置散落成魔法常量：

```rust
let workers = 8;
let batch = 10000;
```

应该有明确结构：

```rust
EngineConfig {
    worker_threads,
    batch_size,
    fail_on_nan,
}
```

并在启动时校验。错误配置越早失败越好。

## metrics 是系统合同

指标名一旦被 dashboard、alert、自动报告依赖，就变成合同。随意改名会破坏运维系统。

常见指标：

- rows_per_second
- batch_latency_ms
- queue_depth
- retry_count
- rejected_nan_total

## 示例走读

```text
book/examples/ch30-observability-config/src/lib.rs
```

`EngineConfig::validate` 把配置错误前置。`Metric` 用稳定名称表达吞吐。

## 常见错误

错误 1：运行一半才发现配置不合法。

启动时就应该 validate。

错误 2：热路径逐条日志。

日志 IO 会毁掉性能。热路径应该用聚合指标。

错误 3：panic 当错误处理。

基础设施应尽量返回可诊断错误，除非进入不可恢复状态。

## 代码走读与运行报告

看配置：

```rust
pub struct EngineConfig {
    pub worker_threads: usize,
    pub batch_size: usize,
    pub fail_on_nan: bool,
}
```

这些字段都会影响系统行为，因此不能散落在代码中。

看校验：

```rust
if self.worker_threads == 0 {
    return Err(ConfigError::ZeroWorkers);
}
```

非法配置启动时失败。不要让一个 worker 数为 0 的系统运行到一半才表现异常。

看指标：

```rust
Metric {
    name: "rows_per_second",
    value,
}
```

指标名是稳定合同。dashboard 和告警会依赖它。

操作清单：

1. 增加 `max_retries`。
2. 增加 `ConfigError::ZeroRetries` 或允许 0 并解释语义。
3. 增加 `batch_latency_ms` metric。
4. 写一个运行报告模板，包含 config、input rows、elapsed、throughput、errors。

专业目标：别人不用读源码，也能知道系统如何运行、为什么失败、性能如何。
