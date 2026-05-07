# 练习参考解法与检查点

这不是用来替代练习的答案库。正确使用方式是：

1. 先自己做 `book/exercises.md`。
2. 跑本章测试。
3. 卡住时只看对应条目的“检查点”。
4. 做完后再看“专业判断”。

自学最大的问题不是不会写代码，而是不知道自己的代码是否已经满足专业标准。本文件的作用是补上这个判断。

## A. Rust 入门计算

### A1：`ch01` 增加学习小时数变量

修改位置：`book/examples/ch01-rust-program-shape/src/bin/variables.rs`

检查点：

- 使用 `let` 定义变量。
- 能打印变量。
- 不需要 mutable，除非你真的修改它。

专业判断：

- Rust 默认 immutable。对计算代码来说，这能减少“变量中途被改掉”的心智负担。

### A2：`ch02` 实现 `min`、`max`、`cumulative_product`

修改位置：`book/examples/ch02-values-types-functions/src/lib.rs`

检查点：

- 空输入要有明确行为，可以返回 `None` 或 `Vec::new()`，不要 panic。
- `min` 和 `max` 处理 `f64` 时要考虑 NaN 策略。
- `cumulative_product` 输出长度应该等于输入长度。

专业判断：

- 金融计算中 NaN 策略必须写清楚。沉默传播 NaN 和显式拒绝 NaN 是两种不同产品语义。

### A3：`ch03` 实现 `take_last(values, n)`

修改位置：`book/examples/ch03-vec-slice-string/src/lib.rs`

检查点：

- 返回类型应该是 borrowed slice：`&[f64]`。
- 不应该分配新的 `Vec<f64>`。
- `n >= values.len()` 时返回整个输入。

专业判断：

- view API 是 Rust 高性能计算的基础。能借用就不要复制。

### A4：`ch04` 实现 `add_in_place`

修改位置：`book/examples/ch04-ownership-borrowing/src/lib.rs`

检查点：

- 参数使用 `&mut [f64]`。
- 原地修改，不返回新 Vec。
- 测试要证明输入 buffer 被修改。

专业判断：

- 原地 API 适合热路径，但调用方必须接受输入被改变。API 名称要体现这一点。

### A5：`ch05` 增加 NaN、inf、非正价格验证

修改位置：`book/examples/ch05-option-result-errors/src/lib.rs`

检查点：

- NaN 和 inf 用 `is_finite()` 检查。
- 非正价格用 `value <= 0.0` 检查。
- 错误类型携带 index 和 value。

专业判断：

- 数据错误应该尽早暴露，并带足上下文。只返回 `false` 会让排查成本很高。

### A6：`ch06` 给 `Signal` 增加 `ClosePosition`

修改位置：`book/examples/ch06-struct-enum-match/src/lib.rs`

检查点：

- enum 增加 variant。
- 所有 `match` 分支必须更新。
- 测试覆盖新 variant。

专业判断：

- Rust 的 exhaustiveness check 会强迫你处理新增业务状态，这是 enum 比字符串状态码更可靠的原因。

### A7：`ch07` 写 `assert_close_vec`

修改位置：`book/examples/ch07-testing-numeric-code/src/lib.rs`

检查点：

- 先比较长度。
- 再逐项近似比较。
- panic 信息包含 index、left、right。

专业判断：

- 数值测试失败时，错误信息要能定位，不要只说 assertion failed。

### A8：`ch08` 实现增量 `rolling_sum`

修改位置：`book/examples/ch08-rolling-mean/src/lib.rs`

检查点：

- 初始化第一个窗口 sum。
- 每向右移动一步，加新值、减旧值。
- 输出长度为 `n - window + 1`。
- 与朴素版测试一致。

专业判断：

- 这是第一次从 `O(n * window)` 迁移到 `O(n)`。你要同时保留正确 baseline 和优化版本。

## B. 专业 Rust 核心

### B1：`ch09` 新增 `risk` 模块

检查点：

- 模块内部字段默认私有。
- 通过构造函数建立有效状态。
- 公开 API 不泄漏内部表示。

专业判断：

- 模块边界是专业工程的第一层防线。公开字段越多，后续越难重构。

### B2：`ch10` 新增 `MaxFactor` 和 `RollingFactor`

检查点：

- trait 名称表达能力，而不是表达某个具体实现。
- generic 函数用 trait bound 表达输入约束。
- 如果不需要运行时多态，不要优先使用 `Box<dyn Trait>`。

专业判断：

- 高频或批量计算核心通常优先 static dispatch。

### B3：`ch11` 实现 `first_window<'a>`

检查点：

- 返回值生命周期来自输入 slice。
- 空输入或 window 过大返回 `None`。
- 不分配。

专业判断：

- lifetime 不是语法噪声，它记录“这个 view 借用了谁”。

### B4：`ch12` 用 `fold`、`scan` 各实现一个函数

检查点：

- `fold` 适合最终汇总。
- `scan` 适合输出中间状态。
- 测试证明 iterator 版本和 loop 版本一致。

专业判断：

- iterator 不是为了炫技。热路径里要能读懂、能测量、能解释。

### B5：`ch13` 复用 scratch buffer

检查点：

- buffer 由调用方或外层 owner 持有。
- 热路径中不重复分配。
- 测试覆盖多次调用后结果仍正确。

专业判断：

- 分配控制经常比微小算术优化更重要。

### B6：`ch14` symbol count 和 timestamp range sum

检查点：

- symbol count 使用 `HashMap`。
- timestamp range 使用 `BTreeMap::range` 或先排序 Vec。
- 测试覆盖重复 symbol 和空范围。

专业判断：

- 选择集合时先问访问模式，不要先背 API。

### B7：`ch15` 设计上层 `DataError`

检查点：

- 下层错误用 `From` 转换。
- 上层错误保留上下文。
- 调用处可以继续使用 `?`。

专业判断：

- 错误架构的目标是让调用方能做决策，不是让错误字符串更漂亮。

### B8：`ch16` partial result + merge

检查点：

- worker 不共享全局可变 Vec。
- 每个 worker 返回自己的结果。
- 主线程合并。

专业判断：

- 高性能并行通常偏好 ownership transfer，而不是共享锁。

### B9：`ch17` duplicate/gap 统计

检查点：

- 状态机记录 last sequence。
- duplicate 不应推进状态。
- gap 要暴露 metric 或错误。

专业判断：

- 在线系统的正确性经常来自状态机，而不是来自单个计算函数。

### B10：`ch18` feature flag 和 doc test

检查点：

- feature 名称表达能力。
- 默认 feature 不应偷偷改变语义。
- doc test 能被 `cargo test` 跑到。

专业判断：

- feature flag 会扩大测试矩阵。增加前要知道谁会维护它。

## C. 高性能计算核心

### C1：`ch19` benchmark 计划

检查点：

- 明确 baseline 和 candidate。
- 使用 release profile。
- 固定数据规模和重复次数。
- 记录机器和 Rust 版本。

专业判断：

- 没有记录上下文的性能数字不可复现，也很难用于工程决策。

### C2：`ch20` `returns_from_close`

检查点：

- 输出长度为 `close.len() - 1`。
- 不在循环里分配。
- 对空输入和单元素输入有明确行为。

专业判断：

- SoA 的收益来自按列连续访问。收益不是抽象概念，要和访问模式绑定。

### C3：`ch21` `row()`、`transpose_copy()`

检查点：

- `row()` 返回 borrowed slice。
- shape 检查必须明确。
- `transpose_copy()` 可以分配，因为输出布局改变。

专业判断：

- view 和 copy 的边界要体现在 API 名称和返回类型里。

### C4：`ch22` 并行输出携带 index

检查点：

- worker 输出 `(index, value)`。
- 合并后按 index 排序或放回固定位置。
- 测试不依赖线程完成顺序。

专业判断：

- 并行系统里，稳定 id 是可复现性的基础。

### C5：`ch23` unsafe safety invariant

检查点：

- 注释说明调用者必须保证什么。
- unsafe 块尽量小。
- safe wrapper 做边界检查。
- 测试覆盖边界条件。

专业判断：

- unsafe 不是“我知道自己在做什么”，而是“我能写出编译器无法证明但人类可以审查的不变量”。

## D. 量化系统工程

### D1：`ch24` copy boundary 和 borrowed kernel

检查点：

- 核心 kernel 只依赖 `&[f64]`。
- Python 边界负责转换和校验。
- copy 是否瓶颈要通过 benchmark 判断。

专业判断：

- 先做清晰边界，再做 zero-copy。

### D2：`ch25` 回测加入交易成本

检查点：

- 成本与成交量或换手绑定。
- cash 和 equity 都要受影响。
- 测试覆盖有成本和无成本两种情况。

专业判断：

- 回测不是只算收益向量，它是一个状态推进系统。

### D3：`ch26` seed 和 experiment id

检查点：

- 相同 seed 产生相同结果。
- experiment id 能追踪参数组合。
- 输出记录包含参数和结果。

专业判断：

- 不可复现实验没有研究价值。

### D4：`ch27` 乱序事件处理策略

检查点：

- 明确 drop、buffer、reorder 或 reject。
- 策略写进测试。
- metric 记录乱序数量。

专业判断：

- 在线计算必须把数据质量问题变成显式策略。

### D5：`ch28` schema/version

检查点：

- record 带 version。
- 解码时检查 version。
- 测试覆盖未知 version。

专业判断：

- 长期系统一定会遇到格式演化。没有 version 的格式很难安全升级。

### D6：`ch29` retry attempt

检查点：

- task id 稳定。
- attempt 递增。
- retry 不重复写最终结果。

专业判断：

- 分布式系统默认会失败。retry 和 idempotency 必须一开始设计。

### D7：`ch30` `max_retries` 和 metric

检查点：

- 配置有默认值和校验。
- metric 名称稳定。
- 测试覆盖非法配置。

专业判断：

- config 是生产接口。随意修改默认值可能改变系统行为。

### D8：`ch31` 端到端 pipeline

检查点：

- data、factor、backtest、report 边界清楚。
- 每个阶段输入输出明确。
- 错误能向上汇总。

专业判断：

- 最终能力不是写一个快函数，而是把快函数放进可靠系统。

## E. 项目练习

项目位置：`projects/01-factor-core`

检查点：

- 所有公开函数都接受 borrowed input。
- 所有非法输入都返回 `ComputeError`。
- rolling 输出右对齐。
- 浮点测试使用近似比较。
- 优化版本必须和 baseline 对照。
- benchmark 必须使用 release profile。

推荐验证：

```bash
cargo test -p factor-core
cargo run -p factor-core --release --bin bench
```

专业判断：

- 项目练习的目标是形成作品，而不是完成题目。你应该能把 README、测试、benchmark 报告一起交给一个面试官看。

## F. 生态与生产级扩展

### F1：`ch32` 最小样本数规则

检查点：

- `decide` 在比较 speedup 前先检查 sample count。
- 样本不足返回 `Inconclusive`。
- 测试覆盖 baseline 样本不足和 candidate 样本不足。

专业判断：

- 性能结论需要统计基础。样本不足时，诚实说无法判断。

### F2：`ch33` `mean_threaded`

检查点：

- 复用 `partition_ranges`。
- 每个 worker 返回 partial sum 和 count。
- merge 时用 total sum / total count。
- 测试和单线程 mean 对照。

专业判断：

- 并行 mean 不应该让多个线程共享一个全局 sum 锁。

### F3：`ch34` shape 校验

检查点：

- shape 为空或维度不符时返回错误。
- shape product 要和 values length 一致。
- kernel 仍然只接收 `&[f64]`。

专业判断：

- Python boundary 负责 dtype、shape、layout；kernel 负责计算。

### F4：`ch35` `Utf8` 列

检查点：

- `ColumnType` 增加 `Utf8`。
- `Column` 增加 `Utf8(Vec<String>)`。
- `len`、`dtype`、`filter` 都同步更新。
- 测试 projection 时保留 symbol 列。

专业判断：

- schema 演进时，所有构造和查询路径都要更新。

### F5：`ch36` 新指标

检查点：

- `events_seen_total` 每个事件都加一。
- `queue_depth` 如果用 gauge，需要定义覆盖语义。
- metric 名称稳定、可测试。

专业判断：

- metrics 是生产接口，不能随意改名。

### F6：`ch37` max attempts 失败测试

检查点：

- 第一次 lease 过期后回到 pending。
- 最后一次 lease 过期后进入 failed。
- failed task 不再被 lease。

专业判断：

- retry 必须有上限，否则坏任务会无限消耗资源。
