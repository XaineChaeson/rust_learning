# 第 1 章：Rust 程序长什么样

本章解决一个最基础的问题：一个 Rust 项目由哪些部分组成，你每天应该如何和它互动。

Python 学习中，你可能习惯直接写一个 `.py` 文件，然后运行：

```bash
python script.py
```

Rust 项目通常通过 Cargo 管理。Cargo 不只是运行工具，它同时负责项目结构、依赖、测试、示例和 benchmark。

## 最小 Rust 项目

一个最小二进制项目通常长这样：

```text
my-project/
  Cargo.toml
  src/
    main.rs
```

`Cargo.toml` 描述项目，`src/main.rs` 是程序入口。

本仓库现在是 workspace，根目录 `Cargo.toml` 不再代表一个具体程序，而是管理多个学习 crate：

```text
book/chapters/01-rust-program-shape/example
book/chapters/02-values-types-functions/example
projects/00-bootstrap-cli
```

这样做的原因是代码不应该散落在根目录。章节示例归章节，阶段项目归项目。

## `main.rs` 和 `lib.rs`

Rust 项目常见两个入口：

```text
src/main.rs
src/lib.rs
```

`main.rs` 负责可执行程序。它通常处理命令行参数、文件读取、打印输出。

`lib.rs` 负责核心逻辑。它应该容易测试，也应该尽量不依赖终端和文件系统。

在高性能量化系统中，这个分离非常重要：

- CLI、Python binding、Web 服务都可能调用同一套核心计算。
- 核心计算必须能独立测试。
- 性能 benchmark 应该直接测核心函数，而不是测打印和文件读取。

## Cargo 常用命令

运行整个 workspace 的测试：

```bash
cargo test
```

运行某个项目：

```bash
cargo run -p bootstrap-cli
```

运行章节示例：

```bash
cargo run -p ch01-rust-program-shape --bin variables
```

运行某个示例 crate 的测试：

```bash
cargo test -p ch02-values-types-functions
```

静态检查：

```bash
cargo clippy --all-targets -- -D warnings
```

格式检查：

```bash
cargo fmt --check
```

## Python 对照

Python 项目里，你可能用：

- `pip` 管依赖
- `pytest` 跑测试
- `ruff` 或 `flake8` 做检查
- `python script.py` 运行脚本

Rust 把这些常用动作集中到了 Cargo 里。你应该把 `cargo test` 当成日常动作，而不是项目结束时才跑。

## 编译和运行是两个阶段

Rust 先编译，再运行。比如：

```bash
cargo run -p ch01-rust-program-shape --bin variables
```

Cargo 会先编译 `variables.rs`，如果编译失败，程序根本不会运行。

这和 Python 很不同。Python 的很多错误只有运行到那一行才出现。Rust 希望尽可能在编译期暴露错误。

## 当前章节示例

查看：

```text
book/chapters/01-rust-program-shape/example/src/bin/
```

里面有：

- `variables.rs`
- `ownership.rs`
- `structs_enums.rs`
- `results.rs`
- `iterators.rs`

运行第一个：

```bash
cargo run -p ch01-rust-program-shape --bin variables
```

你会看到变量、可变变量和打印输出。现在不需要完全理解每一行。你的目标是先会运行、会改、会看错误。

## 故意制造错误

打开 `variables.rs`，把：

```rust
let mut exercise_score = 78.5;
```

改成：

```rust
let exercise_score = 78.5;
```

然后保留后面的：

```rust
exercise_score += 5.0;
```

再次运行。编译器会告诉你不能修改不可变变量。你要观察三件事：

1. 错误发生在哪个文件哪一行。
2. 编译器如何描述问题。
3. 编译器是否给出 `help`。

这个小错误非常重要。Rust 默认不可变，是为了让数据流更可控。在高性能系统里，可控的数据流比随意修改更容易测试和并行化。

## 本章练习

1. 运行 `variables`、`ownership`、`results` 三个示例。
2. 故意制造一个不可变变量修改错误。
3. 记录编译器第一条错误。
4. 运行 `cargo test`。
5. 解释 workspace、package、binary 这三个词的区别。

## 本章验收

你可以进入下一章，如果你能回答：

- 根目录 `Cargo.toml` 现在为什么是 workspace？
- 为什么代码要分为章节目录下的 `example/` 和 `projects`？
- `main.rs` 和 `lib.rs` 的职责有什么不同？
- 为什么高性能核心不应该写死在 CLI 里？

## 教材化补充：从 Python 脚本到 Rust package

Python 初学时，你可能习惯一个 `analysis.py` 或一个 notebook 解决所有问题。Rust 项目从一开始就更强调边界：哪些代码是可复用库，哪些代码只是命令行入口，哪些代码是测试。

`main.rs` 是程序入口。它适合做这些事：

- 读取命令行参数。
- 调用库函数。
- 打印结果。
- 把错误展示给用户。

`lib.rs` 是库入口。它适合放这些内容：

- 纯计算函数。
- 数据结构。
- 错误类型。
- 可复用模块。
- 单元测试。

高性能量化系统尤其要区分二者。你的 rolling 因子计算不应该只存在于 CLI 中，因为同一段核心计算未来可能被 Python 调用、被回测调用、被在线服务调用，也可能被 benchmark 单独调用。

## 操作路径

先运行第 1 章示例：

```bash
cargo run -p ch01-rust-program-shape --bin variables
```

然后看文件：

```text
book/chapters/01-rust-program-shape/example/src/bin/variables.rs
```

你应该能指出：

- 哪一行定义变量。
- 哪一行修改可变变量。
- 哪一行输出内容。
- 哪些变量可以改，哪些不能改。

再运行全仓库测试：

```bash
cargo test
```

注意：`cargo test` 会编译并运行所有 package 的测试。这就是 workspace 的价值。

## 常见错误

错误 1：不知道 `-p` 是什么意思。

`-p` 指定 package。因为本仓库有很多章节 package，所以你需要告诉 Cargo 跑哪一个。

错误 2：把示例和项目混在一起。

章节目录下的 `example/` 是学习概念的地方，`projects` 是阶段成果。示例可以小而单一，项目要更像真实工程。

错误 3：把核心计算写进 `main.rs`。

这会让测试、复用、benchmark 和 Python 绑定都变困难。专业 Rust 项目通常让 `main.rs` 很薄，把核心放进 library。

## 和高性能量化开发的关系

任何严肃基础设施都需要清楚的 crate 边界。后面你会看到 `factor-core`、`numeric-kernel`、`backtest-core` 这些名字。它们不是为了目录好看，而是为了让核心计算能被多种入口复用，并且能独立测试和优化。
