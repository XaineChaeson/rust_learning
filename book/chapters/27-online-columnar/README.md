# 第 27 章：在线特征和列式计算

量化系统通常同时有两条计算路径：离线批处理和在线增量计算。离线关注吞吐，在线关注延迟。两者共享很多因子定义，但执行模型不同。

## 在线 rolling

在线 rolling 不能每来一个新价格就重新扫描历史窗口。它应该维护状态：

- ring buffer
- running sum
- running variance state
- last timestamp
- missing/out-of-order policy

第 27 章示例的 `OnlineMean` 就是最小 ring buffer 思想。

## latency 和 throughput

在线系统常见指标：

- p50/p95/p99 latency
- events per second
- queue depth
- drop count
- out-of-order count

只看平均延迟是不够的。交易系统和实时风控尤其关心尾延迟。

## 列式批处理

离线因子计算适合列式：

```text
close: [ ... ]
volume: [ ... ]
symbol_id: [ ... ]
```

列式布局能减少不需要字段的读取。查询引擎还能做：

- projection pushdown：只读取需要列。
- predicate pushdown：提前过滤。
- lazy plan：优化表达式顺序。

## 本章示例

```bash
cargo test -p ch27-online-columnar
```

重点看：

- `OnlineMean` 如何增量更新。
- `filter_prices_above` 如何表达列式 filter。

## 本章练习

1. 给 `OnlineMean` 增加 `count()`。
2. 写 `OnlineSum`。
3. 增加乱序事件策略设计文档。
4. 写复盘：同一个 rolling mean，离线和在线实现为什么不同？

## 本章验收

你应该能解释：

- 在线 rolling 为什么需要 ring buffer。
- latency 和 throughput 的区别。
- 列式计算为什么适合分析型因子。
- pushdown 优化解决什么问题。

## 教材化补充：离线和在线不是同一个实现

同一个 rolling mean，在离线批处理中可以扫描数组，在在线系统中必须增量更新。

离线版本：

```text
input array -> windows -> output array
```

在线版本：

```text
new event -> update state -> output latest feature
```

在线版本需要保存状态，而不是每次回头扫描历史。

## ring buffer 的直觉

固定窗口只关心最近 `window` 个值。ring buffer 用固定大小数组循环覆盖旧值：

- 新值进入。
- 最旧值离开。
- 更新 running sum。
- 计算当前 mean。

这样每个事件 `O(1)`。

## 列式和在线的关系

离线列式关注吞吐，在线特征关注延迟。两者可以共享因子定义，但执行引擎不同。

生产系统中常见设计：

- 离线 batch 生成历史特征。
- 在线 engine 增量生成最新特征。
- 两者用同一批手算样例校验一致性。

## 示例走读

```text
book/chapters/27-online-columnar/example/src/lib.rs
```

`OnlineMean` 保存窗口、游标、sum 和 filled。`filter_prices_above` 展示列式 filter 的最小形式。

## 常见错误

错误 1：在线每次重算完整窗口。

这会让延迟随历史长度增长。

错误 2：只看平均延迟。

在线系统必须关心 p95/p99。

错误 3：不处理乱序和缺失。

真实行情和数据流一定会有异常。

## 代码走读与状态更新

看 `OnlineMean` 的状态：

```rust
window: usize,
values: Vec<f64>,
cursor: usize,
sum: f64,
filled: usize,
```

这些字段共同表达 ring buffer：

- `values` 保存固定窗口。
- `cursor` 指向下一次写入位置。
- `sum` 避免重新求和。
- `filled` 表示窗口是否已经填满。

更新时有两种情况：

1. 窗口没满：只增加 sum。
2. 窗口已满：先减去被覆盖的旧值，再加新值。

操作清单：

1. 给 `OnlineMean` 增加 `count()`。
2. 写测试覆盖窗口没满时返回 `None`。
3. 写测试覆盖覆盖旧值后的均值。
4. 设计乱序事件时是否更新状态。

专业判断：在线特征的核心是固定延迟和可控状态，不是把离线函数包一层循环。

## 自测与复盘问题

1. 在线 rolling 为什么不能每次扫描完整历史？
2. ring buffer 中 cursor 表示什么？
3. p99 latency 为什么比平均延迟更重要？
4. 在线特征和离线列式批处理的目标差异是什么？
5. 乱序事件应该丢弃、修正还是重算？

如果这些问题回答不出来，先用纸模拟 `OnlineMean` 的四次 update。

## 进入下一章前

确认你能解释离线 batch 和在线 update 的状态差异，并能说明 ring buffer 如何保证固定窗口更新成本。做到这些，再进入存储和序列化。

## 额外复盘

把本章内容映射到实时系统：离线结果可以晚几分钟，在线特征必须稳定响应。你应该能说明吞吐、延迟、乱序和背压分别对应什么工程风险。
