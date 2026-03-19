# 第 02 章：基础语法

本章介绍 CatLang 的基础语法元素：变量、类型、运算符和注释。

## 2.1 注释

CatLang 使用分号 `;` 表示注释，从分号到行尾的所有内容都会被忽略：

```catlang
; 这是单行注释
[
    print("Hello")  ; 行尾注释
    
    ; 多行注释需要每行都加分号
    ; 这是第二行
    ; 这是第三行
    
    return 0
]
```

**注意**：分号 `;` 仅用于注释，**不**用于语句结尾。

## 2.2 变量声明

使用 `new` 关键字声明变量：

```catlang
[
    ; 声明不初始化
    new x i32
    
    ; 声明并初始化（类型推断）
    new y = 10
    new z = 3.14
    new name = "CatLang"
    new flag = true
    
    ; 显式指定类型
    new a i32 = 42
    new b f64 = 2.718
    
    return 0
]
```

### 变量命名规则

```catlang
; 合法的标识符
new x = 1
new my_var = 2
new myVar = 3
new myVar2 = 4
new _private = 5

; 非法的标识符
; new 2var = 1    ; 不能以数字开头
; new my-var = 2  ; 不能包含连字符
; new my var = 3  ; 不能包含空格
```

## 2.3 字面量类型

### 整数

```catlang
[
    new decimal = 42        ; 十进制
    new hex = 0xFF          ; 十六进制 (255)
    new hex2 = 0x1A2B       ; 十六进制 (6699)
    
    return 0
]
```

### 浮点数

```catlang
[
    new pi = 3.14159
    new e = 2.71828
    new large = 1.5e10      ; 科学计数法 (15000000000.0)
    new small = 2.5e-3      ; 0.0025
    
    return 0
]
```

### 字符串

```catlang
[
    new simple = "Hello"
    new with_escape = "Line1\nLine2"  ; 换行
    new with_tab = "Col1\tCol2"       ; 制表符
    
    ; 字符串插值
    new name = "Alice"
    new age = 25
    new intro = "我是{name}，今年{age}岁"
    
    print(intro)  ; 输出：我是 Alice，今年 25 岁
    
    return 0
]
```

### 布尔值

```catlang
[
    new t = true
    new f = false
    
    return 0
]
```

## 2.4 内置类型

CatLang 提供以下内置类型：

| 类型 | 描述 | Zig 映射 |
|------|------|---------|
| `i8` | 8 位有符号整数 | `i8` |
| `i16` | 16 位有符号整数 | `i16` |
| `i32` | 32 位有符号整数 | `i32` |
| `i64` | 64 位有符号整数 | `i64` |
| `u8` | 8 位无符号整数 | `u8` |
| `u16` | 16 位无符号整数 | `u16` |
| `u32` | 32 位无符号整数 | `u32` |
| `u64` | 64 位无符号整数 | `u64` |
| `f32` | 32 位浮点数 | `f32` |
| `f64` | 64 位浮点数 | `f64` |
| `bool` | 布尔值 | `bool` |
| `str` | 字符串 | `[]const u8` |
| `void` | 空类型 | `void` |

### 特殊类型

| 类型 | 描述 | Zig 映射 |
|------|------|---------|
| `ia` | 任意长度整数 | `i128` |
| `fa` | 任意长度浮点 | `f128` |
| `sa` | 任意长度字符串 | `[]const u8` |
| `timer` | 定时器类型 | `Timer` |
| `Result` | 结果类型 | `anyerror!void` |
| `Future` | 未来类型 | `anyerror!void` |

### 任意位宽类型

CatLang 提供任意位宽类型，允许你指定位数的类型：

| 类型 | 描述 | Zig 映射 |
|------|------|---------|
| `a8` | 任意 8 位类型 | `u8` |
| `a16` | 任意 16 位类型 | `u16` |
| `a32` | 任意 32 位类型 | `u32` |
| `a64` | 任意 64 位类型 | `u64` |
| `aa` | 任意/无限长度类型 | `u128` |

```catlang
[
    ; 使用任意位宽类型
    new x: a8 = 42          ; 8 位任意类型
    new y: a16 = 1000       ; 16 位任意类型
    new z: a32 = 100000     ; 32 位任意类型
    new big: aa = 999999999 ; 任意长度类型

    return 0
]
```

### 泛型类型

CatLang 支持泛型类型参数，使用 `<T>` 或 `<T1, T2, ...>` 语法：

```catlang
[
    ; 单类型参数
    new opt: Option<i32> = ...
    new list: List<String> = ...

    ; 多类型参数
    new result: Result<i32, String> = ...
    new map: HashMap<String, i32> = ...

    return 0
]
```

```catlang
[
    ; 使用特殊类型
    new big_int ia = 999999999999999
    new precise fa = 3.141592653589793238
    new long_text sa = "这是一段很长的文本..."

    return 0
]
```

## 2.5 运算符

### 算术运算符

```catlang
[
    new a = 10 + 5    ; 加法，a = 15
    new b = 10 - 5    ; 减法，b = 5
    new c = 10 * 5    ; 乘法，c = 50
    new d = 10 / 5    ; 除法，d = 2
    new e = 10 % 3    ; 取余，e = 1
    
    return 0
]
```

### 复合赋值运算符

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

### 比较运算符

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

### 逻辑运算符

```catlang
[
    new a = true && true   ; true (与)
    new b = true || false  ; true (或)
    
    new x = 5
    new complex = (x > 0) && (x < 10)  ; true
    
    return 0
]
```

### 位运算符（通过复合赋值）

```catlang
[
    new x = 0b1010
    x &= 0b1100   ; 按位与，x = 0b1000
    x |= 0b0011   ; 按位或，x = 0b1011
    x ^= 0b1111   ; 按位异或，x = 0b0100
    
    return 0
]
```

## 2.6 类型系统

### 数组类型

```catlang
[
    ; 动态数组
    new arr [i32]
    
    ; 固定大小数组
    new fixed [i32; 5]
    
    ; 多维数组
    new matrix [[i32; 3]; 3]
    
    return 0
]
```

### 指针类型

```catlang
[
    ; 指针声明
    new ptr *i32
    
    ; 解引用（使用 * 前缀）
    new val = *ptr
    
    return 0
]
```

### 内存重解释

使用 `m+` 进行物理位重解释：

```catlang
[
    ; 将整数位模式重解释为浮点数
    new int_rep = 0x3FF0000000000000
    new pi_val = m+f64 int_rep  ; pi_val ≈ 1.0
    
    return 0
]
```

## 2.7 完整示例

```catlang
; 综合示例：计算圆的属性
[
    ; 常量定义
    new pi = 3.14159265359
    
    ; 输入
    new radius = 5.0
    
    ; 计算
    new area = pi * radius * radius
    new circumference = 2 * pi * radius
    
    ; 输出（使用字符串插值）
    print("半径：{radius}")
    print("面积：{area}")
    print("周长：{circumference}")
    
    ; 条件判断
    if (area > 50) [
        print("这是一个大圆")
    ] else [
        print("这是一个小圆")
    ]
    
    return 0
]
```

## 2.8 练习

1. 声明三个变量分别存储你的姓名、年龄和身高
2. 计算并输出一个边长为 5 的正方形的面积和周长
3. 使用字符串插值输出一句自我介绍

<details>
<summary>参考答案</summary>

```catlang
[
    ; 练习 1
    new name = "张三"
    new age = 25
    new height = 1.75
    
    ; 练习 2
    new side = 5
    new area = side * side
    new perimeter = 4 * side
    print("正方形面积：{area}")
    print("正方形周长：{perimeter}")
    
    ; 练习 3
    print("我是{name}，今年{age}岁，身高{height}米")
    
    return 0
]
```
</details>

## 下一步

- [第 03 章：控制流](03_control_flow.md) - 条件语句、循环、模式匹配
- [第 05 章：数据结构](05_data_structures.md) - 结构体、数组深入
