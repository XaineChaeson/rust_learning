# 第 6 章：`struct`、`enum` 和 `match`

到目前为止，你已经能写一些数值函数。下一步是建模。量化系统不是只有数组，还有资产、时间序列、bar、因子、信号、订单、组合状态。

Rust 用 `struct` 表达数据结构，用 `enum` 表达有限状态或分支。

## `struct`

一个 OHLCV bar 可以这样表达：

```rust
pub struct Bar {
    pub symbol: String,
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}
```

这比随便用 tuple 清楚得多：

```rust
(String, String, f64, f64, f64, f64, f64)
```

字段名就是文档。未来测试失败时，你也能更快定位是 high 错、close 错，还是 volume 错。

## `impl`

方法写在 `impl` 中：

```rust
impl Bar {
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }
}
```

`&self` 表示方法只读借用这个 `Bar`。

## `enum`

交易信号可以是：

```rust
pub enum Signal {
    Buy,
    Sell,
    Hold,
    TargetWeight(f64),
}
```

这比字符串安全：

```python
signal = "BUY"
```

字符串可能写错成 `"BYY"`。Rust enum 限制了合法状态。

## `match`

处理 enum 用 `match`：

```rust
fn signal_value(signal: Signal) -> f64 {
    match signal {
        Signal::Buy => 1.0,
        Signal::Sell => -1.0,
        Signal::Hold => 0.0,
        Signal::TargetWeight(weight) => weight,
    }
}
```

Rust 会检查你是否覆盖了所有分支。后续如果你新增一个 `Signal::CloseAll`，编译器会提醒哪些 match 需要更新。

## 数据建模的原则

量化系统建模时，优先问：

1. 这个概念是数据记录，还是状态分支？
2. 字段是否都必需？
3. 哪些值非法？
4. 错误应该在哪里发现？
5. 是否需要拥有字符串或数组？

比如 `TimeSeries`：

```rust
pub struct TimeSeries {
    pub symbol: String,
    pub values: Vec<f64>,
}
```

它拥有数据。计算函数可以借用：

```rust
impl TimeSeries {
    pub fn values(&self) -> &[f64] {
        &self.values
    }
}
```

## Python 对照

Python 中你可能用 dict：

```python
bar = {
    "symbol": "AAPL",
    "close": 101.5,
}
```

这种写法灵活，但字段缺失、类型错误、拼写错误容易在运行时才发现。

Rust 的 `struct` 更啰嗦，但让数据合同提前明确。

## 本章练习

1. 定义 `Bar`。
2. 给 `Bar` 实现 `typical_price`。
3. 定义 `Signal`。
4. 写函数 `signal_to_weight(signal: Signal) -> f64`。
5. 定义 `TimeSeries`，实现 `len` 和 `is_empty`。
6. 为这些方法写测试。

## 本章验收

你可以进入下一章，如果你能回答：

- 什么时候用 `struct`？
- 什么时候用 `enum`？
- `match` 为什么适合处理信号？
- `&self` 表示什么？
- 为什么用类型建模比 dict 更适合长期系统？

## 教材化补充：从 dict 到类型建模

Python 数据分析里，经常用 dict 或 DataFrame row 表示一条记录：

```python
bar = {"symbol": "AAPL", "close": 100.0}
```

这种写法灵活，但系统变大后会出现问题：

- 字段名拼错运行时才发现。
- 某个字段可能不存在。
- 字段类型不清楚。
- 非法状态很容易混进来。

Rust 用 `struct` 表达固定结构：

```rust
pub struct Bar {
    pub symbol: String,
    pub close: f64,
}
```

编译器会保证字段存在、类型正确。

## enum 表达有限状态

交易信号不是任意字符串，而是有限集合：

```rust
pub enum Signal {
    Buy,
    Sell,
    Hold,
    TargetWeight(f64),
}
```

这比 `"BUY"`、`"Sell"`、`"long"` 混用更可靠。`match` 会迫使你处理每一种情况。

## `impl` 把行为放到数据旁边

```rust
impl Bar {
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }
}
```

`&self` 表示方法借用当前对象，不取得所有权。调用后 `Bar` 仍然可用。

## 动手操作

运行：

```bash
cargo test -p ch06-struct-enum-match
```

打开：

```text
book/chapters/06-struct-enum-match/example/src/lib.rs
```

观察：

- `Bar` 是市场数据记录。
- `Signal` 是策略输出状态。
- `signal_to_weight` 把业务状态转成数值权重。

## 常见错误

错误 1：所有状态都用字符串。

字符串无法让编译器检查分支完整性。

错误 2：struct 字段全部公开。

学习阶段可以公开，专业库中应谨慎。字段公开后，数据布局和字段名都变成 API 承诺。

错误 3：match 用 `_` 兜底太早。

如果你用 `_`，以后 enum 增加新 variant 时，编译器不会提醒你补逻辑。核心业务状态不建议过早用 `_`。
