# 此文章存在大量错误，多半是写的~~人~~AI脑子坏了，这~~TM~~是Rust的类型系统和CatLang的类型系统的融合版

# 第 11 章：类型系统深入

本章深入介绍 CatLang 的类型系统，包括泛型类型和任意位宽类型。

## 11.1 泛型类型

泛型允许你编写与类型无关的代码，提高代码复用性。

### 泛型语法

使用 `<T>` 或 `<T1, T2, ...>` 语法声明类型参数：

```catlang
; 单类型参数
struct Container<T> [
    item: T
]

; 多类型参数
struct Pair<T, U> [
    first: T
    second: U
]

; 泛型方法
impl Container<T> [
    fn get(self: Container<T>) -> T [
        return self.item
    ]

    fn set(self: Container<T>, new_item: T) -> Container<T> [
        return Container { item: new_item }
    ]
]
```

### 泛型实例化

```catlang
[
    ; 实例化泛型结构体
    new int_container: Container<i32> = Container { item: 42 }
    new str_container: Container<String> = Container { item: "Hello" }
    
    ; 使用泛型方法
    new val = int_container.get()
    new new_container = int_container.set(100)
    
    ; 多类型参数实例化
    new pair: Pair<i32, String> = Pair { first: 42, second: "Answer" }
    
    return 0
]
```

### 常见泛型模式

#### Option 类型

```catlang
struct Option<T> [
    value: T
    has_value: bool
]

impl Option<T> [
    fn is_some(self: Option<T>) -> bool [
        return self.has_value
    ]

    fn is_none(self: Option<T>) -> bool [
        return !self.has_value
    ]

    fn unwrap(self: Option<T>) -> T [
        return self.value
    ]
]

[
    new some: Option<i32> = Option { value: 42, has_value: true }
    new none: Option<i32> = Option { value: 0, has_value: false }
    
    if (some.is_some()) [
        print("值：{some.unwrap()}")
    ]
    
    return 0
]
```

#### Result 类型

```catlang
struct Result<T, E> [
    ok: T
    err: E
    is_ok: bool
]

impl Result<T, E> [
    fn is_success(self: Result<T, E>) -> bool [
        return self.is_ok
    ]

    fn get_ok(self: Result<T, E>) -> T [
        return self.ok
    ]

    fn get_err(self: Result<T, E>) -> E [
        return self.err
    ]
]

[
    new success: Result<i32, String> = Result { ok: 100, err: "", is_ok: true }
    new failure: Result<i32, String> = Result { ok: 0, err: "Error occurred", is_ok: false }
    
    if (success.is_success()) [
        print("成功：{success.get_ok()}")
    ] else [
        print("失败：{failure.get_err()}")
    ]
    
    return 0
]
```

#### 泛型链表

```catlang
struct Node<T> [
    value: T
    next: *Node<T>
]

struct LinkedList<T> [
    head: *Node<T>
    length: i32
]

impl LinkedList<T> [
    fn new() -> LinkedList<T> [
        return LinkedList { head: null, length: 0 }
    ]

    fn push(self: *LinkedList<T>, value: T) [
        ; 实现推入逻辑
        self.length = self.length + 1
    ]

    fn len(self: LinkedList<T>) -> i32 [
        return self.length
    ]
]

[
    new list: LinkedList<i32> = LinkedList::new()
    list.push(1)
    list.push(2)
    list.push(3)
    
    print("链表长度：{list.len()}")
    
    return 0
]
```

## 11.2 任意位宽类型

CatLang 提供任意位宽类型，允许你指定特定位数的类型，适用于硬件编程、网络协议等场景。

### 类型列表

| 类型 | 位数 | Zig 映射 | 使用场景 |
|------|------|---------|----------|
| `a8` | 8 | `u8` | 字节、标志位 |
| `a16` | 16 | `u16` | 短整数、端口号 |
| `a32` | 32 | `u32` | 整数、IPv4 地址 |
| `a64` | 64 | `u64` | 长整数、时间戳 |
| `aa` | 任意 | `u128` | 大数、加密 |

### 基本用法

```catlang
[
    ; 8 位类型
    new byte: a8 = 255
    new flag: a8 = 0x01
    
    ; 16 位类型
    new port: a16 = 8080
    new short_max: a16 = 65535
    
    ; 32 位类型
    new int_val: a32 = 2147483647
    new ipv4: a32 = 0x7F000001  ; 127.0.0.1
    
    ; 64 位类型
    new timestamp: a64 = 1699999999999
    new long_max: a64 = 9223372036854775807
    
    ; 任意长度类型
    new big_int: aa = 999999999999999999999999
    new crypto_key: aa = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    
    return 0
]
```

### 硬件编程示例

```catlang
; GPIO 寄存器映射
struct GPIO_Register [
    input_data: a32    ; 输入数据寄存器
    output_data: a32   ; 输出数据寄存器
    set_bits: a32      ; 置位寄存器
    reset_bits: a32    ; 复位寄存器
    config: [a32; 8]   ; 配置寄存器
]

; 控制 LED
fn toggle_led(gpio: *GPIO_Register, pin: a8) [
    ; 计算位掩码
    new mask: a32 = 1 << pin
    
    ; 切换 LED
    gpio.set_bits = mask
]

[
    ; 假设 GPIO 基地址
    new gpio_base: *GPIO_Register = 0x40020000
    
    toggle_led(gpio_base, 5)  ; 切换引脚 5
    
    return 0
]
```

### 网络协议示例

```catlang
; IPv4 包头
struct IPv4_Header [
    version_ihl: a8       ; 版本 (4 位) + 包头长度 (4 位)
    dscp_ecn: a8          ; DSCP (6 位) + ECN (2 位)
    total_length: a16     ; 总长度
    identification: a16   ; 标识
    flags_fragment: a16   ; 标志 (3 位) + 片偏移 (13 位)
    ttl: a8               ; 生存时间
    protocol: a8          ; 协议
    checksum: a16         ; 包头校验和
    src_addr: a32         ; 源 IP 地址
    dst_addr: a32         ; 目的 IP 地址
]

impl IPv4_Header [
    fn version(self: IPv4_Header) -> a8 [
        return (self.version_ihl >> 4) & 0x0F
    ]

    fn ihl(self: IPv4_Header) -> a8 [
        return self.version_ihl & 0x0F
    ]

    fn header_length(self: IPv4_Header) -> a16 [
        return self.ihl() * 4
    ]
]

[
    new packet = IPv4_Header {
        version_ihl: 0x45,      ; IPv4, 5 个 32 位字
        dscp_ecn: 0x00,
        total_length: 64,
        identification: 0x1234,
        flags_fragment: 0x4000,  ; 不分片
        ttl: 64,
        protocol: 6,             ; TCP
        checksum: 0,
        src_addr: 0x7F000001,    ; 127.0.0.1
        dst_addr: 0x7F000001     ; 127.0.0.1
    }
    
    print("IP 版本：{packet.version()}")
    print("包头长度：{packet.header_length()}")
    
    return 0
]
```

### 位域操作

```catlang
; 使用位运算操作位域
struct Control_Register [
    value: a32
]

impl Control_Register [
    ; 设置特定位
    fn set_bit(self: *Control_Register, bit: a8) [
        self.value = self.value | (1 << bit)
    ]

    ; 清除特定位
    fn clear_bit(self: *Control_Register, bit: a8) [
        self.value = self.value & ~(1 << bit)
    ]

    ; 读取特定位
    fn get_bit(self: Control_Register, bit: a8) -> a8 [
        return (self.value >> bit) & 1
    ]

    ; 设置位域
    fn set_field(self: *Control_Register, start: a8, size: a8, val: a32) [
        new mask: a32 = ((1 << size) - 1) << start
        self.value = (self.value & ~mask) | (val << start)
    ]

    ; 读取位域
    fn get_field(self: Control_Register, start: a8, size: a8) -> a32 [
        new mask: a32 = (1 << size) - 1
        return (self.value >> start) & mask
    ]
]

[
    new reg = Control_Register { value: 0 }
    
    reg.set_bit(0)           ; 设置位 0
    reg.set_bit(2)           ; 设置位 2
    reg.set_field(4, 4, 15)  ; 设置位域 [7:4] 为 15
    
    new val = reg.get_field(4, 4)
    print("位域值：{val}")
    
    return 0
]
```

## 11.3 类型转换

### 隐式转换

CatLang 在某些情况下支持隐式类型转换：

```catlang
[
    new int_val: i32 = 42
    new big_int: ia = int_val    ; i32 -> ia (隐式)
    
    new byte: a8 = 100
    new short: a16 = byte        ; a8 -> a16 (隐式)
    
    return 0
]
```

### 显式转换

使用类型注解进行显式转换：

```catlang
[
    new big: ia = 999999999999
    new small: i32 = big as i32  ; 显式转换
    
    new float_val: f64 = 3.14
    new int_val: i32 = float_val as i32  ; f64 -> i32
    
    return 0
]
```

### 位重解释

使用 `m+` 进行物理位重解释：

```catlang
[
    ; 浮点数到位模式
    new float_val: f64 = 1.0
    new bits: i64 = m+i64 float_val
    print("位模式：{bits}")  ; 输出：4607182418800017408 (0x3FF0000000000000)
    
    ; 位模式到浮点数
    new bits2: i64 = 0x3FF0000000000000
    new float_val2: f64 = m+f64 bits2
    print("重解释值：{float_val2}")  ; 输出：1.0
    
    return 0
]
```

## 11.4 类型推断

CatLang 支持强大的类型推断：

```catlang
[
    ; 字面量推断
    new x = 42           ; 推断为 i32
    new y = 3.14         ; 推断为 f64
    new s = "hello"      ; 推断为 str
    new b = true         ; 推断为 bool
    
    ; 表达式推断
    new sum = 1 + 2      ; 推断为 i32
    new product = 2.0 * 3.5  ; 推断为 f64
    
    ; 函数返回推断
    fn get_value() [
        return 42        ; 推断返回 i32
    ]
    
    new val = get_value()  ; val 推断为 i32
    
    return 0
]
```

## 11.5 综合示例

### 泛型栈实现

```catlang
struct Stack<T> [
    data: [T; 1024]
    top: i32
]

impl Stack<T> [
    fn new() -> Stack<T> [
        return Stack { data: undefined, top: 0 }
    ]

    fn push(self: *Stack<T>, item: T) [
        self.data[self.top] = item
        self.top = self.top + 1
    ]

    fn pop(self: *Stack<T>) -> T [
        self.top = self.top - 1
        return self.data[self.top]
    ]

    fn peek(self: Stack<T>) -> T [
        return self.data[self.top - 1]
    ]

    fn is_empty(self: Stack<T>) -> bool [
        return self.top == 0
    ]

    fn len(self: Stack<T>) -> i32 [
        return self.top
    ]
]

[
    ; 整数栈
    new int_stack: Stack<i32> = Stack::new()
    int_stack.push(1)
    int_stack.push(2)
    int_stack.push(3)
    
    print("栈顶：{int_stack.peek()}")
    print("长度：{int_stack.len()}")
    
    while (!int_stack.is_empty()) [
        print("弹出：{int_stack.pop()}")
    ]
    
    ; 字符串栈
    new str_stack: Stack<String> = Stack::new()
    str_stack.push("Hello")
    str_stack.push("World")
    
    while (!str_stack.is_empty()) [
        print("弹出：{str_stack.pop()}")
    ]
    
    return 0
]
```

### 任意精度计算器

```catlang
[
    ; 使用 aa 类型进行任意精度计算
    new a: aa = 123456789012345678901234567890
    new b: aa = 987654321098765432109876543210
    
    new sum = a + b
    new product = a * b
    new diff = b - a
    
    print("和：{sum}")
    print("积：{product}")
    print("差：{diff}")
    
    return 0
]
```

## 11.6 最佳实践

### 1. 选择合适的类型

```catlang
; 好的实践
new age: i32 = 25           ; 常规整数
new port: a16 = 8080        ; 端口号
new big_count: ia = 999999999999  ; 大数

; 不好的实践
new age: aa = 25            ; 过度使用任意类型
new port: i32 = 8080        ; 未使用语义化类型
```

### 2. 泛型命名约定

```catlang
; 单字母类型参数（常见于简单泛型）
struct Container<T> [ ... ]
struct Result<T, E> [ ... ]

; 描述性类型参数（复杂场景）
struct Cache<Key, Value> [ ... ]
struct Graph<Node, Edge> [ ... ]
```

### 3. 位操作安全

```catlang
; 使用 unsafe 块进行底层位操作
unsafe all [
    new raw_reg: *a32 = 0x40020000
    raw_reg[] = 0xFFFFFFFF
]

; 或使用更精细的控制
unsafe close(bounds) [
    ; 进行位域操作
]
```

## 11.7 练习

1. 实现一个泛型队列 `Queue<T>`，支持 `enqueue`、`dequeue` 和 `peek` 操作
2. 使用任意位宽类型实现一个 CRC32 校验和计算器
3. 定义一个泛型二叉树节点 `TreeNode<T>`，并实现插入和遍历方法

<details>
<summary>参考答案</summary>

```catlang
; 练习 1：泛型队列
struct Queue<T> [
    data: [T; 1024]
    front: i32
    back: i32
]

impl Queue<T> [
    fn new() -> Queue<T> [
        return Queue { data: undefined, front: 0, back: 0 }
    ]

    fn enqueue(self: *Queue<T>, item: T) [
        self.data[self.back] = item
        self.back = self.back + 1
    ]

    fn dequeue(self: *Queue<T>) -> T [
        let item = self.data[self.front]
        self.front = self.front + 1
        return item
    ]

    fn peek(self: Queue<T>) -> T [
        return self.data[self.front]
    ]

    fn is_empty(self: Queue<T>) -> bool [
        return self.front == self.back
    ]
]

; 练习 2：CRC32 计算器
fn crc32(data: [a8], crc: a32) -> a32 [
    new polynomial: a32 = 0xEDB88320
    new result = crc
    
    for (new i = 0, i < len(data), i += 1) [
        result = result ^ data[i]
        for (new j = 0; j < 8; j += 1) [
            if (result & 1) [
                result = (result >> 1) ^ polynomial
            ] else [
                result = result >> 1
            ]
        ]
    ]
    
    return result
]

; 练习 3：泛型二叉树
struct TreeNode<T> [
    value: T
    left: *TreeNode<T>
    right: *TreeNode<T>
]

impl TreeNode<T> [
    fn new(val: T) -> *TreeNode<T> [
        return &TreeNode { value: val, left: null, right: null }
    ]

    fn insert(self: *TreeNode<T>, val: T) [
        if (val < self.value) [
            if (self.left == null) [
                self.left = TreeNode::new(val)
            ] else [
                self.left.insert(val)
            ]
        ] else [
            if (self.right == null) [
                self.right = TreeNode::new(val)
            ] else [
                self.right.insert(val)
            ]
        ]
    ]

    fn inorder(self: *TreeNode<T>) [
        if (self.left != null) [
            self.left.inorder()
        ]
        print("{self.value}")
        if (self.right != null) [
            self.right.inorder()
        ]
    ]
]
```
</details>

## 下一步

- [第 07 章：内存管理](07_memory_management.md) - unsafe 深入
- [第 08 章：并发编程](08_concurrency.md) - async/await
