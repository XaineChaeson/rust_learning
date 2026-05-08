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

## Cargo 命令逐段拆开看

先看这一条：

```bash
cargo run -p ch01-rust-program-shape --bin variables
```

它不是一整句咒语，可以拆成几段：

| 片段 | 含义 |
| --- | --- |
| `cargo` | Rust 的项目管理工具。它读取 `Cargo.toml`，决定要编译什么、运行什么、测试什么。 |
| `run` | Cargo 的子命令：先编译，再运行一个可执行目标。 |
| `-p ch01-rust-program-shape` | 选择 package。这里选择名字叫 `ch01-rust-program-shape` 的章节示例 package。 |
| `--bin variables` | 在这个 package 里选择名叫 `variables` 的 binary target。 |

为什么既有 `-p` 又有 `--bin`？

因为 workspace 里有很多 package，而一个 package 里也可能有多个可执行程序。本章示例就是这样：

```text
book/chapters/01-rust-program-shape/example/
  Cargo.toml                 # package 名字：ch01-rust-program-shape
  src/bin/variables.rs       # binary target 名字：variables
  src/bin/ownership.rs       # binary target 名字：ownership
  src/bin/results.rs         # binary target 名字：results
```

所以：

```bash
cargo run -p ch01-rust-program-shape --bin ownership
```

表示：在 workspace 里找到 `ch01-rust-program-shape` 这个 package，然后运行它里面的 `ownership` 这个 binary。

再看测试命令：

```bash
cargo test -p ch02-values-types-functions
```

| 片段 | 含义 |
| --- | --- |
| `cargo` | 读取 manifest，组织编译和测试。 |
| `test` | 编译测试版本并运行测试。 |
| `-p ch02-values-types-functions` | 只测试这个 package，不跑整个 workspace。 |

再看检查命令：

```bash
cargo clippy --all-targets -- -D warnings
```

这条更容易迷惑：

| 片段 | 含义 |
| --- | --- |
| `cargo clippy` | 运行 Rust 的 lint 检查工具 Clippy。 |
| `--all-targets` | 检查 lib、bin、test、example 等所有目标，不只检查默认目标。 |
| `--` | 分隔符。前面是 Cargo/Clippy 的参数，后面是传给底层 lint 工具的参数。 |
| `-D warnings` | 把 warning 当成 error。D 是 deny。 |

格式检查：

```bash
cargo fmt --check
```

| 片段 | 含义 |
| --- | --- |
| `cargo fmt` | 调用 rustfmt 格式化工具。 |
| `--check` | 只检查，不改文件。适合提交前确认格式。 |

## 为什么 `-p` 可以不管当前目录？

你发现从仓库根目录、章节目录、甚至某些子目录运行同一条命令都可以成功，这是 Cargo 的 manifest 查找规则导致的。

Cargo 运行时会从当前目录开始，向上找 `Cargo.toml`：

```text
当前目录
  如果没有 Cargo.toml，就看上一级
    如果还没有，就继续向上
      找到 Cargo.toml 后读取它
```

在这个仓库里，根目录有一个 workspace manifest：

```toml
[workspace]
members = [
    "book/chapters/*/example",
    "projects/00-bootstrap-cli",
    "projects/01-factor-core",
    "projects/02-quant-lab-engine",
]
```

这告诉 Cargo：这些路径都是同一个 workspace 的成员。只要你当前所在目录还在这个仓库里面，Cargo 通常都能向上找到根目录的 `Cargo.toml`，然后加载整个 workspace。

加载 workspace 后，`-p ch01-rust-program-shape` 选的不是目录名，而是 package name。这个名字来自本章示例的 `Cargo.toml`：

```toml
[package]
name = "ch01-rust-program-shape"
edition.workspace = true
version.workspace = true
```

所以 `-p` 的意思不是“去某个目录运行”，而是“在当前 workspace 里找这个 package name”。

这也是为什么下面两种写法都能工作：

```bash
# 从仓库根目录
cargo run -p ch01-rust-program-shape --bin variables

# 从 book/chapters/01-rust-program-shape/example/src/bin 这类子目录
cargo run -p ch01-rust-program-shape --bin variables
```

但如果你离开这个仓库，到一个 Cargo 找不到本仓库 `Cargo.toml` 的目录，再运行同一条命令，就会失败。Cargo 不是记住了你的项目，而是沿当前路径向上找 manifest。

## Cargo.toml 怎么读

`Cargo.toml` 是 Rust package 的 manifest。manifest 的意思是“清单”：这个项目叫什么、属于哪个 edition、有哪些依赖、有哪些可执行入口、有哪些 feature。

它没有一个“所有项目都必须写满”的固定完整模板。Cargo 会根据默认约定推导很多内容。你应该先学会读常见 section，再知道哪些 section 是按需出现的。

TOML 文件按 section 组织。section 用方括号表示：

```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2024"

[dependencies]

[dev-dependencies]

[features]
```

一个较完整但仍然常见的骨架长这样：

```toml
[package]
name = "demo"
version = "0.1.0"
edition = "2024"
description = "optional package description"

[lib]
name = "demo"
path = "src/lib.rs"

[[bin]]
name = "demo-cli"
path = "src/main.rs"

[dependencies]
serde = "1"

[dev-dependencies]
approx = "0.5"

[build-dependencies]

[features]
default = []
simd = []

[workspace]
members = ["crates/*"]
resolver = "3"

[workspace.package]
edition = "2024"
version = "0.1.0"

[workspace.dependencies]
serde = "1"

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

不是每个 `Cargo.toml` 都会同时出现这些 section。本仓库早期章节的示例通常很小，所以大多只有 `[package]`，依赖和 target 都靠 Cargo 默认规则处理。

### `[package]`

描述当前 package 自己：

```toml
[package]
name = "ch01-rust-program-shape"
version = "0.1.0"
edition = "2024"
```

| 字段 | 含义 |
| --- | --- |
| `name` | package 名字。`cargo -p ...` 选的就是它。 |
| `version` | package 版本。发布 crate 时很重要，本仓库主要用于保持 manifest 完整。 |
| `edition` | Rust edition。它决定使用哪一代 Rust 语法规则，本仓库用 2024。 |

本仓库的章节示例常写成：

```toml
[package]
name = "ch01-rust-program-shape"
edition.workspace = true
version.workspace = true
```

这表示 `edition` 和 `version` 不在本文件重复写，而是继承根目录 workspace 的统一配置：

```toml
[workspace.package]
edition = "2024"
version = "0.1.0"
```

这样所有章节示例都保持一致。

### `[dependencies]`

运行代码需要的依赖写在这里：

```toml
[dependencies]
serde = "1"
```

本学习仓库早期章节基本不加依赖，因为目标是先理解 Rust 标准库和语言本身。

### `[dev-dependencies]`

只在测试、benchmark、示例开发时需要的依赖写在这里：

```toml
[dev-dependencies]
approx = "0.5"
```

如果一个库只用于测试，不应该放进 `[dependencies]`，否则会影响正常使用者的依赖树。

### `[features]`

feature 是可选编译开关：

```toml
[features]
simd = []
```

后面你会看到这种命令：

```bash
cargo test -p ch18-macros-features-docs --features simd
```

意思是测试这个 package 时打开 `simd` 这个 feature。

### `[lib]` 和 `[[bin]]`

很多时候你不用手写它们，因为 Cargo 有默认约定：

| 文件位置 | Cargo 默认理解 |
| --- | --- |
| `src/lib.rs` | 当前 package 的 library target |
| `src/main.rs` | 当前 package 的默认 binary target |
| `src/bin/variables.rs` | 名字叫 `variables` 的 binary target |

如果你想显式指定，也可以写：

```toml
[lib]
name = "my_library"
path = "src/lib.rs"

[[bin]]
name = "variables"
path = "src/bin/variables.rs"
```

注意 `[[bin]]` 是双中括号，因为一个 package 可以有多个 binary。

### `[workspace]`

根目录的 `Cargo.toml` 用 `[workspace]` 管理多个 package：

```toml
[workspace]
members = [
    "book/chapters/*/example",
    "projects/00-bootstrap-cli",
    "projects/01-factor-core",
    "projects/02-quant-lab-engine",
]
resolver = "3"
```

| 字段 | 含义 |
| --- | --- |
| `members` | 哪些路径属于这个 workspace。`*` 是通配符。 |
| `resolver` | Cargo 解析依赖 feature 的规则版本。本仓库用 edition 2024 配套的 resolver 3。 |

workspace 本身通常不是一个可运行程序。它更像“总目录”或“课程项目集合”。真正能运行或测试的是 workspace 里的 package。

## workspace、package、crate、target 的关系

这几个词很容易混：

| 词 | 在本仓库里的例子 | 你可以怎么理解 |
| --- | --- | --- |
| workspace | 根目录 `Cargo.toml` 管理的整个仓库 | 一组 package 的集合 |
| package | `ch01-rust-program-shape` | 一个有自己 `Cargo.toml` 的项目单元 |
| crate | 编译器一次编译的 Rust 单元 | package 编译后产生的 lib 或 bin |
| target | `variables`、`ownership`、`lib` | package 里面具体要编译的目标 |

日常命令可以这样理解：

```bash
cargo run -p ch01-rust-program-shape --bin variables
```

翻译成人话就是：

> 在当前 workspace 里，选择 `ch01-rust-program-shape` 这个 package，编译并运行它里面名为 `variables` 的 binary target。

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
