# 最终 Capstone 规格

这份文件定义本项目的最终作品要求。学完整本书后，你应该能做出一个小而完整的高性能量化计算框架，而不是只会写零散函数。

Capstone 的目标不是追求功能数量，而是证明你具备专业 Rust 高性能开发者的核心能力：

- 能设计清晰 API。
- 能写正确数值计算。
- 能用测试锁定行为。
- 能用 benchmark 证明优化。
- 能处理错误、配置、日志和运行报告。
- 能解释 Python 研究层和 Rust 计算层的边界。

## 目标系统

系统名称是 `quant-lab-engine`，已经落地在：

```text
projects/02-quant-lab-engine
```

它至少包含下面的数据流：

```text
market data -> factor engine -> signal -> backtest -> report
```

进阶版本再加入：

```text
experiment grid -> scheduler -> factor/backtest tasks -> result store
market universe -> symbol/date partition -> deterministic task id -> scheduler
online events -> ring buffer -> incremental factor -> metrics
Python research notebook -> Rust boundary -> factor-core
```

## 最小交付范围

最小版本必须包含：

1. `factor-core`
   - rolling mean
   - rolling std
   - rolling zscore
   - rolling beta 或 rolling corr

2. `backtest-core`
   - target weight signal
   - position
   - cash
   - fee
   - equity curve

3. `experiment-runner`
   - 参数网格
   - deterministic seed
   - experiment id
   - 结果汇总

4. `performance`
   - 至少一个 baseline
   - 至少一个优化版本
   - release benchmark
   - benchmark 报告

5. `engineering`
   - 类型化错误
   - 配置校验
   - 稳定 metric 名称
   - README 和架构说明

## 当前目录

本仓库已经提供最小可运行版本：

```text
projects/02-quant-lab-engine/
  Cargo.toml
  README.md
  src/
    lib.rs
    benchmark.rs
    data.rs
    error.rs
    factor.rs
    signal.rs
    backtest.rs
    experiment.rs
    parallel.rs
    python_boundary.rs
    columnar.rs
    observability.rs
    scheduler.rs
    pipeline.rs
    report.rs
    bin/demo.rs
  tests/
    advanced_capstone.rs
    boundary_validations.rs
    pipeline.rs
    research_platform.rs
    scheduler_errors.rs
    signal_backtest.rs
```

运行：

```bash
cargo test -p quant-lab-engine
cargo run -p quant-lab-engine --bin demo
```

如果你还没完成第 31 章，不要急着扩展这个项目。先读懂它的模块边界和测试，再做练习。

## 推荐构建顺序

如果你是第一次从零写这个 capstone，不要按文件名随意实现。按下面顺序推进，每一步都先写或阅读测试，再补实现：

1. `data.rs`：先让输入边界明确。空数据、非正价格、NaN/inf、乱序时间戳都必须拒绝。
2. `data.rs`：再加入 `MarketUniverse` 和 `MarketPartition`，让 symbol/date/data version 成为任务身份的一部分。
3. `factor.rs`：只调用 `factor-core`，不把系统逻辑塞进纯计算 crate。
4. `signal.rs`：写清 rolling 输出如何用 `offset + window - 1` 对齐到市场时间戳。
5. `backtest.rs`：让 position、cash、fee、equity 的状态推进可测试。
6. `pipeline.rs` 和 `report.rs`：把 data -> factor -> signal -> backtest 串起来，但不要在编排层做复杂计算。
7. `experiment.rs`：加入参数网格、seed、稳定 experiment id，并用 strategy version + partition + data version 生成 deterministic task key。
8. `benchmark.rs`：让性能证据有 plan、samples、decision，而不是只看一次耗时。
9. `parallel.rs`：并行运行前先保留 sequential baseline，再证明输出顺序稳定。
10. `python_boundary.rs`：先建模 dtype、shape、contiguous、NaN、copy/borrow，再考虑真实 PyO3。
11. `columnar.rs`：把实验结果变成 schema + columns，先测试长度和 dtype，再做 projection/filter。
12. `observability.rs`：围绕 pipeline 加稳定 metric/span，不让观测逻辑污染 kernel。
13. `scheduler.rs`：最后加入 lease、attempt、duplicate completion、result conflict、failed 状态。

每完成一步，都至少跑：

```bash
cargo test -p quant-lab-engine
```

如果你改了性能相关代码，再跑：

```bash
cargo run -p factor-core --release --bin bench -- --len 200000 --window 252 --repeat 10
```

## API 设计要求

输入：

- 只读数值输入优先使用 `&[f64]`。
- 多列数据可以先用简单结构表示，不急着引入 Arrow 或 Polars。
- 时间、symbol、字段名不要用裸字符串到处传，至少在边界处集中处理。

输出：

- rolling 输出必须写清对齐规则。
- backtest 输出至少包含 equity curve。
- experiment 输出必须包含参数、seed、id、指标。

错误：

- 不使用 `String` 作为主要错误类型。
- 不在库代码中随意 `unwrap()`。
- 错误要能区分输入错误、配置错误、计算错误、系统错误。

## 测试要求

必须有：

- 空输入测试。
- 非法窗口测试。
- NaN/inf 测试。
- 长度不一致测试。
- rolling 对齐测试。
- 回测手续费测试。
- experiment seed 可复现测试。
- pipeline happy path 测试。

数值测试必须使用近似比较。不要因为一个浮点舍入误差让测试变成随机失败。

## 性能要求

必须至少完成一个性能实验：

- baseline：朴素 rolling mean。
- candidate：增量 rolling mean。
- 输入规模：至少 `200_000`。
- window：至少 `252`。
- repeat：至少 `10`。
- profile：release。

命令示例：

```bash
cargo run -p factor-core --release --bin bench -- --len 200000 --window 252 --repeat 20
cargo run -p factor-core --release --bin bench -- --len 200000 --window 252 --repeat 20 --output target/benchmark-reports/factor-core.md
```

报告必须回答：

- candidate 是否和 baseline 输出一致？
- speedup 是多少？
- 是否有噪声？
- 这个优化是否值得保留？
- 下一步瓶颈可能在哪里？

## 架构说明要求

最终 README 至少包含：

```text
系统目标：
模块边界：
数据流：
核心 API：
错误模型：
测试策略：
性能实验：
已知限制：
下一步：
```

模块边界必须能解释：

- Python 研究层在哪里。
- Rust 核心计算在哪里。
- 回测状态在哪里推进。
- 实验调度如何保证可复现。
- 哪些部分可以并行。
- 哪些部分暂时不做分布式。

## 当前实现如何对应专业能力

| 能力 | 当前文件 | 学生应该看懂什么 |
| --- | --- | --- |
| 数据边界 | `src/data.rs` | 输入校验、时间戳顺序、正价格约束 |
| 研究分区 | `src/data.rs`、`src/experiment.rs` | multi-asset universe、symbol/date/data version partition、deterministic task key |
| 错误架构 | `src/error.rs` | 系统错误如何包装 `factor-core` 错误 |
| 因子引擎 | `src/factor.rs` | 系统 crate 如何复用纯计算 crate |
| 信号生成 | `src/signal.rs` | rolling 输出如何按 window 对齐到时间戳 |
| 回测状态 | `src/backtest.rs` | target weight 如何推进 cash、position、fee、equity |
| 实验网格 | `src/experiment.rs` | 参数组合如何生成稳定 experiment id |
| 性能证据 | `src/benchmark.rs` | benchmark plan、samples、speedup、decision 如何形成证据链 |
| 并行实验 | `src/parallel.rs` | 如何并行运行实验并保持结果顺序稳定 |
| Python 边界 | `src/python_boundary.rs` | dtype、shape、contiguous、NaN 如何决定 copy/borrow |
| 列式结果 | `src/columnar.rs` | experiment result 如何进入 schema/projection/filter 模型 |
| 运行观测 | `src/observability.rs` | metrics 和 span 如何围绕 pipeline 设计 |
| 调度硬化 | `src/scheduler.rs` | lease、attempt、idempotent result、conflict 如何保护分布式任务 |
| 编排层 | `src/pipeline.rs` | data -> factor -> signal -> backtest -> report 的边界 |
| 验收测试 | `tests/pipeline.rs`、`tests/advanced_capstone.rs` | 如何测试端到端行为、手续费、确定性和生产级扩展 |

## 生产风险清单

最终作品必须列出风险：

- 数据质量：缺失值、重复时间戳、乱序、NaN。
- 数值风险：浮点误差、零方差、窗口对齐。
- 性能风险：热路径分配、锁竞争、cache miss。
- 系统风险：任务重复、retry 不安全、结果不可复现。
- API 风险：公开字段太多、错误类型太粗、配置默认值不清楚。

## 进阶扩展要求

第 32-37 章的最小模型已经落到 `projects/02-quant-lab-engine`：

1. 第 32 章 -> `benchmark.rs`。
2. 第 33 章 -> `parallel.rs`。
3. 第 34 章 -> `python_boundary.rs`。
4. 第 35 章 -> `columnar.rs`。
5. 第 36 章 -> `observability.rs`。
6. 第 37 章 -> `scheduler.rs`。

下一步进阶不是再写一个孤立模块，而是把这些 std-only 模型逐步替换成真实生态库，同时保持 API、测试和验收不倒退。

真实生态迁移的最终训练见 [production-residency.md](production-residency.md)。这是 capstone 之后的强制生产化阶段：你要至少选择一条真实生态 lane，把 std-only 模型替换成可审查、可测试、可复现的工程增量。

## 面试展示方式

你应该能用 10 分钟讲清：

1. 这个系统解决什么问题。
2. Rust 核心 API 为什么这样设计。
3. 你如何证明 rolling factor 正确。
4. 你如何证明优化有效。
5. Python 和 Rust 的边界在哪里。
6. 如果要扩展到多资产、多参数、分布式，下一步怎么做。

如果你只能展示代码，但说不清这些问题，说明项目还不是专业作品。
