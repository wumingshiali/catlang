# CatLang 性能基准测试对比

## 测试环境
- **CPU**: 13th Gen Intel Core i5-13500H (16 核心)
- **内存**: 32 GB
- **系统**: Windows 11 专业版 Insider Preview
- **编译器版本**:
  - CatLang: 0.1.0 (Zig 后端)
  - C++: MSVC 19.44.35222 (/O2 优化)
  - Rust: 最新稳定版 (-O 优化)
  - Python: 最新稳定版
  - Zig: 0.15.2

## 测试项目

所有测试使用各语言内置的计时器进行精确测量：

### 1. 循环测试 (1 亿次迭代)
```catlang
t.start()
new i<ia> = 0
while (i < 100000000) [ i = i + 1 ]
t.stop()
```

### 2. 累加测试 (1000 万次迭代)
```catlang
t.start()
new result<ia> = 0
new j<ia> = 0
while (j < 10000000) [ j = j + 1; result = result + j ]
t.stop()
```

### 3. 斐波那契 (递归，n=30)
```catlang
t.start()
new fib_result<ia> = fib(30)
t.stop()
```

## 测试结果

### CatLang 不同优化级别对比

| 测试项目 | -O 0 (Debug) | -O 2 (ReleaseSafe) | -O 3 (ReleaseFast) |
|----------|--------------|--------------------|--------------------|
| 循环测试 (1 亿次) | 93.09 ms | 0 ms | 0 ms |
| 累加测试 (1000 万次) | 15.62 ms | 4.37 ms | 0 ms |
| 斐波那契 (n=30) | 5.30 ms | 3.88 ms | 3.04 ms |
| **总时间** | **~114 ms** | **~8 ms** | **~3 ms** |
| **性能提升** | 基准 | **~14x** | **~38x** |

### 跨语言对比 (使用最高优化级别)

| 测试项目 | CatLang (-O3) | C++ (MSVC) | Rust | Python |
|----------|---------------|------------|------|--------|
| 循环测试 (1 亿次) | 0 ms | ~0 ms | ~0 ms | 2145.91 ms |
| 累加测试 (1000 万次) | 0 ms | ~0 ms | ~0 ms | 408.05 ms |
| 斐波那契 (n=30) | 3.04 ms | ~0 ms | 2.62 ms | 79.94 ms |
| **总时间** | **~3 ms** | **~0 ms** | **~3 ms** | **~2634 ms** |

## 性能对比

| 排名 | 语言 | 相对性能 |
|------|------|----------|
| 1 | **C++ (MSVC)** | 基准 (最快) 🥇 |
| 2 | **CatLang (-O3)** | ~1x (与 Rust 相当) 🥈 |
| 3 | **Rust** | ~1x 🥈 |
| 4 | **CatLang (-O2)** | ~3x |
| 5 | **CatLang (-O0)** | ~38x |
| 6 | **Python** | ~878x |

## 优化器功能

CatLang 优化器 (`src/optimizer.rs`) 提供以下优化：

### 优化级别
- **-O 0**: 无优化，快速编译，适合调试
- **-O 1**: 基础优化，常量传播
- **-O 2**: 默认优化，常量折叠 + 代数简化 (默认)
- **-O 3**: 最高优化，内建函数内联 + 全面优化

### 优化技术
1. **常量折叠 (Constant Folding)**
   - 编译时计算常量表达式
   - 例：`2 + 3 * 4` → `14`

2. **代数简化 (Algebraic Simplification)**
   - `x + 0` → `x`
   - `x * 1` → `x`
   - `x * 0` → `0`
   - `x - 0` → `x`
   - `x / 1` → `x`

3. **常量传播 (Constant Propagation)**
   - 变量替换为已知常量值
   - 例：`let x = 5; let y = x + 1` → `let y = 6`

4. **内建函数内联 (Built-in Function Inlining)**
   - `abs()`, `min()`, `max()` 在编译时求值

5. **Zig 后端优化**
   - Debug: 无优化，完整调试信息
   - ReleaseSafe: 安全优化，保留溢出检查
   - ReleaseFast: 激进优化，最高性能

## 分析

### CatLang 优势
- ✅ 编译为原生代码 (通过 Zig)
- ✅ 内置高精度 Timer 支持 (纳秒级，inline 方法)
- ✅ 语法简洁，类似 Python
- ✅ 内存安全 (默认)
- ✅ 比 Python 快约 **878 倍** (-O3)
- ✅ 多级优化器支持 (-O0 到 -O3)
- ✅ ReleaseFast 优化后性能与 Rust 相当

### CatLang 劣势
- ❌ 无优化时比 C++/Rust 慢约 38 倍
- ❌ I/O 操作性能有待优化
- ❌ 优化器仍在开发中

### 结论
CatLang 作为一门新语言，在启用优化后性能可达到系统级语言 (Rust/C++) 的水平。对于不需要极致性能的应用场景，CatLang 提供了良好的性能和开发体验平衡。

**使用建议：**
- 开发调试：`catc source.cat -O 0`
- 日常使用：`catc source.cat` (默认 -O 2)
- 发布版本：`catc source.cat --release` (或 `-O 3`)
