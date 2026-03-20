# Chapter 04: Functions

This chapter introduces functions in CatLang: definition, parameters, return values, and async functions.

## 4.1 Function Definition

### Basic Syntax

Define functions using the `fn` keyword:

```catlang
; Function without parameters and return value
fn say_hello() [
    print("Hello!")
]

[
    say_hello()
    return 0
]
```

### Functions with Parameters

```catlang
; Function with parameters
fn greet(name: str) [
    print("Hello, {name}!")
]

[
    greet("Alice")
    greet("Bob")
    return 0
]
```

### Multiple Parameters

```catlang
fn add(a: i32, b: i32) [
    new sum = a + b
    print("{a} + {b} = {sum}")
]

[
    add(3, 5)  ; Output: 3 + 5 = 8
    return 0
]
```

## 4.2 Return Values

Use the `return` statement to return values:

### Basic Return Value

```catlang
fn add(a: i32, b: i32) -> i32 [
    return a + b
]

[
    new result = add(10, 20)
    print("Result: {result}")  ; Output: 30
    return 0
]
```

### Early Return

```catlang
fn absolute(x: i32) -> i32 [
    if (x >= 0) [
        return x
    ]
    return -x
]

[
    print("{absolute(-5)}")  ; Output: 5
    print("{absolute(5)}")   ; Output: 5
    return 0
]
```

### No Return Value (Implicit void)

```catlang
fn print_sum(a: i32, b: i32) [
    new sum = a + b
    print("Sum: {sum}")
    ; Implicit return, no return statement needed
]

[
    print_sum(3, 7)
    return 0
]
```

## 4.3 Parameter Passing

### Pass by Value

CatLang uses pass by value by default:

```catlang
fn modify(x: i32) [
    x = x + 10
    print("Inside function: {x}")  ; Output: 15
]

[
    new a = 5
    modify(a)
    print("Outside function: {a}")  ; Output: 5 (original unchanged)
    return 0
]
```

### Multiple Parameters Example

```catlang
fn calc_circle(radius: f64) -> f64 [
    new pi = 3.14159265359
    new area = pi * radius * radius
    new circumference = 2 * pi * radius

    print("Radius: {radius}")
    print("Area: {area}")
    print("Circumference: {circumference}")

    return area
]

[
    new result = calc_circle(5.0)
    return 0
]
```

## 4.4 Function Overloading

CatLang doesn't support traditional function overloading, but similar functionality can be achieved through default parameters or different function names:

```catlang
; Using different function names
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

## 4.5 Recursive Functions

### Factorial

```catlang
fn factorial(n: i32) -> i32 [
    if (n <= 1) [
        return 1
    ]
    return n * factorial(n - 1)
]

[
    print("5! = {factorial(5)}")  ; Output: 120
    return 0
]
```

### Fibonacci Sequence

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

## 4.6 Async Functions

Define async functions using `async fn`:

### Basic Async Function

```catlang
async fn fetch_data(id: i32) -> Result [
    ; Simulate async operation
    await sleep(100)
    print("Fetching data: {id}")
    return Result
]

[
    new data = await fetch_data(1)
    return 0
]
```

### Multiple Async Calls

```catlang
async fn task(name: str, delay: i32) [
    await sleep(delay)
    print("{name} completed")
]

async fn run_tasks() [
    ; Sequential execution
    await task("Task A", 100)
    await task("Task B", 200)
    await task("Task C", 150)
]

[
    await run_tasks()
    return 0
]
```

## 4.7 Functions as Values

### Store Function Results

```catlang
fn square(x: i32) -> i32 [
    return x * x
]

[
    new func_result = square(5)
    print("Result: {func_result}")  ; Output: 25
    return 0
]
```

## 4.8 Comprehensive Examples

### Example 1: Math Utility Functions

```catlang
; Calculate maximum
fn max(a: i32, b: i32) -> i32 [
    if (a > b) [
        return a
    ]
    return b
]

; Calculate minimum
fn min(a: i32, b: i32) -> i32 [
    if (a < b) [
        return a
    ]
    return b
]

; Check if prime
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
            print("{i} is prime")
        ]
    ]

    return 0
]
```

### Example 2: String Processing Functions

```catlang
; Calculate string length (pseudo-code, actual implementation depends on built-in functions)
fn str_length(s: str) -> i32 [
    ; Assuming there's a built-in len function
    return len(s)
]

; Concatenate greetings
fn make_greeting(name: str, time: str) -> str [
    return "{time}, {name}!"
]

[
    new greeting = make_greeting("Alice", "Good morning")
    print(greeting)
    return 0
]
```

### Example 3: Data Structure Operations

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
    ; Use square root function (assuming built-in sqrt)
    return sqrt(sq_sum)
]

[
    new p = create_point(3, 4)
    new dist = distance_from_origin(p)
    print("Distance: {dist}")  ; Output: 5
    return 0
]
```

### Example 4: Async Task Orchestration

```catlang
async fn download_file(url: str) -> Result [
    print("Start downloading: {url}")
    await sleep(500)
    print("Download complete: {url}")
    return Result
]

async fn process_data(data: str) -> Result [
    print("Processing data: {data}")
    await sleep(300)
    print("Processing complete")
    return Result
]

async fn main_workflow() [
    ; Download and process
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

## 4.9 Function Best Practices

### 1. Function Naming

Use meaningful verb phrases:

```catlang
; Good naming
fn calculate_area()
fn get_user_input()
fn validate_email()

; Avoid vague naming
fn do_stuff()      ; Too vague
fn process()       ; Not specific enough
```

### 2. Single Responsibility

Each function should do only one thing:

```catlang
; Good example
fn read_file(path: str) -> str [
    ; Only responsible for reading file
]

fn parse_data(data: str) [
    ; Only responsible for parsing data
]

; Bad example
fn read_and_parse_and_save() [
    ; Does too many things
]
```

### 3. Parameter Count

Keep parameter count simple (preferably no more than 3):

```catlang
; If too many parameters, consider using a struct
struct Config [
    host: str
    port: i32
    timeout: i32
    retries: i32
]

fn connect(config: Config) [
    ; Use struct to organize parameters
]
```

## 4.10 Exercises

1. Write a function `is_even(n: i32) -> bool` to check if a number is even
2. Write a function `power(base: i32, exp: i32) -> i32` to calculate power
3. Write an async function `delayed_print(msg: str, delay: i32)` to print a message with delay

<details>
<summary>Reference Answers</summary>

```catlang
; Exercise 1: Check even number
fn is_even(n: i32) -> bool [
    return n % 2 == 0
]

; Exercise 2: Calculate power
fn power(base: i32, exp: i32) -> i32 [
    new result = 1
    for (new i = 0, i < exp, i += 1) [
        result = result * base
    ]
    return result
]

; Exercise 3: Delayed print
async fn delayed_print(msg: str, delay: i32) [
    await sleep(delay)
    print(msg)
]

[
    print("Is even: {is_even(4)}")
    print("2^10 = {power(2, 10)}")
    await delayed_print("Delayed message", 1000)
    return 0
]
```
</details>

## Next Steps

- [Chapter 05: Data Structures](05_data_structures.md) - Structs, arrays, pointers
- [Chapter 08: Concurrency](08_concurrency.md) - async/await deep dive, spawn tasks
