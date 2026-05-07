# 第 31 章：最终架构，量化计算框架

最终目标不是一个单独函数，而是一套可演进的高性能量化计算框架。它既服务 Python 研究者，也能作为 Rust 系统组件独立运行。

## 模块边界

推荐最终模块：

```text
data-core          # schema、record、validation
factor-core        # rolling factor、cross-sectional factor
numeric-kernel     # matrix、reduction、SIMD boundary
factor-python      # PyO3 binding
backtest-core      # portfolio/order/trade state machine
experiment-runner  # parameter search、checkpoint、result store
online-engine      # ingest、ring buffer、incremental factor
batch-engine       # columnar pipeline
scheduler          # task graph、worker、retry
ops                # config、metrics、run report
```

不是每个模块都要第一天拆 crate。拆分依据是依赖方向和变化速度。

## 依赖方向

核心原则：

```text
python binding -> factor-core -> numeric-kernel
backtest-core  -> factor-core
online-engine  -> factor-core
batch-engine   -> factor-core
scheduler      -> experiment-runner
```

反方向依赖通常是坏味道。`factor-core` 不应该知道 Python、CLI、Tokio 或分布式 scheduler。

## API 稳定性

公开 API 要少而稳定：

- 输入类型清楚。
- 错误类型清楚。
- ownership 语义清楚。
- 是否分配清楚。
- 是否保持顺序清楚。

热路径 API 可以更底层，但必须被安全高层 API 包住。

## 性能分层

不是所有模块都追求极致性能：

- numeric kernel：极致性能、少依赖、严测试。
- factor-core：性能和可读性平衡。
- Python binding：边界成本透明。
- scheduler：正确性和容错优先。
- ops/report：可诊断优先。

专业系统知道哪里该快，哪里该稳。

## 本章示例

```bash
cargo test -p ch31-final-architecture
```

重点看：

- `FactorEngine` trait 如何定义模块合同。
- `ReturnEngine` 如何作为具体实现。
- `run_research_pipeline` 如何依赖抽象而不是具体模块细节。

## 最终验收项目

完成一个端到端最小框架：

1. 读取一组价格数据。
2. 计算 rolling factor。
3. 暴露给 Python 或模拟 FFI boundary。
4. 跑一个回测。
5. 跑参数搜索。
6. 输出 benchmark 和运行报告。
7. 记录 config 和 metrics。

## 本章验收

你应该能解释：

- 一个高性能量化框架应该有哪些模块。
- 为什么核心计算不能依赖外层系统。
- 哪些 API 是稳定合同。
- 哪些模块追求性能，哪些模块追求可靠性。

## 教材化补充：最终架构不是目录图，而是依赖纪律

一个系统有很多模块不难，难的是依赖方向正确。核心原则是：越底层、越高性能的模块，依赖越少。

错误方向：

```text
factor-core -> python binding
numeric-kernel -> logging framework
rolling function -> CLI args
```

正确方向：

```text
python binding -> factor-core -> numeric-kernel
CLI -> factor-core
backtest-core -> factor-core
online-engine -> factor-core
```

核心计算应该像发动机，可以装到不同系统里，而不是和某个外壳焊死。

## 分层性能目标

不是每一层都追求极致速度：

- `numeric-kernel`：极致性能，少依赖，强 benchmark。
- `factor-core`：性能和 API 清晰度平衡。
- `python binding`：边界成本透明。
- `backtest-core`：状态正确性优先。
- `scheduler`：容错和幂等优先。
- `ops`：可诊断和可运行优先。

专业系统知道哪些地方该快，哪些地方该稳。

## 示例走读

```text
book/examples/ch31-final-architecture/src/lib.rs
```

`FactorEngine` 是模块合同。`ReturnEngine` 是一个具体实现。`run_research_pipeline` 依赖 trait，而不是依赖具体实现细节。

这只是最终架构的微缩版本。真正项目会把这些概念扩展到多个 crate。

## 常见错误

错误 1：先拆很多 crate，却没有稳定边界。

crate 不是越多越专业。边界稳定、依赖方向正确才重要。

错误 2：所有模块都追求极致性能。

这会让系统难以维护。只有热路径需要极致优化。

错误 3：Python 研究便利性和 Rust 核心性能互相污染。

正确做法是用清晰边界连接，而不是让两边代码混在一起。

## 最终学习后的自测

你应该能用白板说明：

1. 数据从文件或 Python 进入系统。
2. 如何被验证和转换。
3. 因子 kernel 如何计算。
4. 结果如何进入回测或实验。
5. 在线特征如何增量更新。
6. 分布式任务如何重试。
7. 系统如何暴露 metrics 和错误。

如果这些路径能讲清楚，说明你不只是学了 Rust 语法，而是在建立基础设施设计能力。

## 代码走读与最终项目拆解

看输入：

```rust
pub struct MarketBatch {
    pub symbol: String,
    pub prices: Vec<f64>,
}
```

这是批处理输入。它拥有 prices，因为 batch 是一个独立数据单元。真实系统中也可能用 borrowed view 或 Arrow array。

看输出：

```rust
pub struct FactorBatch {
    pub symbol: String,
    pub values: Vec<f64>,
}
```

输出拥有计算结果，因为收益率不是输入中的视图，而是新序列。

看合同：

```rust
pub trait FactorEngine {
    fn compute(&self, input: &MarketBatch) -> FactorBatch;
}
```

这把“如何计算”抽象出来。未来可以有 `ReturnEngine`、`RollingMeanEngine`、`ZScoreEngine`。

最终项目可以按这个顺序落地：

1. `data-core`：定义输入 record 和 validation。
2. `factor-core`：实现 rolling factor。
3. `numeric-kernel`：抽出高性能基础函数。
4. `backtest-core`：消费 factor 输出。
5. `experiment-runner`：批量运行参数。
6. `python-binding`：接入 Python。
7. `ops`：配置、metrics、报告。

每一步都必须保持测试和文档同步。
