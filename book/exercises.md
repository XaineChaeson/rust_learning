# 练习集

练习集中在这里，但代码修改必须进入对应章节的 `book/examples/chXX-*/` 或指定项目。不要跨章乱改。

## 使用规则

每个练习按三步完成：

1. 找位置：`chXX` 练习只改 `book/examples/chXX-*/`；项目练习只改指定的 `projects/` 目录。
2. 写测试：能写测试的练习必须先补测试或补断言，再实现。
3. 跑命令：章节练习跑 `book/progress.md` 对应命令；项目练习跑项目级命令。

常用映射：

| 练习类型 | 修改位置 | 验证命令 |
| --- | --- | --- |
| `ch01`-`ch37` | `book/examples/chXX-*/` | `cargo test -p chXX-name` 或进度表命令 |
| `factor-core` | `projects/01-factor-core/` | `cargo test -p factor-core` 和必要的 `cargo run -p factor-core --release --bin bench` |
| `quant-lab-engine` | `projects/02-quant-lab-engine/` | `cargo test -p quant-lab-engine` 和 `cargo run -p quant-lab-engine --bin demo` |
| Production Residency | 以 `projects/02-quant-lab-engine` 为基础的新集成 lane | 见 `book/production-residency.md` |

完成练习后，把 `book/progress.md` 对应行更新到 `coded` 或 `done`，并写清楚你证明了什么。

## A. Rust 入门计算

1. `ch01`：给 `variables.rs` 增加学习小时数变量并打印。
2. `ch02`：实现 `min`、`max`、`cumulative_product`。
3. `ch03`：实现 `take_last(values, n)`，返回 borrowed slice。
4. `ch04`：实现 `add_in_place(values: &mut [f64], delta: f64)`。
5. `ch05`：给价格序列增加 NaN、inf、非正价格验证。
6. `ch06`：给 `Signal` 增加 `ClosePosition`。
7. `ch07`：写 `assert_close_vec` 并覆盖长度不一致。
8. `ch08`：实现增量 `rolling_sum`，证明和朴素版一致。

## B. 专业 Rust 核心

1. `ch09`：新增 `risk` 模块，保持字段私有。
2. `ch10`：新增 `MaxFactor` 和 `RollingFactor` trait。
3. `ch11`：实现 `first_window<'a>`，返回 borrowed view。
4. `ch12`：用 `fold`、`scan` 各实现一个计算函数。
5. `ch13`：把一个热路径函数改成复用 scratch buffer。
6. `ch14`：实现 symbol count 和 timestamp range sum。
7. `ch15`：设计上层 `DataError`，用 `From` 包装下层错误。
8. `ch16`：把锁版本改成 partial result + merge。
9. `ch17`：给 ingest 状态机增加 duplicate/gap 统计。
10. `ch18`：增加一个 feature flag 和一个 doc test。

## C. 高性能计算核心

1. `ch19`：为 rolling sum 写 benchmark 计划。
2. `ch20`：实现 `returns_from_close`，解释 SoA 的收益。
3. `ch21`：实现 `row()`、`transpose_copy()`。
4. `ch22`：让并行输出携带 index 并恢复顺序。
5. `ch23`：给 unsafe helper 写完整 safety invariant。

## D. 量化系统工程

1. `ch24`：比较 copy boundary 和 borrowed kernel。
2. `ch25`：给回测加入交易成本。
3. `ch26`：给实验参数加入 seed 和 experiment id。
4. `ch27`：设计乱序事件处理策略。
5. `ch28`：给序列化 record 增加 schema/version。
6. `ch29`：给任务调度增加 retry attempt。
7. `ch30`：增加 `max_retries` 配置和 metric。
8. `ch31`：设计一个最小端到端 pipeline：data -> factor -> backtest -> report。

## E. 项目练习

在 `projects/01-factor-core` 中：

1. 给 `rolling_mean` 增加窗口大于输入长度的测试。
2. 给 `rolling_corr` 增加零方差测试。
3. 给 `rolling_beta` 增加长度不一致测试。
4. 为 `FactorSeries` 增加构造函数测试。
5. 跑 `cargo run -p factor-core --release --bin bench`，记录 baseline、candidate、speedup。
6. 写一份 benchmark 计划，说明输入规模、baseline、candidate、目标指标。

## F. Capstone 练习

在 `projects/02-quant-lab-engine` 中：

1. 给 `data.rs` 增加重复 symbol 或空 symbol 的校验。
2. 给 `signal.rs` 增加 long-only 模式。
3. 给 `backtest.rs` 增加最大单次换手限制。
4. 给 `experiment.rs` 增加 `initial_cash` 参数。
5. 给 `report.rs` 增加一行 Markdown 输出：`total_fees`。
6. 给 `tests/pipeline.rs` 增加一个失败用例，再实现代码让它通过。
7. 运行 `cargo test -p quant-lab-engine` 和 `cargo run -p quant-lab-engine --bin demo`。
8. 写一份 10 分钟讲解稿，解释每个模块边界。

## G. 生态与生产级扩展练习

1. `ch32`：给 benchmark 判断增加最小样本数规则。
2. `ch33`：实现 `mean_threaded(values, partitions)`。
3. `ch34`：给 Python boundary 增加 `shape` 校验。
4. `ch35`：增加 `ColumnType::Utf8` 和 projection/filter 测试。
5. `ch36`：增加 `events_seen_total` 和 `queue_depth` 指标。
6. `ch37`：增加 max attempts 后进入 `Failed` 的测试。
7. `quant-lab-engine`：给 `benchmark.rs` 增加最小样本数配置。
8. `quant-lab-engine`：给 `parallel.rs` 增加 worker 数过大时的测试。
9. `quant-lab-engine`：给 `python_boundary.rs` 增加二维 shape 与 values length 的一致性约束。
10. `quant-lab-engine`：给 `columnar.rs` 增加按 `fee_bps` 过滤。
11. `quant-lab-engine`：给 `observability.rs` 增加失败 pipeline 的错误 span。
12. `quant-lab-engine`：给 `scheduler.rs` 增加 max attempts 后进入 `Failed` 的测试。
13. 写一份生态引入决策：什么时候把这些 std-only 模型替换成 Criterion、Rayon、PyO3、Polars/DataFusion、Tokio/tracing。

## H. Production Residency 练习

这些练习不在章节 example 中完成，而是在 `projects/02-quant-lab-engine` 的基础上做真实生态交付。完整要求见 [production-residency.md](production-residency.md)。

1. 选择 Rayon lane：把并行实验迁移到真实数据并行，并证明 sequential/parallel 输出一致。
2. 选择 PyO3/maturin lane：暴露一个 rolling factor 给 Python，并保留 dtype、shape、copy/borrow、错误边界。
3. 选择列式 lane：把 experiment result 接到真实列式数据结构或文件格式，并测试 schema/projection/filter。
4. 选择 tracing/metrics lane：把 pipeline 正常和失败路径都变成可观测证据。
5. 选择 scheduler lane：加入持久化状态，证明 lease、attempt、retry、conflict 在重启后仍然正确。
6. 写迁移报告：说明为什么引入这个生态库，以及它替换了哪个 std-only 模型。

## I. 每章复盘模板

```text
本章核心概念：
它解决了什么专业工程问题：
Python/NumPy/PyTorch 中对应的做法：
Rust 中更严格或更高效的地方：
我写了什么代码：
测试覆盖了什么：
如果要用于生产，还缺什么：
```
