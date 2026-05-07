# 性能实验手册

这份文件是本项目的性能实验入口。它补齐一个常见缺口：很多学习材料会告诉你“Rust 很快”，但不会教你怎样证明你的 Rust 代码真的更快。

本项目的原则是：

1. 先写正确 baseline。
2. 再写优化 candidate。
3. 用 release benchmark 比较。
4. 优化后重新跑正确性测试。
5. 不用没有测量依据的性能结论。

## Debug 和 release

Rust 的 debug 构建保留大量调试信息，优化级别低。它适合开发，不适合性能结论。

性能实验必须使用：

```bash
cargo run -p factor-core --release --bin bench
```

不要用下面命令得出性能结论：

```bash
cargo run -p factor-core --bin bench
```

debug 数字只能用于确认程序能跑，不能用于比较算法优劣。

## 当前可运行实验

`projects/01-factor-core` 提供一个不依赖第三方库的 benchmark harness：

```bash
cargo run -p factor-core --release --bin bench
```

它比较：

- `rolling_mean`：朴素 baseline，复杂度约为 `O(n * window)`。
- `rolling_mean_incremental`：增量版本，复杂度约为 `O(n)`。

默认数据规模：

- `len = 200_000`
- `window = 252`
- `repeat = 20`

你也可以传参：

```bash
cargo run -p factor-core --release --bin bench -- --len 1000000 --window 252 --repeat 10
```

需要留下可审查 artifact 时，写出 Markdown 报告：

```bash
cargo run -p factor-core --release --bin bench -- --len 1000000 --window 252 --repeat 10 --output target/benchmark-reports/factor-core.md
```

`target/benchmark-reports/` 是本地运行产物，不需要提交。你需要把其中关键结论整理进自己的性能实验复盘。

## 输出应该怎么看

关注三件事：

1. baseline 和 candidate 的结果是否一致。
2. 每轮平均耗时。
3. speedup 是否稳定。

如果优化版本更快，但结果不同，这不是优化成功，而是 bug。

如果优化版本只在很小数据上更快，不能直接推广到大规模研究任务。

如果每次运行波动很大，要增加 repeat，关闭其他重负载程序，并记录噪声。

## benchmark 记录模板

```text
日期：
机器：
CPU：
内存：
OS：
Rust 版本：
命令：
profile：
数据规模：
window：
repeat：
baseline：
candidate：
baseline 平均耗时：
candidate 平均耗时：
speedup：
正确性校验：
噪声/异常：
结论：
下一步：
```

## 实验设计规则

### 1. 只比较一个变量

不要同时改变算法、数据布局、并行策略和编译参数。否则你不知道性能变化来自哪里。

好的实验：

- 同一输入。
- 同一输出。
- 同一 profile。
- 只比较朴素 rolling mean 和增量 rolling mean。

坏的实验：

- baseline 用 debug。
- candidate 用 release。
- candidate 改了输出规则。
- 同时加并行和 unsafe。

### 2. baseline 必须保留

baseline 的价值不是快，而是可信。优化版本应该持续和 baseline 对照。

常见做法：

- 小数据用 `assert_close_vec` 和 baseline 对照。
- 大数据抽样检查头尾和长度。
- 每次优化后先跑 `cargo test`。

### 3. IO 和计算分开测

如果一个 benchmark 同时包含读文件、解析 CSV、计算 rolling factor、写结果，你无法判断瓶颈在哪里。

本项目先测纯计算 kernel：

```text
Vec<f64> -> rolling function -> Vec<f64>
```

之后再测系统级 pipeline。

### 4. 数据规模要贴近目标

量化研究常见规模：

- 单资产短窗口：几千到几十万行。
- 多资产日频：几千资产乘几千天。
- 分钟级或 tick 级：百万到十亿级记录。
- 参数搜索：同一计算重复大量参数组合。

练习时可以从小数据开始，但性能结论必须在目标规模上验证。

## 常见误判

误判 1：一次运行更快就是优化成功。

正确做法：重复运行，记录均值和波动。

误判 2：算法复杂度低就一定更快。

正确做法：小数据可能被常数成本支配。复杂度分析和 benchmark 都要看。

误判 3：并行一定更快。

正确做法：并行有调度、同步、内存带宽成本。

误判 4：unsafe 一定更快。

正确做法：unsafe 只移除部分检查或表达特殊优化，不会自动改善缓存、算法或 IO。

误判 5：Python 边界 copy 一定是瓶颈。

正确做法：先测 kernel，再测 boundary，再决定是否 zero-copy。

## profiling 入口

profiling 是定位时间花在哪里。不同系统可用工具不同，所以本仓库不强制依赖这些工具。

可选工具：

```bash
perf record -- cargo run -p factor-core --release --bin bench
perf report
```

或：

```bash
cargo flamegraph -p factor-core --bin bench
```

如果本机没有这些工具，不影响学习主线。先用 std-only benchmark 建立性能实验习惯。

## 优化后必须检查什么

每次优化后都要回答：

1. API 是否改变？
2. 错误行为是否改变？
3. 输出对齐是否改变？
4. NaN、inf、空输入、非法窗口是否仍然处理？
5. 测试是否覆盖 baseline 和 candidate 一致性？
6. benchmark 是否使用 release？
7. 性能收益是否足以抵消复杂度？

专业标准不是“写得更快”，而是“可证明地更快，并且没有破坏正确性和边界”。
