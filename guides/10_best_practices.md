# 第 10 章：最佳实践

本章介绍 CatLang 编程的最佳实践：代码风格、性能优化和常见陷阱。

## 10.1 代码风格

### 命名约定

```catlang
; 变量和函数：使用小写 + 下划线（snake_case）
new user_name = "Alice"
fn calculate_total()
fn get_user_id()

; 结构体：使用大驼峰（PascalCase）
struct UserProfile [
    name: str
    age: i32
]

struct HttpClient [
    timeout: i32
]

; 常量：使用全大写 + 下划线
new MAX_SIZE = 100
new PI = 3.14159265359

; 类型别名：使用大驼峰
new UserId = i32
new Callback = fn() -> void
```

### 代码格式化

```catlang
; 好的格式
fn process_data(input: str, threshold: i32, max_iterations: i32) -> Result [
    new result = validate(input)
    
    if (result.is_valid) [
        return transform(result, threshold)
    ] else [
        return error("无效输入")
    ]
}

; 不好的格式 - 拥挤
fn process_data(input: str,threshold:i32,max_iterations:i32)->Result[new result=validate(input);if(result.is_valid)[return transform(result,threshold)]else[return error("无效输入")]]
```

### 适当的空行

```catlang
; 好的做法 - 使用空行分隔逻辑块
fn process_users(users: [User]) [
    new valid_users = filter_valid(users)
    new sorted = sort_by_name(valid_users)
    
    for (new user in sorted) [
        print("用户：{user.name}")
    ]
    
    print("处理完成，共{len(sorted)}个用户")
}

; 不好的做法 - 没有分隔
fn process_users(users: [User]) [
    new valid_users = filter_valid(users)
    new sorted = sort_by_name(valid_users)
    for (new user in sorted) [
        print("用户：{user.name}")
    ]
    print("处理完成，共{len(sorted)}个用户")
}
```

### 注释规范

```catlang
; 好的注释 - 解释为什么
; 使用快速排序，因为数据量大时性能更好
fn sort_data(data: [i32]) [
    quick_sort(data)
}

; 不好的注释 - 重复代码
; 调用快速排序
fn sort_data(data: [i32]) [
    quick_sort(data)  ; 排序数据
}

; 好的注释 - 说明边界情况
; 注意：当除数为零时返回 0 而不是抛出错误
fn safe_divide(a: i32, b: i32) -> i32 [
    if (b == 0) [
        return 0
    ]
    return a / b
]
```

## 10.2 性能优化

### 避免不必要的复制

```catlang
; 好的做法 - 传递引用（如果支持）
fn process_large_data(data: &LargeData) [
    ; 使用数据但不复制
}

; 不好的做法 - 值传递导致复制
fn process_large_data(data: LargeData) [
    ; 整个结构体被复制
}
```

### 预分配容量

```catlang
; 好的做法 - 预分配容量
new arr [i32; 1000]  ; 预分配 1000 个元素

; 不好的做法 - 动态增长
new arr [i32]  ; 可能需要多次重新分配
for (new i = 0, i < 1000, i += 1) [
    arr.push(i)
]
```

### 使用合适的数据类型

```catlang
; 根据需求选择类型
new small_counter: u8 = 0      ; 0-255 足够
new large_sum: i64 = 0         ; 可能需要大数
new precise_value: fa = 3.14   ; 需要高精度

; 不好的做法 - 过度使用大类型
new counter: i64 = 0           ; 浪费内存
```

### 循环优化

```catlang
; 好的做法 - 减少循环内计算
fn sum_array(arr: [i32]) -> i32 [
    new len = len(arr)  ; 循环外计算长度
    new sum = 0
    
    for (new i = 0, i < len, i += 1) [
        sum = sum + arr[i]
    ]
    
    return sum
}

; 不好的做法 - 循环内重复计算
fn sum_array(arr: [i32]) -> i32 [
    new sum = 0
    for (new i = 0, i < len(arr), i += 1) [  ; 每次都计算长度
        sum = sum + arr[i]
    ]
    return sum
}
```

### 异步并发优化

```catlang
; 好的做法 - 并发执行独立任务
async fn fetch_all_data() [
    new h1 = spawn fetch_users()
    new h2 = spawn fetch_posts()
    new h3 = spawn fetch_comments()
    
    new users = await h1
    new posts = await h2
    new comments = await h3
}

; 不好的做法 - 顺序执行
async fn fetch_all_data() [
    new users = await fetch_users()
    new posts = await fetch_posts()
    new comments = await fetch_comments()
]
```

## 10.3 常见陷阱

### 1. 未初始化变量

```catlang
; 错误：使用未初始化的变量
[
    new x i32
    print("{x}")  ; 错误：x 未初始化
    return 0
]

; 正确：初始化后使用
[
    new x = 0
    print("{x}")
    return 0
]

; 或者在 unsafe 块中明确使用
[
    unsafe close(init) [
        new x i32
        print("{x}")  ; 程序员负责确保安全性
    ]
    return 0
]
```

### 2. 数组越界

```catlang
; 错误：可能越界
[
    new arr = [1, 2, 3]
    unsafe close(bounds) [
        new val = arr[10]  ; 危险！
    ]
    return 0
]

; 正确：检查边界
[
    new arr = [1, 2, 3]
    new index = 10
    
    if (index >= 0 && index < len(arr)) [
        new val = arr[index]
    ] else [
        print("索引越界")
    ]
    return 0
]
```

### 3. 空指针解引用

```catlang
; 错误：可能解引用空指针
[
    new ptr *i32 = null
    unsafe close(null) [
        print("{*ptr}")  ; 危险！
    ]
    return 0
]

; 正确：检查空指针
[
    new ptr *i32 = get_pointer()
    
    if (ptr != null) [
        unsafe close(null) [
            print("{*ptr}")
        ]
    ] else [
        print("空指针")
    ]
    return 0
]
```

### 4. 错误处理遗漏

```catlang
; 错误：忽略可能的错误
[
    new result = risky_operation()  ; 没有错误处理
    print("结果：{result}")
    return 0
]

; 正确：处理错误
[
    try [
        new result = risky_operation()
        print("结果：{result}")
    ] catch (e Any) [
        print("操作失败：{e}")
    ]
    return 0
]
```

### 5. 异步代码中的阻塞

```catlang
; 错误：在异步函数中阻塞
async fn bad_example() [
    sync_wait(something)  ; 阻塞整个事件循环
]

; 正确：使用异步原语
async fn good_example() [
    await something_async()
]
```

### 6. 内存重解释错误

```catlang
; 错误：大小不匹配的重解释
[
    new small = 42 i32
    new large = m+i64 small  ; 可能不是预期的结果
    return 0
]

; 正确：确保大小匹配
[
    new value = 42 i64
    new bits = m+i64 value  ; 大小匹配
    return 0
]
```

## 10.4 错误处理最佳实践

### 使用具体错误类型

```catlang
; 好的做法
struct DatabaseError [
    query: str
    code: i32
    message: str
]

try [
    execute_query(sql)
] catch (e DatabaseError) [
    print("数据库错误：[{e.code}] {e.message}")
    print("查询：{e.query}")
]

; 不好的做法
try [
    execute_query(sql)
] catch (e Any) [
    ; 丢失具体信息
    print("出错了")
]
```

### 提供有意义的错误信息

```catlang
; 好的做法
fn parse_int(s: str) -> i32 [
    if (!is_numeric(s)) [
        throw ParseError {
            input: s,
            position: 0,
            expected: "数字字符串"
        }
    }
}

; 不好的做法
fn parse_int(s: str) -> i32 [
    if (!is_numeric(s)) [
        throw "解析失败"  ; 信息不足
    }
}
```

### 在合适的层级处理错误

```catlang
; 底层：抛出具体错误
fn read_file(path: str) -> str [
    if (!file_exists(path)) [
        throw FileError { path: path, reason: "文件不存在" }
    }
    return read_file_content(path)
}

; 中层：转换错误
fn load_config() -> Config [
    try [
        new content = read_file("config.json")
        return parse_config(content)
    ] catch (e FileError) [
        throw ConfigError { reason: "无法读取配置文件" }
    ]
}

; 高层：向用户展示友好信息
[
    try [
        new config = load_config()
    ] catch (e Any) [
        print("启动失败，请检查配置文件")
    ]
]
```

## 10.5 测试建议

### 单元测试

```catlang
; 假设有一个测试框架
import test_framework as tf

fn add(a: i32, b: i32) -> i32 [
    return a + b
]

tf.test("add positive numbers") [
    new result = add(2, 3)
    tf.assert_equal(result, 5)
]

tf.test("add negative numbers") [
    new result = add(-2, -3)
    tf.assert_equal(result, -5)
]

tf.test("add mixed numbers") [
    new result = add(-2, 3)
    tf.assert_equal(result, 1)
]
```

### 边界测试

```catlang
tf.test("empty array") [
    new arr = []
    new sum = sum_array(arr)
    tf.assert_equal(sum, 0)
]

tf.test("single element") [
    new arr = [42]
    new sum = sum_array(arr)
    tf.assert_equal(sum, 42)
]

tf.test("large array") [
    new arr = create_array(10000)
    new sum = sum_array(arr)
    tf.assert_greater(sum, 0)
]
```

## 10.6 安全检查清单

在提交代码前检查：

- [ ] 所有变量都已初始化
- [ ] 数组访问有边界检查
- [ ] 指针使用前检查空值
- [ ] 错误已适当处理
- [ ] 没有资源泄漏
- [ ] 并发代码没有竞态条件
- [ ] unsafe 块有充分注释
- [ ] 敏感数据已妥善处理

## 10.7 综合示例

### 完整的项目结构

```catlang
; ==========================================
; 文件：main.catlang
; 描述：应用程序入口
; ==========================================

import config_loader as cfg
import database as db
import api_server as api
import logging as log

; 应用配置
struct AppConfig [
    port: i32
    db_url: str
    log_level: str
]

; 初始化日志
fn init_logging(level: str) [
    log.configure(level: level)
    log.info("日志系统已初始化")
]

; 加载配置
fn load_app_config() -> AppConfig [
    try [
        new raw = cfg.load("config.json")
        return AppConfig {
            port: raw.port,
            db_url: raw.db_url,
            log_level: raw.log_level
        }
    ] catch (e Any) [
        log.error("配置加载失败：{e}")
        throw e
    ]
]

; 主程序
[
    try [
        ; 加载配置
        new config = load_app_config()
        
        ; 初始化日志
        init_logging(config.log_level)
        
        ; 连接数据库
        new db_conn = await db.connect(config.db_url)
        log.info("数据库连接成功")
        
        ; 启动 API 服务器
        await api.start(db_conn, config.port)
        
    ] catch (e Any) [
        log.error("应用启动失败：{e}")
        return 1
    ]
    
    return 0
]
```

### 工具模块

```catlang
; ==========================================
; 文件：utils.catlang
; 描述：通用工具函数
; ==========================================

; 字符串修剪
fn trim(s: str) -> str [
    ; 实现
]

; 字符串分割
fn split(s: str, delimiter: str) -> [str] [
    ; 实现
]

; 数组映射
fn map(arr: [T], fn: fn(T) -> U) -> [U] [
    new result [U; len(arr)]
    for (new i = 0, i < len(arr), i += 1) [
        result[i] = fn(arr[i])
    ]
    return result
]

; 数组过滤
fn filter(arr: [T], predicate: fn(T) -> bool) -> [T] [
    new result [T]
    for (new item in arr) [
        if (predicate(item)) [
            result.push(item)
        ]
    ]
    return result
]

; 数组归约
fn reduce(arr: [T], initial: U, fn: fn(U, T) -> U) -> U [
    new acc = initial
    for (new item in arr) [
        acc = fn(acc, item)
    ]
    return acc
]
```

## 10.8 练习

1. 重构以下代码，使其符合最佳实践：

```catlang
; 原始代码
fn p(d)[new r=1;for(new i=1,i<=d,i+=1)[r=r*i];return r]
```

2. 为以下函数添加适当的错误处理：

```catlang
fn divide(a: i32, b: i32) -> i32 [
    return a / b
]
```

3. 优化以下代码的性能：

```catlang
fn find_max(arr: [i32]) -> i32 [
    new max = arr[0]
    for (new i = 0, i < len(arr), i += 1) [
        if (arr[i] > max) [
            max = arr[i]
        ]
    }
    return max
]
```

<details>
<summary>参考答案</summary>

```catlang
; 练习 1：重构
fn factorial(n: i32) -> i32 [
    new result = 1
    
    for (new i = 1, i <= n, i += 1) [
        result = result * i
    ]
    
    return result
]

; 练习 2：添加错误处理
struct DivisionError [
    dividend: i32
    divisor: i32
    reason: str
]

fn divide(a: i32, b: i32) -> i32 [
    if (b == 0) [
        throw DivisionError {
            dividend: a,
            divisor: 0,
            reason: "除数不能为零"
        }
    ]
    return a / b
]

; 练习 3：性能优化
fn find_max(arr: [i32]) -> i32 [
    if (len(arr) == 0) [
        throw "空数组"
    ]
    
    new len = len(arr)  ; 缓存长度
    new max = arr[0]
    
    for (new i = 1, i < len, i += 1) [  ; 从 1 开始
        if (arr[i] > max) [
            max = arr[i]
        ]
    }
    
    return max
]
```
</details>

## 总结

恭喜你完成了 CatLang 教程！你已经学习了：

1. ✅ 基础语法和类型系统
2. ✅ 控制流和函数
3. ✅ 数据结构和内存管理
4. ✅ 错误处理和并发编程
5. ✅ 模块导入和最佳实践

继续探索：
- 阅读 [语法规范](../syntax.txt) 了解完整语法
- 查看 `benchmark/` 和 `test/` 目录中的示例代码
- 开始编写你自己的 CatLang 项目！

祝编程愉快！🐱
