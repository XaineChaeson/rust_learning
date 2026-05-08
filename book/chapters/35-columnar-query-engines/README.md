# 第 35 章：Arrow、Parquet、Polars 和 DataFusion

前面的第 27、28 章已经介绍了在线特征、列式批处理、存储和序列化。本章进入真实 Rust 数据生态：Arrow、Parquet、Polars、DataFusion。

这些库是高性能量化基础设施中非常重要的工具，但它们也很容易让初学者迷失。你必须先理解列式数据模型、schema、projection、predicate filtering，再去使用真实库。

## 本章解决什么问题

量化研究中的数据规模会快速增长：

- 多资产。
- 多字段。
- 多频率。
- 多年历史。
- 多版本因子。
- 多次实验结果。

如果所有数据都放在 `Vec<Struct>` 中，你会遇到：

- 读取不需要的字段。
- cache locality 差。
- 序列化效率低。
- 批处理吞吐不足。
- 和 Python/Arrow 生态不兼容。

列式系统解决的是“分析型计算”的核心问题：只读需要的列，按列连续处理，尽量减少无关数据移动。

## 学习前提

你需要理解：

- 第 14 章：collections、bytes、string。
- 第 20 章：SoA/AoS、cache locality。
- 第 27 章：online feature 和 columnar batch。
- 第 28 章：serialization 和 schema。

如果你还不能解释 SoA 为什么适合按列计算，先回到第 20 章。

## Python 对照

Python 中你常用：

- Pandas DataFrame。
- Polars DataFrame。
- PyArrow Table。
- Parquet 文件。

你可能会写：

```python
df.select(["timestamp", "close"]).filter(pl.col("close") > 100)
```

Rust 中 Polars 也有类似能力，DataFusion 则更接近 SQL/query engine。

但底层问题是一致的：

- schema 如何定义？
- 每列长度是否一致？
- dtype 是否匹配？
- projection 是否只取需要的列？
- filter 是否应用到所有列？
- 结果是否仍然保持有效 batch？

本章示例就是这些问题的最小模型。

## Arrow 是什么

Arrow 是跨语言的列式内存格式。

它强调：

- 明确 schema。
- 列式数组。
- zero-copy 共享。
- 与 Parquet、DataFusion、Polars 等生态互通。

对于量化系统，Arrow 适合：

- 批量因子计算。
- Python/Rust 边界数据交换。
- 结果表和中间表。
- 多语言系统之间共享数据格式。

## Parquet 是什么

Parquet 是列式存储格式。

它适合：

- 历史行情。
- 因子结果。
- 回测结果。
- 实验输出。

它的关键能力：

- 列裁剪。
- 压缩。
- row group。
- predicate pushdown。

如果你只需要 `close` 和 `volume`，不应该读取所有字段。

## Polars 是什么

Polars 是高性能 DataFrame 库，Rust 和 Python 都可用。

它适合：

- 快速数据探索。
- 批处理转换。
- lazy query。
- 与 Python 研究流对齐。

对你的学习目标来说，Polars 是 Python 数据科学经验迁移到 Rust 的重要桥梁。

## DataFusion 是什么

DataFusion 是 Rust 中的 query engine。

它适合：

- SQL 查询。
- 查询优化。
- execution plan。
- Arrow batch 执行。

如果你要构建自定义研究数据平台或分布式批处理，DataFusion 的思想非常重要。

## 核心概念一：schema 是契约

在列式系统中，schema 不只是文档。

它定义：

- 列名。
- 数据类型。
- nullability。
- 顺序。
- 语义约束。

如果 schema 说 `close` 是 `f64`，但实际列是 `u64`，系统必须拒绝。

本章示例的 `RecordBatch::try_new` 会检查 schema 和 column 是否匹配。

## 核心概念二：projection 减少无关数据移动

projection 是只取需要的列。

例如：

```text
timestamp, close, volume, vwap
```

如果计算 rolling mean 只需要 `timestamp` 和 `close`，就不应该移动 `volume` 和 `vwap`。

这对大数据很重要。

本章示例的 `project(&["timestamp", "close"])` 展示这个过程。

## 核心概念三：predicate filtering 必须作用于整行

过滤条件可能只看一列：

```text
close > 100
```

但结果必须对所有列应用同一个 mask。

否则列之间会错位。

错位在量化系统中很危险：价格、时间戳和成交量可能不再对应同一条记录。

本章示例的 `filter_f64_gt` 会先生成 mask，再过滤所有列。

## 示例代码走读

示例位置：

```text
book/chapters/35-columnar-query-engines/example/src/lib.rs
```

运行：

```bash
cargo test -p ch35-columnar-query-engines
```

关键类型：

- `Field`
- `ColumnType`
- `Column`
- `RecordBatch`
- `QueryError`

这是一套极简 Arrow-like 模型。

## 代码走读：RecordBatch::try_new

构造 batch 时检查：

- schema 不为空。
- columns 不为空。
- schema 数量等于 column 数量。
- 所有列长度一致。
- 每个 field dtype 和 column dtype 匹配。

这就是专业数据系统的入口校验。

不要让坏数据进入计算内核。

## 代码走读：project

`project` 接收列名数组：

```rust
&["timestamp", "close"]
```

它通过列名查 index，然后 clone 对应列。

真实 Arrow/Polars 系统中可能使用引用计数、zero-copy 或 lazy plan 来减少复制。

但语义一样：只保留需要的列。

## 代码走读：filter_f64_gt

函数先找到列：

```rust
column_index(name)
```

然后检查列类型是 `F64`。

再生成 mask：

```text
close > threshold
```

最后对所有列应用 mask。

这就是 predicate filtering 的最小实现。

## 动手操作

1. 跑测试：

```bash
cargo test -p ch35-columnar-query-engines
```

2. 增加 `ColumnType::Utf8`。

3. 实现 `filter_u64_ge("timestamp", value)`。

4. 增加 null bitmap 的概念说明，先不实现。

5. 设计一个 Parquet 文件 layout：按日期分区还是按 symbol 分区？

## 常见错误

错误 1：schema 只写文档，不做校验。

schema 必须成为构造时的检查。

错误 2：过滤一列后忘记同步过滤其他列。

这会破坏行对齐。

错误 3：所有查询都读取全列。

projection 是列式系统的核心收益之一。

错误 4：把在线低延迟和离线列式混在一起。

在线系统关注增量和延迟，列式批处理关注吞吐和扫描效率。

错误 5：过早引入重型查询引擎。

先明确数据模型和访问模式，再引入 DataFusion 或 Polars。

## 量化/HPC 连接

列式生态适合：

- 多资产因子批处理。
- 历史数据扫描。
- 实验结果查询。
- Python/Rust 数据交换。
- 研究平台的数据中间层。

不适合：

- 每条 tick 都要低延迟更新的小状态。
- 强顺序事件驱动状态机。
- 单条记录频繁小写入。

专业系统通常同时存在：

```text
online row/event path
offline columnar batch path
```

你要知道边界在哪里。

## 本章验收

你通过本章时，应该能做到：

1. 解释 Arrow、Parquet、Polars、DataFusion 的区别。
2. 解释 schema 为什么是契约。
3. 实现 projection。
4. 实现 predicate filtering。
5. 说明列式系统如何服务因子批处理。
6. 判断什么时候不该用列式系统。

## 进入下一章前

确认你能把 Pandas/Polars 的 DataFrame 操作翻译成 schema、column、projection、filter。下一章进入生产 runtime、tracing 和 metrics。
