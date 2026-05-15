# 测试指南

## 快速开始

运行所有测试：

```bash
cargo test
```

## 测试分类

项目包含 130 个单元测试，覆盖以下模块：

| 模块 | 测试数量 | 覆盖内容 |
|------|----------|----------|
| Lexer | 20 | 词法分析：整数、浮点数、字符串、关键字、运算符、注释等 |
| Parser | 35 | 语法分析：变量声明、函数、控制流、结构体、导入、try/catch 等 |
| Optimizer | 33 | 优化器：常量折叠、代数简化、常量传播、死代码消除、内置函数内联 |
| Integration | 42 | 端到端编译：完整 CatLang 源码到 Zig 代码的编译流程 |

## 运行特定测试

### 按模块运行

```bash
# 仅运行 Lexer 测试
cargo test lexer::

# 仅运行 Parser 测试
cargo test parser::

# 仅运行 Optimizer 测试
cargo test optimizer::

# 仅运行集成测试
cargo test tests::
```

### 运行单个测试

```bash
cargo test test_tokenize_basic
cargo test test_parse_function
cargo test test_constant_folding_add
```

### 显示测试输出

```bash
cargo test -- --nocapture
```

### 并行/串行运行

```bash
# 并行运行（默认）
cargo test

# 串行运行（调试时有用）
cargo test -- --test-threads=1
```

## 测试文件位置

测试直接嵌入在源代码文件中：

- `src/lexer.rs` - Lexer 测试（底部 `#[cfg(test)] mod tests`）
- `src/parser.rs` - Parser 测试（底部 `#[cfg(test)] mod tests`）
- `src/optimizer.rs` - Optimizer 测试（底部 `#[cfg(test)] mod tests`）
- `src/lib.rs` - 集成编译测试（底部 `#[cfg(test)] mod tests`）

## 添加新测试

在对应模块的 `#[cfg(test)] mod tests` 块中添加：

```rust
#[test]
fn test_my_new_feature() {
    // 测试代码
    assert!(some_condition);
}
```

## CI 测试

GitHub Actions 会在每次推送和 PR 时自动运行测试。
