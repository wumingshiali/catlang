# Chapter 02: Basic Syntax

This chapter introduces CatLang's basic syntax elements: variables, types, operators, and comments.

## 2.1 Comments

CatLang uses semicolon `;` for comments. Everything from the semicolon to the end of the line is ignored:

```catlang
; This is a single-line comment
[
    print("Hello")  ; End-of-line comment

    ; Multi-line comments need semicolon on each line
    ; This is the second line
    ; This is the third line

    return 0
]
```

**Note**: Semicolon `;` is only used for comments, **not** for statement endings.

## 2.2 Variable Declaration

Declare variables using the `new` keyword:

```catlang
[
    ; Declare without initialization
    new x i32

    ; Declare and initialize (type inference)
    new y = 10
    new z = 3.14
    new name = "CatLang"
    new flag = true

    ; Explicitly specify type
    new a i32 = 42
    new b f64 = 2.718

    return 0
]
```

### Variable Naming Rules

```catlang
; Legal identifiers
new x = 1
new my_var = 2
new myVar = 3
new myVar2 = 4
new _private = 5

; Illegal identifiers
; new 2var = 1    ; Cannot start with digit
; new my-var = 2  ; Cannot contain hyphen
; new my var = 3  ; Cannot contain space
```

## 2.3 Literal Types

### Integers

```catlang
[
    new decimal = 42        ; Decimal
    new hex = 0xFF          ; Hexadecimal (255)
    new hex2 = 0x1A2B       ; Hexadecimal (6699)

    return 0
]
```

### Floating-Point Numbers

```catlang
[
    new pi = 3.14159
    new e = 2.71828
    new large = 1.5e10      ; Scientific notation (15000000000.0)
    new small = 2.5e-3      ; 0.0025

    return 0
]
```

### Strings

```catlang
[
    new simple = "Hello"
    new with_escape = "Line1\nLine2"  ; Newline
    new with_tab = "Col1\tCol2"       ; Tab

    ; String interpolation
    new name = "Alice"
    new age = 25
    new intro = "I'm {name}, {age} years old"

    print(intro)  ; Output: I'm Alice, 25 years old

    return 0
]
```

### Booleans

```catlang
[
    new t = true
    new f = false

    return 0
]
```

## 2.4 Built-in Types

CatLang provides the following built-in types:

| Type | Description | Zig Mapping |
|------|------|---------|
| `i8` | 8-bit signed integer | `i8` |
| `i16` | 16-bit signed integer | `i16` |
| `i32` | 32-bit signed integer | `i32` |
| `i64` | 64-bit signed integer | `i64` |
| `u8` | 8-bit unsigned integer | `u8` |
| `u16` | 16-bit unsigned integer | `u16` |
| `u32` | 32-bit unsigned integer | `u32` |
| `u64` | 64-bit unsigned integer | `u64` |
| `f32` | 32-bit float | `f32` |
| `f64` | 64-bit float | `f64` |
| `bool` | Boolean | `bool` |
| `str` | String | `[]const u8` |
| `void` | Void type | `void` |

### Special Types

| Type | Description | Zig Mapping |
|------|------|---------|
| `ia` | Arbitrary-length integer | `i128` |
| `fa` | Arbitrary-length float | `f128` |
| `sa` | Arbitrary-length string | `[]const u8` |
| `timer` | Timer type | `Timer` |
| `Result` | Result type | `anyerror!void` |
| `Future` | Future type | `anyerror!void` |

### Arbitrary Bit-Width Types

CatLang provides arbitrary bit-width types, allowing you to specify the number of bits:

| Type | Description | Zig Mapping |
|------|------|---------|
| `a8` | Arbitrary 8-bit type | `u8` |
| `a16` | Arbitrary 16-bit type | `u16` |
| `a32` | Arbitrary 32-bit type | `u32` |
| `a64` | Arbitrary 64-bit type | `u64` |
| `aa` | Arbitrary/infinite length type | `u128` |

```catlang
[
    ; Use arbitrary bit-width types
    new x: a8 = 42          ; 8-bit arbitrary type
    new y: a16 = 1000       ; 16-bit arbitrary type
    new z: a32 = 100000     ; 32-bit arbitrary type
    new big: aa = 999999999 ; Arbitrary length type

    return 0
]
```

### Generic Types

CatLang supports generic type parameters using `<T>` or `<T1, T2, ...>` syntax:

```catlang
[
    ; Single type parameter
    new opt: Option<i32> = ...
    new list: List<String> = ...

    ; Multiple type parameters
    new result: Result<i32, String> = ...
    new map: HashMap<String, i32> = ...

    return 0
]
```

```catlang
[
    ; Use special types
    new big_int ia = 999999999999999
    new precise fa = 3.141592653589793238
    new long_text sa = "This is a very long text..."

    return 0
]
```

## 2.5 Operators

### Arithmetic Operators

```catlang
[
    new a = 10 + 5    ; Addition, a = 15
    new b = 10 - 5    ; Subtraction, b = 5
    new c = 10 * 5    ; Multiplication, c = 50
    new d = 10 / 5    ; Division, d = 2
    new e = 10 % 3    ; Modulo, e = 1

    return 0
]
```

### Compound Assignment Operators

```catlang
[
    new x = 10
    x += 5    ; x = x + 5 = 15
    x -= 3    ; x = x - 3 = 12
    x *= 2    ; x = x * 2 = 24
    x /= 4    ; x = x / 4 = 6
    x %= 4    ; x = x % 4 = 2

    return 0
]
```

### Comparison Operators

```catlang
[
    new a = 10 > 5     ; true
    new b = 10 < 5     ; false
    new c = 10 >= 10   ; true
    new d = 10 <= 5    ; false
    new e = 10 == 10   ; true
    new f = 10 != 5    ; true

    return 0
]
```

### Logical Operators

```catlang
[
    new a = true && true   ; true (AND)
    new b = true || false  ; true (OR)

    new x = 5
    new complex = (x > 0) && (x < 10)  ; true

    return 0
]
```

### Bitwise Operators (via compound assignment)

```catlang
[
    new x = 0b1010
    x &= 0b1100   ; Bitwise AND, x = 0b1000
    x |= 0b0011   ; Bitwise OR, x = 0b1011
    x ^= 0b1111   ; Bitwise XOR, x = 0b0100

    return 0
]
```

## 2.6 Type System

### Array Types

```catlang
[
    ; Dynamic array
    new arr [i32]

    ; Fixed-size array
    new fixed [i32; 5]

    ; Multi-dimensional array
    new matrix [[i32; 3]; 3]

    return 0
]
```

### Pointer Types

```catlang
[
    ; Pointer declaration
    new ptr *i32

    ; Dereference (using * prefix)
    new val = *ptr

    return 0
]
```

### Memory Reinterpretation

Use `m+` for physical bit reinterpretation:

```catlang
[
    ; Reinterpret integer bit pattern as float
    new int_rep = 0x3FF0000000000000
    new pi_val = m+f64 int_rep  ; pi_val ≈ 1.0

    return 0
]
```

## 2.7 Complete Example

```catlang
; Comprehensive example: Calculate circle properties
[
    ; Constant definition
    new pi = 3.14159265359

    ; Input
    new radius = 5.0

    ; Calculation
    new area = pi * radius * radius
    new circumference = 2 * pi * radius

    ; Output (using string interpolation)
    print("Radius: {radius}")
    print("Area: {area}")
    print("Circumference: {circumference}")

    ; Conditional judgment
    if (area > 50) [
        print("This is a large circle")
    ] else [
        print("This is a small circle")
    ]

    return 0
]
```

## 2.8 Exercises

1. Declare three variables to store your name, age, and height
2. Calculate and output the area and perimeter of a square with side length 5
3. Use string interpolation to output a self-introduction

<details>
<summary>Reference Answers</summary>

```catlang
[
    ; Exercise 1
    new name = "John Doe"
    new age = 25
    new height = 1.75

    ; Exercise 2
    new side = 5
    new area = side * side
    new perimeter = 4 * side
    print("Square area: {area}")
    print("Square perimeter: {perimeter}")

    ; Exercise 3
    print("I'm {name}, {age} years old, {height} meters tall")

    return 0
]
```
</details>

## Next Steps

- [Chapter 03: Control Flow](03_control_flow.md) - Conditional statements, loops, pattern matching
- [Chapter 05: Data Structures](05_data_structures.md) - Structs, arrays deep dive
