# 第 24 章：Python/Rust FFI 边界

你的现实工作流在 Python、NumPy、PyTorch、pandas 里。Rust 的价值不是替代所有 Python，而是把高成本、可明确边界的计算 kernel 下沉到 Rust。

## FFI 的核心问题

Python 调 Rust 时，真正的问题通常不是函数调用开销，而是数据边界：

- Python object 如何变成 Rust slice？
- 是否发生拷贝？
- GIL 是否阻塞其他 Python 线程？
- Rust panic 如何转成 Python exception？
- Rust 错误类型如何映射？
- wheel 如何构建和发布？

## copy 和 zero-copy

最简单的边界是拷贝：

```rust
let values = python_values.to_vec();
```

它安全、清楚，但大数据上可能贵。

zero-copy 需要更严格条件：

- 输入内存连续。
- dtype 匹配。
- lifetime 不超过 Python owner。
- Rust 不在 Python 释放后继续引用。
- 可变借用不能违反别名规则。

专业判断：先做清楚的 copy boundary，再在 benchmark 证明必要时优化 zero-copy。

## GIL

GIL 保护 Python 解释器状态。CPU-heavy Rust kernel 如果不释放 GIL，Python 侧并发会受影响。PyO3 支持在长计算时释放 GIL，但你必须保证 Rust 计算不访问 Python object。

## crate 分层

推荐结构：

```text
factor-core      # Rust 纯核心
factor-python    # PyO3 绑定
```

不要让 `factor-core` 依赖 PyO3。否则 CLI、在线服务、batch job 都被 Python 绑定污染。

## 本章示例

```bash
cargo test -p ch24-python-ffi-boundaries
```

本章示例不引入 PyO3 依赖，而是用标准库模拟 boundary copy 和 borrowed kernel 的区别。

## 本章练习

1. 给 `OwnedArray` 增加 `len()`。
2. 写 `mean_borrowed(values: &[f64])`。
3. 写 `mean_owned_boundary(values: &[f64])`，显式复制再计算。
4. 写复盘：什么时候 copy 是可接受的，什么时候必须追求 zero-copy？

## 本章验收

你应该能解释：

- Rust 加速 Python 的合理边界。
- copy 和 zero-copy 的工程取舍。
- GIL 对 CPU kernel 的影响。
- 为什么核心 crate 不应该依赖 Python 绑定 crate。

## 教材化补充：Python 绑定是边界层，不是核心层

你真正想复用的是 Rust kernel，而不是 Python 绑定代码。绑定层应该薄，只负责：

- 把 Python 输入转换成 Rust 输入。
- 调用 `factor-core`。
- 把 Rust 输出转换回 Python。
- 把 Rust 错误变成 Python exception。

如果核心计算依赖 PyO3，那么它就很难被 CLI、回测、在线系统或 benchmark 独立使用。

## copy boundary 的合理性

初学者听到 zero-copy 会觉得 copy 很低级。实际工程里，清楚的 copy boundary 常常是第一版最合理选择：

- 安全简单。
- lifetime 好处理。
- 错误边界清楚。
- 容易测试。

只有当 benchmark 证明 copy 是瓶颈时，再考虑 zero-copy。

## zero-copy 的约束

zero-copy 不是“不花成本”，而是把复杂度转移到边界：

- 输入必须连续。
- dtype 必须匹配。
- Python owner 必须活得足够久。
- Rust 不能保存超出边界的引用。
- 可变访问必须遵守别名规则。

## 示例走读

```text
book/examples/ch24-python-ffi-boundaries/src/lib.rs
```

示例用 `OwnedArray::from_python_like_input` 显式表示边界 copy。`borrowed_kernel` 表示核心 kernel 只需要 `&[f64]`，它不关心输入来自 Python、文件还是测试。

## 常见错误

错误 1：一开始就追求 zero-copy。

先做正确边界，再测 copy 是否真是瓶颈。

错误 2：在释放 GIL 后访问 Python object。

释放 GIL 的 Rust 代码不能依赖 Python 对象状态。

错误 3：让核心 crate 依赖 PyO3。

这会污染依赖方向。

## 代码走读与边界设计

看核心 kernel：

```rust
pub fn borrowed_kernel(values: &[f64]) -> f64
```

它只需要 slice。这个函数完全不知道输入来自 Python、CSV 还是测试。

看 boundary：

```rust
pub fn owned_boundary_then_kernel(values: &[f64]) -> f64
```

它先复制成 `OwnedArray`，再调用 borrowed kernel。这个例子把 copy 明确放在边界层。

真实 PyO3 项目中，结构通常是：

```text
Python object -> Rust boundary conversion -> &[f64] or Vec<f64> -> factor-core
```

操作清单：

1. 给 `OwnedArray` 增加 `len()`。
2. 写 `mean_borrowed(values: &[f64])`。
3. 写 `mean_owned_boundary(values: &[f64])`。
4. 记录两者语义差异。

专业判断：第一版绑定优先清楚和正确。zero-copy 是优化目标，不是起点。

## 自测与复盘问题

1. 为什么 `factor-core` 不应该依赖 PyO3？
2. copy boundary 的优点是什么？
3. zero-copy 需要满足哪些 lifetime 和内存连续性条件？
4. 释放 GIL 后还能访问 Python object 吗？
5. Python 绑定层应该包含业务逻辑吗？

如果这些问题回答不出来，先设计清楚 copy 版本，再考虑 zero-copy。

## 进入下一章前

确认你能画出 Python object 到 Rust slice 或 Vec 的转换路径，并能标出哪里发生 copy，哪里受 GIL 影响。做到这些，再进入回测系统。

## 额外复盘

把本章内容映射到研究工作流：Python 负责实验组织和可视化，Rust 负责重计算 kernel。你应该能解释为什么这个边界比“全部用 Rust 重写研究系统”更务实。

## 专业判断清单

设计 Python/Rust 边界时先问：

1. Rust 核心是否能脱离 Python 单独测试？
2. 边界 copy 的成本是否已经被 benchmark 证明是瓶颈？
3. dtype、shape、contiguous layout 是否被显式检查？
4. Rust 是否保存了任何来自 Python 的短生命周期引用？
5. 错误是否能转成 Python 用户能理解的异常？

一个好的绑定层应该很薄：它负责转换、校验和错误映射；真正的计算逻辑留在纯 Rust crate 里。
