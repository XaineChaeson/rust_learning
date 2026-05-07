# 第 28 章：存储、序列化和数据格式边界

高性能计算系统最终都要读写数据。数据格式选择会影响 IO、CPU、内存、跨语言兼容和可恢复性。

## 文本格式

CSV、JSON 容易调试，但解析成本高、类型弱、体积大。

适合：

- 小数据。
- 人工检查。
- 边界输入输出。
- 学习阶段。

不适合：

- 高频 tick。
- 大规模因子矩阵。
- 低延迟在线路径。

## 二进制格式

二进制格式体积小、解析快，但需要显式处理：

- endian。
- schema。
- version。
- null。
- compatibility。

`f64::to_le_bytes()` 让 endian 明确。生产系统不建议自己发明复杂二进制格式，除非你完全控制读写双方。

## Arrow 和 Parquet

Arrow 是内存列式格式，适合跨语言零拷贝或少拷贝分析。

Parquet 是列式存储格式，适合离线批处理和压缩。

Polars、DataFusion、PyArrow 都围绕这些思想工作。你不必第一天使用它们，但必须理解：列式格式把数据布局和查询优化绑定在一起。

## schema 演进

生产数据会演进：

- 增加列。
- 修改含义。
- 修复错误。
- 不同版本并存。

没有 schema 策略，历史实验不可复现。

## 本章示例

```bash
cargo test -p ch28-storage-serialization
```

重点看：

- 文本编码如何 roundtrip。
- binary encoding 如何显式 endian。

## 本章练习

1. 给 `FactorRecord` 增加 `factor_name`。
2. 更新 encode/decode 并保持测试。
3. 增加非法行测试。
4. 写复盘：为什么生产系统通常不用手写 CSV 保存大规模因子矩阵？

## 本章验收

你应该能解释：

- CSV/JSON/binary/Arrow/Parquet 的取舍。
- endian 和 schema 为什么重要。
- 为什么列式格式适合因子批处理。
- 数据格式如何影响实验可复现。

## 教材化补充：格式选择是系统设计

数据格式不是“保存成什么文件”这么简单。它影响：

- 读写速度。
- 文件大小。
- 类型信息。
- 跨语言兼容。
- schema 演进。
- 历史实验能否复现。

CSV 容易看，Parquet 更适合大规模列式分析。二者不是谁绝对更好，而是服务不同阶段。

## 文本格式的边界

CSV 适合学习和小样例，因为你能直接打开看。但它有问题：

- 没有强 schema。
- 数字解析成本高。
- 缺失值规则不统一。
- 大文件读取慢。

所以生产因子矩阵通常不会长期用 CSV 作为主格式。

## Arrow/Parquet 的直觉

Arrow 关注内存中的列式表示，Parquet 关注磁盘上的列式存储。它们都服务分析型计算：只读需要的列，尽量少搬运数据。

如果一个策略只用 `close` 和 `volume`，列式格式不需要读取其他字段。

## 示例走读

```text
book/examples/ch28-storage-serialization/src/lib.rs
```

`encode_line`/`decode_line` 是文本 roundtrip。`encode_f64_le` 展示二进制格式必须明确 endian。

## 常见错误

错误 1：手写复杂二进制格式却没有 schema。

未来读不回来，或者读错也不知道。

错误 2：只保存结果，不保存生成结果的参数和数据版本。

这会破坏实验可复现。

错误 3：把存储格式和内存格式混为一谈。

磁盘格式、网络格式、内存格式可以不同，需要明确转换边界。

## 代码走读与格式演进

看 record：

```rust
pub struct FactorRecord {
    pub timestamp: u64,
    pub symbol: String,
    pub value: f64,
}
```

这是逻辑数据模型，不等于最终存储格式。

看文本编码：

```rust
format!("{},{},{}", record.timestamp, record.symbol, record.value)
```

这适合学习 roundtrip，但没有 escaping、schema version、null 处理。生产 CSV 需要更完整的库。

看二进制编码：

```rust
value.to_le_bytes()
```

`le` 表示 little endian。二进制格式必须显式规定 endian，否则跨机器/语言读取会出问题。

操作清单：

1. 给 `FactorRecord` 增加 `factor_name`。
2. 给编码增加 version 字段。
3. 测试旧格式是否解析失败或兼容。
4. 写下什么时候应该换成 Arrow/Parquet。

专业目标：数据格式要能长期演进，而不是只服务今天的输出。

## 自测与复盘问题

1. CSV 的优点和缺点分别是什么？
2. binary 格式为什么必须明确 endian？
3. schema version 解决什么问题？
4. Arrow 和 Parquet 为什么适合列式分析？
5. 存储格式如何影响实验可复现？

如果这些问题回答不出来，先为 `FactorRecord` 写一个带 version 的文本格式。

## 进入下一章前

确认你能说清 CSV、binary、Arrow、Parquet 分别适合什么阶段，并能解释 schema version 为什么影响历史实验复现。做到这些，再进入分布式调度。
