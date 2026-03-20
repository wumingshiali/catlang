# Chapter 06: Error Handling

This chapter introduces CatLang's error handling mechanism: try/catch, throw, and custom error types.

## 6.1 Throwing Errors

Use the `throw` keyword to throw errors:

### Basic Usage

```catlang
[
    new error_msg = "An error occurred"
    throw error_msg
]
```

### Throw in Functions

```catlang
fn divide(a: i32, b: i32) -> i32 [
    if (b == 0) [
        throw "Division by zero"
    ]
    return a / b
]

[
    new result = divide(10, 2)
    print("Result: {result}")

    ; This line won't execute because it throws an error
    new error_result = divide(10, 0)

    return 0
]
```

## 6.2 Catching Errors

Use `try/catch` statements to catch and handle errors:

### Basic try/catch

```catlang
try [
    new result = divide(10, 0)
    print("Result: {result}")
] catch (e Any) [
    print("Caught error: {e}")
]
```

### Type-Matching catch

```catlang
struct MathError [
    code: i32
    message: str
]

fn safe_divide(a: i32, b: i32) -> i32 [
    if (b == 0) [
        throw MathError { code: 1, message: "Division by zero error" }
    ]
    return a / b
]

[
    try [
        new result = safe_divide(10, 0)
        print("Result: {result}")
    ] catch (err MathError) [
        print("Math error: [{err.code}] {err.message}")
    ] catch (e Any) [
        print("Unknown error: {e}")
    ]

    return 0
]
```

### Multiple catch Clauses

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
    ; May throw different types of errors
    throw FileError { path: "/data.txt", reason: "File not found" }
]

[
    try [
        risky_operation()
    ] catch (fe FileError) [
        print("File error: {fe.path} - {fe.reason}")
    ] catch (ne NetworkError) [
        print("Network error: {ne.url} - Status {ne.status}")
    ] catch (e Any) [
        print("Other error: {e}")
    ]

    return 0
]
```

## 6.3 Custom Error Types

### Define Error Structs

```catlang
; Validation error
struct ValidationError [
    field: str
    message: str
]

; Parse error
struct ParseError [
    input: str
    position: i32
    expected: str
]

; System error
struct SystemError [
    code: i32
    message: str
]
```

### Error Type Hierarchy

```catlang
; Base error type
struct AppError [
    code: i32
    message: str
]

; Specific error types
struct DatabaseError [
    query: str
    inner: AppError
]

struct HttpError [
    status: i32
    inner: AppError
]
```

## 6.4 Error Propagation

### Re-throw Errors

```catlang
fn process_data(data: str) [
    try [
        validate(data)
        parse(data)
        save(data)
    ] catch (e ValidationError) [
        print("Validation failed: {e.message}")
        throw e  ; Re-throw
    ] catch (e Any) [
        print("Processing failed: {e}")
        throw e
    ]
]
```

### Error Translation

```catlang
fn read_config(path: str) -> Config [
    try [
        new content = read_file(path)
        return parse_config(content)
    ] catch (fe FileError) [
        throw ConfigError {
            code: 1,
            message: "Cannot read config file: {fe.reason}"
        }
    ] catch (pe ParseError) [
        throw ConfigError {
            code: 2,
            message: "Config format error: {pe.expected}"
        }
    ]
]
```

## 6.5 try Expressions

### try Blocks as Expressions

```catlang
fn get_value() -> i32 [
    new result = try [
        risky_calculation()
    ] catch (e Any) [
        print("Calculation failed, using default value")
        0  ; Return value of catch block
    ]
    return result
]
```

## 6.6 Comprehensive Examples

### Example 1: User Registration Validation

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
            message: "Email address is too short"
        }
    ]
    if (!contains(email, "@")) [
        throw RegistrationError {
            field: "email",
            code: "INVALID_FORMAT",
            message: "Email format is incorrect"
        }
    ]
]

fn validate_password(password: str) [
    if (len(password) < 8) [
        throw RegistrationError {
            field: "password",
            code: "TOO_WEAK",
            message: "Password must be at least 8 characters"
        }
    ]
]

fn register_user(email: str, password: str) [
    try [
        validate_email(email)
        validate_password(password)
        print("Registration successful!")
    ] catch (e RegistrationError) [
        print("Registration failed: [{e.field}] {e.code} - {e.message}")
    ]
]

[
    register_user("alice@example.com", "secure123")  ; Success
    register_user("bob", "weak")  ; Failure
    register_user("charlie@example.com", "strongpass")  ; Success

    return 0
]
```

### Example 2: File Processing

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
                reason: "File does not exist"
            }
        ]
        return read_file(path)
    ] catch (e FileError) [
        print("File operation failed: {e.path} - {e.reason}")
        throw e
    ]
]

fn process_file(input_path: str, output_path: str) [
    try [
        new content = read_file_safe(input_path)
        new processed = transform(content)
        write_file(output_path, processed)
        print("File processing complete")
    ] catch (e FileError) [
        print("Cannot process file: {e.path}")
    ] catch (e Any) [
        print("Unknown error: {e}")
    ]
]

[
    process_file("input.txt", "output.txt")
    return 0
]
```

### Example 3: Network Requests

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
            message: "Invalid timeout"
        }
    ]

    ; Simulate network request
    if (contains(url, "error")) [
        throw HttpError {
            url: url,
            status: 500,
            message: "Server error"
        }
    ]

    if (timeout < 100) [
        throw TimeoutError {
            url: url,
            timeout_ms: timeout
        }
    ]

    return "Response content"
]

fn fetch_with_retry(url: str, max_retries: i32) -> str [
    new attempt = 0

    while (attempt < max_retries) [
        try [
            return fetch_url(url, 1000 * (attempt + 1))
        ] catch (e TimeoutError) [
            attempt = attempt + 1
            print("Timeout, retry {attempt}/{max_retries}")
        ] catch (e HttpError) [
            if (e.status >= 500) [
                attempt = attempt + 1
                print("Server error, retry {attempt}/{max_retries}")
            ] else [
                print("Client error: {e.message}")
                throw e
            ]
        ] catch (e Any) [
            print("Unknown error: {e}")
            throw e
        ]
    ]

    throw HttpError {
        url: url,
        status: 503,
        message: "Service unavailable"
    }
]

[
    try [
        new content = fetch_with_retry("https://api.example.com/data", 3)
        print("Fetch successful: {content}")
    ] catch (e Any) [
        print("Request failed: {e}")
    ]

    return 0
]
```

### Example 4: Calculator Error Handling

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
                    message: "Division by zero"
                }
            ]
            return a / b
        case "%":
            if (b == 0) [
                throw CalcError {
                    operation: "%",
                    message: "Modulo by zero"
                }
            ]
            return a % b
        default:
            throw CalcError {
                operation: op,
                message: "Unknown operator"
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
            print("Calculation error: {a} {op} {b} - {e.message}")
        ]
    ]

    return 0
]
```

### Example 5: Database Operations

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
    print("Connect to {host}:{port}")
]

fn execute_query(query: str) [
    if (contains(query, "DROP")) [
        throw DbError {
            query: query,
            code: 403,
            message: "DROP operation forbidden"
        }
    ]
    print("Execute query: {query}")
]

fn run_transaction(queries: [str]) [
    try [
        connect_db("localhost", 5432)

        for (new i = 0, i < len(queries), i += 1) [
            execute_query(queries[i])
        ]

        print("Transaction complete")
    ] catch (e ConnectionError) [
        print("Connection failed: {e.host}:{e.port}")
        throw e
    ] catch (e DbError) [
        print("Database error: [{e.code}] {e.message}")
        throw e
    ] catch (e Any) [
        print("Unknown error: {e}")
        throw e
    ]
]

[
    try [
        run_transaction(["SELECT * FROM users", "INSERT INTO logs", "UPDATE stats"])
    ] catch (e Any) [
        print("Transaction failed")
    ]

    return 0
]
```

## 6.7 Error Handling Best Practices

### 1. Use Specific Error Types

```catlang
; Good practice
struct SpecificError [
    context: str
    message: str
]

try [
    operation()
] catch (e SpecificError) [
    ; Can access specific fields
    print("Context: {e.context}")
]

; Bad practice
try [
    operation()
] catch (e Any) [
    ; Lost specific information
]
```

### 2. Provide Meaningful Error Messages

```catlang
; Good practice
throw ValidationError {
    field: "email",
    message: "Email format is incorrect, should contain @"
}

; Bad practice
throw "Error"
```

### 3. Handle Errors at Appropriate Levels

```catlang
; Low level: throw specific errors
fn parse_int(s: str) -> i32 [
    if (!is_numeric(s)) [
        throw ParseError { input: s, expected: "Number" }
    ]
    ; ...
]

; Middle level: translate or propagate errors
fn process_input(s: str) [
    try [
        new num = parse_int(s)
        use_number(num)
    ] catch (e ParseError) [
        throw InputError { field: "number", reason: e.message }
    ]
]

; High level: show user-friendly messages
[
    try [
        process_input(user_input)
    ] catch (e Any) [
        print("Invalid input, please check and try again")
    ]
]
```

### 4. Don't Ignore Errors

```catlang
; Bad practice - empty catch block
try [
    risky_operation()
] catch (e Any) [
    ; Ignore error
]

; Good practice - at least log the error
try [
    risky_operation()
] catch (e Any) [
    print("Warning: Operation failed - {e}")
]
```

## 6.8 Exercises

1. Create a `DivisionError` type with dividend, divisor, and reason fields
2. Write a function to safely parse strings to integers, handling various error cases
3. Implement a simple state machine that may throw different types of errors in different states

<details>
<summary>Reference Answers</summary>

```catlang
; Exercise 1: Division error
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
            reason: "Division by zero"
        }
    ]
    return a / b
]

[
    try [
        new result = safe_divide(10, 0)
    ] catch (e DivisionError) [
        print("Error: {e.dividend} / {e.divisor} - {e.reason}")
    ]
    return 0
]

; Exercise 2: Safe integer parsing
fn parse_int_safe(s: str) -> i32 [
    if (len(s) == 0) [
        throw ParseError { input: s, position: 0, expected: "Non-empty string" }
    ]
    ; Simplified implementation
    return 42
]

; Exercise 3: State machine
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

## Next Steps

- [Chapter 07: Memory Management](07_memory_management.md) - unsafe deep dive, memory operations
- [Chapter 08: Concurrency](08_concurrency.md) - async/await, spawn tasks
