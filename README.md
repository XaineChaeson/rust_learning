# Rust HPC Quant Learning

本仓库是一套面向 **Rust 零基础、Python 数据科学背景** 的专业 Rust 高性能量化开发训练材料。

唯一学习入口：

[book/README.md](book/README.md)

## 当前结构

```text
book/       # 教科书、路线、能力矩阵、示例、练习、附录
projects/   # 阶段项目代码
```

不要从多个目录里找路线。学习顺序由 `book/README.md` 和 `book/roadmap.md` 定义。

## 快速验证

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## 示例命令

```bash
cargo run -p ch00-hello-world
cargo run -p ch01-rust-program-shape --bin variables
cargo test -p ch10-traits-generics
cargo test -p ch20-memory-layout-cache
cargo test -p ch31-final-architecture
cargo test -p ch32-criterion-profiling
cargo test -p ch37-scheduler-hardening
cargo run -p bootstrap-cli
cargo test -p factor-core
cargo run -p factor-core --release --bin bench
cargo run -p factor-core --release --bin bench -- --output target/benchmark-reports/factor-core.md
cargo test -p quant-lab-engine
cargo run -p quant-lab-engine --bin demo
```

## 专业目标

学完后，你应该具备：

- 专业 Rust 核心：ownership、lifetime、trait、generic、error、concurrency、crate engineering。
- 高性能计算核心：benchmark、profiling、memory layout、parallelism、SIMD、unsafe boundary。
- 量化系统工程：factor engine、Python binding、backtesting、experiment runner、online features、columnar batch、distributed scheduler、observability。

能力矩阵见：[book/competency-matrix.md](book/competency-matrix.md)

自学验收见：[book/assessment.md](book/assessment.md)

性能实验见：[book/performance-lab.md](book/performance-lab.md)

最终综合项目见：[projects/02-quant-lab-engine](projects/02-quant-lab-engine)

毕业验收见：[book/graduation.md](book/graduation.md)

生产化驻场训练见：[book/production-residency.md](book/production-residency.md)
