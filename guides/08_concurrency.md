# Chapter 08: Concurrency

This chapter introduces CatLang's concurrency model: async/await, spawn tasks, and concurrency primitives.

## 8.1 Async Functions

### Define Async Functions

Define async functions using `async fn`:

```catlang
async fn fetch_data(id: i32) -> Result [
    ; Simulate network delay
    await sleep(100)
    print("Fetching data: {id}")
    return Result
]

[
    ; Call async function
    new data = await fetch_data(1)
    return 0
]
```

### Async Function Characteristics

- Can only be called in `async` context
- Can contain `await` expressions
- Returns `Result` or `Future` type

## 8.2 Await Expressions

### Basic Usage

```catlang
async fn task1() [
    await sleep(100)
    print("Task 1 complete")
]

async fn task2() [
    await sleep(200)
    print("Task 2 complete")
]

async fn run_tasks() [
    ; Sequential execution
    await task1()
    await task2()
]

[
    await run_tasks()
    return 0
]
```

### Await Restrictions

`await` can only be used in async contexts:

```catlang
; Error: await in non-async function
fn wrong() [
    await sleep(100)  ; Compile error
]

; Correct
async fn correct() [
    await sleep(100)
]
```

## 8.3 Spawn Tasks

Use `spawn` to start independent concurrent tasks:

### Basic Usage

```catlang
[
    ; Start background task
    new handle = spawn async [
        for (new i = 0, i < 5, i += 1) [
            await sleep(100)
            print("Background task: {i}")
        ]
    ]

    ; Wait for task completion
    await handle

    print("All tasks complete")
    return 0
]
```

### Multiple Concurrent Tasks

```catlang
[
    ; Start multiple concurrent tasks
    new handle1 = spawn async [
        await sleep(100)
        print("Task 1")
    ]

    new handle2 = spawn async [
        await sleep(150)
        print("Task 2")
    ]

    new handle3 = spawn async [
        await sleep(200)
        print("Task 3")
    ]

    ; Wait for all tasks
    await handle1
    await handle2
    await handle3

    return 0
]
```

## 8.4 Async Comprehensive Examples

### Example 1: Concurrent Data Fetching

```catlang
async fn fetch_user(id: i32) -> str [
    await sleep(50)
    return "User{id}"
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
    ; Sequential approach (slower)
    new user = await fetch_user(user_id)
    new posts = await fetch_posts(user)
    new comments = await fetch_comments(posts)

    print("User: {user}, Posts: {posts}, Comments: {comments}")
]

async fn get_user_stats_parallel(user_id: i32) [
    ; Parallel approach (faster)
    new user_future = spawn fetch_user(user_id)
    new posts_future = spawn fetch_posts("User 1")
    new comments_future = spawn fetch_comments(10)

    new user = await user_future
    new posts = await posts_future
    new comments = await comments_future

    print("User: {user}, Posts: {posts}, Comments: {comments}")
]

[
    await get_user_stats(1)
    await get_user_stats_parallel(1)
    return 0
]
```

### Example 2: Producer-Consumer Pattern

```catlang
struct Channel [
    buffer: [i32; 10]
    head: i32
    tail: i32
    count: i32
]

async fn producer(channel: Channel, count: i32) [
    for (new i = 0, i < count, i += 1) [
        ; Produce data
        channel.buffer[channel.tail] = i
        channel.tail = (channel.tail + 1) % 10
        channel.count = channel.count + 1

        print("Produce: {i}")
        await sleep(50)
    ]
}

async fn consumer(channel: Channel) [
    for (new i = 0, i < 5, i += 1) [
        while (channel.count == 0) [
            await sleep(10)
        ]

        ; Consume data
        new value = channel.buffer[channel.head]
        channel.head = (channel.head + 1) % 10
        channel.count = channel.count - 1

        print("Consume: {value}")
        await sleep(30)
    ]
]

[
    new ch = Channel { head: 0, tail: 0, count: 0 }

    new prod_handle = spawn producer(ch, 5)
    new cons_handle = spawn consumer(ch)

    await prod_handle
    await cons_handle

    return 0
]
```

### Example 3: Timeout Handling

```catlang
async fn slow_operation() -> str [
    await sleep(500)
    return "Complete"
]

async fn with_timeout(operation: async, timeout_ms: i32) -> str [
    new op_handle = spawn operation

    new timeout_handle = spawn async [
        await sleep(timeout_ms)
        throw "Timeout"
    ]

    ; Wait for either to complete
    ; Note: This is pseudo-code, actual select/race semantics needed
    return await op_handle
]

[
    try [
        new result = await with_timeout(slow_operation(), 1000)
        print("Result: {result}")
    ] catch (e Any) [
        print("Error: {e}")
    ]

    return 0
]
```

### Example 4: Parallel Computing

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

    ; Start multiple compute tasks
    for (new i = 0, i < chunks, i += 1) [
        new start = i * chunk_size
        new end = (i + 1) * chunk_size
        new handle = spawn compute_chunk(start, end)
        handles[i] = handle
    ]

    ; Collect results
    new total_sum = 0
    for (new i = 0, i < chunks, i += 1) [
        new partial = await handles[i]
        total_sum = total_sum + partial
    ]

    return total_sum
]

[
    new result = await parallel_sum(1000, 4)
    print("Sum of squares: {result}")
    return 0
]
```

### Example 5: Async Event Loop

```catlang
struct Event [
    type: str
    data: str
]

async fn event_listener(queue: Channel) [
    ; Simulate event listening
    for (new i = 0, i < 5, i += 1) [
        await sleep(100)
        new event = Event { type: "click", data: "Button{i}" }
        queue.send(event)
    ]
}

async fn event_handler(queue: Channel) [
    while (true) [
        new event = await queue.receive()
        print("Handle event: {event.type} - {event.data}")

        if (event.data == "Button 4") [
            break
        ]
    ]
]

[
    new queue = Channel { }

    new listener_handle = spawn event_listener(queue)
    new handler_handle = spawn event_handler(queue)

    await listener_handle
    await handler_handle

    return 0
]
```

## 8.5 Concurrency Primitives

### Mutex (Pseudo-code)

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
    print("Thread{id} enters critical section")
    await sleep(50)
    print("Thread{id} leaves critical section")
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

### Semaphore (Pseudo-code)

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

### Read-Write Lock (Pseudo-code)

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

## 8.6 Async Best Practices

### 1. Avoid Blocking

```catlang
; Bad practice - use sync wait in async code
async fn bad() [
    ; Blocks entire event loop
    sync_wait(something)
]

; Good practice - use async primitives
async fn good() [
    await something_async()
]
```

### 2. Use Concurrency Appropriately

```catlang
; No concurrency needed (sequential is fine)
async fn sequential() [
    new a = await fetch_a()
    new b = await fetch_b(a)  ; Depends on a
    new c = await fetch_c(b)  ; Depends on b
    return c
]

; Concurrency needed
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

### 3. Error Propagation

```catlang
async fn safe_operation() -> Result [
    try [
        new data = await fetch_data()
        new processed = await process(data)
        return processed
    ] catch (e Any) [
        print("Operation failed: {e}")
        throw e
    ]
]
```

### 4. Resource Cleanup

```catlang
async fn with_resource() [
    new resource = await acquire_resource()

    try [
        await use_resource(resource)
    ] catch (e Any) [
        print("Resource usage failed: {e}")
    ]

    ; Ensure cleanup
    await release_resource(resource)
]
```

## 8.7 Exercises

1. Create two concurrent tasks, one printing odd numbers and one printing even numbers, each printing 5 times
2. Implement an async function that simulates fetching data from multiple APIs in parallel and merging results
3. Use spawn to create a background monitoring task that periodically checks system status

<details>
<summary>Reference Answers</summary>

```catlang
; Exercise 1: Odd/Even printing
async fn print_odd() [
    for (new i = 1, i < 10, i += 2) [
        print("Odd: {i}")
        await sleep(50)
    ]
]

async fn print_even() [
    for (new i = 2, i <= 10, i += 2) [
        print("Even: {i}")
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

; Exercise 2: Parallel API calls
async fn fetch_api1() -> str [
    await sleep(100)
    return "Data 1"
]

async fn fetch_api2() -> str [
    await sleep(150)
    return "Data 2"
]

async fn fetch_api3() -> str [
    await sleep(80)
    return "Data 3"
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

; Exercise 3: Background monitoring
async fn monitor_system() [
    for (new i = 0, i < 5, i += 1) [
        await sleep(1000)
        print("System check {i}: CPU normal, Memory normal")
    ]
]

[
    new monitor_handle = spawn monitor_system()

    ; Main program continues other work
    await sleep(3000)
    print("Main program work complete")

    await monitor_handle
    return 0
]
```
</details>

## Next Steps

- [Chapter 09: Modules & Imports](09_modules_imports.md) - Third-party library imports
- [Chapter 10: Best Practices](10_best_practices.md) - Code style, performance tips
