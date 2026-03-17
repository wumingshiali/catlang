# CatLang Programming Language

<div align="center">

**A concise, safe, and high-performance systems programming language**

[Benchmark Results](BENCHMARK_RESULTS.md) | [Syntax Guide(Deprecated)](syntax.txt) | [Examples](test) | [Guide(Only Chinese)](guides/README.md) | [简体中文](README_ZH.md)

</div>

---

## 📖 Introduction

**CatLang** is a modern systems programming language designed to provide **zero boilerplate** development experience while maintaining **high performance** comparable to C++/Rust and **memory safety**.

Through an innovative compiler auto-injection mechanism and Python-style concise syntax, CatLang enables developers to focus on business logic rather than tedious type declarations and import statements.

---

## ✨ Core Features

### 🚀 Zero Boilerplate
All standard library functions (IO, concurrency primitives, error types) are **implicitly injected** into the global scope by the compiler—no manual imports required.

```catlang
; Use print directly, no import needed
print("Hello, World!")
```

### ⚡ High Performance
Compiles to native machine code through the Zig backend with multi-level optimizer for extreme performance.

| Optimization Level | Performance |
|-------------------|-------------|
| `-O 0` (Debug)    | Baseline    |
| `-O 2` (ReleaseSafe) | **14x faster** |
| `-O 3` (ReleaseFast) | **38x faster** |

In benchmarks, CatLang (-O3) performance is comparable to **Rust**, approximately **878x faster** than Python.

### 🛡️ Memory Safety
Memory safety by default, with `unsafe` blocks for low-level control when needed.

```catlang
; Safe code - compiler checks automatically
new arr = [1, 2, 3]
new val = arr[0]  ; ✓ Safe

; Unsafe code - explicit declaration
unsafe close(bounds) [
    new val = arr[10]  ; Bypass bounds check
]
```

### 🐍 Python-Style Syntax
Clean and intuitive syntax design with a low learning curve.

```catlang
; Variable declarations
new x = 42
new pi = 3.14159

; Control flow
if (x > 10) [
    print("x is large")
] else [
    print("x is small")
]

; Function definition
fn fib(n: i32) -> i32 [
    if (n < 2) [ return n ]
    return fib(n - 1) + fib(n - 2)
]
```

---

## 🔧 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/wumingshiali/catlang.git
cd catlang

# Build the compiler
cargo build --release

# Verify installation
./target/release/catc --version
```

### First Program

Create `hello.cat`:

```catlang
[
    print("Hello, CatLang!")
    print("2 + 3 = {2 + 3}")
]
```

Compile and run:

```bash
catc hello.cat    # Generates hello.exe
./hello.exe       # Run
```

### Compilation Options

```bash
# Debug mode (no optimization)
catc program.cat -O 0

# Default optimization (recommended for development)
catc program.cat -O 2

# Release mode (maximum performance)
catc program.cat --release
```

---

## 📚 Language Features

### 1. Variables & Types

```catlang
; Type inference
new x = 42           ; i32
new y = 3.14         ; f64
new s = "hello"      ; str

; Explicit types
new a: i32 = 100
new b: f64 = 2.718

; Special types
new big: ia          ; Arbitrary precision integer (i128)
new bigf: fa         ; Arbitrary precision float (f128)
new timer: timer     ; High-precision timer
```

### 2. Control Flow

```catlang
; If-Else
if (score >= 90) [
    print("Excellent!")
] else if (score >= 60) [
    print("Passed")
] else [
    print("Failed")
]

; While loop
new i = 0
while (i < 10) [
    print("i = {i}")
    i = i + 1
]

; For loop
for (new i = 0, i < 10, i += 1) [
    print("Iteration {i}")
]

; Switch pattern matching
switch (status) [
    case 200: print("OK")
    case 404: print("Not Found")
    case 500: print("Server Error")
    default: print("Unknown")
]
```

### 3. Functions

```catlang
; Regular function
fn add(a: i32, b: i32) -> i32 [
    return a + b
]

; Async function
async fn fetch_data(url: str) -> Result [
    await sleep(100)
    return Result
]

; Usage
new result = add(1, 2)
await fetch_data("https://api.example.com")
```

### 4. Error Handling

```catlang
; Define error type
struct MyError [
    code: i32
    message: str
]

; Throw error
throw MyError { code: 1, message: "Something wrong" }

; Catch error
try [
    new data = await risky_operation()
    print("Got: {data}")
] catch (err MyError) [
    print("Error {err.code}: {err.message}")
] catch (e Any) [
    print("Unknown error: {e}")
]
```

### 5. Concurrent Programming

```catlang
; Async task
async fn worker(id: i32) [
    print("Worker {id} started")
    await sleep(100)
    print("Worker {id} done")
]

; Main program
[
    ; Spawn background task
    new handle = spawn async [
        while (true) [
            await sleep(50)
        ]
    ]

    ; Wait for async function
    await worker(1)

    ; Wait for background task
    await handle
]
```

### 6. High-Precision Timer

```catlang
new t: timer = timer.init()

t.start()
; ... execute code ...
t.stop()

print("Elapsed: {t.ms()} ms")
print("Elapsed: {t.us()} μs")
print("Elapsed: {t.seconds()} s")
```

### 7. Unsafe Programming

```catlang
; Disable all safety checks
unsafe all [
    new raw = m+*i32 0x1000
]

; Disable specific checks
unsafe close(init) [
    new uninitialized: i32
]

unsafe close(bounds) [
    new arr = [1, 2, 3]
    new val = arr[10]  ; No bounds check
]
```

---

## 📊 Performance Comparison

### Benchmark Results

| Benchmark | CatLang (-O3) | Rust | C++ | Python |
|-----------|---------------|------|-----|--------|
| Loop (100M) | 0 ms | ~0 ms | ~0 ms | 2146 ms |
| Accumulation (10M) | 0 ms | ~0 ms | ~0 ms | 408 ms |
| Fibonacci (n=30) | 3.04 ms | 2.62 ms | ~0 ms | 79.94 ms |

See [BENCHMARK_RESULTS.md](BENCHMARK_RESULTS.md) for detailed data.

---

## 🎯 Design Philosophy

| Principle | Description |
|-----------|-------------|
| **Simplicity First** | Reduce boilerplate, make code more readable |
| **Safe by Default** | Memory safety by default, dangerous operations require explicit declaration |
| **Controllable Performance** | Multi-level optimization, choose performance/safety balance as needed |
| **Learning Friendly** | Python-style syntax, low learning curve |

---

## 📦 Project Structure

```
catlang/
├── src/              # Compiler source code
│   ├── lexer.rs      # Lexical analyzer
│   ├── parser.rs     # Syntax analyzer
│   ├── ast.rs        # Abstract Syntax Tree
│   ├── optimizer.rs  # Optimizer
│   ├── codegen.rs    # Code generator
│   └── main.rs       # CLI entry point
├── test/             # Test cases
├── benchmark/        # Performance benchmarks
└── guides/           # Learning guides
```

---

## 🤝 Contributing

Contributions, issues, and suggestions are welcome!

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🔗 Related Links

- [Syntax Documentation](syntax.txt)
- [Performance Benchmarks](BENCHMARK_RESULTS.md)
- [Test Cases](test/)
- [Zig Programming Language](https://ziglang.org/)

---

<div align="center">

**CatLang** - Simple as Python, Fast as Rust 🚀

</div>
