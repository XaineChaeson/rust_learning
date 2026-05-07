# factor-core

这是第一个正式项目：rolling 因子计算核心。

运行：

```bash
cargo test -p factor-core
cargo run -p factor-core --release --bin bench
```

当前实现是教学版，目标是：

- API 使用 `&[f64]` 借用输入。
- 错误使用 `ComputeError` 类型表达。
- 输出右对齐，长度为 `n - window + 1`。
- 先实现正确 baseline，再进入 benchmark 和优化。
- 优化版本必须和 baseline 做正确性对照。

已实现：

- `rolling_mean`
- `rolling_mean_incremental`
- `rolling_std`
- `rolling_zscore`
- `rolling_min`
- `rolling_max`
- `rolling_corr`
- `rolling_beta`

## 性能实验

当前 benchmark 比较：

- `rolling_mean`：朴素 baseline。
- `rolling_mean_incremental`：增量优化版本。

默认命令：

```bash
cargo run -p factor-core --release --bin bench
```

可调参数：

```bash
cargo run -p factor-core --release --bin bench -- --len 1000000 --window 252 --repeat 10
```

benchmark 只用于训练性能实验方法。得出结论前，先确认 `cargo test -p factor-core` 通过。
