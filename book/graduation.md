# 毕业验收：专业 Rust 高性能量化开发者

这份文件是本项目的最终验收表。它回答一个最关键的问题：

> 学完本仓库后，你是否已经具备专业 Rust 高性能开发者的实操能力？

不是读完章节就毕业。毕业条件是：你能独立修改、测试、优化、解释并审查一个 Rust 高性能量化系统。

## 使用方式

完成所有章节和 `projects/02-quant-lab-engine` 后，按本文件验收。

你需要提交一份“毕业包”：

```text
1. 代码：通过所有质量门
2. 测试：说明新增或修改的测试覆盖什么
3. 性能报告：至少一个 release benchmark
4. 架构说明：说明模块边界和数据流
5. 风险清单：说明生产化限制和下一步
6. 口头讲解：10-15 分钟讲清系统
```

## 一票否决项

出现下面任一情况，不算毕业：

- `cargo test` 不通过。
- `cargo clippy --all-targets -- -D warnings` 不通过。
- 不能解释 `&[f64]`、`Vec<f64>`、ownership、borrow 的 API 含义。
- 性能结论没有 release benchmark。
- 优化版本没有和 baseline 做正确性对照。
- capstone 只能运行，不能解释模块边界。
- 错误处理主要依赖 `unwrap()` 或字符串错误。
- 无法说明 Python/Rust 边界是否 copy。
- 无法说明并行结果如何保持可复现。
- 无法说明 scheduler 如何处理重复完成或结果冲突。

## 评分总表

总分 100。达到 85 分以上，且没有一票否决项，才算达到本项目毕业标准。

| 模块 | 分值 |
| --- | ---: |
| Rust 核心 API 能力 | 15 |
| 数值计算和测试能力 | 15 |
| 性能工程能力 | 15 |
| 系统工程能力 | 20 |
| 生态边界能力 | 15 |
| capstone 作品能力 | 20 |

## 1. Rust 核心 API 能力：15 分

验收点：

- 3 分：能解释 workspace、package、crate、bin、lib、test。
- 3 分：能为只读输入选择 `&[T]`、`&str`，而不是无意义取得所有权。
- 3 分：能解释 move、borrow、mutable borrow。
- 2 分：能设计 `struct` / `enum` 表达业务状态。
- 2 分：能使用 `Result` 和类型化错误表达失败。
- 2 分：能解释 trait/generic/lifetime 在本项目中的实际位置。

必须能讲的代码：

- `projects/01-factor-core/src/lib.rs`
- `projects/02-quant-lab-engine/src/error.rs`
- `projects/02-quant-lab-engine/src/data.rs`

## 2. 数值计算和测试能力：15 分

验收点：

- 3 分：能解释 rolling 输出对齐规则。
- 3 分：能处理空输入、非法窗口、NaN、inf、长度不一致。
- 3 分：能写近似浮点比较。
- 3 分：能解释 baseline 和优化版本的正确性对照。
- 3 分：能为回测手续费、equity、drawdown 写测试。

必须能讲的代码：

- `projects/01-factor-core/tests/rolling_factors.rs`
- `projects/02-quant-lab-engine/tests/pipeline.rs`

## 3. 性能工程能力：15 分

验收点：

- 3 分：能区分 debug 和 release。
- 3 分：能设计 benchmark plan。
- 3 分：能解释 samples、mean、median、noise、speedup。
- 3 分：能说明一个优化是否值得保留。
- 3 分：能指出下一步 profiling 应该看哪里。

必须运行：

```bash
cargo run -p factor-core --release --bin bench -- --len 20000 --window 252 --repeat 3
```

必须能讲的代码：

- `projects/01-factor-core/src/bin/bench.rs`
- `projects/02-quant-lab-engine/src/benchmark.rs`

## 4. 系统工程能力：20 分

验收点：

- 3 分：能解释 `MarketSeries -> factor -> signal -> backtest -> report`。
- 3 分：能解释 target weight 如何变成 position、cash、fee、equity。
- 3 分：能解释 experiment grid 如何保证可复现。
- 3 分：能解释 parallel grid 如何保持结果顺序稳定。
- 3 分：能解释 metrics 和 span 分别解决什么。
- 3 分：能解释 scheduler lease、attempt、idempotent result。
- 2 分：能说明当前 capstone 的生产限制。

必须能讲的代码：

- `projects/02-quant-lab-engine/src/pipeline.rs`
- `projects/02-quant-lab-engine/src/backtest.rs`
- `projects/02-quant-lab-engine/src/experiment.rs`
- `projects/02-quant-lab-engine/src/parallel.rs`
- `projects/02-quant-lab-engine/src/observability.rs`
- `projects/02-quant-lab-engine/src/scheduler.rs`

## 5. 生态边界能力：15 分

验收点：

- 3 分：能解释 Criterion 何时替换当前 benchmark model。
- 3 分：能解释 Rayon 何时替换 std thread model。
- 3 分：能解释 PyO3/maturin 绑定层为什么应该很薄。
- 3 分：能解释 Arrow/Polars/DataFusion 的 schema、projection、filter。
- 3 分：能解释 Tokio/tracing/metrics 进入系统的位置。

必须能讲的代码：

- `projects/02-quant-lab-engine/src/python_boundary.rs`
- `projects/02-quant-lab-engine/src/columnar.rs`
- `book/chapters/32-criterion-profiling.md`
- `book/chapters/37-scheduler-hardening.md`

## 6. Capstone 作品能力：20 分

验收点：

- 4 分：能独立新增一个功能并写测试。
- 4 分：能独立修复一个失败测试。
- 4 分：能解释所有模块边界。
- 4 分：能写一份 benchmark 或性能实验报告。
- 4 分：能写一份生产风险和下一步计划。

最低修改任务：

从下面任选 2 个完成：

1. 给 `backtest.rs` 增加最大单次换手限制。
2. 给 `experiment.rs` 增加 `initial_cash` 参数。
3. 给 `columnar.rs` 增加按 `fee_bps` 过滤。
4. 给 `observability.rs` 增加失败 pipeline 的错误 span。
5. 给 `scheduler.rs` 增加 max attempts 后进入 `Failed` 的测试。
6. 给 `python_boundary.rs` 增加 shape 与 values length 一致性约束。

## 毕业命令

最终必须运行：

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run -p quant-lab-engine --bin demo
cargo run -p factor-core --release --bin bench -- --len 20000 --window 252 --repeat 3
```

全部通过后，填写：

```text
Rust API 我能解释的三个设计：
我新增或修改的功能：
我新增或修改的测试：
benchmark 命令和结果：
capstone 当前生产限制：
下一步真实生态替换计划：
我还不确定的地方：
```

## 面试展示脚本

用 10-15 分钟讲：

1. 我为什么用 Rust 做这个系统。
2. `factor-core` 和 `quant-lab-engine` 为什么分开。
3. 数据如何从 market data 进入 factor。
4. signal 如何进入 backtest。
5. benchmark 如何证明优化。
6. Python boundary 如何设计 copy/borrow。
7. columnar result 为什么适合实验结果。
8. scheduler 如何处理 worker failure。
9. 当前系统还缺什么生产能力。

如果你能不看稿讲清这些，并能现场改一个小功能、补一个测试、跑完整质量门，就达到了本项目的培养目标。
