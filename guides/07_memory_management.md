# Chapter 07: Memory Management

This chapter introduces CatLang's memory management mechanism: unsafe blocks, memory reinterpretation, memory copying, and low-level control.

## 7.1 Safety Model

CatLang provides memory safety by default, but allows low-level control through `unsafe` blocks:

```catlang
[
    ; Safe code - compiler checks
    new x = 10
    new arr = [1, 2, 3]
    new val = arr[0]  ; Bounds checked

    ; Unsafe code - programmer responsibility
    unsafe all [
        new raw = m+*i32 0x1000
        print("{*raw}")
    ]

    return 0
]
```

## 7.2 Unsafe Blocks

### Safety Targets

CatLang provides multiple safety checks:

| Check Type | Description |
|---------|------|
| `init` | Initialization check - ensures variables are initialized before use |
| `bounds` | Bounds check - ensures array access doesn't go out of bounds |
| `lifetime` | Lifetime check - ensures references are valid |
| `null` | Null pointer check - ensures pointers are non-null |

### Disable Specific Checks

```catlang
; Disable initialization check
unsafe close(init) [
    new uninitialized i32
    ; Can use uninitialized variable
    print("{uninitialized}")
]

; Disable bounds check
unsafe close(bounds) [
    new arr = [1, 2, 3]
    new val = arr[100]  ; Won't trigger bounds check
]

; Disable lifetime check
unsafe close(lifetime) [
    ; Perform operations that may violate lifetime
]

; Disable null pointer check
unsafe close(null) [
    new ptr *i32
    ; Can dereference potentially null pointer
]
```

### Keep Specific Checks

```catlang
; Keep only lifetime check, disable others
unsafe keep(lifetime) [
    ; Other checks are disabled
]

; Keep initialization and bounds checks
unsafe keep(init, bounds) [
    ; Only keep these two checks
]
```

### Disable All Checks

```catlang
unsafe all [
    ; All safety checks are disabled
    ; Full low-level control
    new raw_memory = m+*u8 0x0000
    print("{*raw_memory}")
]
```

## 7.3 Memory Reinterpretation

Use `m+` for physical bit reinterpretation (doesn't change bit pattern, only interpretation):

### Basic Syntax

```catlang
m+<type> <expression>
```

### Integer to Float

```catlang
[
    ; IEEE 754 double precision representation of 1.0
    new int_rep = 0x3FF0000000000000
    new float_val = m+f64 int_rep

    print("Reinterpreted: {float_val}")  ; Output: 1.0

    ; IEEE 754 double precision representation of 2.0
    new int_rep2 = 0x4000000000000000
    new float_val2 = m+f64 int_rep2

    print("Reinterpreted: {float_val2}")  ; Output: 2.0

    return 0
]
```

### Float to Integer

```catlang
[
    new pi = 3.14159265359
    new bits = m+i64 pi

    print("Pi bit pattern: {bits}")
    print("Hexadecimal: 0x{bits:x}")

    return 0
]
```

### Pointer Reinterpretation

```catlang
[
    new addr = 0x7fff0000
    new ptr = m+*i32 addr

    unsafe all [
        print("{*ptr}")
    ]

    return 0
]
```

## 7.4 Memory Copy (cpy)

Use `cpy` for raw memory copying:

### Basic Syntax

```catlang
cpy <destination> <type> (<source>)
```

### Copy Basic Types

```catlang
[
    new source = 42
    new dest i32

    ; Copy 4 bytes (i32 size)
    cpy dest i32(source)

    print("Destination value: {dest}")  ; Output: 42

    return 0
]
```

### Copy Structs

```catlang
struct Data [
    a: i32
    b: i32
    c: i32
    d: i32
]

[
    new src = Data { a: 1, b: 2, c: 3, d: 4 }
    new dst Data

    ; Copy entire struct
    cpy dst Data(src)

    print("dst.a = {dst.a}, dst.b = {dst.b}")
    print("dst.c = {dst.c}, dst.d = {dst.d}")

    return 0
]
```

### Partial Copy

```catlang
[
    new source = 0x1234567890ABCDEF
    new dest i32

    ; Copy only lower 4 bytes
    cpy dest i32(source)

    print("Destination value: {dest}")  ; Output: lower 32 bits

    return 0
]
```

## 7.5 Pointer Operations

### Pointer Declaration and Dereference

```catlang
[
    new value = 100
    new ptr *i32 = &value

    ; Dereference
    new deref = *ptr
    print("Value: {deref}")  ; Output: 100

    ; Modify through pointer
    *ptr = 200
    print("New value: {value}")  ; Output: 200

    return 0
]
```

### Pointer Arithmetic (unsafe)

```catlang
[
    new arr = [10, 20, 30, 40, 50]
    new base_ptr *i32 = &arr[0]

    unsafe close(bounds) [
        ; Pointer offset
        new ptr1 = base_ptr
        new ptr2 = base_ptr + 1  ; Points to next element
        new ptr3 = base_ptr + 2

        print("{*ptr1}")  ; Output: 10
        print("{*ptr2}")  ; Output: 20
        print("{*ptr3}")  ; Output: 30
    ]

    return 0
]
```

### Null Pointer

```catlang
[
    new null_ptr *i32 = null

    unsafe close(null) [
        ; Dereference null pointer (dangerous!)
        ; print("{*null_ptr}")
    ]

    ; Check for null pointer
    if (null_ptr == null) [
        print("Null pointer")
    ]

    return 0
]
```

## 7.6 Memory Layout

### Struct Memory Layout

```catlang
struct Point [
    x: i32    ; 4 bytes
    y: i32    ; 4 bytes
]

; Point struct occupies 8 bytes (may have alignment padding)

struct PackedData [
    a: u8     ; 1 byte
    b: i32    ; 4 bytes (may have 3 bytes padding)
    c: u8     ; 1 byte
]

; PackedData may occupy 12 bytes (due to alignment)
```

### Calculate Offset

```catlang
unsafe all [
    ; Calculate struct field offset
    new base = 0
    new ptr = m+*Point base

    ; x offset is 0
    ; y offset is 4

    new y_ptr = ptr + 1  ; Points to y field
]
```

## 7.7 Comprehensive Examples

### Example 1: Type Punning

```catlang
; Implement type punning using memory reinterpretation

union FloatInt [
    as_float: f64
    as_int: i64
]

[
    ; Method 1: Using union
    new fi = FloatInt { as_float: 3.14159 }
    print("Bit pattern: {fi.as_int}")

    ; Method 2: Using m+ reinterpretation
    new float_val = 2.71828
    new int_bits = m+i64 float_val
    print("Bit pattern: {int_bits}")

    return 0
]
```

### Example 2: Raw Memory Operations

```catlang
struct MemoryBlock [
    address: u64
    size: u64
]

impl MemoryBlock [
    fn read_byte(self: MemoryBlock, offset: u64) -> u8 [
        unsafe all [
            new addr = self.address + offset
            new ptr = m+*u8 addr
            return *ptr
        ]
    ]

    fn write_byte(self: MemoryBlock, offset: u64, value: u8) [
        unsafe all [
            new addr = self.address + offset
            new ptr = m+*u8 addr
            *ptr = value
        ]
    ]

    fn read_u32(self: MemoryBlock, offset: u64) -> u32 [
        unsafe all [
            new addr = self.address + offset
            new ptr = m+*u32 addr
            return *ptr
        ]
    ]
]

[
    new block = MemoryBlock { address: 0x1000, size: 256 }

    ; Read memory (simulated)
    new byte0 = block.read_byte(0)
    new word0 = block.read_u32(0)

    return 0
]
```

### Example 3: Custom Memory Pool

```catlang
struct MemoryPool [
    buffer: [u8; 1024]
    offset: u32
]

impl MemoryPool [
    fn alloc(self: MemoryPool, size: u32) -> *u8 [
        unsafe close(bounds) [
            if (self.offset + size > 1024) [
                throw "Memory pool overflow"
            ]
            new ptr = &self.buffer[self.offset]
            self.offset = self.offset + size
            return ptr
        ]
    ]

    fn reset(self: MemoryPool) [
        self.offset = 0
    ]
]

[
    new pool = MemoryPool { offset: 0 }

    new ptr1 = pool.alloc(64)
    new ptr2 = pool.alloc(128)

    ; Use allocated memory
    unsafe all [
        *ptr1 = 42
    ]

    ; Reset pool
    pool.reset()

    return 0
]
```

### Example 4: Serialization/Deserialization

```catlang
struct Serializer [
    buffer: [u8; 256]
    position: u32
]

impl Serializer [
    fn write_i32(self: Serializer, value: i32) [
        unsafe all [
            new bits = m+u32 value
            new byte0 = bits & 0xFF
            new byte1 = (bits >> 8) & 0xFF
            new byte2 = (bits >> 16) & 0xFF
            new byte3 = (bits >> 24) & 0xFF

            self.buffer[self.position] = byte0
            self.buffer[self.position + 1] = byte1
            self.buffer[self.position + 2] = byte2
            self.buffer[self.position + 3] = byte3

            self.position = self.position + 4
        ]
    ]

    fn write_f64(self: Serializer, value: f64) [
        unsafe all [
            new bits = m+u64 value

            for (new i = 0, i < 8, i += 1) [
                new byte = (bits >> (i * 8)) & 0xFF
                self.buffer[self.position + i] = byte
            ]

            self.position = self.position + 8
        ]
    ]
]

[
    new serializer = Serializer { position: 0 }

    serializer.write_i32(42)
    serializer.write_f64(3.14159)

    return 0
]
```

### Example 5: Hardware Register Access

```catlang
struct GPIO [
    base_addr: u64
]

impl GPIO [
    fn write_reg(self: GPIO, offset: u32, value: u32) [
        unsafe all [
            new addr = self.base_addr + offset
            new ptr = m+*u32 addr
            *ptr = value
        ]
    ]

    fn read_reg(self: GPIO, offset: u32) -> u32 [
        unsafe all [
            new addr = self.base_addr + offset
            new ptr = m+*u32 addr
            return *ptr
        ]
    ]

    fn set_bit(self: GPIO, offset: u32, bit: u32) [
        unsafe all [
            new addr = self.base_addr + offset
            new ptr = m+*u32 addr
            *ptr = *ptr | (1 << bit)
        ]
    ]

    fn clear_bit(self: GPIO, offset: u32, bit: u32) [
        unsafe all [
            new addr = self.base_addr + offset
            new ptr = m+*u32 addr
            *ptr = *ptr & ~(1 << bit)
        ]
    ]
]

[
    new gpio = GPIO { base_addr: 0x48000000 }

    ; Configure pin
    gpio.write_reg(0x00, 0x00000001)  ; Set pin 0 as output
    gpio.set_bit(0x04, 0)             ; Output high
    gpio.clear_bit(0x04, 0)           ; Output low

    new status = gpio.read_reg(0x08)

    return 0
]
```

## 7.8 Safety Best Practices

### 1. Minimize unsafe Scope

```catlang
; Bad practice - large unsafe scope
unsafe all [
    new x = 10
    new ptr = m+*i32 0x1000
    new y = 20
    print("{*ptr}")
]

; Good practice - minimize unsafe
new x = 10
new y = 20
unsafe all [
    new ptr = m+*i32 0x1000
    print("{*ptr}")
]
```

### 2. Add Comments to Explain

```catlang
unsafe close(bounds) [
    ; Safe: known array length is 100, i range is 0-99
    for (new i = 0, i < 100, i += 1) [
        new val = arr[i]
    ]
]
```

### 3. Encapsulate unsafe Operations

```catlang
; Encapsulate unsafe in safe interface
fn safe_read(ptr: *i32) -> i32 [
    if (ptr == null) [
        throw "Null pointer"
    ]
    unsafe close(null) [
        return *ptr
    ]
]
```

### 4. Validate Pointer Validity

```catlang
unsafe all [
    new ptr = m+*i32 addr

    ; Validate address range
    if (addr < 0x1000 || addr > 0xFFFFFFFF) [
        throw "Invalid address"
    ]

    print("{*ptr}")
]
```

## 7.9 Exercises

1. Use `m+` to reinterpret float 0.5 as integer and print its bit pattern
2. Write a function that uses `cpy` to copy part of a struct's fields
3. Implement a simple stack data structure using unsafe blocks for low-level memory operations

<details>
<summary>Reference Answers</summary>

```catlang
; Exercise 1: Float reinterpretation
[
    new val = 0.5
    new bits = m+i64 val
    print("0.5 bit pattern: {bits}")
    print("Hexadecimal: 0x{bits:x}")
    return 0
]

; Exercise 2: Partial copy
struct Source [
    a: i32
    b: i32
    c: i32
]

[
    new src = Source { a: 1, b: 2, c: 3 }
    new dest i32

    ; Copy only first field
    cpy dest i32(src)

    print("dest = {dest}")  ; Output: 1
    return 0
]

; Exercise 3: Simple stack
struct Stack [
    data: [i32; 100]
    top: i32
]

impl Stack [
    fn push(self: Stack, value: i32) [
        unsafe close(bounds) [
            self.data[self.top] = value
            self.top = self.top + 1
        ]
    ]

    fn pop(self: Stack) -> i32 [
        unsafe close(bounds) [
            self.top = self.top - 1
            return self.data[self.top]
        ]
    ]
]
```
</details>

## Next Steps

- [Chapter 08: Concurrency](08_concurrency.md) - async/await, spawn tasks
- [Chapter 10: Best Practices](10_best_practices.md) - Code style, performance tips
