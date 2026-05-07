# 第 34 章：PyO3、maturin 和 Python 扩展边界

第 24 章已经讲过 Python/Rust FFI 的基础边界：copy、zero-copy、GIL、生命周期。本章把它提升到真实生态层面：PyO3 和 maturin。

本章示例不直接构建 wheel，而是用 std-only 模型训练绑定层设计。真正进入生产时，你会用 PyO3 写 Python module，用 maturin 构建 wheel；但核心原则仍然是：Python 绑定层必须薄，Rust 计算核心必须纯。

## 本章解决什么问题

你是数据科学背景，Python 研究流很重要。

现实工作中，完全抛弃 Python 不务实：

- 研究员用 Python 做实验组织。
- NumPy/Pandas/PyTorch 生态成熟。
- 可视化和 notebook 在 Python 中方便。
- 量化研究流程往往已经围绕 Python 构建。

Rust 的角色通常是：

- 加速重计算 kernel。
- 提供稳定低延迟服务。
- 构建高吞吐批处理模块。
- 保护内存安全和并发正确性。

所以专业目标不是“所有东西都用 Rust 重写”，而是让 Rust 和 Python 有清晰边界。

## 学习前提

你需要理解：

- 第 03 章：`Vec<T>` 和 `&[T]`。
- 第 04 章：borrow 和 ownership。
- 第 11 章：lifetime。
- 第 24 章：copy boundary 和 zero-copy。
- `projects/01-factor-core` 的纯 Rust API。

如果你还不能解释 `&[f64]` 为什么适合 kernel API，先回到第 03、04、11 章。

## Python 对照

Python 中你可能会写：

```python
result = rust_factor.rolling_mean(numpy_array, window=252)
```

这个调用看起来简单，但边界上有很多问题：

- `numpy_array` 是不是 `float64`？
- 是否 contiguous？
- 是否包含 NaN？
- Rust 是否复制了数据？
- Rust 是否保存了 Python object 的引用？
- 是否释放 GIL？
- 错误如何转成 Python exception？

Python 调用一行，Rust 边界要处理所有这些事实。

## PyO3 是什么

PyO3 是 Rust 和 Python 交互的主流库之一。它可以：

- 写 Python extension module。
- 从 Rust 调 Python。
- 暴露 Rust function/class 给 Python。
- 处理 Python object 和 Rust 类型转换。
- 管理 GIL 相关 API。

典型生产结构：

```text
python package
  -> PyO3 binding crate
  -> pure Rust factor-core crate
```

注意依赖方向：

`factor-core` 不应该依赖 PyO3。

如果核心 crate 依赖 PyO3，你的计算逻辑会被 Python runtime 污染，测试、benchmark、服务化都会变困难。

## maturin 是什么

maturin 用于构建和发布 Python wheel。

它解决：

- Rust extension 编译。
- Python packaging。
- 本地开发安装。
- wheel 构建。

真实项目中，你可能会使用：

```bash
maturin develop
maturin build --release
```

本课程不强制学生现在安装 maturin。原因是：你要先掌握边界设计，再进入 packaging。

## 核心概念一：copy 不是失败

很多初学者一开始就追求 zero-copy。

这是危险的。

copy boundary 的优点：

- ownership 清楚。
- lifetime 简单。
- Python object 释放后 Rust 不悬垂。
- 错误更容易定位。
- API 更容易测试。

缺点是多一次内存复制。

专业做法：

1. 先写 copy boundary。
2. benchmark 证明 copy 是瓶颈。
3. 再考虑 zero-copy。

## 核心概念二：zero-copy 转移复杂度

zero-copy 不代表没有成本。它把成本转移到不变量上：

- dtype 必须匹配。
- layout 必须 contiguous 或 stride 可处理。
- Python owner 必须活得足够久。
- Rust 不能保存越界引用。
- mutable alias 必须遵守 Rust 规则。
- GIL 释放后不能访问 Python object。

这些不变量任何一个错了，都可能从“性能优化”变成内存安全问题。

## 核心概念三：GIL 和 CPU kernel

Python 有 GIL。

Rust CPU kernel 如果长时间运行，应该尽量在不访问 Python object 的时候释放 GIL。

但释放 GIL 的前提是：

- 输入已经转换成 Rust 可用的数据。
- kernel 不再访问 Python object。
- 错误能安全带回边界层。

所以绑定层要薄：

```text
validate Python object
convert or borrow input
release GIL around pure Rust kernel
map Rust error to Python exception
return Python object
```

## 示例代码走读

示例位置：

```text
book/examples/ch34-python-extension-boundary/src/lib.rs
```

运行：

```bash
cargo test -p ch34-python-extension-boundary
```

关键类型：

- `PythonArrayView`
- `DType`
- `BoundaryMode`
- `BoundaryPlan`
- `BoundaryError`

这些类型模拟 PyO3 边界需要判断的信息。

## 代码走读：plan_boundary

`plan_boundary` 判断：

- dtype 是否是 F64。
- 输入是否 contiguous。
- 是否需要 owned output。

如果 dtype 不匹配，选择 copy。

如果 non-contiguous，选择 copy。

如果 contiguous f64 且不需要 owned staging，选择 borrow。

这就是 Python/Rust 边界判断的最小模型。

## 代码走读：execute_boundary

`execute_boundary` 做三件事：

1. 检查 dtype。
2. 检查 contiguous。
3. 调用纯 Rust `mean_kernel(&[f64])`。

注意：`mean_kernel` 不知道 Python 的存在。

这正是专业设计。

## 动手操作

1. 跑测试：

```bash
cargo test -p ch34-python-extension-boundary
```

2. 增加 `shape: Vec<usize>` 字段。

3. 拒绝二维以上输入，或把 shape 转成 matrix view。

4. 增加 `contains_nan` 校验。

5. 写一段伪 PyO3 绑定说明：Python object -> boundary validation -> Rust kernel -> Python result。

## 常见错误

错误 1：让核心 crate 依赖 PyO3。

核心计算应该保持纯 Rust。

错误 2：过早 zero-copy。

没有 benchmark 证明前，copy boundary 更安全。

错误 3：释放 GIL 后访问 Python object。

释放 GIL 的代码只能处理 Rust-owned 或安全 borrowed 数据。

错误 4：错误信息丢失。

Python 用户需要知道哪个参数错了。

错误 5：没有 wheel 构建策略。

生产交付需要明确 Python 版本、平台、CI 和 release。

## 量化/HPC 连接

典型工作流：

```text
Python notebook
  -> numpy array
  -> PyO3 boundary
  -> factor-core rolling kernel
  -> numpy result
  -> Python analysis/report
```

Rust 负责重计算，Python 负责研究组织。

这能让你在不重写整个研究系统的情况下，把最慢、最关键的 kernel 下沉到 Rust。

## 本章验收

你通过本章时，应该能做到：

1. 解释 PyO3 和 maturin 分别解决什么。
2. 解释为什么 `factor-core` 不依赖 PyO3。
3. 设计 copy boundary。
4. 说明 zero-copy 的不变量。
5. 说明何时释放 GIL。
6. 把 Rust error 映射成 Python 用户可理解的错误。

## 进入下一章前

确认你能画出 Python object 到 Rust kernel 的完整路径。下一章进入列式计算生态：Arrow、Polars、DataFusion。
