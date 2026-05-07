# 专业能力矩阵

这份矩阵回答一个问题：学完整个项目后，你是否接近专业 Rust 高性能开发者。

## Rust 核心能力

| 能力 | 必须能做到 | 对应章节 |
| --- | --- | --- |
| Cargo 和 workspace | 能解释 package、crate、bin、lib、test、feature | 01、09、18 |
| 所有权和借用 | 能设计不多拷贝、不悬垂、不泄漏内部状态的 API | 03、04、11、13 |
| 类型建模 | 能用 struct/enum/match 表达业务状态和非法状态 | 05、06、15 |
| trait 和泛型 | 能写可替换组件、associated type、generic bound | 10、31 |
| lifetime | 能返回 borrowed view，并解释返回值受哪个输入约束 | 11、21 |
| iterator/closure | 能用组合式计算表达 pipeline，并知道何时改回循环 | 12、19 |
| 错误架构 | 能设计库错误、转换外部错误、保留上下文 | 05、15 |
| memory/RAII | 能解释 Copy/Clone/Drop、堆分配、资源生命周期 | 13、20 |
| collections/bytes | 能选择 HashMap/BTreeMap/Vec/String/&str/&[u8] | 03、14、28 |
| 并发基础 | 能解释 Send/Sync/Arc/Mutex/channel 和数据竞争 | 16、22 |
| async 边界 | 能区分 IO 并发和 CPU 并行，不把 runtime 当加速器 | 17、27 |
| macros/features/docs | 能维护测试宏、feature gate、doc test、release 配置 | 18、30 |

## 高性能计算能力

| 能力 | 必须能做到 | 对应章节 |
| --- | --- | --- |
| benchmark | 能区分 debug/release、冷启动、噪声、输入规模 | 19 |
| profiling | 能用证据定位 CPU、allocation、cache、IO 瓶颈 | 19、20 |
| 数据布局 | 能解释 contiguous、stride、row-major、SoA/AoS | 20、21 |
| 数值 kernel | 能实现 rolling、reduction、dot、matrix view | 08、21、23 |
| cache locality | 能判断访问顺序为什么影响吞吐 | 20、21 |
| allocation control | 能复用 buffer，避免热路径重复分配 | 13、19、20 |
| 并行归约 | 能设计 chunk、partial result、merge | 16、22、26 |
| unsafe 纪律 | 能写清 safety invariant，用 safe API 包住 unsafe | 23 |
| SIMD 边界 | 能判断何时值得 SIMD，何时 compiler 已足够 | 23 |
| 统计 benchmark | 能解释样本、噪声、median/mean、Criterion 报告 | 32 |
| 生态并行 | 能把 Rayon par_iter 映射到 partition/merge/ordering | 33 |

## 量化基础设施能力

| 能力 | 必须能做到 | 对应章节 |
| --- | --- | --- |
| 因子引擎 | 能批量计算 rolling 因子并处理异常数据 | 08、21、31 |
| Python 集成 | 能解释 PyO3/maturin/GIL/copy/zero-copy 边界 | 24 |
| Python 扩展交付 | 能设计薄绑定层、wheel 构建边界和错误映射 | 34 |
| 回测引擎 | 能设计 portfolio state、order、trade、fee、slippage | 25 |
| 实验引擎 | 能做 grid/random search、seed、checkpoint、result store | 26 |
| Monte Carlo | 能做可复现路径模拟和并行统计 | 26 |
| 在线特征 | 能用 ring buffer 做增量更新并处理延迟/乱序 | 17、27 |
| 列式批处理 | 能解释 Arrow/Parquet、projection/predicate pushdown | 27、28 |
| 查询生态 | 能解释 Polars/DataFusion 的 schema、projection、filter、execution plan | 35 |
| 分布式调度 | 能设计 task、worker、partition、retry、idempotency | 29 |
| 生产工程 | 能配置、观测、诊断、压测、保护 API 稳定性 | 30、31、36 |
| 调度硬化 | 能设计 lease、attempt、result conflict、max retry | 37 |

## 顶级技术设施岗位验收

学完后应该能独立完成这些任务：

1. 设计一个 Rust crate，提供稳定的 rolling factor API。
2. 为核心 kernel 写单元测试、边界测试、属性式思维测试和 benchmark。
3. 证明一个优化来自算法复杂度、内存布局、并行策略还是 SIMD。
4. 把 Rust kernel 暴露给 Python，并能解释数据拷贝成本。
5. 设计一个实验调度器，支持参数搜索、失败重试和结果聚合。
6. 设计在线特征计算模块，避免重复扫描历史窗口。
7. 为系统增加 config、metrics、错误上下文和生产排障文档。
8. 把 Criterion、Rayon、PyO3、Arrow/Polars/DataFusion、Tokio/tracing/metrics 纳入架构，而不是停留在概念认识。

这不等价于“读完就自动拥有多年生产经验”。它的目标是让你的知识结构、代码习惯、性能判断方式和系统设计语言达到专业面试与真实项目的门槛。
