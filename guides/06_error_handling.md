# 第 06 章：错误处理

本章介绍 CatLang 的错误处理机制：try/catch、throw 和自定义错误类型。

## 6.1 抛出错误

使用 `throw` 关键字抛出错误：

### 基本用法

```catlang
[
    new error_msg = "发生错误"
    throw error_msg
]
```

### 在函数中抛出

```catlang
fn divide(a: i32, b: i32) -> i32 [
    if (b == 0) [
        throw "除数不能为零"
    ]
    return a / b
]

[
    new result = divide(10, 2)
    print("结果：{result}")
    
    ; 这行不会执行，因为会抛出错误
    new error_result = divide(10, 0)
    
    return 0
]
```

## 6.2 捕获错误

使用 `try/catch` 语句捕获和处理错误：

### 基本 try/catch

```catlang
try [
    new result = divide(10, 0)
    print("结果：{result}")
] catch (e Any) [
    print("捕获到错误：{e}")
]
```

### 类型匹配的 catch

```catlang
struct MathError [
    code: i32
    message: str
]

fn safe_divide(a: i32, b: i32) -> i32 [
    if (b == 0) [
        throw MathError { code: 1, message: "除零错误" }
    ]
    return a / b
]

[
    try [
        new result = safe_divide(10, 0)
        print("结果：{result}")
    ] catch (err MathError) [
        print("数学错误：[{err.code}] {err.message}")
    ] catch (e Any) [
        print("未知错误：{e}")
    ]
    
    return 0
]
```

### 多个 catch 子句

```catlang
struct FileError [
    path: str
    reason: str
]

struct NetworkError [
    url: str
    status: i32
]

fn risky_operation() [
    ; 可能抛出不同类型的错误
    throw FileError { path: "/data.txt", reason: "文件不存在" }
]

[
    try [
        risky_operation()
    ] catch (fe FileError) [
        print("文件错误：{fe.path} - {fe.reason}")
    ] catch (ne NetworkError) [
        print("网络错误：{ne.url} - 状态码 {ne.status}")
    ] catch (e Any) [
        print("其他错误：{e}")
    ]
    
    return 0
]
```

## 6.3 自定义错误类型

### 定义错误结构体

```catlang
; 验证错误
struct ValidationError [
    field: str
    message: str
]

; 解析错误
struct ParseError [
    input: str
    position: i32
    expected: str
]

; 系统错误
struct SystemError [
    code: i32
    message: str
]
```

### 错误类型层次

```catlang
; 基础错误类型
struct AppError [
    code: i32
    message: str
]

; 特定错误类型
struct DatabaseError [
    query: str
    inner: AppError
]

struct HttpError [
    status: i32
    inner: AppError
]
```

## 6.4 错误传播

### 重新抛出错误

```catlang
fn process_data(data: str) [
    try [
        validate(data)
        parse(data)
        save(data)
    ] catch (e ValidationError) [
        print("验证失败：{e.message}")
        throw e  ; 重新抛出
    ] catch (e Any) [
        print("处理失败：{e}")
        throw e
    ]
]
```

### 错误转换

```catlang
fn read_config(path: str) -> Config [
    try [
        new content = read_file(path)
        return parse_config(content)
    ] catch (fe FileError) [
        throw ConfigError {
            code: 1,
            message: "无法读取配置文件：{fe.reason}"
        }
    ] catch (pe ParseError) [
        throw ConfigError {
            code: 2,
            message: "配置格式错误：{pe.expected}"
        }
    ]
]
```

## 6.5 try 表达式

### try 块作为表达式

```catlang
fn get_value() -> i32 [
    new result = try [
        risky_calculation()
    ] catch (e Any) [
        print("计算失败，使用默认值")
        0  ; catch 块的返回值
    ]
    return result
]
```

## 6.6 综合示例

### 示例 1：用户注册验证

```catlang
struct RegistrationError [
    field: str
    code: str
    message: str
]

fn validate_email(email: str) [
    if (len(email) < 5) [
        throw RegistrationError {
            field: "email",
            code: "TOO_SHORT",
            message: "邮箱地址太短"
        }
    ]
    if (!contains(email, "@")) [
        throw RegistrationError {
            field: "email",
            code: "INVALID_FORMAT",
            message: "邮箱格式不正确"
        }
    ]
]

fn validate_password(password: str) [
    if (len(password) < 8) [
        throw RegistrationError {
            field: "password",
            code: "TOO_WEAK",
            message: "密码长度至少为 8 位"
        }
    ]
]

fn register_user(email: str, password: str) [
    try [
        validate_email(email)
        validate_password(password)
        print("注册成功！")
    ] catch (e RegistrationError) [
        print("注册失败：[{e.field}] {e.code} - {e.message}")
    ]
]

[
    register_user("alice@example.com", "secure123")  ; 成功
    register_user("bob", "weak")  ; 失败
    register_user("charlie@example.com", "strongpass")  ; 成功
    
    return 0
]
```

### 示例 2：文件处理

```catlang
struct FileError [
    path: str
    operation: str
    reason: str
]

fn read_file_safe(path: str) -> str [
    try [
        if (!file_exists(path)) [
            throw FileError {
                path: path,
                operation: "read",
                reason: "文件不存在"
            }
        ]
        return read_file(path)
    ] catch (e FileError) [
        print("文件操作失败：{e.path} - {e.reason}")
        throw e
    ]
]

fn process_file(input_path: str, output_path: str) [
    try [
        new content = read_file_safe(input_path)
        new processed = transform(content)
        write_file(output_path, processed)
        print("文件处理完成")
    ] catch (e FileError) [
        print("无法处理文件：{e.path}")
    ] catch (e Any) [
        print("未知错误：{e}")
    ]
]

[
    process_file("input.txt", "output.txt")
    return 0
]
```

### 示例 3：网络请求

```catlang
struct HttpError [
    url: str
    status: i32
    message: str
]

struct TimeoutError [
    url: str
    timeout_ms: i32
]

fn fetch_url(url: str, timeout: i32) -> str [
    if (timeout < 0) [
        throw HttpError {
            url: url,
            status: 400,
            message: "无效的超时时间"
        }
    ]
    
    ; 模拟网络请求
    if (contains(url, "error")) [
        throw HttpError {
            url: url,
            status: 500,
            message: "服务器错误"
        }
    ]
    
    if (timeout < 100) [
        throw TimeoutError {
            url: url,
            timeout_ms: timeout
        }
    ]
    
    return "响应内容"
]

fn fetch_with_retry(url: str, max_retries: i32) -> str [
    new attempt = 0
    
    while (attempt < max_retries) [
        try [
            return fetch_url(url, 1000 * (attempt + 1))
        ] catch (e TimeoutError) [
            attempt = attempt + 1
            print("超时，重试 {attempt}/{max_retries}")
        ] catch (e HttpError) [
            if (e.status >= 500) [
                attempt = attempt + 1
                print("服务器错误，重试 {attempt}/{max_retries}")
            ] else [
                print("客户端错误：{e.message}")
                throw e
            ]
        ] catch (e Any) [
            print("未知错误：{e}")
            throw e
        ]
    ]
    
    throw HttpError {
        url: url,
        status: 503,
        message: "服务不可用"
    }
]

[
    try [
        new content = fetch_with_retry("https://api.example.com/data", 3)
        print("获取成功：{content}")
    ] catch (e Any) [
        print("请求失败：{e}")
    ]
    
    return 0
]
```

### 示例 4：计算器错误处理

```catlang
struct CalcError [
    operation: str
    message: str
]

fn calculate(a: i32, b: i32, op: str) -> i32 [
    switch (op) [
        case "+":
            return a + b
        case "-":
            return a - b
        case "*":
            return a * b
        case "/":
            if (b == 0) [
                throw CalcError {
                    operation: "/",
                    message: "除数不能为零"
                }
            ]
            return a / b
        case "%":
            if (b == 0) [
                throw CalcError {
                    operation: "%",
                    message: "取模的除数不能为零"
                }
            ]
            return a % b
        default:
            throw CalcError {
                operation: op,
                message: "未知运算符"
            }
    ]
]

[
    new operations = [["+", 10, 5], ["-", 10, 5], ["*", 10, 5], ["/", 10, 0], ["%", 10, 0], ["^", 10, 5]]
    
    for (new i = 0, i < 6, i += 1) [
        new op = operations[i][0]
        new a = operations[i][1]
        new b = operations[i][2]
        
        try [
            new result = calculate(a, b, op)
            print("{a} {op} {b} = {result}")
        ] catch (e CalcError) [
            print("计算错误：{a} {op} {b} - {e.message}")
        ]
    ]
    
    return 0
]
```

### 示例 5：数据库操作

```catlang
struct DbError [
    query: str
    code: i32
    message: str
]

struct ConnectionError [
    host: str
    port: i32
]

fn connect_db(host: str, port: i32) [
    if (host == "") [
        throw ConnectionError { host: host, port: port }
    ]
    print("连接到 {host}:{port}")
]

fn execute_query(query: str) [
    if (contains(query, "DROP")) [
        throw DbError {
            query: query,
            code: 403,
            message: "禁止执行 DROP 操作"
        }
    ]
    print("执行查询：{query}")
]

fn run_transaction(queries: [str]) [
    try [
        connect_db("localhost", 5432)
        
        for (new i = 0, i < len(queries), i += 1) [
            execute_query(queries[i])
        ]
        
        print("事务完成")
    ] catch (e ConnectionError) [
        print("连接失败：{e.host}:{e.port}")
        throw e
    ] catch (e DbError) [
        print("数据库错误：[{e.code}] {e.message}")
        throw e
    ] catch (e Any) [
        print("未知错误：{e}")
        throw e
    ]
]

[
    try [
        run_transaction(["SELECT * FROM users", "INSERT INTO logs", "UPDATE stats"])
    ] catch (e Any) [
        print("事务失败")
    ]
    
    return 0
]
```

## 6.7 错误处理最佳实践

### 1. 使用具体的错误类型

```catlang
; 好的做法
struct SpecificError [
    context: str
    message: str
]

try [
    operation()
] catch (e SpecificError) [
    ; 可以访问具体字段
    print("上下文：{e.context}")
]

; 不好的做法
try [
    operation()
] catch (e Any) [
    ; 丢失了具体信息
]
```

### 2. 提供有意义的错误信息

```catlang
; 好的做法
throw ValidationError {
    field: "email",
    message: "邮箱格式不正确，应包含 @"
}

; 不好的做法
throw "错误"
```

### 3. 在合适的层级处理错误

```catlang
; 底层：抛出具体错误
fn parse_int(s: str) -> i32 [
    if (!is_numeric(s)) [
        throw ParseError { input: s, expected: "数字" }
    ]
    ; ...
]

; 中层：转换或传播错误
fn process_input(s: str) [
    try [
        new num = parse_int(s)
        use_number(num)
    ] catch (e ParseError) [
        throw InputError { field: "number", reason: e.message }
    ]
]

; 高层：向用户展示友好信息
[
    try [
        process_input(user_input)
    ] catch (e Any) [
        print("输入无效，请检查后重试")
    ]
]
```

### 4. 不要忽略错误

```catlang
; 不好的做法 - 空的 catch 块
try [
    risky_operation()
] catch (e Any) [
    ; 忽略错误
]

; 好的做法 - 至少记录错误
try [
    risky_operation()
] catch (e Any) [
    print("警告：操作失败 - {e}")
]
```

## 6.8 练习

1. 创建一个 `DivisionError` 类型，包含被除数、除数和原因字段
2. 编写一个函数安全地解析字符串为整数，处理各种错误情况
3. 实现一个简单的状态机，在不同状态下可能抛出不同类型的错误

<details>
<summary>参考答案</summary>

```catlang
; 练习 1：除法错误
struct DivisionError [
    dividend: i32
    divisor: i32
    reason: str
]

fn safe_divide(a: i32, b: i32) -> i32 [
    if (b == 0) [
        throw DivisionError {
            dividend: a,
            divisor: 0,
            reason: "除数不能为零"
        }
    ]
    return a / b
]

[
    try [
        new result = safe_divide(10, 0)
    ] catch (e DivisionError) [
        print("错误：{e.dividend} / {e.divisor} - {e.reason}")
    ]
    return 0
]

; 练习 2：安全解析整数
fn parse_int_safe(s: str) -> i32 [
    if (len(s) == 0) [
        throw ParseError { input: s, position: 0, expected: "非空字符串" }
    ]
    ; 简化实现
    return 42
]

; 练习 3：状态机
struct StateError [
    current_state: str
    expected_state: str
    action: str
]

fn transition(current: str, action: str) -> str [
    if (current == "idle" && action == "start") [
        return "running"
    ]
    if (current == "running" && action == "stop") [
        return "stopped"
    ]
    throw StateError {
        current_state: current,
        expected_state: "unknown",
        action: action
    }
]
```
</details>

## 下一步

- [第 07 章：内存管理](07_memory_management.md) - unsafe 深入、内存操作
- [第 08 章：并发编程](08_concurrency.md) - async/await、spawn 任务
