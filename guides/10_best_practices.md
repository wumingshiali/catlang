# Chapter 10: Best Practices

This chapter introduces CatLang programming best practices: code style, performance optimization, and common pitfalls.

## 10.1 Code Style

### Naming Conventions

```catlang
; Variables and functions: use lowercase with underscores (snake_case)
new user_name = "Alice"
fn calculate_total()
fn get_user_id()

; Structs: use PascalCase
struct UserProfile [
    name: str
    age: i32
]

struct HttpClient [
    timeout: i32
]

; Constants: use all uppercase with underscores
new MAX_SIZE = 100
new PI = 3.14159265359

; Type aliases: use PascalCase
new UserId = i32
new Callback = fn() -> void
```

### Code Formatting

```catlang
; Good formatting
fn process_data(input: str, threshold: i32, max_iterations: i32) -> Result [
    new result = validate(input)

    if (result.is_valid) [
        return transform(result, threshold)
    ] else [
        return error("Invalid input")
    ]
]

; Bad formatting - cramped
fn process_data(input: str,threshold:i32,max_iterations:i32)->Result[new result=validate(input);if(result.is_valid)[return transform(result,threshold)]else[return error("Invalid input")]]
```

### Appropriate Blank Lines

```catlang
; Good practice - use blank lines to separate logical blocks
fn process_users(users: [User]) [
    new valid_users = filter_valid(users)
    new sorted = sort_by_name(valid_users)

    for (new user in sorted) [
        print("User: {user.name}")
    ]

    print("Processing complete, total {len(sorted)} users")
]

; Bad practice - no separation
fn process_users(users: [User]) [
    new valid_users = filter_valid(users)
    new sorted = sort_by_name(valid_users)
    for (new user in sorted) [
        print("User: {user.name}")
    ]
    print("Processing complete, total {len(sorted)} users")
]
```

### Comment Guidelines

```catlang
; Good comments - explain why
; Use quicksort because it performs better with large datasets
fn sort_data(data: [i32]) [
    quick_sort(data)
]

; Bad comments - repeat code
; Call quicksort
fn sort_data(data: [i32]) [
    quick_sort(data)  ; Sort data
]

; Good comments - explain edge cases
; Note: returns 0 instead of throwing error when divisor is zero
fn safe_divide(a: i32, b: i32) -> i32 [
    if (b == 0) [
        return 0
    ]
    return a / b
]
```

## 10.2 Performance Optimization

### Avoid Unnecessary Copies

```catlang
; Good practice - pass by reference (if supported)
fn process_large_data(data: &LargeData) [
    ; Use data without copying
]

; Bad practice - value passing causes copying
fn process_large_data(data: LargeData) [
    ; Entire struct is copied
]
```

### Pre-allocate Capacity

```catlang
; Good practice - pre-allocate capacity
new arr [i32; 1000]  ; Pre-allocate 1000 elements

; Bad practice - dynamic growth
new arr [i32]  ; May require multiple reallocations
for (new i = 0, i < 1000, i += 1) [
    arr.push(i)
]
```

### Use Appropriate Data Types

```catlang
; Choose types based on requirements
new small_counter: u8 = 0      ; 0-255 is enough
new large_sum: i64 = 0         ; May need large numbers
new precise_value: fa = 3.14   ; Need high precision

; Bad practice - overusing large types
new counter: i64 = 0           ; Wastes memory
```

### Loop Optimization

```catlang
; Good practice - reduce calculations inside loop
fn sum_array(arr: [i32]) -> i32 [
    new len = len(arr)  ; Calculate length outside loop
    new sum = 0

    for (new i = 0, i < len, i += 1) [
        sum = sum + arr[i]
    ]

    return sum
]

; Bad practice - repeated calculations inside loop
fn sum_array(arr: [i32]) -> i32 [
    new sum = 0
    for (new i = 0, i < len(arr), i += 1) [  ; Calculate length every time
        sum = sum + arr[i]
    ]
    return sum
]
```

### Async Concurrency Optimization

```catlang
; Good practice - execute independent tasks concurrently
async fn fetch_all_data() [
    new h1 = spawn fetch_users()
    new h2 = spawn fetch_posts()
    new h3 = spawn fetch_comments()

    new users = await h1
    new posts = await h2
    new comments = await h3
]

; Bad practice - sequential execution
async fn fetch_all_data() [
    new users = await fetch_users()
    new posts = await fetch_posts()
    new comments = await fetch_comments()
]
```

## 10.3 Common Pitfalls

### 1. Uninitialized Variables

```catlang
; Error: using uninitialized variable
[
    new x i32
    print("{x}")  ; Error: x is uninitialized
    return 0
]

; Correct: use after initialization
[
    new x = 0
    print("{x}")
    return 0
]

; Or explicitly use in unsafe block
[
    unsafe close(init) [
        new x i32
        print("{x}")  ; Programmer responsible for ensuring safety
    ]
    return 0
]
```

### 2. Array Out of Bounds

```catlang
; Error: may go out of bounds
[
    new arr = [1, 2, 3]
    unsafe close(bounds) [
        new val = arr[10]  ; Dangerous!
    ]
    return 0
]

; Correct: check bounds
[
    new arr = [1, 2, 3]
    new index = 10

    if (index >= 0 && index < len(arr)) [
        new val = arr[index]
    ] else [
        print("Index out of bounds")
    ]
    return 0
]
```

### 3. Null Pointer Dereference

```catlang
; Error: may dereference null pointer
[
    new ptr *i32 = null
    unsafe close(null) [
        print("{*ptr}")  ; Dangerous!
    ]
    return 0
]

; Correct: check for null pointer
[
    new ptr *i32 = get_pointer()

    if (ptr != null) [
        unsafe close(null) [
            print("{*ptr}")
        ]
    ] else [
        print("Null pointer")
    ]
    return 0
]
```

### 4. Missing Error Handling

```catlang
; Error: ignoring possible errors
[
    new result = risky_operation()  ; No error handling
    print("Result: {result}")
    return 0
]

; Correct: handle errors
[
    try [
        new result = risky_operation()
        print("Result: {result}")
    ] catch (e Any) [
        print("Operation failed: {e}")
    ]
    return 0
]
```

### 5. Blocking in Async Code

```catlang
; Error: blocking in async function
async fn bad_example() [
    sync_wait(something)  ; Blocks entire event loop
]

; Correct: use async primitives
async fn good_example() [
    await something_async()
]
```

### 6. Memory Reinterpretation Errors

```catlang
; Error: size mismatch in reinterpretation
[
    new small = 42 i32
    new large = m+i64 small  ; May not be expected result
    return 0
]

; Correct: ensure size matches
[
    new value = 42 i64
    new bits = m+i64 value  ; Size matches
    return 0
]
```

## 10.4 Error Handling Best Practices

### Use Specific Error Types

```catlang
; Good practice
struct DatabaseError [
    query: str
    code: i32
    message: str
]

try [
    execute_query(sql)
] catch (e DatabaseError) [
    print("Database error: [{e.code}] {e.message}")
    print("Query: {e.query}")
]

; Bad practice
try [
    execute_query(sql)
] catch (e Any) [
    ; Lost specific information
    print("Error occurred")
]
```

### Provide Meaningful Error Messages

```catlang
; Good practice
fn parse_int(s: str) -> i32 [
    if (!is_numeric(s)) [
        throw ParseError {
            input: s,
            position: 0,
            expected: "Numeric string"
        }
    ]
]

; Bad practice
fn parse_int(s: str) -> i32 [
    if (!is_numeric(s)) [
        throw "Parse failed"  ; Insufficient information
    ]
]
```

### Handle Errors at Appropriate Levels

```catlang
; Low level: throw specific errors
fn read_file(path: str) -> str [
    if (!file_exists(path)) [
        throw FileError { path: path, reason: "File does not exist" }
    ]
    return read_file_content(path)
]

; Middle level: translate errors
fn load_config() -> Config [
    try [
        new content = read_file("config.json")
        return parse_config(content)
    ] catch (e FileError) [
        throw ConfigError { reason: "Cannot read config file" }
    ]
]

; High level: show user-friendly messages
[
    try [
        new config = load_config()
    ] catch (e Any) [
        print("Startup failed, please check config file")
    ]
]
```

## 10.5 Testing Suggestions

### Unit Tests

```catlang
; Assuming a test framework
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

### Boundary Tests

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

## 10.6 Safety Checklist

Check before submitting code:

- [ ] All variables are initialized
- [ ] Array access has bounds checking
- [ ] Pointers are checked for null before use
- [ ] Errors are handled appropriately
- [ ] No resource leaks
- [ ] Concurrent code has no race conditions
- [ ] unsafe blocks have adequate comments
- [ ] Sensitive data is handled properly

## 10.7 Comprehensive Examples

### Complete Project Structure

```catlang
; ==========================================
; File: main.catlang
; Description: Application entry point
; ==========================================

import config_loader as cfg
import database as db
import api_server as api
import logging as log

; Application configuration
struct AppConfig [
    port: i32
    db_url: str
    log_level: str
]

; Initialize logging
fn init_logging(level: str) [
    log.configure(level: level)
    log.info("Logging system initialized")
]

; Load configuration
fn load_app_config() -> AppConfig [
    try [
        new raw = cfg.load("config.json")
        return AppConfig {
            port: raw.port,
            db_url: raw.db_url,
            log_level: raw.log_level
        }
    ] catch (e Any) [
        log.error("Config load failed: {e}")
        throw e
    ]
]

; Main program
[
    try [
        ; Load configuration
        new config = load_app_config()

        ; Initialize logging
        init_logging(config.log_level)

        ; Connect to database
        new db_conn = await db.connect(config.db_url)
        log.info("Database connection successful")

        ; Start API server
        await api.start(db_conn, config.port)

    ] catch (e Any) [
        log.error("Application startup failed: {e}")
        return 1
    ]

    return 0
]
```

### Utility Module

```catlang
; ==========================================
; File: utils.catlang
; Description: Common utility functions
; ==========================================

; String trim
fn trim(s: str) -> str [
    ; Implementation
]

; String split
fn split(s: str, delimiter: str) -> [str] [
    ; Implementation
]

; Array map
fn map(arr: [T], fn: fn(T) -> U) -> [U] [
    new result [U; len(arr)]
    for (new i = 0, i < len(arr), i += 1) [
        result[i] = fn(arr[i])
    ]
    return result
]

; Array filter
fn filter(arr: [T], predicate: fn(T) -> bool) -> [T] [
    new result [T]
    for (new item in arr) [
        if (predicate(item)) [
            result.push(item)
        ]
    ]
    return result
]

; Array reduce
fn reduce(arr: [T], initial: U, fn: fn(U, T) -> U) -> U [
    new acc = initial
    for (new item in arr) [
        acc = fn(acc, item)
    ]
    return acc
]
```

## 10.8 Exercises

1. Refactor the following code to follow best practices:

```catlang
; Original code
fn p(d)[new r=1;for(new i=1,i<=d,i+=1)[r=r*i];return r]
```

2. Add appropriate error handling to the following function:

```catlang
fn divide(a: i32, b: i32) -> i32 [
    return a / b
]
```

3. Optimize the performance of the following code:

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
<summary>Reference Answers</summary>

```catlang
; Exercise 1: Refactor
fn factorial(n: i32) -> i32 [
    new result = 1

    for (new i = 1, i <= n, i += 1) [
        result = result * i
    ]

    return result
]

; Exercise 2: Add error handling
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
            reason: "Division by zero"
        }
    ]
    return a / b
]

; Exercise 3: Performance optimization
fn find_max(arr: [i32]) -> i32 [
    if (len(arr) == 0) [
        throw "Empty array"
    ]

    new len = len(arr)  ; Cache length
    new max = arr[0]

    for (new i = 1, i < len, i += 1) [  ; Start from 1
        if (arr[i] > max) [
            max = arr[i]
        ]
    ]

    return max
]
```
</details>

## Summary

Congratulations on completing the CatLang tutorial! You've learned:

1. ✅ Basic syntax and type system
2. ✅ Control flow and functions
3. ✅ Data structures and memory management
4. ✅ Error handling and concurrent programming
5. ✅ Module imports and best practices

Continue exploring:
- Read [Syntax Specification (Deprecated)](../syntax.txt) for complete syntax
- Check example code in `benchmark/` and `test/` directories
- Start writing your own CatLang projects!

Happy coding! 🐱
