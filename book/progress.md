# 学习进度表

状态建议：

- `todo`：还没开始。
- `reading`：正在读正文。
- `coded`：已经跑过并改过本章示例。
- `done`：示例、练习、测试、复盘都完成。

状态只能按顺序推进：`todo -> reading -> coded -> done`。不要因为“看懂了”就标记 `done`。每章至少留下三类证据：

1. 运行证据：本章命令通过。
2. 修改证据：你改过本章 example 或指定 project。
3. 复盘证据：能解释本章概念如何服务高性能量化系统。

如果某章练习要求改项目代码，先在复盘里写清楚文件路径，再提交测试证据。不要把跨章练习混进当前章节 example。

| 章节 | 状态 | 必跑命令 | 复盘 |
| --- | --- | --- | --- |
| 00 学习方法 | todo | 无 |  |
| 01 Rust 程序形状 | todo | `cargo run -p ch01-rust-program-shape --bin variables` |  |
| 02 值、类型、函数 | todo | `cargo test -p ch02-values-types-functions` |  |
| 03 Vec、slice、String | todo | `cargo test -p ch03-vec-slice-string` |  |
| 04 所有权和借用 | todo | `cargo test -p ch04-ownership-borrowing` |  |
| 05 Option、Result、错误 | todo | `cargo test -p ch05-option-result-errors` |  |
| 06 struct、enum、match | todo | `cargo test -p ch06-struct-enum-match` |  |
| 07 数值测试 | todo | `cargo test -p ch07-testing-numeric-code` |  |
| 08 rolling mean | todo | `cargo test -p ch08-rolling-mean` |  |
| 09 modules/crates | todo | `cargo test -p ch09-modules-crates` |  |
| 10 traits/generics | todo | `cargo test -p ch10-traits-generics` |  |
| 11 lifetimes/API | todo | `cargo test -p ch11-lifetimes-api-design` |  |
| 12 iterators/closures | todo | `cargo test -p ch12-iterators-closures` |  |
| 13 memory/RAII | todo | `cargo test -p ch13-memory-drop-raii` |  |
| 14 collections/bytes | todo | `cargo test -p ch14-collections-bytes` |  |
| 15 error architecture | todo | `cargo test -p ch15-error-architecture` |  |
| 16 concurrency primitives | todo | `cargo test -p ch16-concurrency-primitives` |  |
| 17 async boundaries | todo | `cargo test -p ch17-async-boundaries` |  |
| 18 macros/features/docs | todo | `cargo test -p ch18-macros-features-docs` |  |
| 19 performance engineering | todo | `cargo test -p ch19-performance-engineering` |  |
| 20 memory layout/cache | todo | `cargo test -p ch20-memory-layout-cache` |  |
| 21 numerical kernel | todo | `cargo test -p ch21-numerical-kernel` |  |
| 22 parallel computing | todo | `cargo test -p ch22-parallel-computing` |  |
| 23 SIMD/unsafe | todo | `cargo test -p ch23-simd-unsafe` |  |
| 24 Python FFI boundary | todo | `cargo test -p ch24-python-ffi-boundaries` |  |
| 25 backtesting | todo | `cargo test -p ch25-backtesting` |  |
| 26 experiments/Monte Carlo | todo | `cargo test -p ch26-experiment-monte-carlo` |  |
| 27 online/columnar | todo | `cargo test -p ch27-online-columnar` |  |
| 28 storage/serialization | todo | `cargo test -p ch28-storage-serialization` |  |
| 29 distributed scheduling | todo | `cargo test -p ch29-distributed-capstone` |  |
| 30 observability/config | todo | `cargo test -p ch30-observability-config` |  |
| 31 final architecture | todo | `cargo test -p ch31-final-architecture` |  |
| 32 Criterion/profiling | todo | `cargo test -p ch32-criterion-profiling` |  |
| 33 Rayon parallelism | todo | `cargo test -p ch33-rayon-parallelism` |  |
| 34 Python extension boundary | todo | `cargo test -p ch34-python-extension-boundary` |  |
| 35 columnar query engines | todo | `cargo test -p ch35-columnar-query-engines` |  |
| 36 runtime observability | todo | `cargo test -p ch36-runtime-observability` |  |
| 37 scheduler hardening | todo | `cargo test -p ch37-scheduler-hardening` |  |
| Production Residency | todo | 见 `book/production-residency.md` |  |

每章完成后回答：

```text
本章核心概念：
Python/NumPy/PyTorch 对照：
我亲手改了哪段代码：
我用什么测试证明它是对的：
性能或工程风险是什么：
还不理解什么：
```
