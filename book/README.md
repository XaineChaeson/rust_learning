# Rust 高性能量化计算自学书

这是本仓库唯一的学习入口。学习路径只有一条：读章节、跑本章示例、做练习、记录进度、推进项目。

## 目录结构

```text
book/
  README.md              # 当前入口
  roadmap.md             # 专业路线图
  competency-matrix.md   # 能力矩阵和验收标准
  chapter-writing-standard.md # 每章写作和验收标准
  assessment.md          # 阶段闸门和能力验收
  solutions.md           # 练习参考解法与检查点
  performance-lab.md     # 性能实验手册
  capstone.md            # 最终综合项目规格
  graduation.md          # 毕业验收和职业准备评分表
  production-residency.md # 真实生态生产化驻场训练
  progress.md            # 每章进度记录
  exercises.md           # 统一练习集
  appendix.md            # 术语、评审、benchmark、项目规格
  chapters/              # 教科书正文
  examples/              # 按章节拆分的示例 crate

projects/
  00-bootstrap-cli/
  01-factor-core/
  02-quant-lab-engine/
```

## 起步命令

```bash
cargo test
cargo run -p ch01-rust-program-shape --bin variables
cargo test -p ch10-traits-generics
cargo test -p ch23-simd-unsafe
cargo test -p ch32-criterion-profiling
cargo test -p ch37-scheduler-hardening
cargo run -p bootstrap-cli
cargo test -p factor-core
cargo run -p factor-core --release --bin bench
cargo run -p factor-core --release --bin bench -- --output target/benchmark-reports/factor-core.md
cargo test -p quant-lab-engine
cargo run -p quant-lab-engine --bin demo
```

## 阅读顺序

### 第一部分：Rust 零基础到 rolling baseline

1. [00-learning-method.md](chapters/00-learning-method.md)
2. [01-rust-program-shape.md](chapters/01-rust-program-shape.md)
3. [02-values-types-functions.md](chapters/02-values-types-functions.md)
4. [03-vec-slice-string.md](chapters/03-vec-slice-string.md)
5. [04-ownership-borrowing.md](chapters/04-ownership-borrowing.md)
6. [05-option-result-errors.md](chapters/05-option-result-errors.md)
7. [06-struct-enum-match.md](chapters/06-struct-enum-match.md)
8. [07-testing-numeric-code.md](chapters/07-testing-numeric-code.md)
9. [08-rolling-mean-from-python-to-rust.md](chapters/08-rolling-mean-from-python-to-rust.md)

### 第二部分：专业 Rust 核心

10. [09-modules-crates.md](chapters/09-modules-crates.md)
11. [10-traits-generics.md](chapters/10-traits-generics.md)
12. [11-lifetimes-api-design.md](chapters/11-lifetimes-api-design.md)
13. [12-iterators-closures.md](chapters/12-iterators-closures.md)
14. [13-memory-drop-raii.md](chapters/13-memory-drop-raii.md)
15. [14-collections-bytes.md](chapters/14-collections-bytes.md)
16. [15-error-architecture.md](chapters/15-error-architecture.md)
17. [16-concurrency-primitives.md](chapters/16-concurrency-primitives.md)
18. [17-async-boundaries.md](chapters/17-async-boundaries.md)
19. [18-macros-features-docs.md](chapters/18-macros-features-docs.md)

### 第三部分：高性能计算核心

20. [19-performance-engineering.md](chapters/19-performance-engineering.md)
21. [20-memory-layout-cache.md](chapters/20-memory-layout-cache.md)
22. [21-numerical-kernel.md](chapters/21-numerical-kernel.md)
23. [22-parallel-computing.md](chapters/22-parallel-computing.md)
24. [23-simd-unsafe.md](chapters/23-simd-unsafe.md)

### 第四部分：量化系统工程

25. [24-python-ffi-boundaries.md](chapters/24-python-ffi-boundaries.md)
26. [25-backtesting.md](chapters/25-backtesting.md)
27. [26-experiment-monte-carlo.md](chapters/26-experiment-monte-carlo.md)
28. [27-online-columnar.md](chapters/27-online-columnar.md)
29. [28-storage-serialization.md](chapters/28-storage-serialization.md)
30. [29-distributed-capstone.md](chapters/29-distributed-capstone.md)
31. [30-observability-config.md](chapters/30-observability-config.md)
32. [31-final-architecture.md](chapters/31-final-architecture.md)

### 第五部分：生态与生产级扩展

33. [32-criterion-profiling.md](chapters/32-criterion-profiling.md)
34. [33-rayon-parallelism.md](chapters/33-rayon-parallelism.md)
35. [34-python-extension-boundary.md](chapters/34-python-extension-boundary.md)
36. [35-columnar-query-engines.md](chapters/35-columnar-query-engines.md)
37. [36-runtime-observability.md](chapters/36-runtime-observability.md)
38. [37-scheduler-hardening.md](chapters/37-scheduler-hardening.md)

## 每章怎么学

1. 读章节正文，不跳过概念解释。
2. 跑当前章节 example：`cargo test -p chXX-name`。
3. 只修改当前章节对应的 `book/examples/chXX-*/`。
4. 做 `book/exercises.md` 中对应练习。
5. 把 `book/progress.md` 的状态改掉，并写复盘。
6. 跑质量门：

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## 自学闭环

读章节和跑示例只是第一层。要确认自己真的学会，使用下面这些辅助文件：

- [assessment.md](assessment.md)：每个阶段的闸门、口头自测和通过标准。
- [solutions.md](solutions.md)：练习参考解法与检查点，做完练习后再看。
- [performance-lab.md](performance-lab.md)：如何做 release benchmark、如何记录性能结论。
- [capstone.md](capstone.md)：最终专业作品规格。
- [graduation.md](graduation.md)：最终毕业验收表，判断是否达到专业实操能力。
- [production-residency.md](production-residency.md)：把 std-only 教学模型迁移到真实生态的生产化训练。

## 能力验收

先看 [competency-matrix.md](competency-matrix.md)。它定义本项目的专业能力边界：哪些是所有 Rust 开发都必须懂的核心知识，哪些是高性能量化基础设施专项能力。

章节深度标准见 [chapter-writing-standard.md](chapter-writing-standard.md)。如果某章读起来像提纲而不是教材，就应该按这个标准继续扩写。
