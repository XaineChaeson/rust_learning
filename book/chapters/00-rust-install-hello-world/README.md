# 入门准备：安装 Rust 与 Hello World

本章解决一个更靠前的问题：你还没有 Rust 环境，也不知道一个 Rust 程序怎样从文本变成终端输出。

如果你已经能在本仓库根目录运行：

```bash
cargo run -p ch00-hello-world
```

并看到一行输出，那么你可以快速读完本章，确认概念后进入
[00-learning-method](../00-learning-method/)。

如果你看到 `cargo: command not found`、`package not found`、链接器错误，或者不知道应该在哪个目录运行命令，就不要跳过本章。

## 本章解决什么问题

在 Python 里，你可能习惯这样开始：

```bash
python hello.py
```

Rust 的第一步多了一层工具链：

- `rustup`：安装和管理 Rust 版本。
- `rustc`：真正的 Rust 编译器。
- `cargo`：日常使用的项目工具，负责创建项目、编译、运行、测试、管理依赖。

刚开始你不需要理解它们的全部细节，但必须知道谁负责什么。后面所有章节都会用 `cargo`。

## 学习前提

本章可以从零开始。你只需要会打开一个终端。

如果你在 Windows 上使用 WSL，本章的 Linux/WSL 命令应该在 WSL 终端里运行，不是在 PowerShell 里运行。Windows 原生开发和 WSL 开发是两套环境，Rust 也要分别安装。

## 官方资料的角色

Rust 官方资料是权威来源，尤其适合确认安装方式、Cargo 命令和语言定义。本教材的角色不同：它会把这些概念翻译到 Python 数据科学和高性能量化工程的语境里。

建议使用方式：

- 第一次安装 Rust 时，看官方安装页或本章摘要。
- 学本仓库时，以 `book/README.md` 为主线。
- 遇到标准概念不确定时，再查官方对应章节。

官方对应资料：

- Rust 安装页：<https://www.rust-lang.org/tools/install>
- Rust Book 第 1 章 Getting Started：<https://doc.rust-lang.org/book/ch01-00-getting-started.html>
- Rust Book 安装：<https://doc.rust-lang.org/book/ch01-01-installation.html>
- Rust Book Hello World：<https://doc.rust-lang.org/book/ch01-02-hello-world.html>
- Rust Book Hello Cargo：<https://doc.rust-lang.org/book/ch01-03-hello-cargo.html>

你不需要先通读整本官方教程。先把本章和官方第 1 章中安装、Hello World、Cargo 的部分跑通即可。

## 安装 Rust

Rust 官方推荐用 `rustup` 安装 Rust 工具链。`rustup` 会安装 `rustc`、`cargo` 和相关标准工具。

Linux、macOS、WSL：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装时通常选择默认选项即可。安装完成后，重新打开终端，或者按安装提示加载环境变量。

Windows 原生环境：

1. 打开 <https://www.rust-lang.org/tools/install>。
2. 下载并运行 `rustup-init.exe`。
3. 如果安装器提示需要 Visual Studio C++ Build Tools，按提示安装。
4. 重新打开 PowerShell 或终端。

如果你使用的是 Windows + WSL，本课程建议优先在 WSL 里学习和运行本仓库，因为当前仓库路径就是 Linux/WSL 风格路径。

## 验证安装

打开新终端，运行：

```bash
rustc --version
cargo --version
rustup --version
```

你不需要记住版本号，只要三个命令都能输出版本信息，就说明工具链基本可用。

如果 `rustc` 或 `cargo` 找不到，常见原因是：

- 安装后没有重新打开终端。
- `~/.cargo/bin` 没有进入 `PATH`。
- 你在 Windows 安装了 Rust，却在 WSL 里运行命令，或者反过来。

## 在本仓库运行第一个 Rust 程序

进入本仓库根目录：

```bash
cd /home/xainewsl/code/rust_learning
```

运行：

```bash
cargo run -p ch00-hello-world
```

你应该看到类似输出：

```text
Hello, Rust quant learner!
```

`-p ch00-hello-world` 的意思是指定 package。因为本仓库是 workspace，里面有很多章节 crate，Cargo 需要知道你要运行哪一个。

## 看第一段代码

打开：

```text
book/chapters/00-rust-install-hello-world/example/src/main.rs
```

内容是：

```rust
fn main() {
    println!("Hello, Rust quant learner!");
}
```

先只看三件事：

1. `fn main()` 是可执行程序的入口。程序从这里开始运行。
2. `{ ... }` 包住函数体。Rust 用花括号表示代码块。
3. `println!` 把文本输出到终端。它后面有 `!`，说明这是一个 macro，不是普通函数。

现在不需要深入理解 macro。你只需要知道：看到 `println!` 时，它是在打印内容。

## Cargo 做了什么

当你运行：

```bash
cargo run -p ch00-hello-world
```

Cargo 做了两步：

1. 编译 `ch00-hello-world`。
2. 如果编译成功，运行编译出来的程序。

这和 Python 不同。Python 通常边运行边发现错误；Rust 会先编译。很多错误在程序真正运行前就会被拦下来。

如果你只想检查代码能不能编译，可以运行：

```bash
cargo check -p ch00-hello-world
```

如果你想运行测试，可以运行：

```bash
cargo test -p ch00-hello-world
```

这个 Hello World crate 还没有真正的测试，所以 `cargo test` 主要证明它能作为 package 被 Cargo 正确编译。

## Python 对照

Python：

```python
print("Hello, Python")
```

Rust：

```rust
fn main() {
    println!("Hello, Rust");
}
```

差异不是为了复杂而复杂：

- Rust 可执行程序需要明确入口 `main`。
- Rust 先编译，再运行。
- Rust 项目通常由 Cargo 管理，而不是散落一个源文件。
- Cargo 会把编译产物放进 `target/`，不会把生成文件混进源码目录。

这些习惯会服务后面的高性能工程：明确入口、明确构建、明确测试、明确产物。

## 故意制造一个错误

把 `main.rs` 临时改成：

```rust
fn start() {
    println!("Hello, Rust quant learner!");
}
```

再运行：

```bash
cargo run -p ch00-hello-world
```

编译器会告诉你找不到 `main` 函数。你要观察：

1. 错误发生在哪个 package。
2. 错误信息是否提到 `main`。
3. Cargo 是否阻止程序继续运行。

观察完后，把函数名改回 `main`。

## 常见错误

错误 1：`cargo: command not found`

说明 Cargo 没有在当前终端可见。先重新打开终端，再检查 `cargo --version`。如果仍然失败，检查 Rust 是否安装在当前环境里。

错误 2：在仓库根目录运行 `cargo run` 失败

本仓库根目录是 virtual workspace，不是单个可执行程序。你需要指定 package：

```bash
cargo run -p ch00-hello-world
```

错误 3：链接器错误

Rust 编译后还需要链接成可执行文件。Linux/WSL 上可能需要系统 C 编译工具；Ubuntu 常见包是 `build-essential`。Windows 原生环境通常需要 Visual Studio C++ Build Tools。

错误 4：把临时 `cargo new hello-rust` 项目提交进本仓库

本仓库已经有章节示例。临时练习可以放在 `/tmp` 或你自己的练习目录，不要混进课程结构。

## 和高性能量化开发的关系

Hello World 看起来和高性能量化无关，但它建立了最底层的工程反馈回路：

- 能安装工具链。
- 能确认当前终端使用的是哪套 Rust。
- 能运行一个 package。
- 能看懂 Cargo 的编译和运行边界。
- 能把错误先交给编译器，而不是等运行很久才发现。

后面你写 rolling factor、backtest、scheduler 时，仍然是在这个反馈回路里工作：修改代码，编译，运行测试，解释结果。

## 本章练习

1. 运行 `rustc --version`、`cargo --version`、`rustup --version`。
2. 运行 `cargo run -p ch00-hello-world`。
3. 把输出文字改成你自己的学习目标，再运行一次。
4. 把 `main` 临时改名为 `start`，观察第一条编译错误，然后改回。
5. 用自己的话解释 `rustup`、`rustc`、`cargo` 分别负责什么。

## 本章验收

你可以进入 [00-learning-method](../00-learning-method/)，如果你能做到：

- 能在当前终端运行 `cargo --version`。
- 能在仓库根目录运行 `cargo run -p ch00-hello-world`。
- 能指出 `book/chapters/00-rust-install-hello-world/example/src/main.rs` 里的程序入口。
- 能解释为什么本仓库运行示例时通常要加 `-p`。
- 能说出官方教程和本教材的分工。
