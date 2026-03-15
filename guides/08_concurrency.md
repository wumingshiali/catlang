# 第 08 章：并发编程

本章介绍 CatLang 的并发编程模型：async/await、spawn 任务和并发原语。

## 8.1 异步函数

### 定义异步函数

使用 `async fn` 定义异步函数：

```catlang
async fn fetch_data(id: i32) -> Result [
    ; 模拟网络延迟
    await sleep(100)
    print("获取数据：{id}")
    return Result
]

[
    ; 调用异步函数
    new data = await fetch_data(1)
    return 0
]
```

### 异步函数特点

- 只能在 `async` 上下文中调用
- 可以包含 `await` 表达式
- 返回 `Result` 或 `Future` 类型

## 8.2 Await 表达式

### 基本用法

```catlang
async fn task1() [
    await sleep(100)
    print("任务 1 完成")
]

async fn task2() [
    await sleep(200)
    print("任务 2 完成")
]

async fn run_tasks() [
    ; 顺序执行
    await task1()
    await task2()
]

[
    await run_tasks()
    return 0
]
```

### Await 限制

`await` 只能在异步上下文中使用：

```catlang
; 错误：await 在非异步函数中
fn wrong() [
    await sleep(100)  ; 编译错误
]

; 正确
async fn correct() [
    await sleep(100)
]
```

## 8.3 Spawn 任务

使用 `spawn` 启动独立并发任务：

### 基本用法

```catlang
[
    ; 启动后台任务
    new handle = spawn async [
        for (new i = 0, i < 5, i += 1) [
            await sleep(100)
            print("后台任务：{i}")
        ]
    ]
    
    ; 等待任务完成
    await handle
    
    print("所有任务完成")
    return 0
]
```

### 多个并发任务

```catlang
[
    ; 启动多个并发任务
    new handle1 = spawn async [
        await sleep(100)
        print("任务 1")
    ]
    
    new handle2 = spawn async [
        await sleep(150)
        print("任务 2")
    ]
    
    new handle3 = spawn async [
        await sleep(200)
        print("任务 3")
    ]
    
    ; 等待所有任务
    await handle1
    await handle2
    await handle3
    
    return 0
]
```

## 8.4 异步综合示例

### 示例 1：并发数据获取

```catlang
async fn fetch_user(id: i32) -> str [
    await sleep(50)
    return "用户{id}"
]

async fn fetch_posts(user: str) -> i32 [
    await sleep(30)
    return 10
]

async fn fetch_comments(post_id: i32) -> i32 [
    await sleep(20)
    return 5
]

async fn get_user_stats(user_id: i32) [
    ; 顺序方式（较慢）
    new user = await fetch_user(user_id)
    new posts = await fetch_posts(user)
    new comments = await fetch_comments(posts)
    
    print("用户：{user}, 帖子：{posts}, 评论：{comments}")
]

async fn get_user_stats_parallel(user_id: i32) [
    ; 并发方式（较快）
    new user_future = spawn fetch_user(user_id)
    new posts_future = spawn fetch_posts("用户 1")
    new comments_future = spawn fetch_comments(10)
    
    new user = await user_future
    new posts = await posts_future
    new comments = await comments_future
    
    print("用户：{user}, 帖子：{posts}, 评论：{comments}")
]

[
    await get_user_stats(1)
    await get_user_stats_parallel(1)
    return 0
]
```

### 示例 2：生产者 - 消费者模式

```catlang
struct Channel [
    buffer: [i32; 10]
    head: i32
    tail: i32
    count: i32
]

async fn producer(channel: Channel, count: i32) [
    for (new i = 0, i < count, i += 1) [
        ; 生产数据
        channel.buffer[channel.tail] = i
        channel.tail = (channel.tail + 1) % 10
        channel.count = channel.count + 1
        
        print("生产：{i}")
        await sleep(50)
    ]
}

async fn consumer(channel: Channel) [
    for (new i = 0, i < 5, i += 1) [
        while (channel.count == 0) [
            await sleep(10)
        ]
        
        ; 消费数据
        new value = channel.buffer[channel.head]
        channel.head = (channel.head + 1) % 10
        channel.count = channel.count - 1
        
        print("消费：{value}")
        await sleep(30)
    ]
}

[
    new ch = Channel { head: 0, tail: 0, count: 0 }
    
    new prod_handle = spawn producer(ch, 5)
    new cons_handle = spawn consumer(ch)
    
    await prod_handle
    await cons_handle
    
    return 0
]
```

### 示例 3：超时处理

```catlang
async fn slow_operation() -> str [
    await sleep(500)
    return "完成"
]

async fn with_timeout(operation: async, timeout_ms: i32) -> str [
    new op_handle = spawn operation
    
    new timeout_handle = spawn async [
        await sleep(timeout_ms)
        throw "超时"
    ]
    
    ; 等待任一完成
    ; 注意：这是伪代码，实际需要 select/race 语义
    return await op_handle
]

[
    try [
        new result = await with_timeout(slow_operation(), 1000)
        print("结果：{result}")
    ] catch (e Any) [
        print("错误：{e}")
    ]
    
    return 0
]
```

### 示例 4：并行计算

```catlang
async fn compute_chunk(start: i32, end: i32) -> i32 [
    new sum = 0
    for (new i = start, i < end, i += 1) [
        sum = sum + i * i
    ]
    return sum
]

async fn parallel_sum(total: i32, chunks: i32) -> i32 [
    new chunk_size = total / chunks
    new handles [async]
    
    ; 启动多个计算任务
    for (new i = 0, i < chunks, i += 1) [
        new start = i * chunk_size
        new end = (i + 1) * chunk_size
        new handle = spawn compute_chunk(start, end)
        handles[i] = handle
    ]
    
    ; 收集结果
    new total_sum = 0
    for (new i = 0, i < chunks, i += 1) [
        new partial = await handles[i]
        total_sum = total_sum + partial
    ]
    
    return total_sum
]

[
    new result = await parallel_sum(1000, 4)
    print("平方和：{result}")
    return 0
]
```

### 示例 5：异步事件循环

```catlang
struct Event [
    type: str
    data: str
]

async fn event_listener(queue: Channel) [
    ; 模拟事件监听
    for (new i = 0, i < 5, i += 1) [
        await sleep(100)
        new event = Event { type: "click", data: "按钮{i}" }
        queue.send(event)
    ]
}

async fn event_handler(queue: Channel) [
    while (true) [
        new event = await queue.receive()
        print("处理事件：{event.type} - {event.data}")
        
        if (event.data == "按钮 4") [
            break
        ]
    ]
}

[
    new queue = Channel { }
    
    new listener_handle = spawn event_listener(queue)
    new handler_handle = spawn event_handler(queue)
    
    await listener_handle
    await handler_handle
    
    return 0
]
```

## 8.5 并发原语

### 互斥锁（伪代码）

```catlang
struct Mutex [
    locked: bool
]

impl Mutex [
    fn lock(self: Mutex) [
        while (self.locked) [
            await sleep(1)
        ]
        self.locked = true
    ]
    
    fn unlock(self: Mutex) [
        self.locked = false
    ]
]

async fn critical_section(mutex: Mutex, id: i32) [
    mutex.lock()
    print("线程{id}进入临界区")
    await sleep(50)
    print("线程{id}离开临界区")
    mutex.unlock()
]

[
    new mutex = Mutex { locked: false }
    
    new h1 = spawn critical_section(mutex, 1)
    new h2 = spawn critical_section(mutex, 2)
    new h3 = spawn critical_section(mutex, 3)
    
    await h1
    await h2
    await h3
    
    return 0
]
```

### 信号量（伪代码）

```catlang
struct Semaphore [
    count: i32
]

impl Semaphore [
    fn acquire(self: Semaphore) [
        while (self.count <= 0) [
            await sleep(1)
        ]
        self.count = self.count - 1
    ]
    
    fn release(self: Semaphore) [
        self.count = self.count + 1
    ]
]
```

### 读写锁（伪代码）

```catlang
struct RwLock [
    readers: i32
    writer: bool
]

impl RwLock [
    fn read_lock(self: RwLock) [
        while (self.writer) [
            await sleep(1)
        ]
        self.readers = self.readers + 1
    ]
    
    fn read_unlock(self: RwLock) [
        self.readers = self.readers - 1
    ]
    
    fn write_lock(self: RwLock) [
        while (self.writer || self.readers > 0) [
            await sleep(1)
        ]
        self.writer = true
    ]
    
    fn write_unlock(self: RwLock) [
        self.writer = false
    ]
]
```

## 8.6 异步最佳实践

### 1. 避免阻塞

```catlang
; 不好的做法 - 在异步代码中使用同步等待
async fn bad() [
    ; 阻塞整个事件循环
    sync_wait(something)
]

; 好的做法 - 使用异步原语
async fn good() [
    await something_async()
]
```

### 2. 合理使用并发

```catlang
; 不需要并发的情况（顺序执行即可）
async fn sequential() [
    new a = await fetch_a()
    new b = await fetch_b(a)  ; 依赖 a
    new c = await fetch_c(b)  ; 依赖 b
    return c
]

; 需要并发的情况
async fn parallel() [
    new h1 = spawn fetch_x()
    new h2 = spawn fetch_y()
    new h3 = spawn fetch_z()
    
    new x = await h1
    new y = await h2
    new z = await h3
    
    return combine(x, y, z)
]
```

### 3. 错误传播

```catlang
async fn safe_operation() -> Result [
    try [
        new data = await fetch_data()
        new processed = await process(data)
        return processed
    ] catch (e Any) [
        print("操作失败：{e}")
        throw e
    ]
]
```

### 4. 资源清理

```catlang
async fn with_resource() [
    new resource = await acquire_resource()
    
    try [
        await use_resource(resource)
    ] catch (e Any) [
        print("使用资源失败：{e}")
    ]
    
    ; 确保清理
    await release_resource(resource)
]
```

## 8.7 练习

1. 创建两个并发任务，一个打印奇数，一个打印偶数，各打印 5 次
2. 实现一个异步函数，模拟从多个 API 并行获取数据并合并结果
3. 使用 spawn 创建一个后台监控任务，定期检查系统状态

<details>
<summary>参考答案</summary>

```catlang
; 练习 1：奇偶打印
async fn print_odd() [
    for (new i = 1, i < 10, i += 2) [
        print("奇数：{i}")
        await sleep(50)
    ]
]

async fn print_even() [
    for (new i = 2, i <= 10, i += 2) [
        print("偶数：{i}")
        await sleep(50)
    ]
]

[
    new h1 = spawn print_odd()
    new h2 = spawn print_even()
    
    await h1
    await h2
    return 0
]

; 练习 2：并行 API 调用
async fn fetch_api1() -> str [
    await sleep(100)
    return "数据 1"
]

async fn fetch_api2() -> str [
    await sleep(150)
    return "数据 2"
]

async fn fetch_api3() -> str [
    await sleep(80)
    return "数据 3"
]

async fn merge_data() -> str [
    new h1 = spawn fetch_api1()
    new h2 = spawn fetch_api2()
    new h3 = spawn fetch_api3()
    
    new d1 = await h1
    new d2 = await h2
    new d3 = await h3
    
    return "{d1} + {d2} + {d3}"
]

; 练习 3：后台监控
async fn monitor_system() [
    for (new i = 0, i < 5, i += 1) [
        await sleep(1000)
        print("系统检查 {i}: CPU 正常，内存正常")
    ]
]

[
    new monitor_handle = spawn monitor_system()
    
    ; 主程序继续其他工作
    await sleep(3000)
    print("主程序工作完成")
    
    await monitor_handle
    return 0
]
```
</details>

## 下一步

- [第 09 章：模块与导入](09_modules_imports.md) - 第三方库导入
- [第 10 章：最佳实践](10_best_practices.md) - 代码风格、性能提示
