# 专业 Rust 高性能量化开发路线

目标不是“会写 Rust 语法”，而是能用 Rust 构建可测试、可维护、可测量、可扩展的高性能计算基础设施。

本路线把知识分成三层：

1. **Rust 专业核心**：任何严肃 Rust 开发都必须熟练掌握。
2. **高性能计算核心**：数值内核、内存布局、并行、SIMD、性能证据。
3. **量化系统工程**：Python 接入、回测、实验、在线特征、列式批处理、分布式调度、生产化。

验收标准：

- 能设计库 API，而不是只写脚本函数。
- 能解释所有权、生命周期、trait、泛型和并发边界。
- 能写出正确、可测、可 benchmark 的数值 kernel。
- 能把 Rust 核心接入 Python 研究流。
- 能处理大规模离线计算、在线增量计算和任务调度。
- 能用工程证据支撑性能结论。

## 阶段 A：从 Python 数据科学到 Rust 计算函数

| 章节 | 主题 | 核心验收 |
| --- | --- | --- |
| 准备 | 安装 Rust 与 Hello World | 会安装/验证工具链，能运行第一个 package |
| 00 | 学习方法 | 会按“读、跑、改、测、复盘”学习 |
| 01 | Rust 程序形状 | 会运行 workspace、package、bin、test |
| 02 | 值、类型、函数 | 会写纯计算函数和返回契约 |
| 03 | `Vec`、slice、`String` | 会用借用视图避免无意义拷贝 |
| 04 | 所有权和借用 | 能判断数据归属和 API 入参形式 |
| 05 | `Option`、`Result` | 会表达失败，而不是用魔法值 |
| 06 | `struct`、`enum`、`match` | 会用类型建模市场数据和计算状态 |
| 07 | 数值测试 | 会测试浮点数、边界、失败路径 |
| 08 | rolling mean | 能把 Python rolling 迁移成 Rust baseline |

## 阶段 B：Rust 专业核心

| 章节 | 主题 | 核心验收 |
| --- | --- | --- |
| 09 | modules、crates、visibility | 能设计 crate 边界和公开 API |
| 10 | traits、generics、associated types | 能写零成本抽象和可替换计算组件 |
| 11 | lifetime 深入 | 能设计返回借用的 view API |
| 12 | iterators、closures | 能写可组合的计算管线而不牺牲性能 |
| 13 | memory、Drop、RAII | 理解资源释放、拷贝、移动和堆分配 |
| 14 | collections、bytes | 能选择 `HashMap`、`BTreeMap`、字节/字符串边界 |
| 15 | error architecture | 能设计库级错误类型和转换链 |
| 16 | concurrency primitives | 理解 `Send`、`Sync`、`Arc`、`Mutex`、channel |
| 17 | async boundaries | 知道 async 适合 IO，不把 CPU kernel 写进 runtime 热路径 |
| 18 | macros、features、docs、release | 会维护 crate 工程质量和编译配置 |

## 阶段 C：高性能计算核心

| 章节 | 主题 | 核心验收 |
| --- | --- | --- |
| 19 | 性能工程 | 所有优化都有 benchmark/profiling 证据 |
| 20 | 内存布局、cache、allocation | 能解释 SoA/AoS、连续内存、cache locality |
| 21 | 小型 ndarray / numerical kernel | 能实现 shape、stride、view、reduction |
| 22 | 并行计算 | 能按任务形状选择线程、chunk、归约策略 |
| 23 | SIMD 和 unsafe 边界 | 能用安全 API 包住 unsafe kernel，并写不变量 |

## 阶段 D：量化系统工程

| 章节 | 主题 | 核心验收 |
| --- | --- | --- |
| 24 | Python/Rust FFI | 能解释 GIL、copy、zero-copy、wheel、边界成本 |
| 25 | 回测引擎 | 能设计状态机、订单、成交、费用、组合权益 |
| 26 | 实验和 Monte Carlo | 能做可复现实验、参数搜索、并行模拟 |
| 27 | 在线特征和列式计算 | 能同时理解低延迟增量和离线列式吞吐 |
| 28 | 存储和序列化 | 能选择 CSV、binary、Arrow/Parquet 边界 |
| 29 | 分布式调度 | 能设计 task graph、partition、retry、idempotency |
| 30 | observability、config、production | 能让系统可运行、可诊断、可演进 |
| 31 | 最终架构 | 能整合 factor、kernel、Python、backtest、online、batch、scheduler |

## 阶段 E：生态与生产级扩展

| 章节 | 主题 | 核心验收 |
| --- | --- | --- |
| 32 | Criterion 和 profiling | 能把性能结论变成可审查证据 |
| 33 | Rayon 式数据并行 | 能把任务切分、partial result、deterministic merge 设计清楚 |
| 34 | PyO3/maturin Python 扩展 | 能设计薄绑定层、copy/zero-copy 边界和 GIL 策略 |
| 35 | Arrow/Parquet/Polars/DataFusion | 能解释 schema、projection、predicate filtering 和列式批处理 |
| 36 | Tokio/tracing/metrics | 能设计 async IO 边界、结构化 span、稳定 metrics 和 config validation |
| 37 | 分布式 scheduler 硬化 | 能设计 lease、attempt、idempotent result、冲突检测和 retry 上限 |

## 阶段 F：Production Residency

阶段 F 不是新增语法章节，而是把前面 std-only 教学模型迁移到真实生态的生产化训练。详细规格见 [production-residency.md](production-residency.md)。

| Lane | 主题 | 核心验收 |
| --- | --- | --- |
| A | Rayon 并行实验 | sequential 与 parallel 完全一致，release benchmark 证明收益边界 |
| B | PyO3/maturin Python 扩展 | binding 层薄、错误不丢、copy/borrow/GIL 语义可测试 |
| C | Arrow/Parquet/Polars/DataFusion | schema、projection、filter、落盘或查询链路可审查 |
| D | tracing/metrics | 正常和失败路径都有稳定 span/metric，不污染 kernel |
| E | 持久化 scheduler | lease、attempt、idempotency、conflict、restart recovery 都有证据 |

阶段 F 的通过标准：至少完成 A、B、C；若目标是顶级量化技术设施岗位，完成 A-E。

## 学习纪律

- 每章必须跑当前章节 example。
- 每个计算函数必须有正常路径、边界路径、失败路径测试。
- 每个性能结论必须来自 release build 和可复现实验。
- unsafe 只能出现在边界清晰的小模块。
- 并行优化必须证明确定性、线程安全和收益。
- 专业化内容只服务高性能量化计算，不扩展无关 Web 业务。
- 生态库学习必须先理解 std-only 最小模型，再引入真实 crate。
- 真实生态迁移必须保留原有行为测试，并补充迁移后的运行或性能证据。

## 招聘级能力对照

完成本路线后，你应该能应对这些问题：

- 为什么库 API 常用 `&[f64]`，而不是 `Vec<f64>`？
- 什么时候用 generic，什么时候用 trait object？
- lifetime 标注到底在约束什么？
- `Iterator` 链为什么可能和手写循环一样快？
- 为什么 `Arc<Mutex<T>>` 不是并行计算的默认答案？
- 如何证明一个 rolling kernel 的优化有效？
- 什么情况下需要 unsafe？unsafe 不变量写在哪里？
- Python 调 Rust 时最贵的成本可能是什么？
- 列式计算为什么适合因子批处理？
- 分布式任务失败后如何保证不重复、不丢结果？
- 如何用 Criterion、Rayon、PyO3、Arrow、Tokio、tracing 等生态工具服务高性能量化系统，而不是为了使用库而使用库？
