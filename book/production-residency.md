# Production Residency：从教学模型走向真实生产生态

本仓库前 37 章刻意使用 std-only 或最小依赖模型，是为了让你先看清系统边界：数据如何进入、计算如何对齐、错误如何传播、性能结论如何被证明、调度如何保证幂等。真正进入专业岗位时，你还必须证明自己能把这些模型替换成真实 Rust 生态，并保持行为、测试、性能证据不倒退。

Production Residency 是本书的最后一段训练，不再是“再读一章”，而是一次小型生产化驻场。你要在 `projects/02-quant-lab-engine` 的基础上，选择一条真实生态交付 lane，把它做成可以审查的工程增量。

## 前置条件

开始前必须已经通过：

- Gate 5：最终专业能力验收。
- Gate 6：生态与生产级扩展。
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test`
- `cargo run -p quant-lab-engine --bin demo`
- `cargo run -p factor-core --release --bin bench -- --len 200000 --window 252 --repeat 10`

如果你还不能解释当前 std-only 模块为什么这样设计，不要急着上真实 crate。直接引入生态库只会把“不会设计边界”的问题藏得更深。

## 交付目标

每条真实集成 lane 都必须包含：

1. 一个小而明确的生产问题。
2. 一个真实 Rust 生态库或工具链。
3. 一个保留不变的核心 API 或显式迁移说明。
4. 行为测试。
5. 性能或运行证据。
6. 失败边界说明。
7. README 更新。

单条 lane 的通过标准不是“能跑”，而是“别人能审查你的决策、复现你的证据、理解你的限制”。本项目的最终毕业标准更高：至少完成 Rayon、PyO3/maturin、列式生态三条 lane；如果要对标高水平量化技术设施岗位，五条 lane 都应完成。

## Lane A：Rayon 并行实验

目标：把 `src/parallel.rs` 的 std thread 教学模型迁移到 Rayon 式数据并行。

必须证明：

- 并行结果和 sequential `run_grid` 完全一致。
- 输出顺序稳定。
- worker/chunk 策略不会改变 experiment id。
- panic、错误和 partial result 的处理策略明确。
- release benchmark 说明何时并行值得，何时线程开销反而更慢。

建议验收命令：

```bash
cargo test -p quant-lab-engine --test advanced_capstone
cargo test -p quant-lab-engine --test boundary_validations
cargo run -p quant-lab-engine --bin demo
```

最终说明要回答：

- 为什么这里适合数据并行，而不是 async。
- 为什么不能直接共享一个 `Vec` 到处加锁。
- 浮点计算是否会因为并行顺序改变结果。

## Lane B：PyO3/maturin Python 扩展

目标：把 `src/python_boundary.rs` 的 copy/borrow/GIL 决策模型迁移成真实 Python extension。

必须证明：

- Rust 核心计算仍然在纯函数或小型 kernel 中。
- Python binding 层足够薄，不吞掉错误，不隐藏 copy。
- dtype、shape、contiguous、NaN 的检查和当前模型一致。
- GIL 释放只包住 CPU kernel，不包住 Python 对象访问。
- Python 端最小测试能调用 Rust 函数并验证输出。

最终说明要回答：

- 哪些输入可以 zero-copy，哪些必须 copy。
- owner/lifetime 风险在哪里。
- 为什么 Python 绑定层不应该成为业务逻辑层。

## Lane C：Arrow/Parquet/Polars/DataFusion 列式结果

目标：把 `src/columnar.rs` 的最小列式 batch 迁移到真实列式生态。

必须证明：

- schema 是契约，而不是注释。
- projection 不破坏行对齐。
- predicate/filter 不改变列长度一致性。
- 结果能落盘或被查询。
- 错误可以区分 schema 错误、数据错误和 IO 错误。

最终说明要回答：

- 为什么因子批处理适合列式。
- Arrow array 和 Rust `Vec<T>` 的边界是什么。
- 什么时候选 Polars，什么时候选 DataFusion。

## Lane D：tracing/metrics 运行观测

目标：把 `src/observability.rs` 的 in-memory metrics/span 模型迁移到真实 `tracing`/metrics 风格观测。

必须证明：

- span 名称稳定。
- metric 名称稳定。
- config validation 失败也能被记录。
- 正常 pipeline 和失败 pipeline 都有可读证据。
- 观测代码不污染数值 kernel。

最终说明要回答：

- log、span、metric 分别回答什么问题。
- 高基数字段为什么危险。
- 为什么观测不应该改变计算结果。

## Lane E：持久化 scheduler

目标：把 `src/scheduler.rs` 的 in-memory lease/attempt/idempotency 模型迁移到可持久化的任务状态。

可以使用 SQLite、Postgres、文件 WAL 或其他你能解释清楚的持久层。重点不是数据库炫技，而是任务语义。

必须证明：

- task id 决定性生成。
- lease 过期后 retry 安全。
- 同一 task 重复提交相同结果可幂等忽略。
- 同一 task 重复提交不同结果必须冲突。
- max attempts 后进入 failed。
- 重启后状态可恢复。

最终说明要回答：

- 为什么分布式任务最怕“看起来成功但结果不确定”。
- 哪些字段必须进入 task id。
- 写入结果和更新状态是否需要事务。

## 推荐顺序

如果你是 Python 数据科学背景，推荐顺序是：

1. Lane A：Rayon 并行实验。先把 CPU 离线吞吐做扎实。
2. Lane C：真实列式结果。把大规模实验输出接到分析系统。
3. Lane B：PyO3/maturin。把 Rust kernel 放回 Python 研究流。
4. Lane D：tracing/metrics。让系统可运行、可诊断。
5. Lane E：持久化 scheduler。为多机或长任务铺路。

如果你的目标是量化研究基础设施，至少完成 A、B、C。若要对标高水平平台工程岗位，五条都应完成。

## 交付文档模板

每条 lane 完成后，在项目 README 或单独设计文档中写：

```text
生产问题：
选择的生态库：
保留的核心 API：
迁移后的模块边界：
新增错误类型：
新增测试：
性能或运行证据：
已知限制：
下一步：
```

## 一票否决

出现下面任一情况，不算通过 Production Residency：

- 引入生态库后删除原有测试。
- 性能结论没有 release 证据。
- Python binding 层直接吞掉 Rust 错误。
- 并行版本和 sequential 版本结果不一致却没有解释。
- 列式 schema 只写在 README，没有进入代码或测试。
- scheduler 不能解释 duplicate completion 和 result conflict。
- README 没有记录生产风险。

Production Residency 的作用，是把你从“会做练习的人”推到“能在真实工程中做取舍的人”。这一步完成后，本仓库的训练目标才真正闭环。
