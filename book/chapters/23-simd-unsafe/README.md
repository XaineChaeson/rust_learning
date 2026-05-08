# 第 23 章：SIMD、unsafe 和安全边界

Rust 的 safe 代码已经能写出很快的数值循环。unsafe 和 SIMD 是更底层的工具，不是入门优化手段。专业开发者需要知道什么时候用、如何封装、如何证明收益。

## unsafe 的含义

`unsafe` 不表示代码“不安全”，而是表示编译器不能证明某些条件成立，需要程序员承担证明责任。

常见 unsafe 场景：

- 裸指针解引用。
- `get_unchecked` 跳过边界检查。
- FFI。
- 手写 SIMD intrinsics。
- 实现底层并发 trait。

每个 unsafe block 都必须有不变量说明。

## 安全 API 包住 unsafe

外部 API 应该是安全的：

```rust
pub fn dot_with_unsafe_boundary(left: &[f64], right: &[f64]) -> Result<f64, DotError>
```

函数先检查长度，再调用内部 unsafe helper。调用者不能绕过检查。

unsafe 范围越小，越容易 review。

## SIMD

SIMD 让一个指令处理多个数据。它适合：

- dot product
- elementwise arithmetic
- reductions
- filter mask

但 SIMD 收益受限于：

- 内存带宽。
- 对齐。
- 数据长度。
- 分支。
- 编译器自动向量化能力。

没有 benchmark，不保留手写 SIMD。

## 本章示例

```bash
cargo test -p ch23-simd-unsafe
```

重点看：

- safe baseline 和 unsafe boundary 结果一致。
- unsafe helper 不直接暴露给调用者。
- 注释写明安全前提。

## 本章练习

1. 给长度不一致写测试。
2. 增加 `dot_safe_loop`，不用 iterator。
3. 给 unsafe helper 写更详细的 safety 注释。
4. 写 benchmark 计划：safe iterator、safe loop、unsafe loop 如何比较？

## 本章验收

你应该能解释：

- unsafe 不变量是什么。
- 为什么 unsafe 应该被 safe API 包住。
- SIMD 可能快，也可能被内存带宽限制。
- 什么情况下应该删除 unsafe 优化。

## 教材化补充：unsafe 是证明责任，不是性能开关

`unsafe` 代码的真正含义是：编译器无法证明安全，但你声称自己能证明。这个证明要写在代码附近。

例如使用 `get_unchecked(index)` 时，你必须证明：

- index 一定小于 slice 长度。
- 两个 slice 的长度关系已经检查。
- 没有非法别名或悬垂引用。

如果你说不清这些前提，就不应该写 unsafe。

## safe wrapper 的重要性

外部调用者应该使用 safe API：

```rust
dot_with_unsafe_boundary(left, right)
```

函数内部先检查长度，再进入 unsafe helper。这样调用者无法跳过安全检查。

## SIMD 的现实判断

SIMD 常见于 dot、sum、elementwise arithmetic。但收益取决于数据是否连续、长度是否足够、是否被内存带宽限制。

很多时候编译器已经能自动向量化普通循环。手写 SIMD 前应该先看 benchmark 或汇编证据。

## 示例走读

```text
book/chapters/23-simd-unsafe/example/src/lib.rs
```

`dot_safe` 是 baseline。`dot_with_unsafe_boundary` 是带检查的 public API。`dot_unchecked_same_len` 是内部 unsafe helper。

## 常见错误

错误 1：把 unsafe 函数公开给所有调用者。

这会把证明责任推给调用者，API 风险变大。

错误 2：没有测试长度不一致。

unsafe 前的 safe 检查必须有测试。

错误 3：没有 benchmark 就保留 unsafe。

unsafe 增加维护成本。没有收益证据就应该删除。

## 代码走读与安全证明

看 public API：

```rust
pub fn dot_with_unsafe_boundary(left: &[f64], right: &[f64]) -> Result<f64, DotError>
```

它先检查长度：

```rust
if left.len() != right.len() {
    return Err(...);
}
```

只有长度相等时，才进入 unsafe helper：

```rust
unsafe { dot_unchecked_same_len(left, right) }
```

unsafe helper 的安全不变量是：

- `left` 和 `right` 长度相等。
- 循环 index 范围是 `0..left.len()`。
- 因此 `right[index]` 也一定存在。

操作清单：

1. 给长度不一致写测试。
2. 给空 slice 写测试。
3. 给一个大 slice 写 baseline 对比。
4. 在注释里写出 safety invariant。

只有当这些都具备，再谈 benchmark 和 SIMD。否则 unsafe 只是风险。

## 自测与复盘问题

1. unsafe block 需要证明哪些不变量？
2. 为什么 public API 应该保持 safe？
3. `get_unchecked` 省掉了什么检查？
4. SIMD 为什么可能受内存带宽限制？
5. 没有 benchmark 的 unsafe 优化应该如何处理？

如果这些问题回答不出来，保留 safe baseline，不要写 unsafe。

## 进入下一章前

确认你能写出 unsafe block 的 safety 注释，并能说明 safe wrapper 如何保护调用者。你还应该能说明没有 benchmark 时为什么要删除 unsafe 优化。做到这些，再进入 Python FFI。

## 额外复盘

把本章内容映射到招聘面试：如果被问到 unsafe，你不能只说“危险”。你应该能给出一个具体 kernel、一个 safe wrapper、一个 safety invariant 和一个 benchmark 计划。
