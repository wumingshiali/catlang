# 第 04 章：函数

本章介绍 CatLang 中的函数：定义、参数、返回值以及异步函数。

## 4.1 函数定义

### 基本语法

使用 `fn` 关键字定义函数：

```catlang
; 无参数、无返回值的函数
fn say_hello() [
    print("Hello!")
]

[
    say_hello()
    return 0
]
```

### 带参数的函数

```catlang
; 带参数的函数
fn greet(name: str) [
    print("Hello, {name}!")
]

[
    greet("Alice")
    greet("Bob")
    return 0
]
```

### 多个参数

```catlang
fn add(a: i32, b: i32) [
    new sum = a + b
    print("{a} + {b} = {sum}")
]

[
    add(3, 5)  ; 输出：3 + 5 = 8
    return 0
]
```

## 4.2 返回值

使用 `return` 语句返回值：

### 基本返回值

```catlang
fn add(a: i32, b: i32) -> i32 [
    return a + b
]

[
    new result = add(10, 20)
    print("结果：{result}")  ; 输出：30
    return 0
]
```

### 早期返回

```catlang
fn absolute(x: i32) -> i32 [
    if (x >= 0) [
        return x
    ]
    return -x
]

[
    print("{absolute(-5)}")  ; 输出：5
    print("{absolute(5)}")   ; 输出：5
    return 0
]
```

### 无返回值（隐式 void）

```catlang
fn print_sum(a: i32, b: i32) [
    new sum = a + b
    print("和：{sum}")
    ; 隐式返回，无需 return 语句
]

[
    print_sum(3, 7)
    return 0
]
```

## 4.3 参数传递

### 值传递

CatLang 默认使用值传递：

```catlang
fn modify(x: i32) [
    x = x + 10
    print("函数内：{x}")  ; 输出：15
]

[
    new a = 5
    modify(a)
    print("函数外：{a}")  ; 输出：5（原值不变）
    return 0
]
```

### 多参数示例

```catlang
fn calc_circle(radius: f64) -> f64 [
    new pi = 3.14159265359
    new area = pi * radius * radius
    new circumference = 2 * pi * radius
    
    print("半径：{radius}")
    print("面积：{area}")
    print("周长：{circumference}")
    
    return area
]

[
    new result = calc_circle(5.0)
    return 0
]
```

## 4.4 函数重载

CatLang 不支持传统意义上的函数重载，但可以通过默认参数或不同函数名实现类似功能：

```catlang
; 使用不同函数名
fn greet(name: str) [
    print("Hello, {name}!")
]

fn greet_with_title(title: str, name: str) [
    print("Hello, {title} {name}!")
]

[
    greet("Alice")
    greet_with_title("Mr.", "Smith")
    return 0
]
```

## 4.5 递归函数

### 阶乘

```catlang
fn factorial(n: i32) -> i32 [
    if (n <= 1) [
        return 1
    ]
    return n * factorial(n - 1)
]

[
    print("5! = {factorial(5)}")  ; 输出：120
    return 0
]
```

### 斐波那契数列

```catlang
fn fibonacci(n: i32) -> i32 [
    if (n <= 0) [
        return 0
    ]
    if (n == 1) [
        return 1
    ]
    return fibonacci(n - 1) + fibonacci(n - 2)
]

[
    for (new i = 0, i < 10, i += 1) [
        print("fib({i}) = {fibonacci(i)}")
    ]
    return 0
]
```

## 4.6 异步函数

使用 `async fn` 定义异步函数：

### 基本异步函数

```catlang
async fn fetch_data(id: i32) -> Result [
    ; 模拟异步操作
    await sleep(100)
    print("获取数据：{id}")
    return Result
]

[
    new data = await fetch_data(1)
    return 0
]
```

### 多个异步调用

```catlang
async fn task(name: str, delay: i32) [
    await sleep(delay)
    print("{name} 完成")
]

async fn run_tasks() [
    ; 顺序执行
    await task("任务 A", 100)
    await task("任务 B", 200)
    await task("任务 C", 150)
]

[
    await run_tasks()
    return 0
]
```

## 4.7 函数作为值

### 存储函数结果

```catlang
fn square(x: i32) -> i32 [
    return x * x
]

[
    new func_result = square(5)
    print("结果：{func_result}")  ; 输出：25
    return 0
]
```

## 4.8 综合示例

### 示例 1：数学工具函数

```catlang
; 计算最大值
fn max(a: i32, b: i32) -> i32 [
    if (a > b) [
        return a
    ]
    return b
]

; 计算最小值
fn min(a: i32, b: i32) -> i32 [
    if (a < b) [
        return a
    ]
    return b
]

; 判断素数
fn is_prime(n: i32) -> bool [
    if (n <= 1) [
        return false
    ]
    if (n <= 3) [
        return true
    ]
    if (n % 2 == 0 || n % 3 == 0) [
        return false
    ]
    
    new i = 5
    while (i * i <= n) [
        if (n % i == 0 || n % (i + 2) == 0) [
            return false
        ]
        i = i + 6
    ]
    
    return true
]

[
    print("max(10, 20) = {max(10, 20)}")
    print("min(10, 20) = {min(10, 20)}")
    
    for (new i = 1, i <= 20, i += 1) [
        if (is_prime(i)) [
            print("{i} 是素数")
        ]
    ]
    
    return 0
]
```

### 示例 2：字符串处理函数

```catlang
; 计算字符串长度（伪代码，实际实现依赖内置函数）
fn str_length(s: str) -> i32 [
    ; 这里假设有一个内置的 len 函数
    return len(s)
]

; 连接问候语
fn make_greeting(name: str, time: str) -> str [
    return "{time}, {name}!"
]

[
    new greeting = make_greeting("Alice", "Good morning")
    print(greeting)
    return 0
]
```

### 示例 3：数据结构操作

```catlang
struct Point [
    x: i32
    y: i32
]

fn create_point(x: i32, y: i32) -> Point [
    return Point { x: x, y: y }
]

fn distance_from_origin(p: Point) -> f64 [
    new sq_sum = p.x * p.x + p.y * p.y
    ; 使用平方根函数（假设有内置 sqrt）
    return sqrt(sq_sum)
]

[
    new p = create_point(3, 4)
    new dist = distance_from_origin(p)
    print("距离：{dist}")  ; 输出：5
    return 0
]
```

### 示例 4：异步任务编排

```catlang
async fn download_file(url: str) -> Result [
    print("开始下载：{url}")
    await sleep(500)
    print("下载完成：{url}")
    return Result
]

async fn process_data(data: str) -> Result [
    print("处理数据：{data}")
    await sleep(300)
    print("处理完成")
    return Result
]

async fn main_workflow() [
    ; 下载并处理
    await download_file("https://example.com/file1.txt")
    await process_data("file1.txt")
    
    await download_file("https://example.com/file2.txt")
    await process_data("file2.txt")
]

[
    await main_workflow()
    return 0
]
```

## 4.9 函数最佳实践

### 1. 函数命名

使用有意义的动词短语：

```catlang
; 好的命名
fn calculate_area()
fn get_user_input()
fn validate_email()

; 避免模糊命名
fn do_stuff()      ; 太模糊
fn process()       ; 不够具体
```

### 2. 单一职责

每个函数只做一件事：

```catlang
; 好的示例
fn read_file(path: str) -> str [
    ; 只负责读取文件
]

fn parse_data(data: str) [
    ; 只负责解析数据
]

; 不好的示例
fn read_and_parse_and_save() [
    ; 做了太多事情
]
```

### 3. 参数数量

保持参数数量简洁（最好不超过 3 个）：

```catlang
; 如果参数太多，考虑使用结构体
struct Config [
    host: str
    port: i32
    timeout: i32
    retries: i32
]

fn connect(config: Config) [
    ; 使用结构体组织参数
]
```

## 4.10 练习

1. 编写一个函数 `is_even(n: i32) -> bool` 判断数字是否为偶数
2. 编写一个函数 `power(base: i32, exp: i32) -> i32` 计算幂
3. 编写一个异步函数 `delayed_print(msg: str, delay: i32)` 延迟打印消息

<details>
<summary>参考答案</summary>

```catlang
; 练习 1：判断偶数
fn is_even(n: i32) -> bool [
    return n % 2 == 0
]

; 练习 2：计算幂
fn power(base: i32, exp: i32) -> i32 [
    new result = 1
    for (new i = 0, i < exp, i += 1) [
        result = result * base
    ]
    return result
]

; 练习 3：延迟打印
async fn delayed_print(msg: str, delay: i32) [
    await sleep(delay)
    print(msg)
]

[
    print("是偶数：{is_even(4)}")
    print("2^10 = {power(2, 10)}")
    await delayed_print("延迟消息", 1000)
    return 0
]
```
</details>

## 下一步

- [第 05 章：数据结构](05_data_structures.md) - 结构体、数组、指针
- [第 08 章：并发编程](08_concurrency.md) - async/await 深入、spawn 任务
