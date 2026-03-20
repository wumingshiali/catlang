# Chapter 01: Quick Start

Welcome to CatLang! This chapter will guide you through running your first CatLang program.

## 1.1 Prerequisites

Before getting started, ensure you have completed the environment setup:

- ✅ Zig is installed
- ✅ CatLang compiler is compiled or downloaded

If you haven't configured it yet, please read [Chapter 00: Environment Setup](00_base_env.md).

## 1.2 Your First CatLang Program

Create file `hello.cat`:

```catlang
; My first CatLang program
[
    print("Hello, CatLang!")
    print("Welcome to the world of systems programming")

    ; Variable declaration and string interpolation
    new name = "CatLang"
    new version = 1.0
    print("Running: {name} v{version}")

    return 0
]
```

## 1.3 Run the Program

```bash
# Run CatLang program
./target/release/catlang hello.cat
```

## 1.4 Program Structure Analysis

Let's look at the program above:

```catlang
; My first CatLang program    ; Semicolon starts a comment
[                          ; Square bracket starts code block (main entry)
    print("Hello")         ; Call built-in function, no import needed
    new name = "CatLang"   ; Declare and initialize variable
    return 0               ; Return value
]                          ; End code block
```

### Core Features

| Feature | Description |
|------|------|
| `;` Comment | Everything from semicolon to end of line is a comment |
| `[]` Code Block | All code blocks use square brackets |
| Zero Boilerplate | `print` can be used directly, no `import` needed |
| `new` Declare Variable | Unified variable declaration keyword |

## 1.5 Try Modifications

Try modifying the program by adding the following:

```catlang
[
    ; Try different literal types
    new int_val = 42
    new float_val = 3.14159
    new hex_val = 0xFF
    new bool_val = true
    new string_val = "Hello"

    ; Output them
    print("Integer: {int_val}")
    print("Float: {float_val}")
    print("Hexadecimal: {hex_val}")
    print("Boolean: {bool_val}")
    print("String: {string_val}")

    return 0
]
```

## 1.6 Next Steps

Now that you've run your first program, continue learning:

- [Chapter 00: Environment Setup](00_base_env.md) - See how to set up the environment (if not yet configured)
- [Chapter 02: Basic Syntax](02_basic_syntax.md) - Deep dive into variables, types, and operators
- [Syntax Specification](../syntax.txt) - Complete syntax rules reference

## Quiz

1. What symbol does CatLang use for comments?
2. What brackets are used for code blocks?
3. Do you need `import` to use `print`?

<details>
<summary>View Answers</summary>

1. Semicolon `;`
2. Square brackets `[]`
3. No, standard library features are automatically injected
</details>
