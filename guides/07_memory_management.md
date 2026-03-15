# 第 07 章：内存管理

本章介绍 CatLang 的内存管理机制：unsafe 块、内存重解释、内存复制和底层控制。

## 7.1 安全模型

CatLang 默认提供内存安全保障，但允许通过 `unsafe` 块进行底层控制：

```catlang
[
    ; 安全代码 - 编译器检查
    new x = 10
    new arr = [1, 2, 3]
    new val = arr[0]  ; 边界检查
    
    ; 不安全代码 - 程序员负责
    unsafe all [
        new raw = m+*i32 0x1000
        print("{*raw}")
    ]
    
    return 0
]
```

## 7.2 Unsafe 块

### 安全目标

CatLang 提供多种安全检查：

| 检查类型 | 说明 |
|---------|------|
| `init` | 初始化检查 - 确保变量使用前已初始化 |
| `bounds` | 边界检查 - 确保数组访问不越界 |
| `lifetime` | 生命周期检查 - 确保引用有效 |
| `null` | 空指针检查 - 确保指针非空 |

### 关闭特定检查

```catlang
; 关闭初始化检查
unsafe close(init) [
    new uninitialized i32
    ; 可以使用未初始化的变量
    print("{uninitialized}")
]

; 关闭边界检查
unsafe close(bounds) [
    new arr = [1, 2, 3]
    new val = arr[100]  ; 不会触发边界检查
]

; 关闭生命周期检查
unsafe close(lifetime) [
    ; 进行可能违反生命周期的操作
]

; 关闭空指针检查
unsafe close(null) [
    new ptr *i32
    ; 可以解引用可能为空的指针
]
```

### 保持特定检查

```catlang
; 只保持生命周期检查，关闭其他
unsafe keep(lifetime) [
    ; 其他检查被禁用
]

; 保持初始化和边界检查
unsafe keep(init, bounds) [
    ; 只保持这两种检查
]
```

### 完全关闭所有检查

```catlang
unsafe all [
    ; 所有安全检查都被禁用
    ; 完全的底层控制
    new raw_memory = m+*u8 0x0000
    print("{*raw_memory}")
]
```

## 7.3 内存重解释

使用 `m+` 进行物理位重解释（不改变位模式，只改变解释方式）：

### 基本语法

```catlang
m+<type> <expression>
```

### 整数到浮点数

```catlang
[
    ; IEEE 754 双精度表示 1.0
    new int_rep = 0x3FF0000000000000
    new float_val = m+f64 int_rep
    
    print("重解释：{float_val}")  ; 输出：1.0
    
    ; IEEE 754 双精度表示 2.0
    new int_rep2 = 0x4000000000000000
    new float_val2 = m+f64 int_rep2
    
    print("重解释：{float_val2}")  ; 输出：2.0
    
    return 0
]
```

### 浮点数到整数

```catlang
[
    new pi = 3.14159265359
    new bits = m+i64 pi
    
    print("Pi 的位模式：{bits}")
    print("十六进制：0x{bits:x}")
    
    return 0
]
```

### 指针重解释

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

## 7.4 内存复制 (cpy)

使用 `cpy` 进行原始内存复制：

### 基本语法

```catlang
cpy <destination> <type> (<source>)
```

### 复制基本类型

```catlang
[
    new source = 42
    new dest i32
    
    ; 复制 4 字节（i32 大小）
    cpy dest i32(source)
    
    print("目标值：{dest}")  ; 输出：42
    
    return 0
]
```

### 复制结构体

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
    
    ; 复制整个结构体
    cpy dst Data(src)
    
    print("dst.a = {dst.a}, dst.b = {dst.b}")
    print("dst.c = {dst.c}, dst.d = {dst.d}")
    
    return 0
]
```

### 部分复制

```catlang
[
    new source = 0x1234567890ABCDEF
    new dest i32
    
    ; 只复制低 4 字节
    cpy dest i32(source)
    
    print("目标值：{dest}")  ; 输出：低 32 位
    
    return 0
]
```

## 7.5 指针操作

### 指针声明和解引用

```catlang
[
    new value = 100
    new ptr *i32 = &value
    
    ; 解引用
    new deref = *ptr
    print("值：{dereF}")  ; 输出：100
    
    ; 通过指针修改
    *ptr = 200
    print("新值：{value}")  ; 输出：200
    
    return 0
]
```

### 指针算术（unsafe）

```catlang
[
    new arr = [10, 20, 30, 40, 50]
    new base_ptr *i32 = &arr[0]
    
    unsafe close(bounds) [
        ; 指针偏移
        new ptr1 = base_ptr
        new ptr2 = base_ptr + 1  ; 指向下一个元素
        new ptr3 = base_ptr + 2
        
        print("{*ptr1}")  ; 输出：10
        print("{*ptr2}")  ; 输出：20
        print("{*ptr3}")  ; 输出：30
    ]
    
    return 0
]
```

### 空指针

```catlang
[
    new null_ptr *i32 = null
    
    unsafe close(null) [
        ; 解引用空指针（危险！）
        ; print("{*null_ptr}")
    ]
    
    ; 检查空指针
    if (null_ptr == null) [
        print("空指针")
    ]
    
    return 0
]
```

## 7.6 内存布局

### 结构体内存布局

```catlang
struct Point [
    x: i32    ; 4 字节
    y: i32    ; 4 字节
]

; Point 结构体占用 8 字节（可能有对齐填充）

struct PackedData [
    a: u8     ; 1 字节
    b: i32    ; 4 字节（可能有 3 字节填充）
    c: u8     ; 1 字节
]

; PackedData 可能占用 12 字节（由于对齐）
```

### 计算偏移量

```catlang
unsafe all [
    ; 计算结构体字段的偏移量
    new base = 0
    new ptr = m+*Point base
    
    ; x 的偏移量是 0
    ; y 的偏移量是 4
    
    new y_ptr = ptr + 1  ; 指向 y 字段
]
```

## 7.7 综合示例

### 示例 1：类型双关

```catlang
; 使用内存重解释实现类型双关

union FloatInt [
    as_float: f64
    as_int: i64
]

[
    ; 方法 1：使用 union
    new fi = FloatInt { as_float: 3.14159 }
    print("位模式：{fi.as_int}")
    
    ; 方法 2：使用 m+ 重解释
    new float_val = 2.71828
    new int_bits = m+i64 float_val
    print("位模式：{int_bits}")
    
    return 0
]
```

### 示例 2：原始内存操作

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
    
    ; 读取内存（模拟）
    new byte0 = block.read_byte(0)
    new word0 = block.read_u32(0)
    
    return 0
]
```

### 示例 3：自定义内存池

```catlang
struct MemoryPool [
    buffer: [u8; 1024]
    offset: u32
]

impl MemoryPool [
    fn alloc(self: MemoryPool, size: u32) -> *u8 [
        unsafe close(bounds) [
            if (self.offset + size > 1024) [
                throw "内存池溢出"
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
    
    ; 使用分配的内存
    unsafe all [
        *ptr1 = 42
    ]
    
    ; 重置池
    pool.reset()
    
    return 0
]
```

### 示例 4：序列化/反序列化

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

### 示例 5：硬件寄存器访问

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
    
    ; 配置引脚
    gpio.write_reg(0x00, 0x00000001)  ; 设置引脚 0 为输出
    gpio.set_bit(0x04, 0)             ; 输出高电平
    gpio.clear_bit(0x04, 0)           ; 输出低电平
    
    new status = gpio.read_reg(0x08)
    
    return 0
]
```

## 7.8 安全最佳实践

### 1. 最小化 unsafe 范围

```catlang
; 不好的做法 - 大范围 unsafe
unsafe all [
    new x = 10
    new ptr = m+*i32 0x1000
    new y = 20
    print("{*ptr}")
]

; 好的做法 - 最小化 unsafe
new x = 10
new y = 20
unsafe all [
    new ptr = m+*i32 0x1000
    print("{*ptr}")
]
```

### 2. 添加注释说明

```catlang
unsafe close(bounds) [
    ; 安全：已知数组长度为 100，i 范围是 0-99
    for (new i = 0, i < 100, i += 1) [
        new val = arr[i]
    ]
]
```

### 3. 封装 unsafe 操作

```catlang
; 将 unsafe 封装在安全接口中
fn safe_read(ptr: *i32) -> i32 [
    if (ptr == null) [
        throw "空指针"
    ]
    unsafe close(null) [
        return *ptr
    ]
]
```

### 4. 验证指针有效性

```catlang
unsafe all [
    new ptr = m+*i32 addr
    
    ; 验证地址范围
    if (addr < 0x1000 || addr > 0xFFFFFFFF) [
        throw "无效地址"
    ]
    
    print("{*ptr}")
]
```

## 7.9 练习

1. 使用 `m+` 将浮点数 0.5 重解释为整数，并打印其位模式
2. 编写一个函数，使用 `cpy` 复制结构体的部分字段
3. 实现一个简单的栈数据结构，使用 unsafe 块进行底层内存操作

<details>
<summary>参考答案</summary>

```catlang
; 练习 1：浮点数重解释
[
    new val = 0.5
    new bits = m+i64 val
    print("0.5 的位模式：{bits}")
    print("十六进制：0x{bits:x}")
    return 0
]

; 练习 2：部分复制
struct Source [
    a: i32
    b: i32
    c: i32
]

[
    new src = Source { a: 1, b: 2, c: 3 }
    new dest i32
    
    ; 只复制第一个字段
    cpy dest i32(src)
    
    print("dest = {dest}")  ; 输出：1
    return 0
]

; 练习 3：简单栈
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

## 下一步

- [第 08 章：并发编程](08_concurrency.md) - async/await、spawn 任务
- [第 10 章：最佳实践](10_best_practices.md) - 代码风格、性能提示
