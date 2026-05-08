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
  chapters/              # 每章一个目录：README.md 是教材，example/ 是本章示例 crate

projects/
  00-bootstrap-cli/
  01-factor-core/
  02-quant-lab-engine/
```

## 起步命令

如果你还没有安装 Rust，先读：

- [00-rust-install-hello-world](chapters/00-rust-install-hello-world/)

```bash
cargo run -p ch00-hello-world
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

0. [00-rust-install-hello-world](chapters/00-rust-install-hello-world/)
1. [00-learning-method](chapters/00-learning-method/)
2. [01-rust-program-shape](chapters/01-rust-program-shape/)
3. [02-values-types-functions](chapters/02-values-types-functions/)
4. [03-vec-slice-string](chapters/03-vec-slice-string/)
5. [04-ownership-borrowing](chapters/04-ownership-borrowing/)
6. [05-option-result-errors](chapters/05-option-result-errors/)
7. [06-struct-enum-match](chapters/06-struct-enum-match/)
8. [07-testing-numeric-code](chapters/07-testing-numeric-code/)
9. [08-rolling-mean-from-python-to-rust](chapters/08-rolling-mean-from-python-to-rust/)

### 第二部分：专业 Rust 核心

10. [09-modules-crates](chapters/09-modules-crates/)
11. [10-traits-generics](chapters/10-traits-generics/)
12. [11-lifetimes-api-design](chapters/11-lifetimes-api-design/)
13. [12-iterators-closures](chapters/12-iterators-closures/)
14. [13-memory-drop-raii](chapters/13-memory-drop-raii/)
15. [14-collections-bytes](chapters/14-collections-bytes/)
16. [15-error-architecture](chapters/15-error-architecture/)
17. [16-concurrency-primitives](chapters/16-concurrency-primitives/)
18. [17-async-boundaries](chapters/17-async-boundaries/)
19. [18-macros-features-docs](chapters/18-macros-features-docs/)

### 第三部分：高性能计算核心

20. [19-performance-engineering](chapters/19-performance-engineering/)
21. [20-memory-layout-cache](chapters/20-memory-layout-cache/)
22. [21-numerical-kernel](chapters/21-numerical-kernel/)
23. [22-parallel-computing](chapters/22-parallel-computing/)
24. [23-simd-unsafe](chapters/23-simd-unsafe/)

### 第四部分：量化系统工程

25. [24-python-ffi-boundaries](chapters/24-python-ffi-boundaries/)
26. [25-backtesting](chapters/25-backtesting/)
27. [26-experiment-monte-carlo](chapters/26-experiment-monte-carlo/)
28. [27-online-columnar](chapters/27-online-columnar/)
29. [28-storage-serialization](chapters/28-storage-serialization/)
30. [29-distributed-capstone](chapters/29-distributed-capstone/)
31. [30-observability-config](chapters/30-observability-config/)
32. [31-final-architecture](chapters/31-final-architecture/)

### 第五部分：生态与生产级扩展

33. [32-criterion-profiling](chapters/32-criterion-profiling/)
34. [33-rayon-parallelism](chapters/33-rayon-parallelism/)
35. [34-python-extension-boundary](chapters/34-python-extension-boundary/)
36. [35-columnar-query-engines](chapters/35-columnar-query-engines/)
37. [36-runtime-observability](chapters/36-runtime-observability/)
38. [37-scheduler-hardening](chapters/37-scheduler-hardening/)

## 每章怎么学

1. 读章节正文，不跳过概念解释。
2. 跑当前章节 example：`cargo test -p chXX-name`；如果是 Hello World 章，先跑 `cargo run -p ch00-hello-world`。
3. 只修改当前章节目录下的 `example/`。
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
