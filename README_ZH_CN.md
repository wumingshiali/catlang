此项目只允许 Codeberg 的官方镜像，不允许其他未经授权镜像。  
# CatLang 编程语言

<div align="center">

**一门简洁、安全、高性能的系统级编程语言**

[性能基准](BENCHMARK_RESULTS.md) | [语法文档(弃用)](syntax.txt) | [教程](guides_zh_cn/README.md) | [安全](SECURITY_ZH_CN.md) | [版本控制](VERSION.md) | [测试](TEST.md) 

</div>

---

## 📖 简介

**CatLang** 是一门现代系统级编程语言，旨在提供 **零样板代码** 的开发体验，同时保持与 C++/Rust 相媲美的 **高性能** 和 **内存安全** 特性。

CatLang 通过创新的编译器自动注入机制和 Python 风格的简洁语法，让开发者能够专注于业务逻辑，而非繁琐的类型声明和导入语句。

---

## ✨ 核心特性

### 🚀 零样板代码 (Zero Boilerplate)
所有标准库功能（IO、并发原语、错误类型）由编译器**隐式注入**全局作用域，无需手动导入。

```catlang
; 直接使用 print，无需 import
print("Hello, World!")
```

### ⚡ 高性能 (High Performance)
编译为原生机器码，通过 Zig 后端和多层优化器实现极致性能。

| 优化级别 | 性能表现 |
|----------|----------|
| `-O 0` (Debug) | 基准 |
| `-O 2` (ReleaseSafe) | **14x 提升** |
| `-O 3` (ReleaseFast) | **38x 提升** |

在基准测试中，CatLang (-O3) 性能与 **Rust** 相当，比 Python 快约 **878 倍**。

### 🛡️ 内存安全 (Memory Safety)
默认保证内存安全，通过 `unsafe` 块提供底层控制能力。

```catlang
; 安全代码 - 编译器自动检查
new arr = [1, 2, 3]
new val = arr[0]  ; ✓ 安全

; 不安全代码 - 显式声明
unsafe close(bounds) [
    new val = arr[10]  ; 绕过边界检查
]
```

### 🐍 Python 风格语法
简洁直观的语法设计，降低学习曲线。

```catlang
; 变量声明
new x = 42
new pi = 3.14159

; 控制流
if (x > 10) [
    print("x is large")
] else [
    print("x is small")
]

; 函数定义
fn fib(n: i32) -> i32 [
    if (n < 2) [ return n ]
    return fib(n - 1) + fib(n - 2)
]
```

---

## 🔧 快速开始

### 安装

```bash
# 克隆仓库
git clone https://github.com/your-org/catlang.git
cd catlang

# 编译编译器
cargo build --release

# 验证安装
./target/release/catc --version
```

### 第一个程序

创建 `hello.cat`：

```catlang
[
    print("Hello, CatLang!")
    print("2 + 3 = {2 + 3}")
]
```

编译并运行：

```bash
catc hello.cat    # 生成 hello.exe
./hello.exe       # 运行
```

### 编译选项

```bash
# 调试模式 (无优化)
catc program.cat -O 0

# 默认优化 (推荐开发使用)
catc program.cat -O 2

# 发布模式 (最高性能)
catc program.cat --release
```

---

## 📚 语言特性

### 1. 变量与类型

```catlang
; 类型推断
new x = 42           ; i32
new y = 3.14         ; f64
new s = "hello"      ; str

; 显式类型
new a: i32 = 100
new b: f64 = 2.718

; 特殊类型
new big<ia>          ; 任意精度整数 (i128)
new bigf<fa>         ; 任意精度浮点 (f128)
new timer<timer>     ; 高精度计时器
```

### 2. 控制流

```catlang
; If-Else
if (score >= 90) [
    print("Excellent!")
] else if (score >= 60) [
    print("Passed")
] else [
    print("Failed")
]

; While 循环
new i = 0
while (i < 10) [
    print("i = {i}")
    i = i + 1
]

; For 循环
for (new i = 0, i < 10, i += 1) [
    print("Iteration {i}")
]

; Switch 模式匹配
switch (status) [
    case 200: print("OK")
    case 404: print("Not Found")
    case 500: print("Server Error")
    default: print("Unknown")
]
```

### 3. 函数

```catlang
; 普通函数
fn add(a: i32, b: i32) -> i32 [
    return a + b
]

; 异步函数
async fn fetch_data(url: str) -> Result [
    await sleep(100)
    return Result
]

; 使用
new result = add(1, 2)
await fetch_data("https://api.example.com")
```

### 4. 错误处理

```catlang
; 定义错误类型
struct MyError [
    code: i32
    message: str
]

; 抛出错误
throw MyError { code: 1, message: "Something wrong" }

; 捕获错误
try [
    new data = await risky_operation()
    print("Got: {data}")
] catch (err MyError) [
    print("Error {err.code}: {err.message}")
] catch (e Any) [
    print("Unknown error: {e}")
]
```

### 5. 并发编程

```catlang
; 异步任务
async fn worker(id: i32) [
    print("Worker {id} started")
    await sleep(100)
    print("Worker {id} done")
]

; 主程序
[
    ; 生成后台任务
    new handle = spawn async [
        while (true) [
            await sleep(50)
        ]
    ]

    ; 等待异步函数
    await worker(1)

    ; 等待后台任务
    await handle
]
```

### 6. 高精度计时器

```catlang
new t: timer = timer.init()

t.start()
; ... 执行代码 ...
t.stop()

print("Elapsed: {t.ms()} ms")
print("Elapsed: {t.us()} μs")
print("Elapsed: {t.seconds()} s")
```

### 7. Unsafe 编程

```catlang
; 完全关闭安全检查
unsafe all [
    new raw = m+*i32 0x1000
]

; 关闭特定检查
unsafe close(init) [
    new uninitialized: i32
]

unsafe close(bounds) [
    new arr = [1, 2, 3]
    new val = arr[10]  ; 不检查边界
]
```

---

## 📊 性能对比

### 基准测试结果

| 测试项目 | CatLang (-O3) | Rust | C++ | Python |
|----------|---------------|------|-----|--------|
| 循环 (1 亿次) | 0 ms | ~0 ms | ~0 ms | 2146 ms |
| 累加 (1000 万次) | 0 ms | ~0 ms | ~0 ms | 408 ms |
| 斐波那契 (n=30) | 3.04 ms | 2.62 ms | ~0 ms | 79.94 ms |

详细数据请查看 [BENCHMARK_RESULTS.md](BENCHMARK_RESULTS.md)

---

## 🎯 设计哲学

| 原则 | 说明 |
|------|------|
| **简洁优先** | 减少样板代码，让代码更易读 |
| **安全默认** | 默认保证内存安全，危险操作需显式声明 |
| **性能可控** | 多级优化，按需选择性能/安全平衡 |
| **学习友好** | Python 风格语法，低学习曲线 |

---

## 📦 项目结构

```
catlang/
├── src/              # 编译器源代码
│   ├── lexer.rs      # 词法分析器
│   ├── parser.rs     # 语法分析器
│   ├── ast.rs        # 抽象语法树
│   ├── optimizer.rs  # 优化器
│   ├── codegen.rs    # 代码生成器
│   └── main.rs       # CLI 入口
├── test/             # 测试用例
├── benchmark/        # 性能基准测试
└── guides/           # 学习指南
```

---

## 🤝 贡献

欢迎贡献代码、报告问题或提出建议！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

---

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

---

## 🔗 相关链接

- [语法文档](syntax.txt)
- [性能基准](BENCHMARK_RESULTS.md)
- [测试用例](test/)
- [Zig 编程语言](https://ziglang.org/)

---

<div align="center">

**CatLang** - 简洁如 Python，快速如 Rust 🚀

</div>
