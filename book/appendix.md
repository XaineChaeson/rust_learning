# 附录：速查、评审和项目规范

这个文件集中放置辅助信息。学习主线只看 `book/chapters/`，卡住或需要自查时再看附录。

## 术语速查

Rust 核心：

- **ownership**：值由谁负责释放。
- **borrow**：函数临时使用值，不取得所有权。
- **lifetime**：借用关系的有效范围约束。
- **trait**：类型能力合同。
- **generic**：编译期参数化，常用于静态分发。
- **associated type**：绑定到 trait 实现者的类型。
- **RAII**：资源随 owner 离开作用域自动释放。
- **Send/Sync**：跨线程移动和共享的安全边界。
- **feature flag**：编译期可选能力。

高性能计算：

- **baseline**：正确、简单、可对照的第一版实现。
- **benchmark**：可重复的性能测量。
- **profiling**：定位时间和资源消耗在哪里。
- **cache locality**：访问模式是否有效利用 CPU cache。
- **SoA/AoS**：按列组织和按结构体组织数据。
- **SIMD**：单指令多数据。
- **unsafe invariant**：unsafe 代码依赖但编译器无法证明的前提。

量化系统：

- **factor engine**：批量计算因子的核心模块。
- **event loop**：按事件推进状态的执行模型。
- **ring buffer**：固定窗口增量更新常用结构。
- **columnar memory**：按列组织数据，适合分析型计算。
- **task graph**：任务依赖图。
- **idempotency**：同一任务重复执行不会造成重复副作用。

## 学习验收

每一章完成后至少满足：

1. 能解释本章核心概念。
2. 能运行对应示例。
3. 能改一个小练习。
4. 能写或读懂测试。
5. 能说明 Python 写法和 Rust 写法的差异。
6. 能说明这个知识点在高性能量化系统里的位置。

每个项目完成后至少满足：

1. `cargo test` 通过。
2. `cargo clippy --all-targets -- -D warnings` 通过。
3. API 不强迫调用方转移不必要的所有权。
4. 错误使用类型表达，不用字符串乱传。
5. 性能结论有 benchmark 或明确说明尚未测量。
6. 公共 API、错误类型、配置项有文档说明。

## Benchmark 记录模板

```text
日期：
机器：
Rust 版本：
命令：
profile：
数据规模：
baseline：
candidate：
重复次数：
结果：
噪声/方差：
结论：
下一步：
```

## 代码评审清单

Rust API：

- 只读输入是否使用 `&[T]`、`&str` 或明确的 view？
- 是否存在无意义 `clone()`？
- 库代码是否避免 `unwrap()`？
- 错误是否是类型化的？
- trait/generic 是否真的减少重复或表达边界？
- lifetime 是否表达了真实借用来源？

数值计算：

- 空输入、非法窗口、NaN、inf 是否处理？
- 浮点测试是否使用近似比较？
- rolling 输出对齐规则是否写进测试？
- 并行归约是否考虑浮点顺序差异？

性能：

- 是否区分 debug 和 release？
- 是否有 baseline？
- 是否把 IO 和计算分开测？
- 是否记录输入规模和机器环境？
- 优化后是否重新跑正确性测试？
- 是否避免热路径分配和锁竞争？

unsafe：

- unsafe 是否足够小？
- 是否有 safety 注释？
- 外部 API 是否仍然 safe？
- 是否有 benchmark 证明值得保留？
- 是否有测试覆盖边界条件？

生产工程：

- config 是否有默认值和校验？
- metrics 名称是否稳定？
- 错误日志是否保留上下文？
- 任务是否能 retry？
- 输出是否能保证不重复、不丢失？

## 生态库使用原则

先手写小版本，再引入成熟库。

推荐顺序：

1. 标准库：建立 Rust 基础。
2. `criterion`：严肃统计 benchmark。
3. `rayon`：CPU-bound 数据并行。
4. `pyo3` / `maturin`：Python 扩展。
5. `ndarray`：成熟数组结构。
6. `arrow` / `polars` / `datafusion`：列式计算和查询执行。
7. `tokio`：在线系统需要异步 IO 时再学。
8. `tracing` / `metrics`：结构化观测和生产指标。
9. `serde`：稳定序列化边界。

引入依赖前回答：

- 它解决什么问题？
- 我是否已经手写过最小版本？
- 它是否进入性能关键路径？
- 它改变 API 边界吗？
- 它是否增加 feature/test 矩阵？

## 项目规格摘要

### factor-core

已落地：`projects/01-factor-core`

必须支持：

- `rolling_mean`
- `rolling_mean_incremental`
- `rolling_std`
- `rolling_zscore`
- `rolling_min`
- `rolling_max`
- `rolling_corr`
- `rolling_beta`

验收：

- 输入使用 `&[f64]`。
- 输出右对齐。
- 有错误类型。
- 有单元测试和集成测试。
- 有 benchmark 计划和可运行 release benchmark。

### ndarray-mini

目标：

- row-major `Matrix`
- shape 检查
- row view
- transpose view/copy
- matrix-vector multiply
- reduction

### quant-py

目标：

- 用 PyO3/maturin 暴露 Rust 因子函数给 Python。
- 对齐 Python 研究流程。
- 明确 copy 和 zero-copy 边界。
- release wheel 可构建。

### backtest-core

目标：

- signal。
- order。
- trade。
- position。
- cash。
- 手续费和滑点。
- 收益、回撤、换手。

### experiment-runner

目标：

- 参数网格。
- deterministic seed。
- checkpoint。
- retry。
- result store。

### online-engine

目标：

- ingest。
- ring buffer。
- out-of-order policy。
- incremental factor。
- latency metrics。

### capstone

已落地：`projects/02-quant-lab-engine`

目标：

- Rust 核心。
- 因子引擎。
- 数值内核。
- 回测。
- 实验调度。
- 批处理 pipeline。
- 运行报告。

当前最小版本：

- `MarketSeries` 输入校验。
- `factor-core` rolling 因子复用。
- zscore mean reversion signal。
- fee-aware target weight backtest。
- deterministic experiment grid。
- benchmark decision model。
- parallel experiment grid with deterministic merge。
- Python boundary model。
- columnar experiment result batch。
- metrics and span model。
- lease/attempt/idempotent scheduler model。
- Markdown report。
- end-to-end pipeline tests。

后续扩展：

- 真实 PyO3/maturin Python wheel。
- 在线特征。
- 真实 Rayon 并行执行。
- 真实 Arrow/Polars/DataFusion 列式结果。
- 真实 Tokio/tracing/metrics runtime。
- 持久化 scheduler 和运行报告。

## 进阶生态对照

| 生态方向 | 对应章节 | 本项目最小模型 | 真实生态引入时机 |
| --- | --- | --- | --- |
| Criterion | 32 | `BenchmarkSummary` 和 `BenchmarkDecision` | 需要稳定 benchmark 报告和回归比较 |
| Rayon | 33 | partition、partial result、restore order | CPU-bound 数据并行收益被 benchmark 证明 |
| PyO3/maturin | 34 | `PythonArraySpec` 和 boundary plan | Rust kernel 要交付给 Python 研究流 |
| Arrow/Polars/DataFusion | 35 | schema、record batch、projection、filter | 数据规模需要列裁剪、lazy query 或 SQL |
| Tokio/tracing/metrics | 36 | config、span record、metric registry | 系统进入在线服务或生产运行 |
| Scheduler hardening | 37 | lease、attempt、idempotent result | 分布式任务必须容忍 worker 失败和重复完成 |
