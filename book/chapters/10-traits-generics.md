# 第 10 章：traits、generics 和 associated types

Rust 的 trait 不是 Java/Python 里的“接口”简单翻版。它同时承担抽象、静态分发、约束表达、扩展方法和类型级设计的职责。专业 Rust 开发者写高性能框架时，trait 是把“可替换策略”和“零成本抽象”同时做到的主要工具。

## 为什么量化计算需要 trait

你会遇到大量同形问题：

- 不同因子都接收时间序列，输出一列值。
- 不同风险模型都接收收益矩阵，输出暴露或协方差。
- 不同执行器都接收任务，输出结果。
- 不同存储后端都写入 factor record。

如果用函数到处传，会失去结构；如果用 enum 把所有可能类型写死，会让扩展困难。trait 允许你表达“这类东西都能 compute”，而不关心具体实现。

```rust
pub trait Factor {
    type Output;

    fn name(&self) -> &'static str;
    fn compute(&self, values: &[f64]) -> Self::Output;
}
```

`type Output` 是 associated type。它把输出类型绑定到实现者身上。`MeanFactor` 可以输出 `Option<f64>`，另一个 rolling factor 可以输出 `Result<Vec<f64>, Error>`。

## generics 和 trait object

泛型写法：

```rust
pub fn run_factor<F: Factor>(factor: F, values: &[f64]) -> F::Output
```

编译器会为具体类型生成专门版本，通常可以内联。这叫静态分发，适合热路径。

trait object 写法：

```rust
Box<dyn Factor<Output = Option<f64>>>
```

这允许运行时存放不同实现，但需要动态分发，且 associated type 必须固定。它适合插件、配置驱动的策略列表，不适合最内层数值循环。

专业判断：

- kernel 热路径优先 generic。
- 系统边界、插件、任务队列可以用 trait object。
- 不要为了“抽象”牺牲数据布局和内联机会。

## trait bound 是输入契约

```rust
pub fn normalize<T>(values: &[T]) -> Vec<f64>
where
    T: Copy + Into<f64>,
```

这段签名说清楚：

- 输入元素可以便宜复制。
- 输入元素可以转换为 `f64`。
- 函数不拥有输入 slice。

这比在函数内部到处写转换错误更清晰。

## coherence 和 orphan rule

Rust 不允许你随便给任意外部类型实现任意外部 trait。你必须拥有 trait 或类型中的一个。这叫 orphan rule。它保护 crate 生态不会出现两个库都给同一个类型实现同一个 trait 的冲突。

工程启示：

- 如果你需要给外部类型加行为，可以定义自己的 trait。
- 如果你需要包装外部类型，可以用 newtype。
- 不要指望像 Python monkey patch 一样改生态库行为。

## 本章示例

```bash
cargo test -p ch10-traits-generics
```

重点看：

- `Factor` 的 associated type。
- `run_factor` 的泛型静态分发。
- `normalize<T>` 的 trait bound。

## 本章练习

1. 新增 `MaxFactor`，输出最大值。
2. 定义 `RollingFactor` trait，输出 `Vec<f64>`。
3. 写 `run_many<F: Factor>`，接收 factor slice 是否可行？为什么？
4. 尝试用 `Vec<Box<dyn Factor<Output = Option<f64>>>>` 存多个 factor。
5. 写复盘：什么时候 generic 更好，什么时候 trait object 更好？

## 本章验收

你应该能解释：

- trait、generic、trait object 的区别。
- associated type 解决什么问题。
- 为什么 generic 常用于高性能热路径。
- orphan rule 为什么存在。

## 教材化补充：trait 是“能力”，不是“父类”

如果你来自 Python，可能会把 trait 类比成 duck typing：只要对象有某个方法就能用。Rust 更严格：类型必须显式实现 trait，编译器才承认它具备这种能力。

```rust
impl Factor for MeanFactor {
    type Output = Option<f64>;
    fn compute(&self, values: &[f64]) -> Self::Output { ... }
}
```

这表示 `MeanFactor` 确实是一个 `Factor`。调用方不用知道它内部怎么实现，只依赖 trait 合同。

## 为什么 generic 是高性能工具

泛型函数：

```rust
pub fn run_factor<F>(factor: F, values: &[f64]) -> F::Output
where
    F: Factor,
```

这里没有运行时查找。编译器知道 `F` 的具体类型，可以生成专门代码，并有机会内联 `compute`。这就是零成本抽象的来源。

相比之下，`Box<dyn Factor>` 是动态分发。它更灵活，但每次调用需要通过 vtable 找函数。这个成本不一定大，但在最内层 kernel 中要谨慎。

## associated type 的直觉

如果 trait 里有：

```rust
type Output;
```

意思是“每个实现者自己决定输出类型”。这比把所有 factor 都强行规定成同一种输出更灵活。

例如：

- `MeanFactor` 输出 `Option<f64>`。
- `RollingMeanFactor` 输出 `Result<Vec<f64>, Error>`。
- `CrossSectionalRank` 输出一个带 symbol 的结果表。

## 常见错误

错误 1：为了抽象而抽象。

如果只有一个实现，也看不到未来变化，先写普通函数更清楚。trait 应该解决真实重复和边界问题。

错误 2：热路径全部用 trait object。

这可能阻止内联。不是不能用，而是要知道代价。

错误 3：trait 设计太大。

一个 trait 应该表达一组紧密能力。不要写一个 `Engine` trait，里面同时包含读取数据、计算因子、写文件、打印日志。

## 和量化系统的连接

trait 最适合表达可替换策略：

- 不同 factor。
- 不同 backtest fill model。
- 不同 storage backend。
- 不同 scheduler policy。

generic 适合把这些抽象放在性能敏感路径里。trait object 适合配置驱动、插件列表、运行时组合。

## 代码走读与操作清单

先看 trait：

```rust
pub trait Factor {
    type Output;
    fn name(&self) -> &'static str;
    fn compute(&self, values: &[f64]) -> Self::Output;
}
```

`name` 返回 `&'static str`，因为 factor 名称是编译期固定字符串，不需要分配。`compute` 使用 `&[f64]`，因为 factor 只读取输入序列。

再看实现：

```rust
impl Factor for MeanFactor {
    type Output = Option<f64>;
}
```

这说明 mean 的输出可能不存在。空输入时没有均值，因此不是 `f64`。

最后看泛型运行器：

```rust
pub fn run_factor<F>(factor: F, values: &[f64]) -> F::Output
where
    F: Factor,
```

这个函数可以接收任何实现 `Factor` 的类型。它没有使用 `Box<dyn Factor>`，所以是静态分发。

操作清单：

1. 新增 `MinFactor`。
2. 让它实现 `Factor<Output = Option<f64>>`。
3. 用 `run_factor(MinFactor, &[3.0, 1.0])` 测试。
4. 再尝试把 `MeanFactor` 和 `MinFactor` 放进同一个 `Vec`。

这个实验会让你理解：泛型适合“调用时类型已知”，trait object 适合“运行时组合多个不同实现”。
