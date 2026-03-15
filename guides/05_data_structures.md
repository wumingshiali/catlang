# 第 05 章：数据结构

本章介绍 CatLang 的数据结构：结构体、数组、指针和内存操作。

## 5.1 结构体 (struct)

### 定义结构体

使用 `struct` 关键字定义结构体：

```catlang
struct Person [
    name: str
    age: i32
    email: str
]
```

### 创建实例

```catlang
struct Point [
    x: i32
    y: i32
]

[
    new p = Point { x: 10, y: 20 }
    return 0
]
```

### 访问字段

```catlang
struct Rectangle [
    width: f64
    height: f64
]

[
    new rect = Rectangle { width: 10.5, height: 5.0 }
    
    ; 访问字段
    new w = rect.width
    new h = rect.height
    
    ; 修改字段
    rect.width = 15.0
    rect.height = 8.0
    
    return 0
]
```

## 5.2 结构体方法 (impl)

使用 `impl` 块为结构体定义方法：

```catlang
struct Circle [
    radius: f64
]

impl Circle [
    fn area(self: Circle) -> f64 [
        new pi = 3.14159265359
        return pi * self.radius * self.radius
    ]
    
    fn circumference(self: Circle) -> f64 [
        new pi = 3.14159265359
        return 2 * pi * self.radius
    ]
    
    fn scale(self: Circle, factor: f64) -> Circle [
        return Circle { radius: self.radius * factor }
    ]
]

[
    new c = Circle { radius: 5.0 }
    new a = c.area()
    new c2 = c.scale(2.0)
    
    print("面积：{a}")
    print("放大后半径：{c2.radius}")
    
    return 0
]
```

## 5.3 数组

### 动态数组

```catlang
[
    ; 声明动态数组
    new arr [i32]
    
    ; 声明并初始化
    new nums = [1, 2, 3, 4, 5]
    
    ; 访问元素
    new first = nums[0]
    new second = nums[1]
    
    ; 修改元素
    nums[0] = 10
    
    return 0
]
```

### 固定大小数组

```catlang
[
    ; 固定大小数组
    new fixed [i32; 5]
    
    ; 初始化
    fixed[0] = 1
    fixed[1] = 2
    fixed[2] = 3
    fixed[3] = 4
    fixed[4] = 5
    
    ; 或者使用字面量
    new colors = ["red", "green", "blue"]
    
    return 0
]
```

### 多维数组

```catlang
[
    ; 3x3 矩阵
    new matrix [[i32; 3]; 3]
    
    ; 初始化
    matrix[0][0] = 1
    matrix[0][1] = 2
    matrix[0][2] = 3
    matrix[1][0] = 4
    ; ...
    
    ; 访问
    new val = matrix[1][2]
    
    return 0
]
```

### 数组遍历

```catlang
[
    new arr = [10, 20, 30, 40, 50]
    new sum = 0
    
    for (new i = 0, i < 5, i += 1) [
        sum = sum + arr[i]
    ]
    
    print("数组和：{sum}")  ; 输出：150
    
    return 0
]
```

## 5.4 指针

### 指针声明

```catlang
[
    ; 声明指针
    new ptr *i32
    
    ; 声明并初始化
    new x = 42
    new ptr2 = &x  ; 假设 & 是取地址符
    
    return 0
]
```

### 解引用

```catlang
[
    new value = 100
    new ptr *i32 = &value
    
    ; 解引用访问
    new deref = *ptr
    
    print("值：{dereF}")  ; 输出：100
    
    return 0
]
```

### 指针运算（unsafe）

```catlang
[
    new arr = [10, 20, 30, 40, 50]
    new ptr *i32 = &arr[0]
    
    unsafe close(bounds) [
        ; 指针算术（谨慎使用）
        new val1 = *ptr
        ; ptr = ptr + 1  ; 移动到下一个元素
        ; new val2 = *ptr
    ]
    
    return 0
]
```

## 5.5 内存重解释

使用 `m+` 进行物理位重解释：

### 整数到浮点数

```catlang
[
    ; IEEE 754 双精度表示
    new int_rep = 0x3FF0000000000000
    new pi_val = m+f64 int_rep
    
    print("重解释后的值：{pi_val}")  ; 约等于 1.0
    
    return 0
]
```

### 浮点数到整数

```catlang
[
    new float_val = 2.0
    new int_val = m+i64 float_val
    
    print("位模式：{int_val}")
    
    return 0
]
```

## 5.6 内存复制 (cpy)

使用 `cpy` 进行内存复制：

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
]

[
    new src = Data { a: 10, b: 20 }
    new dst Data
    
    cpy dst Data(src)
    
    print("dst.a = {dst.a}, dst.b = {dst.b}")
    
    return 0
]
```

## 5.7 Unsafe 块

### 关闭安全检查

```catlang
[
    new arr = [1, 2, 3]
    
    unsafe close(bounds) [
        ; 关闭边界检查
        new val = arr[100]  ; 不会触发边界检查错误
    ]
    
    return 0
]
```

### 关闭初始化检查

```catlang
[
    unsafe close(init) [
        new uninitialized i32
        ; 使用未初始化的变量（危险！）
        print("{uninitialized}")
    ]
    
    return 0
]
```

### 完全关闭安全检查

```catlang
[
    unsafe all [
        ; 关闭所有安全检查
        new raw = m+*i32 0x1000
        print("{*raw}")
    ]
    
    return 0
]
```

### 保持特定检查

```catlang
[
    unsafe keep(lifetime) [
        ; 只保持生命周期检查，关闭其他
        ; 进行底层操作
    ]
    
    return 0
]
```

## 5.8 特殊类型

### 任意长度整数 (ia)

```catlang
[
    new big_num ia = 999999999999999999
    new result = big_num * 2
    
    print("大数计算：{result}")
    
    return 0
]
```

### 任意长度浮点 (fa)

```catlang
[
    new precise_pi fa = 3.141592653589793238462643383279502884197
    new area = precise_pi * 100 * 100
    
    print("精确面积：{area}")
    
    return 0
]
```

### 任意长度字符串 (sa)

```catlang
[
    new long_text sa = "这是一段非常长的文本，可以包含任意数量的字符..."
    
    print("长度：{len(long_text)}")
    
    return 0
]
```

### 定时器 (timer)

```catlang
[
    new t timer
    
    ; 启动定时器
    t.start()
    
    ; 执行一些操作
    new result = expensive_computation()
    
    ; 获取经过时间
    new elapsed = t.elapsed()
    print("耗时：{elapsed}ms")
    
    return 0
]
```

## 5.9 综合示例

### 示例 1：学生管理系统

```catlang
struct Student [
    id: i32
    name: str
    grade: f64
]

impl Student [
    fn is_passing(self: Student) -> bool [
        return self.grade >= 60.0
    ]
    
    fn get_letter_grade(self: Student) -> str [
        if (self.grade >= 90) [
            return "A"
        ] else if (self.grade >= 80) [
            return "B"
        ] else if (self.grade >= 70) [
            return "C"
        ] else if (self.grade >= 60) [
            return "D"
        ] else [
            return "F"
        ]
    ]
]

[
    new s1 = Student { id: 1, name: "Alice", grade: 95.5 }
    new s2 = Student { id: 2, name: "Bob", grade: 72.0 }
    new s3 = Student { id: 3, name: "Charlie", grade: 58.0 }
    
    print("{s1.name} 的成绩：{s1.get_letter_grade()}")
    print("{s2.name} 的成绩：{s2.get_letter_grade()}")
    print("{s3.name} 的成绩：{s3.get_letter_grade()}")
    
    if (s1.is_passing()) [
        print("{s1.name} 及格")
    ]
    if (s3.is_passing()) [
        print("{s3.name} 及格")
    ] else [
        print("{s3.name} 不及格")
    ]
    
    return 0
]
```

### 示例 2：向量运算

```catlang
struct Vector3 [
    x: f64
    y: f64
    z: f64
]

impl Vector3 [
    fn magnitude(self: Vector3) -> f64 [
        return sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    ]
    
    fn normalize(self: Vector3) -> Vector3 [
        new mag = self.magnitude()
        return Vector3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag
        }
    ]
    
    fn dot(self: Vector3, other: Vector3) -> f64 [
        return self.x * other.x + self.y * other.y + self.z * other.z
    ]
]

fn add_vectors(a: Vector3, b: Vector3) -> Vector3 [
    return Vector3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z
    }
]

[
    new v1 = Vector3 { x: 1.0, y: 2.0, z: 3.0 }
    new v2 = Vector3 { x: 4.0, y: 5.0, z: 6.0 }
    
    new sum = add_vectors(v1, v2)
    new dot_product = v1.dot(v2)
    new mag = v1.magnitude()
    
    print("向量和：({sum.x}, {sum.y}, {sum.z})")
    print("点积：{dot_product}")
    print("模长：{mag}")
    
    return 0
]
```

### 示例 3：动态数组操作

```catlang
; 注意：以下是伪代码，实际实现依赖标准库
[
    ; 创建动态数组
    new nums = [1, 2, 3, 4, 5]
    
    ; 添加元素
    nums.push(6)
    nums.push(7)
    
    ; 删除元素
    nums.pop()
    
    ; 获取长度
    new len = len(nums)
    
    ; 遍历
    for (new i = 0, i < len, i += 1) [
        print("nums[{i}] = {nums[i]}")
    ]
    
    return 0
]
```

### 示例 4：链表节点

```catlang
struct ListNode [
    value: i32
    next: *ListNode
]

fn create_node(value: i32) -> ListNode [
    return ListNode { value: value, next: null }
]

[
    ; 创建链表节点
    new node1 = create_node(10)
    new node2 = create_node(20)
    new node3 = create_node(30)
    
    ; 链接节点
    node1.next = &node2
    node2.next = &node3
    
    ; 遍历链表
    new current = &node1
    while (current != null) [
        print("{current.value}")
        current = current.next
    ]
    
    return 0
]
```

## 5.10 练习

1. 定义一个 `Book` 结构体，包含书名、作者、价格字段，并实现一个计算折扣价的方法
2. 创建一个 3x3 矩阵并实现矩阵转置函数
3. 使用 unsafe 块进行内存重解释，将浮点数 1.0 的位模式打印为整数

<details>
<summary>参考答案</summary>

```catlang
; 练习 1：Book 结构体
struct Book [
    title: str
    author: str
    price: f64
]

impl Book [
    fn discounted_price(self: Book, discount: f64) -> f64 [
        return self.price * (1 - discount)
    ]
]

[
    new book = Book { title: "CatLang 编程", author: "张三", price: 59.99 }
    new sale_price = book.discounted_price(0.2)
    print("折扣价：{sale_price}")
    return 0
]

; 练习 2：矩阵转置
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] [
    new result [[i32; 3]; 3]
    for (new i = 0, i < 3, i += 1) [
        for (new j = 0, j < 3, j += 1) [
            result[i][j] = matrix[j][i]
        ]
    ]
    return result
]

; 练习 3：内存重解释
[
    new float_val = 1.0
    new int_bits = m+i64 float_val
    print("1.0 的位模式：{int_bits}")
    return 0
]
```
</details>

## 下一步

- [第 06 章：错误处理](06_error_handling.md) - try/catch、throw
- [第 07 章：内存管理](07_memory_management.md) - unsafe 深入、内存操作
