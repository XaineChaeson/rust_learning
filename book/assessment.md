# 学习验收与阶段闸门

这份文件回答一个问题：你怎么知道自己不是“看过了”，而是真的学会了。

本项目的目标不是让你背 Rust 语法，而是让你能写出可维护、可测试、可测量的高性能量化计算代码。每个阶段结束时，都要通过一个闸门。没有通过闸门，就不要急着进入下一阶段。

## 使用方法

每完成一个阶段，按下面顺序验收：

1. 跑完阶段命令。
2. 做完对应练习。
3. 不看答案复述关键概念。
4. 回到代码里做一次小修改。
5. 写下“我怎么证明它是对的”。

如果只能读懂文字，但不能改代码、写测试、解释错误边界，就还没有通过。

## 评分标准

每个能力点按 0/1/2 评分：

- `0`：只能认出术语，不能解释，也不能写代码。
- `1`：能照着示例改，能跑测试，但解释不稳定。
- `2`：能独立实现、测试、解释 tradeoff，并能指出生产风险。

阶段通过标准：

- 每个核心能力点至少 `1`。
- 阶段关键能力点必须 `2`。
- 所有命令通过。
- 至少写一段复盘，说明本阶段对高性能量化系统的意义。

## Gate 0：环境与学习方法

范围：

- `book/chapters/00-rust-install-hello-world/README.md`
- `book/chapters/00-learning-method/README.md`
- `book/README.md`
- `book/progress.md`

必须能做到：

- 知道如何安装和验证 Rust 工具链。
- 知道本仓库唯一入口是 `book/README.md`。
- 知道每章只改自己章节目录下的 `example/`。
- 知道 `cargo fmt`、`cargo clippy`、`cargo test` 分别证明什么。

验收命令：

```bash
cargo run -p ch00-hello-world
cargo test
```

通过标准：

- 能解释 `rustup`、`rustc`、`cargo` 分别负责什么。
- 能解释 workspace、package、crate 的区别。
- 能说明为什么本项目不让代码示例散落在多个目录。
- 能写出自己的每周学习节奏和复盘方式。

## Gate 1：Rust 零基础到 rolling baseline

范围：

- `book/chapters/01-rust-program-shape/` 到
  `book/chapters/08-rolling-mean-from-python-to-rust/`

关键能力：

- 值、类型、函数。
- `Vec<T>`、slice、`String`、`&str`。
- ownership、borrow、mutable borrow。
- `Option`、`Result`、类型化错误。
- `struct`、`enum`、`match`。
- 数值测试和近似比较。
- 从 Python 思维迁移到 Rust baseline。

验收命令：

```bash
cargo test -p ch02-values-types-functions
cargo test -p ch04-ownership-borrowing
cargo test -p ch05-option-result-errors
cargo test -p ch08-rolling-mean
```

必须独立完成：

- 实现 `rolling_sum`。
- 写一个测试证明它和朴素窗口求和一致。
- 解释输出为什么是右对齐、长度为什么是 `n - window + 1`。

口头自测：

- Python 函数传 list 和 Rust 函数接收 `&[f64]` 有什么本质差异？
- 为什么 Rust 要区分 move、borrow、mutable borrow？
- 为什么数值测试不能直接依赖浮点完全相等？

通过标准：

- 能独立写一个接受 `&[f64]`、返回 `Result<Vec<f64>, Error>` 的小计算函数。
- 能解释至少 3 个编译器错误，而不是只复制修复。

## Gate 2：专业 Rust 核心

范围：

- `book/chapters/09-modules-crates/` 到
  `book/chapters/18-macros-features-docs/`

关键能力：

- 模块边界和 crate API。
- trait、generic、static dispatch、trait object。
- lifetime 和 borrowed view API。
- iterator、closure、零成本抽象判断。
- RAII、Drop、分配控制。
- collections、bytes、string 边界。
- 错误架构。
- thread、channel、`Arc<Mutex<T>>`。
- async 边界判断。
- macro、feature flag、doc test。

验收命令：

```bash
cargo test -p ch10-traits-generics
cargo test -p ch11-lifetimes-api-design
cargo test -p ch16-concurrency-primitives
cargo test -p ch18-macros-features-docs --features simd
```

必须独立完成：

- 为一个 rolling factor 设计 trait。
- 写一个 borrowed window view API。
- 把一个共享锁版本改成 partial result + merge。
- 给错误类型补 `Display`，并用 `?` 传播。

口头自测：

- 什么情况下 generic 比 trait object 更合适？
- lifetime 是不是“延长变量寿命”？
- 为什么 `Arc` 不等于线程安全可变状态？
- 为什么库代码不应该到处 `unwrap()`？

通过标准：

- 能设计一个小型 crate 的公开 API。
- 能说明哪些类型应该公开，哪些字段应该私有。
- 能用测试保护 API 行为。

## Gate 3：高性能计算核心

范围：

- `book/chapters/19-performance-engineering/` 到
  `book/chapters/23-simd-unsafe/`

关键能力：

- baseline、benchmark、profiling。
- memory layout、cache locality、SoA/AoS。
- numerical kernel、shape、stride。
- 数据并行和任务并行。
- SIMD、unsafe boundary、安全不变量。

验收命令：

```bash
cargo test -p ch19-performance-engineering
cargo test -p ch20-memory-layout-cache
cargo test -p ch23-simd-unsafe
cargo run -p factor-core --release --bin bench
```

必须独立完成：

- 保留一个正确 baseline。
- 写一个优化版本。
- 用 release benchmark 比较两者。
- 写出 benchmark 记录：机器、命令、数据规模、结论。

口头自测：

- 为什么 debug 模式的性能数字不能用于结论？
- 为什么热路径分配会破坏性能？
- unsafe 代码的 safety invariant 应该写给谁看？

通过标准：

- 能用数据证明一个优化有效或无效。
- 能说明优化是否改变 API、错误行为或数值结果。
- 能主动拒绝没有测量依据的“优化”。

## Gate 4：量化系统工程

范围：

- `book/chapters/24-python-ffi-boundaries/` 到
  `book/chapters/31-final-architecture/`

关键能力：

- event-driven backtesting。
- Python/Rust copy 和 zero-copy 边界。
- experiment runner 和 deterministic seed。
- online feature 与 columnar batch 的边界。
- serialization、schema、version。
- distributed task scheduling。
- observability、config、metrics。
- 最终架构分层。

验收命令：

```bash
cargo test -p ch24-python-ffi-boundaries
cargo test -p ch25-backtesting
cargo test -p ch26-experiment-monte-carlo
cargo test -p ch29-distributed-capstone
cargo test -p ch31-final-architecture
```

必须独立完成：

- 给回测加入交易成本。
- 给实验结果加入 `experiment_id` 和 seed。
- 给任务调度加入 retry attempt。
- 画出最终系统的数据流：data -> factor -> backtest -> report。

口头自测：

- 回测系统为什么不能只是一组向量公式？
- Python 绑定层为什么不应该污染 Rust 核心 crate？
- 为什么实验必须可复现？
- 分布式任务为什么需要 idempotency？
- metrics 名称为什么要稳定？

通过标准：

- 能把计算 kernel 放进系统架构中，而不是只写孤立函数。
- 能说明离线批处理、在线更新、Python 研究接口之间的边界。

## Gate 5：最终专业能力验收

最终验收不是再读一章，而是完成一个完整作品。标准见：

- [capstone.md](capstone.md)
- [performance-lab.md](performance-lab.md)
- [graduation.md](graduation.md)

验收命令：

```bash
cargo test -p quant-lab-engine
cargo run -p quant-lab-engine --bin demo
cargo run -p factor-core --release --bin bench
```

最终作品必须包含：

- 一个纯 Rust 核心 crate。
- 至少 3 个 rolling factor。
- 类型化错误。
- 单元测试和集成测试。
- 一个 release benchmark。
- 一份 benchmark 报告。
- 一份架构说明。
- 一份生产风险清单。

`projects/02-quant-lab-engine` 是当前最小作品版本。学生通过 Gate 5 时，至少要能独立修改其中一个模块，并补一条能失败后再通过的测试。

最终毕业评分使用 [graduation.md](graduation.md)。没有通过毕业表的一票否决项，不算完成本项目。

## Gate 6：生态与生产级扩展

范围：

- `book/chapters/32-criterion-profiling/` 到
  `book/chapters/37-scheduler-hardening/`

关键能力：

- Criterion 式 benchmark 判断。
- Rayon 式数据并行设计。
- PyO3/maturin Python extension 边界。
- Arrow/Parquet/Polars/DataFusion 列式模型。
- Tokio/tracing/metrics 生产 runtime 观测。
- lease/attempt/idempotent result 的 scheduler 硬化。

验收命令：

```bash
cargo test -p ch32-criterion-profiling
cargo test -p ch33-rayon-parallelism
cargo test -p ch34-python-extension-boundary
cargo test -p ch35-columnar-query-engines
cargo test -p ch36-runtime-observability
cargo test -p ch37-scheduler-hardening
cargo test -p quant-lab-engine
cargo test -p quant-lab-engine --test advanced_capstone
```

必须独立完成：

- 读懂 `quant-lab-engine/src/benchmark.rs`，并解释 benchmark decision 为什么可能是 `Inconclusive`。
- 读懂 `quant-lab-engine/src/parallel.rs`，并解释并行结果如何恢复顺序。
- 读懂 `quant-lab-engine/src/python_boundary.rs`，并解释什么时候 borrow、什么时候 copy。
- 读懂 `quant-lab-engine/src/columnar.rs`，并解释 projection/filter 如何保持行对齐。
- 读懂 `quant-lab-engine/src/observability.rs`，并解释 metrics 和 span 各自解决什么问题。
- 读懂 `quant-lab-engine/src/scheduler.rs`，并解释重复完成和结果冲突如何处理。

口头自测：

- Criterion 比手写 `Instant` 多解决了什么？
- Rayon 适合什么任务形状，不适合什么任务形状？
- PyO3 绑定层为什么应该很薄？
- Arrow schema 为什么是契约，不只是文档？
- Tokio 为什么不能直接让 CPU kernel 更快？
- 同一个 task id 重复写入不同结果为什么必须报错？

通过标准：

- 能把真实生态库映射到本项目里的 std-only 最小模型。
- 能说明何时应该引入依赖，何时应该保持手写 baseline。
- 能扩展 `projects/02-quant-lab-engine`，并保持测试、benchmark、错误边界、列式结果、观测材料和调度语义同步。

面试级自测题：

1. 解释 `&[f64]`、`Vec<f64>`、`Box<[f64]>` 在 API 中的取舍。
2. 解释 rolling mean 的朴素版和增量版复杂度。
3. 解释一个 lifetime 错误如何帮助你避免 dangling reference。
4. 解释为什么并行归约可能改变浮点结果。
5. 解释什么时候应该引入 Rayon，什么时候不应该。
6. 解释 Python zero-copy 边界中 owner 和 lifetime 的风险。
7. 解释 unsafe helper 的最小安全不变量。
8. 解释一个分布式实验任务如何做到 retry 安全。
9. 解释 Criterion、Rayon、PyO3、Arrow、Tokio、tracing 在高性能量化系统中的位置。

如果这些问题能结合代码回答，而不是只背概念，说明你已经进入专业 Rust 高性能开发者的训练区间。

## Gate 7：Production Residency

范围：

- [production-residency.md](production-residency.md)
- `projects/02-quant-lab-engine`
- 你选择的真实生态集成 lane

关键能力：

- 能把 std-only 教学模型迁移到真实 crate。
- 能保留原有 API 语义或写清楚迁移破坏点。
- 能用测试证明迁移前后核心行为一致。
- 能用 release benchmark 或运行证据证明引入依赖是必要的。
- 能写出真实生产限制，而不是只展示 happy path。

分层通过标准：

- 单条 lane 局部通过：至少完成 Rayon、PyO3/maturin、列式生态、tracing/metrics、持久化 scheduler 中的一条端到端交付。
- 本项目最终毕业通过：至少完成 Rayon、PyO3/maturin、列式生态三条 lane。
- 高水平量化技术设施岗位目标：完成 Rayon、PyO3/maturin、列式生态、tracing/metrics、持久化 scheduler 五条 lane。
- README 或设计文档必须记录：生产问题、选择的生态库、保留的核心 API、迁移后的模块边界、新增错误类型、新增测试、性能或运行证据、已知限制、下一步。

一票否决：

- 引入依赖后删除原有测试。
- 只写 demo，不写失败路径。
- 性能结论没有 release 证据。
- Python binding、列式 schema、scheduler 状态语义无法用测试或运行证据复现。

通过 Gate 7 后，你才算从“完成教学项目”进入“能独立承担真实 Rust 高性能量化基础设施模块”的水平。
