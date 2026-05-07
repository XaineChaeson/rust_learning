# 第 12 章：iterators、closures 和 zero-cost abstraction

Rust iterator 不是 Python generator 的简单替代。Rust iterator 通常是静态分发、可内联、可优化的抽象。写得好时，它既表达清楚，也不必比手写循环慢。

## iterator 是惰性的

```rust
values.iter().map(|x| x * 2.0)
```

这行本身不计算。只有遇到 `collect`、`sum`、`fold`、`for` 等消费动作，计算才发生。

这点和 Python 的 iterator 相似，但 Rust 的不同之处是：闭包和 iterator 类型在编译期确定，编译器可以消掉很多抽象层。

## 闭包捕获

闭包可以捕获环境：

```rust
let threshold = 0.01;
returns.iter().filter(|value| value.abs() > threshold)
```

闭包有三类 trait：

- `Fn`：只读捕获环境，可重复调用。
- `FnMut`：会修改捕获环境，可重复调用但需要 mutable。
- `FnOnce`：消耗捕获值，只能调用一次。

`rolling_apply` 用 `FnMut`，因为某些统计函数可能需要内部计数器或 scratch state。

## iterator vs for loop

专业判断不是“iterator 永远好”或“for loop 永远快”。判断标准：

- iterator 链短、清楚、可内联，优先 iterator。
- 热路径需要复杂状态、避免 bounds、复用 buffer，手写 loop 可能更清楚。
- benchmark 证明差异后再做风格调整。

## collect 的成本

`collect::<Vec<_>>()` 会分配新 `Vec`。如果中间结果只用一次，可能应该继续链式计算；如果需要复用或调试，中间 `Vec` 合理。

高性能代码最常见的误区是每一步都 collect：

```rust
let a = raw.iter().map(...).collect::<Vec<_>>();
let b = a.iter().filter(...).collect::<Vec<_>>();
```

这会制造额外分配和内存带宽消耗。

## 本章示例

```bash
cargo test -p ch12-iterators-closures
```

重点看：

- `rolling_apply` 如何接收闭包。
- `scan` 如何表达有状态累计。
- iterator 链什么时候产生输出 `Vec`。

## 本章练习

1. 用 `fold` 实现 `sum_of_squares`。
2. 用 `scan` 实现 drawdown 曲线。
3. 给 `rolling_apply` 写一个闭包，计算窗口最大值。
4. 写一个手写 loop 版本，对比可读性。

## 本章验收

你应该能解释：

- iterator 为什么是惰性的。
- `Fn`、`FnMut`、`FnOnce` 的区别。
- `collect` 在性能上意味着什么。
- 为什么 Rust iterator 可以是 zero-cost abstraction。

## 教材化补充：iterator 链不是 Python for 循环的语法糖

Python 里 list comprehension 和 generator 更像运行时对象组合。Rust iterator 链通常在编译期被具体化，编译器可以内联和优化。

例如：

```rust
values.iter().map(|x| x * 2.0).sum::<f64>()
```

这不一定会创建中间数组。只有当你调用 `collect::<Vec<_>>()`，才明确要求分配输出。

## closure 的捕获方式

闭包可以读取、修改或消耗外部变量，因此 Rust 把闭包分成三种能力：

- `Fn`：只读捕获。
- `FnMut`：可修改捕获。
- `FnOnce`：会消耗捕获，只能调用一次。

`rolling_apply` 使用 `FnMut` 是因为你可能传入带内部状态的函数，比如统计调用次数或复用 scratch buffer。

## iterator 何时不适合

不要迷信 iterator。以下情况手写 loop 可能更合适：

- 需要复用输出 buffer。
- 需要多个状态变量。
- 需要提前退出并返回复杂错误。
- benchmark 显示 iterator 链无法被优化到理想状态。

专业开发者不会按风格站队，而是按清晰性和证据判断。

## 示例走读

```text
book/examples/ch12-iterators-closures/src/lib.rs
```

`rolling_apply` 把“如何遍历窗口”和“窗口上做什么计算”分开。调用方可以传不同闭包：

```rust
rolling_apply(values, 20, |window| window.iter().sum())
```

这就是抽象复用，但仍然保持输入是 borrowed slice。

## 常见错误

错误 1：每一步都 `collect`。

这会制造中间分配。除非你真的需要保存中间结果，否则继续链式计算。

错误 2：闭包里偷偷修改外部大对象。

这会让代码难以推理，尤其在并行场景中更危险。

错误 3：以为 iterator 一定比 loop 慢。

Rust iterator 很多时候会被优化成和 loop 接近的机器码。是否慢，要 benchmark。

## 代码走读与操作清单

`rolling_apply` 是本章最重要的函数：

```rust
pub fn rolling_apply<F>(values: &[f64], window: usize, function: F) -> Vec<f64>
where
    F: FnMut(&[f64]) -> f64,
```

它把两个职责拆开：

- `rolling_apply` 负责产生窗口。
- `function` 负责计算每个窗口的值。

Python 里你可能写：

```python
rolling_apply(values, window, lambda x: sum(x))
```

Rust 版本更严格，因为闭包能力通过 `FnMut` 写在类型系统中。

再看 `cumulative_sum`：

```rust
values.iter().scan(0.0, |state, value| { ... })
```

`scan` 保存内部状态，每次输入一个值，输出一个累计值。它适合表达 running state。

操作清单：

1. 用 `rolling_apply` 计算窗口最大值。
2. 用 `rolling_apply` 计算窗口最后一个值减第一个值。
3. 用 `scan` 写 cumulative product。
4. 把其中一个实现改成 for loop，比较可读性。

你需要养成的判断是：iterator 用来表达数据流，loop 用来表达复杂状态。两者都是专业工具。
