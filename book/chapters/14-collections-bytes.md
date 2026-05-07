# 第 14 章：collections、bytes、String 和数据边界

量化系统处理的数据既有数值列，也有 symbol、timestamp、CSV、binary、network payload。专业 Rust 开发者必须知道什么时候处理字符串，什么时候处理字节，什么时候使用 map。

## `HashMap` 和 `BTreeMap`

`HashMap` 适合按 key 快速查最新值：

```rust
symbol -> latest_price
```

它不保证遍历顺序。不要把 `HashMap` 的输出顺序写进测试或业务逻辑。

`BTreeMap` 保持 key 有序，适合：

- 按 timestamp 顺序遍历。
- 做范围查询。
- 需要确定性输出。

代价是查找常数通常比 hash 大。

## String、&str、&[u8]

`String` 拥有 UTF-8 文本。`&str` 是 UTF-8 文本视图。`&[u8]` 是字节视图，不保证是合法 UTF-8。

CSV、网络包、二进制文件一开始都是 bytes。只有当你需要按文本解释时，才验证 UTF-8：

```rust
let text = std::str::from_utf8(bytes)?;
```

不要在热路径里频繁把 bytes 转 String。如果只是解析数字或字段，可以尽量借用原 buffer。

## symbol 的成本

量化数据里 symbol 会重复出现几百万次。如果每行都保存 `String`，会有大量重复分配。生产系统常用：

- symbol interning
- integer symbol id
- dictionary encoding
- Arrow dictionary array

学习阶段先用 `String`，但你必须知道这不是最终高性能形态。

## 本章示例

```bash
cargo test -p ch14-collections-bytes
```

重点看：

- `latest_prices` 使用 `HashMap` 表达 latest lookup。
- `ordered_by_timestamp` 使用 `BTreeMap` 表达有序数据。
- `parse_ascii_price` 先验证 bytes 再解析。

## 本章练习

1. 写 `symbol_counts(rows: &[&str]) -> HashMap<String, usize>`。
2. 写 `range_sum(map: &BTreeMap<u64, f64>, start, end)`。
3. 尝试解析非法 UTF-8，观察返回值。
4. 写复盘：为什么 symbol 不应该在最终系统里每行一个 String？

## 本章验收

你应该能解释：

- `HashMap` 和 `BTreeMap` 的选择标准。
- `String`、`&str`、`&[u8]` 的边界。
- 为什么文本解析常常不是数值计算的热路径。
- symbol interning 解决什么问题。

## 教材化补充：集合选择影响系统语义

`HashMap` 和 `BTreeMap` 不只是性能不同，它们表达的语义也不同。

`HashMap` 表达“我需要按 key 快速找到值”。它不承诺顺序。

`BTreeMap` 表达“key 的顺序重要”。它适合时间戳、范围查询和确定性输出。

如果测试依赖 `HashMap` 遍历顺序，测试本身就是错的。

## bytes 到 text 的边界

网络、文件、压缩块、Parquet page，进入程序时本质上都是 bytes。只有当你确认它是合法 UTF-8，才能把它当作 `&str`。

```rust
std::str::from_utf8(bytes)
```

这一步可能失败。失败不是异常情况，而是输入数据质量的一部分。

## symbol 不是普通字符串

量化数据中 symbol 重复率极高。每行一个 `String` 可能导致大量重复分配。生产系统常用：

- `u32` symbol id。
- dictionary encoding。
- intern table。
- Arrow dictionary array。

学习阶段使用 `String` 是为了简单，但你要知道它不是最终高性能表示。

## 示例走读

```text
book/examples/ch14-collections-bytes/src/lib.rs
```

`latest_prices` 用 `HashMap`，因为我们关心 symbol 的最新值。`ordered_by_timestamp` 用 `BTreeMap`，因为时间顺序是语义的一部分。`parse_ascii_price` 展示 bytes 需要先转成合法文本。

## 常见错误

错误 1：为了排序使用 HashMap。

HashMap 不保证顺序。需要顺序就用 BTreeMap 或排序 Vec。

错误 2：把所有 bytes 都当 String。

String 必须是 UTF-8。二进制数据不是文本。

错误 3：忽视 key 的分配成本。

如果 key 是频繁重复的 symbol，应该考虑 interning 或 id 映射。

## 代码走读与操作清单

`latest_prices` 的输入是：

```rust
rows: &[(&str, f64)]
```

这里 symbol 是 borrowed `&str`，说明输入不拥有 symbol。函数内部要把 symbol 存进 `HashMap<String, f64>`，所以必须创建 owned `String`。

这一步是一次明确分配。它合理，因为 map 需要拥有 key。

`ordered_by_timestamp` 使用：

```rust
BTreeMap<u64, f64>
```

timestamp 用 `u64`，因为它是非负整数，可以表达 epoch、纳秒或序列号。BTreeMap 保证按 key 有序迭代。

`parse_ascii_price` 展示 bytes 到 text：

```rust
let text = std::str::from_utf8(bytes).ok()?;
```

如果 bytes 不是合法 UTF-8，返回 `None`。这比假设输入永远合法更专业。

操作清单：

1. 给 `latest_prices` 增加 symbol count。
2. 给 `ordered_by_timestamp` 增加范围求和。
3. 传入非法 UTF-8，确认解析失败。
4. 写下什么时候 map 应该拥有 key，什么时候可以借用 key。

## 自测与复盘问题

1. `HashMap` 为什么不能用于依赖遍历顺序的逻辑？
2. `BTreeMap` 的有序性适合哪些量化数据场景？
3. `String` 和 `&str` 谁拥有数据？
4. `&[u8]` 什么时候才能变成 `&str`？
5. symbol interning 为什么能减少内存和分配？

如果这些问题回答不出来，重新做 `parse_ascii_price` 和 `latest_prices` 的练习。

## 进入下一章前

确认你能为一个 symbol 最新价表选择 `HashMap`，为一个时间序列表选择 `BTreeMap`，并能解释 bytes、text、owned string 的转换成本。做到这些，再进入错误架构。

## 额外复盘

把本章内容映射到真实数据管线：原始文件通常先是 bytes，解析后变成 typed record，symbol 可能被编码成 id，最后进入列式或行式结构。你应该能说清每一步是否分配、是否验证、是否保序。

## 专业判断清单

选择集合时按问题问自己：

1. 查询是按 key、按顺序、还是按连续下标？
2. 数据是否需要稳定遍历顺序？
3. key 是否频繁重复，是否值得编码成整数 id？
4. 字符串是否真的需要拥有，还是只需要临时借用？
5. 输入 bytes 是否已经验证过编码和格式？

量化系统中，集合选择会直接影响延迟、内存和可复现性。把这些问题说清楚，比背诵 `HashMap` 和 `BTreeMap` 的 API 更重要。
