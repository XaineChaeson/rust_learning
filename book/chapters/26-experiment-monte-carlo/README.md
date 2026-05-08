# 第 26 章：实验引擎和 Monte Carlo

量化研究不是跑一次策略，而是系统性地跑大量可复现实验。实验引擎的价值是让参数、数据、代码版本、随机种子和结果可追踪。

## 参数搜索

常见搜索：

- grid search
- random search
- Bayesian optimization
- walk-forward

学习阶段先做 grid search，因为它最容易测试。

参数组合应该是明确类型，而不是到处传 `HashMap<String, f64>`。类型能防止把 window 和 threshold 搞反。

## reproducibility

每个实验必须能复现：

- 固定 seed。
- 记录参数。
- 记录数据范围。
- 记录代码版本。
- 记录 binary/config。

Monte Carlo 中，seed 是实验输入的一部分，不是随手写的常量。

## checkpoint 和 retry

大规模实验会失败。失败不可怕，不可恢复才可怕。

系统应该能：

- 跳过已完成任务。
- 重跑失败任务。
- 不重复写入结果。
- 标记 partial result。

这和第 29 章分布式调度直接相关。

## 本章示例

```bash
cargo test -p ch26-experiment-monte-carlo
```

重点看：

- `parameter_grid` 如何枚举参数。
- `deterministic_walk` 如何固定 seed。

## 本章练习

1. 给 `ExperimentParam` 增加 `lookback`。
2. 写 `experiment_id(param, seed)`。
3. 增加多个 seed 的 path simulation。
4. 写复盘：并行 Monte Carlo 为什么仍然需要可复现？

## 本章验收

你应该能解释：

- 参数搜索为什么需要类型建模。
- seed 为什么是实验输入。
- checkpoint/retry 解决什么问题。
- Monte Carlo 并行为什么可能改变浮点归约顺序。

## 教材化补充：实验系统的敌人是不可复现

研究中最糟糕的情况不是结果差，而是你不知道为什么昨天和今天结果不一样。实验引擎必须记录足够信息，让结果可复现。

必须记录：

- 参数。
- seed。
- 数据版本和时间范围。
- 代码版本。
- 配置。
- 运行环境。

## 参数类型化

不要用散乱的 `HashMap<String, f64>` 表示参数。类型化参数更安全：

```rust
struct ExperimentParam {
    window: usize,
    threshold: f64,
}
```

这样编译器能帮助你避免把窗口和阈值搞混。

## Monte Carlo 和并行

Monte Carlo path 通常彼此独立，适合并行。但并行带来两个问题：

- seed 如何分配，保证可复现。
- 统计归约顺序不同，浮点结果可能略有差异。

测试应该允许合理容差，并明确 seed 策略。

## 示例走读

```text
book/chapters/26-experiment-monte-carlo/example/src/lib.rs
```

`parameter_grid` 展示 grid search。`deterministic_walk` 展示固定 seed 后，同样输入得到同样路径。

## 常见错误

错误 1：随机种子写在函数内部。

这样调用者无法复现实验。

错误 2：只保存最优结果，不保存失败或中间结果。

这会让研究过程不可审计。

错误 3：失败后从头跑。

大规模实验必须支持 checkpoint 和 retry。

## 代码走读与实验记录

看参数类型：

```rust
pub struct ExperimentParam {
    pub window: usize,
    pub threshold: f64,
}
```

window 和 threshold 是不同类型语义，即使底层都可以是数字，也不应该随意用 tuple 或 map 混过去。

看 `parameter_grid`：

```rust
for window in windows {
    for threshold in thresholds {
        ...
    }
}
```

这是笛卡尔积。每个参数组合都应该能生成稳定 experiment id。

看 deterministic walk：

```rust
pub fn deterministic_walk(seed: u64, steps: usize) -> Vec<f64>
```

seed 是输入。相同 seed 和 steps 必须生成相同路径。

操作清单：

1. 给 `ExperimentParam` 增加 `seed`。
2. 写 `experiment_id()`。
3. 把结果按 id 存入 map。
4. 模拟失败后只重跑缺失 id。

专业目标：实验系统要能审计，不只是能跑循环。

## 自测与复盘问题

1. 为什么 seed 是实验输入的一部分？
2. 参数为什么应该类型化？
3. checkpoint 解决什么问题？
4. retry 为什么需要稳定 experiment id？
5. 并行 Monte Carlo 为什么可能改变浮点归约结果？

如果这些问题回答不出来，不要把参数搜索扩成分布式任务。

## 进入下一章前

确认你能为每个实验生成稳定 ID，并知道 seed、参数、数据版本都属于实验输入。做到这些，再进入在线特征和列式计算。
