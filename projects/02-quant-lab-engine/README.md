# quant-lab-engine

这是最终 Capstone 的最小可运行版本：一个 std-only 的高性能量化计算训练项目。它把前面章节学过的能力串成一条端到端 pipeline。

运行：

```bash
cargo test -p quant-lab-engine
cargo run -p quant-lab-engine --bin demo
```

## 系统目标

`quant-lab-engine` 的目标是训练一套小而完整的量化研究基础设施骨架：从市场数据进入 Rust 核心，到 multi-asset partition、rolling factor、mean reversion signal、手续费感知回测、实验网格、并行执行、列式结果、运行观测和任务调度。它不是生产系统本身，而是生产系统的可审查最小模型。

## 数据流

```text
MarketSeries -> factor set -> mean reversion signals -> fee-aware backtest -> report
```

扩展数据流：

```text
ExperimentGrid -> scheduler/parallel runner -> pipeline -> ExperimentResultBatch
MarketUniverse -> MarketPartition -> deterministic task key
Python array spec -> boundary plan -> Rust kernel
PipelineConfig -> observed run -> metrics/spans
```

## 模块边界

- `benchmark`：记录 benchmark plan、samples、speedup 和结论。
- `data`：市场数据结构、multi-asset universe、partition identity 和输入校验。
- `factor`：调用 `factor-core` 计算 rolling mean 和 rolling zscore。
- `signal`：把 zscore 转成 target weight。
- `backtest`：按信号推进 position、cash、fee、equity。
- `experiment`：展开参数网格并运行可复现实验。
- `parallel`：并行运行 experiment grid，并保持 deterministic order。
- `python_boundary`：模拟 Python/Rust binding 的 dtype、shape、layout、NaN 边界。
- `columnar`：把 experiment result 转成列式 batch，支持 projection/filter。
- `observability`：记录 pipeline metrics 和 span。
- `scheduler`：用 lease、attempt、idempotent result 管理 experiment task。
- `report`：生成训练用报告摘要。
- `pipeline`：编排端到端流程。

## 核心 API

- `MarketSeries::new`：市场数据入口，负责价格和时间戳校验。
- `MarketUniverse::new`：多资产研究入口，要求 symbol 唯一。
- `MarketSeries::partition`：把单资产序列变成带 data version 的可调度 partition。
- `ExperimentConfig::task_key`：把 strategy version、symbol/date partition、data version、seed 和参数合成 deterministic task id。
- `compute_factor_set`：系统 crate 复用 `factor-core` 的纯计算函数。
- `generate_mean_reversion_signals`：把 zscore 对齐到 timestamp，并输出 target weight。
- `run_backtest`：按信号推进 cash、position、fee、equity。
- `run_pipeline`：端到端编排，不承载具体数值算法。
- `run_grid` / `run_grid_parallel`：实验网格的 sequential baseline 和 deterministic parallel 版本。
- `evaluate_benchmark`：把 benchmark plan、样本、输出一致性变成保留或拒绝优化的决策。

## 错误模型

所有库级失败都通过 `EngineError` 表达：

- `EmptyInput`：空市场数据、空 signals、空 batch、空 benchmark samples。
- `InvalidConfig`：symbol、partition、窗口、阈值、费用、benchmark plan、projection、worker 数等配置错误。
- `InvalidMarketData`：非有限价格、非正价格、乱序时间戳、非法 zscore、Python NaN。
- `LengthMismatch`：zscore 对齐、schema/column 数量、列长度不一致。
- `MissingTimestamp`：signal 时间戳不在市场数据中。
- `Factor`：下层 `factor-core` 数值错误透传到系统 crate。

调度层使用单独的 `SchedulerError`，因为它描述的是任务状态语义：`InvalidTask`、`UnknownTask`、`AttemptMismatch`、`ResultConflict`。

## 测试策略

测试分三层：

- `tests/pipeline.rs`：端到端 pipeline、手续费、实验确定性、市场数据失败路径。
- `tests/advanced_capstone.rs`：benchmark decision、parallel order、Python boundary、columnar result、observability、scheduler happy path 和冲突路径。
- 专项测试：`signal_backtest.rs`、`scheduler_errors.rs`、`boundary_validations.rs`、`research_platform.rs` 负责锁定交易状态机、调度错误语义、列式/并行/Python 边界失败路径、多资产 partition 和 deterministic task key。

每次修改后至少运行：

```bash
cargo test -p quant-lab-engine
```

如果改了 workspace 公共行为，再运行：

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test
```

## 性能实验

本项目的性能证据分两层：

- `projects/01-factor-core/src/bin/bench.rs`：真实 release benchmark，比较 rolling mean baseline 和 incremental candidate。
- `src/benchmark.rs`：benchmark evidence model，把样本数、噪声、输出一致性、最小 speedup 合成 `CandidateWins`、`KeepBaseline` 或 `Inconclusive`。

推荐命令：

```bash
cargo run -p factor-core --release --bin bench -- --len 200000 --window 252 --repeat 10
```

性能结论必须说明机器、命令、输入规模、window、repeat、输出是否一致、speedup、是否保留优化。

## 教学目标

完成本项目后，学生应该能解释：

1. 为什么计算核心使用 borrowed input。
2. 为什么因子 crate 和系统 crate 分开。
3. rolling 输出如何和时间戳对齐。
4. target weight 如何变成持仓和现金变化。
5. 手续费如何影响最终 equity。
6. 参数网格为什么必须有稳定 experiment id。
7. 测试如何覆盖数据、计算、回测和实验边界。
8. 如何把 Criterion/Rayon/PyO3/Arrow/Tokio/tracing 的核心思想落到项目中。
9. 为什么 scheduler 必须处理 lease、attempt、duplicate completion 和 result conflict。

## 已知限制

- 没有外部数据读取。
- 没有真实 PyO3/maturin wheel，只实现了 Python boundary 设计模型。
- 没有真实 Rayon 线程池，只实现了 std thread 的 deterministic merge。
- 没有真实 Arrow/Polars/DataFusion，只实现了列式 batch 最小模型。
- 没有真实 Tokio/tracing/metrics 集成，只实现了可测试的观测模型。
- 没有真实分布式执行，只实现了 scheduler 状态机和幂等结果模型。
- 没有 Criterion 统计报告，只实现了 benchmark decision model。

这些限制是刻意保留的：当前版本用于训练架构、边界和测试。学生掌握后，再把每个 std-only 模块替换成真实生态库。

## 下一步

完成本项目后，进入 `book/production-residency.md`：

1. 用 Rayon 替换 `parallel.rs` 的 std thread 模型。
2. 用 PyO3/maturin 把一个 factor 暴露给 Python。
3. 用 Arrow/Parquet/Polars/DataFusion 替换 `columnar.rs` 的最小 batch 模型。
4. 用 tracing/metrics 替换 `observability.rs` 的 in-memory 观测模型。
5. 给 `scheduler.rs` 加持久化状态，证明重启后 retry/idempotency/conflict 仍然正确。
