# 第 7 章：测试数值代码

高性能代码必须先正确。没有测试的性能优化没有意义。

量化数值代码尤其需要测试，因为错误不一定会导致程序崩溃，更多时候是悄悄产生错误结果。

## 测试放在哪里

Rust 有两类常见测试。

单元测试通常和代码放在同一个文件：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_computes_average() {
        assert_eq!(mean(&[1.0, 2.0, 3.0]), Some(2.0));
    }
}
```

集成测试放在 `tests/` 目录，像外部用户一样调用你的 crate。

本仓库当前项目 `projects/00-bootstrap-cli` 就有集成测试。

## 浮点数不能总用 `assert_eq`

浮点计算可能有微小误差。不要总写：

```rust
assert_eq!(left, right);
```

更稳妥：

```rust
fn assert_close(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-12);
}
```

对于 rolling、corr、beta 这种计算，误差容忍更重要。

## 测试边界

一个数值函数至少测试：

- 空输入。
- 单元素输入。
- 正常输入。
- 非法参数。
- 含 NaN 或 inf。
- 长度不匹配。
- 常数序列。
- 手算小样例。

比如 `rolling_mean`：

```rust
#[test]
fn rolling_mean_computes_right_aligned_windows() {
    let output = rolling_mean(&[1.0, 2.0, 3.0, 4.0], 2).expect("window is valid");

    assert_eq!(output, vec![1.5, 2.5, 3.5]);
}
```

这个测试不仅验证结果，还隐含说明了对齐规则：右对齐，输出长度是 `n - window + 1`。

## 测试也是设计文档

当你写：

```rust
assert_eq!(
    rolling_mean(&[1.0, 2.0], 0),
    Err(NumericError::InvalidWindow)
);
```

你就在声明：窗口为 0 是错误，而不是返回空数组。这是设计选择。

测试能防止未来你自己忘记这个选择。

## Python 参考结果

后续因子引擎需要和 Python 对齐。建议用三类测试：

1. 手算小样例。
2. Python/numpy/pandas 参考输出。
3. 大数据 smoke test。

手算小样例负责解释规则。Python 参考负责防止实现偏离研究习惯。大数据 smoke test 负责发现性能和内存问题。

## 测试先于优化

比如你准备把 rolling mean 从朴素 `O(n * window)` 优化为增量 `O(n)`。在优化前必须有测试。否则你不知道优化是否改变了结果。

优化流程：

1. 写 baseline。
2. 写测试。
3. 跑测试。
4. 写优化版。
5. 跑测试。
6. 跑 benchmark。

## 本章练习

在 `book/examples/ch07-testing-numeric-code/src/lib.rs` 中：

1. 给 `returns` 增加价格少于 2 个的测试。
2. 给 `rolling_mean` 增加窗口大于输入长度的测试。
3. 写 `assert_close_vec`。
4. 故意改错 `rolling_mean`，确认测试失败。
5. 修复实现。

## 本章验收

你可以进入下一章，如果你能回答：

- 为什么浮点数比较需要容差？
- 测试如何表达对齐规则？
- 为什么优化前必须有测试？
- 单元测试和集成测试有什么区别？

## 教材化补充：测试是数值代码的合同

数据科学里，你可能习惯“画图看一下大概对不对”。这在研究探索阶段可以，但在基础设施里不够。Rust 高性能开发要求每个核心计算都有可重复测试。

测试不只是防 bug。它也是文档，告诉后来的开发者：

- 空输入怎么处理。
- 窗口过大怎么处理。
- rolling 输出对齐在哪里。
- NaN 是否允许。
- 浮点误差容忍多少。

## 浮点数为什么不能总用 assert_eq

`0.1 + 0.2` 在二进制浮点里通常不是精确的 `0.3`。所以数值测试要用容差：

```rust
assert!((left - right).abs() < 1e-12);
```

容差不是随便写。它应该和算法、数据规模、业务精度有关。

## 测试边界比测试大数据更重要

大数据 smoke test 能证明程序没崩，但很难证明逻辑对。小的手算样例更重要：

```text
prices = [100, 110]
return = 0.1
```

你应该先用小数据锁定规则，再用大数据测性能。

## 动手操作

运行：

```bash
cargo test -p ch07-testing-numeric-code
```

打开：

```text
book/examples/ch07-testing-numeric-code/src/lib.rs
```

重点看：

- `assert_close` 为什么打印 left/right/tolerance。
- `assert_close_vec` 为什么先检查长度。

## 常见错误

错误 1：只测试正常路径。

错误路径同样重要，尤其是空输入、非法窗口、NaN、长度不一致。

错误 2：优化后不跑测试。

性能优化最容易引入边界错误。每次优化都必须先跑正确性测试。

错误 3：测试依赖 HashMap 顺序或线程返回顺序。

如果顺序不稳定，测试应该排序或使用集合语义。
