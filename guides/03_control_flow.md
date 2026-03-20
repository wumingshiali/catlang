# Chapter 03: Control Flow

This chapter introduces CatLang's control flow structures: conditional statements, loops, and pattern matching.

## 3.1 if Conditional Statements

### Basic Syntax

```catlang
[
    new age = 18

    if (age >= 18) [
        print("You are an adult")
    ]

    return 0
]
```

### if-else

```catlang
[
    new score = 85

    if (score >= 60) [
        print("Passed")
    ] else [
        print("Failed")
    ]

    return 0
]
```

### if-else if-else

```catlang
[
    new score = 85

    if (score >= 90) [
        print("Excellent")
    ] else if (score >= 80) [
        print("Good")
    ] else if (score >= 60) [
        print("Passed")
    ] else [
        print("Failed")
    ]

    return 0
]
```

### Nested if

```catlang
[
    new age = 20
    new has_license = true

    if (age >= 18) [
        if (has_license) [
            print("You can drive")
        ] else [
            print("You need to get a license first")
        ]
    ] else [
        print("You are not an adult yet")
    ]

    return 0
]
```

## 3.2 while Loop

### Basic Syntax

```catlang
[
    new count = 0

    while (count < 5) [
        print("Count: {count}")
        count = count + 1
    ]

    return 0
]
```

### while with Condition

```catlang
[
    new sum = 0
    new i = 1

    while (i <= 100) [
        sum = sum + i
        i = i + 1
    ]

    print("Sum from 1 to 100: {sum}")  ; Output: 5050

    return 0
]
```

### Infinite Loop (exit with break)

```catlang
[
    new counter = 0

    while (true) [
        if (counter >= 5) [
            break
        ]
        print("Count: {counter}")
        counter = counter + 1
    ]

    return 0
]
```

## 3.3 for Loop

CatLang's for loop uses C-style syntax:

```catlang
for (initialization, condition, update) loop_body
```

### Basic Syntax

```catlang
[
    for (new i = 0, i < 5, i += 1) [
        print("i = {i}")
    ]

    return 0
]
```

### Calculate Factorial

```catlang
[
    new n = 5
    new factorial = 1

    for (new i = 1, i <= n, i += 1) [
        factorial = factorial * i
    ]

    print("{n}! = {factorial}")  ; Output: 120

    return 0
]
```

### Traverse Array (by index)

```catlang
[
    new arr = [10, 20, 30, 40, 50]
    new len = 5
    new sum = 0

    for (new i = 0, i < len, i += 1) [
        sum = sum + arr[i]
    ]

    print("Array sum: {sum}")  ; Output: 150

    return 0
]
```

## 3.4 switch Pattern Matching

The switch statement provides powerful pattern matching:

### Basic Syntax

```catlang
[
    new day = 3

    switch (day) [
        case 1:
            print("Monday")
        case 2:
            print("Tuesday")
        case 3:
            print("Wednesday")
        case 4:
            print("Thursday")
        case 5:
            print("Friday")
        case 6:
            print("Saturday")
        case 7:
            print("Sunday")
        default:
            print("Invalid day")
    ]

    return 0
]
```

### Match Strings

```catlang
[
    new command = "start"

    switch (command) [
        case "start":
            print("Start service")
        case "stop":
            print("Stop service")
        case "restart":
            print("Restart service")
        default:
            print("Unknown command")
    ]

    return 0
]
```

### Match Booleans

```catlang
[
    new is_ready = true

    switch (is_ready) [
        case true:
            print("Ready")
        case false:
            print("Not ready yet")
    ]

    return 0
]
```

### Using Identifier Patterns

```catlang
[
    new value = 42

    switch (value) [
        case 0:
            print("Zero value")
        case negative:
            ; negative is bound to the value
            print("Negative: {negative}")
        case positive:
            print("Positive: {positive}")
    ]

    return 0
]
```

## 3.5 Control Flow Expressions

### Logical Operator Short-Circuit

```catlang
[
    new x = 5

    ; && short-circuit: if left is false, right is not executed
    if (x > 0 && x < 10) [
        print("x is between 0 and 10")
    ]

    ; || short-circuit: if left is true, right is not executed
    if (x < 0 || x > 100) [
        print("x is out of normal range")
    ]

    return 0
]
```

### Ternary Operator Alternative

CatLang doesn't have a ternary operator, but if-else can express the same logic:

```catlang
[
    new age = 20
    new status

    if (age >= 18) [
        status = "Adult"
    ] else [
        status = "Minor"
    ]

    print("Status: {status}")

    return 0
]
```

## 3.6 Comprehensive Examples

### Example 1: Print Multiplication Table

```catlang
[
    for (new i = 1, i <= 9, i += 1) [
        for (new j = 1, j <= i, j += 1) [
            new product = i * j
            print("{j}x{i}={product}\t")
        ]
        print("\n")
    ]

    return 0
]
```

### Example 2: Leap Year Judgment

```catlang
[
    new year = 2024
    new is_leap = false

    if (year % 4 == 0) [
        if (year % 100 == 0) [
            if (year % 400 == 0) [
                is_leap = true
            ] else [
                is_leap = false
            ]
        ] else [
            is_leap = true
        ]
    ]

    if (is_leap) [
        print("{year} is a leap year")
    ] else [
        print("{year} is a common year")
    ]

    return 0
]
```

### Example 3: Simple Calculator

```catlang
[
    new a = 10
    new b = 5
    new op = "+"
    new result

    switch (op) [
        case "+":
            result = a + b
        case "-":
            result = a - b
        case "*":
            result = a * b
        case "/":
            if (b != 0) [
                result = a / b
            ] else [
                print("Error: Division by zero")
                result = 0
            ]
        default:
            print("Unknown operator")
            result = 0
    ]

    print("Result: {result}")

    return 0
]
```

### Example 4: Fibonacci Sequence

```catlang
[
    new n = 10
    new a = 0
    new b = 1

    print("First {n} terms of Fibonacci sequence:")

    for (new i = 0, i < n, i += 1) [
        print("{a} ")
        new temp = a + b
        a = b
        b = temp
    ]

    return 0
]
```

## 3.7 Exercises

1. Use a while loop to calculate the sum of all even numbers from 1 to 100
2. Use a switch statement to output grades based on scores (A: 90-100, B: 80-89, C: 70-79, D: 60-69, F: <60)
3. Use a for loop to print the following pattern:
   ```
   *
   **
   ***
   ****
   *****
   ```

<details>
<summary>Reference Answers</summary>

```catlang
; Exercise 1: Sum of even numbers
[
    new sum = 0
    new i = 2

    while (i <= 100) [
        sum = sum + i
        i = i + 2
    ]

    print("Sum of even numbers: {sum}")

    return 0
]

; Exercise 2: Grade levels
[
    new score = 85

    if (score >= 90) [
        print("Grade: A")
    ] else if (score >= 80) [
        print("Grade: B")
    ] else if (score >= 70) [
        print("Grade: C")
    ] else if (score >= 60) [
        print("Grade: D")
    ] else [
        print("Grade: F")
    ]

    return 0
]

; Exercise 3: Print pattern
[
    for (new i = 1, i <= 5, i += 1) [
        for (new j = 0, j < i, j += 1) [
            print("*")
        ]
        print("\n")
    ]

    return 0
]
```
</details>

## Next Steps

- [Chapter 04: Functions](04_functions.md) - Function definition, parameters, return values
- [Chapter 06: Error Handling](06_error_handling.md) - try/catch, exception handling
