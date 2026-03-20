# Chapter 05: Data Structures

This chapter introduces CatLang's data structures: structs, arrays, pointers, and memory operations.

## 5.1 Structs

### Define a Struct

Define a struct using the `struct` keyword:

```catlang
struct Person [
    name: str
    age: i32
    email: str
]
```

### Create Instances

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

### Access Fields

```catlang
struct Rectangle [
    width: f64
    height: f64
]

[
    new rect = Rectangle { width: 10.5, height: 5.0 }

    ; Access fields
    new w = rect.width
    new h = rect.height

    ; Modify fields
    rect.width = 15.0
    rect.height = 8.0

    return 0
]
```

### Generic Structs

CatLang supports generic structs, allowing type parameters in struct definitions:

```catlang
; Define generic struct
struct Option<T> [
    value: T
    has_value: bool
]

struct Result<T, E> [
    ok: T
    err: E
    is_ok: bool
]

struct Container<T> [
    item: T
    count: i32
]

[
    ; Use generic structs
    new opt_int: Option<i32> = Option { value: 42, has_value: true }
    new opt_str: Option<String> = Option { value: "Hello", has_value: true }

    new res: Result<i32, String> = Result { ok: 100, err: "", is_ok: true }

    new container: Container<f64> = Container { item: 3.14, count: 1 }

    return 0
]
```

```catlang
; Generic struct methods
struct Box<T> [
    data: T
]

impl Box<T> [
    fn get_data(self: Box<T>) -> T [
        return self.data
    ]

    fn set_data(self: Box<T>, new_data: T) -> Box<T> [
        return Box { data: new_data }
    ]
]

[
    new int_box: Box<i32> = Box { data: 42 }
    new str_box: Box<String> = Box { data: "Hello" }

    new val = int_box.get_data()
    new new_box = int_box.set_data(100)

    return 0
]
```

## 5.2 Struct Methods (impl)

Define methods for structs using `impl` blocks:

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

    print("Area: {a}")
    print("Radius after scaling: {c2.radius}")

    return 0
]
```

## 5.3 Arrays

### Dynamic Arrays

```catlang
[
    ; Declare dynamic array
    new arr [i32]

    ; Declare and initialize
    new nums = [1, 2, 3, 4, 5]

    ; Access elements
    new first = nums[0]
    new second = nums[1]

    ; Modify elements
    nums[0] = 10

    return 0
]
```

### Fixed-Size Arrays

```catlang
[
    ; Fixed-size array
    new fixed [i32; 5]

    ; Initialize
    fixed[0] = 1
    fixed[1] = 2
    fixed[2] = 3
    fixed[3] = 4
    fixed[4] = 5

    ; Or use literal
    new colors = ["red", "green", "blue"]

    return 0
]
```

### Multi-dimensional Arrays

```catlang
[
    ; 3x3 matrix
    new matrix [[i32; 3]; 3]

    ; Initialize
    matrix[0][0] = 1
    matrix[0][1] = 2
    matrix[0][2] = 3
    matrix[1][0] = 4
    ; ...

    ; Access
    new val = matrix[1][2]

    return 0
]
```

### Array Traversal

```catlang
[
    new arr = [10, 20, 30, 40, 50]
    new sum = 0

    for (new i = 0, i < 5, i += 1) [
        sum = sum + arr[i]
    ]

    print("Array sum: {sum}")  ; Output: 150

    return 0
]
```

## 5.4 Pointers

### Pointer Declaration

```catlang
[
    ; Declare pointer
    new ptr *i32

    ; Declare and initialize
    new x = 42
    new ptr2 = &x  ; Assuming & is address-of operator

    return 0
]
```

### Dereference

```catlang
[
    new value = 100
    new ptr *i32 = &value

    ; Dereference access
    new deref = *ptr

    print("Value: {deref}")  ; Output: 100

    return 0
]
```

### Pointer Arithmetic (unsafe)

```catlang
[
    new arr = [10, 20, 30, 40, 50]
    new ptr *i32 = &arr[0]

    unsafe close(bounds) [
        ; Pointer arithmetic (use with caution)
        new val1 = *ptr
        ; ptr = ptr + 1  ; Move to next element
        ; new val2 = *ptr
    ]

    return 0
]
```

## 5.5 Memory Reinterpretation

Use `m+` for physical bit reinterpretation:

### Integer to Float

```catlang
[
    ; IEEE 754 double precision representation
    new int_rep = 0x3FF0000000000000
    new pi_val = m+f64 int_rep

    print("Reinterpreted value: {pi_val}")  ; Approximately 1.0

    return 0
]
```

### Float to Integer

```catlang
[
    new float_val = 2.0
    new int_val = m+i64 float_val

    print("Bit pattern: {int_val}")

    return 0
]
```

## 5.6 Memory Copy (cpy)

Use `cpy` for memory copying:

```catlang
[
    new source = 42
    new dest i32

    ; Copy 4 bytes (i32 size)
    cpy dest i32(source)

    print("Destination value: {dest}")  ; Output: 42

    return 0
]
```

### Copy Structs

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

## 5.7 Unsafe Blocks

### Disable Safety Checks

```catlang
[
    new arr = [1, 2, 3]

    unsafe close(bounds) [
        ; Disable bounds checking
        new val = arr[100]  ; Won't trigger bounds check error
    ]

    return 0
]
```

### Disable Initialization Check

```catlang
[
    unsafe close(init) [
        new uninitialized i32
        ; Use uninitialized variable (dangerous!)
        print("{uninitialized}")
    ]

    return 0
]
```

### Disable All Safety Checks

```catlang
[
    unsafe all [
        ; Disable all safety checks
        new raw = m+*i32 0x1000
        print("{*raw}")
    ]

    return 0
]
```

### Keep Specific Checks

```catlang
[
    unsafe keep(lifetime) [
        ; Only keep lifetime checks, disable others
        ; Perform low-level operations
    ]

    return 0
]
```

## 5.8 Special Types

### Arbitrary Length Integer (ia)

```catlang
[
    new big_num ia = 999999999999999999
    new result = big_num * 2

    print("Big number calculation: {result}")

    return 0
]
```

### Arbitrary Length Float (fa)

```catlang
[
    new precise_pi fa = 3.141592653589793238462643383279502884197
    new area = precise_pi * 100 * 100

    print("Precise area: {area}")

    return 0
]
```

### Arbitrary Length String (sa)

```catlang
[
    new long_text sa = "This is a very long text that can contain any number of characters..."

    print("Length: {len(long_text)}")

    return 0
]
```

### Arbitrary Bit-Width Types (a8, a16, a32, a64, aa)

CatLang provides arbitrary bit-width types, allowing you to specify types with specific bit counts:

```catlang
[
    ; 8-bit arbitrary type
    new x: a8 = 42
    new byte_val: a8 = 0xFF

    ; 16-bit arbitrary type
    new y: a16 = 1000
    new short_val: a16 = 0x7FFF

    ; 32-bit arbitrary type
    new z: a32 = 100000
    new int_val: a32 = 0x7FFFFFFF

    ; 64-bit arbitrary type
    new big: a64 = 9999999999
    new long_val: a64 = 0x7FFFFFFFFFFFFFFF

    ; Arbitrary length type (theoretically infinite)
    new huge: aa = 999999999999999999999
    new massive: aa = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF

    return 0
]
```

**Use Cases:**

```catlang
; Hardware register access
struct HardwareRegister [
    control: a8      ; 8-bit control register
    status: a8       ; 8-bit status register
    data: a32        ; 32-bit data register
]

; Network protocol packet
struct NetworkPacket [
    version: a8      ; Version (4 bits) + Type (4 bits)
    flags: a8        ; Flags
    length: a16      ; Packet length
    checksum: a32    ; Checksum
]

[
    new reg = HardwareRegister { control: 0x01, status: 0x00, data: 0x12345678 }
    new packet = NetworkPacket { version: 0x45, flags: 0x02, length: 1500, checksum: 0xDEADBEEF }

    return 0
]
```

### Timer (timer)

```catlang
[
    new t timer

    ; Start timer
    t.start()

    ; Perform some operations
    new result = expensive_computation()

    ; Get elapsed time
    new elapsed = t.elapsed()
    print("Elapsed: {elapsed}ms")

    return 0
]
```

## 5.9 Comprehensive Examples

### Example 1: Student Management System

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

    print("{s1.name}'s grade: {s1.get_letter_grade()}")
    print("{s2.name}'s grade: {s2.get_letter_grade()}")
    print("{s3.name}'s grade: {s3.get_letter_grade()}")

    if (s1.is_passing()) [
        print("{s1.name} passed")
    ]
    if (s3.is_passing()) [
        print("{s3.name} passed")
    ] else [
        print("{s3.name} failed")
    ]

    return 0
]
```

### Example 2: Vector Operations

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

    print("Vector sum: ({sum.x}, {sum.y}, {sum.z})")
    print("Dot product: {dot_product}")
    print("Magnitude: {mag}")

    return 0
]
```

### Example 3: Dynamic Array Operations

```catlang
; Note: The following is pseudo-code, actual implementation depends on standard library
[
    ; Create dynamic array
    new nums = [1, 2, 3, 4, 5]

    ; Add elements
    nums.push(6)
    nums.push(7)

    ; Remove element
    nums.pop()

    ; Get length
    new len = len(nums)

    ; Traverse
    for (new i = 0, i < len, i += 1) [
        print("nums[{i}] = {nums[i]}")
    ]

    return 0
]
```

### Example 4: Linked List Node

```catlang
struct ListNode [
    value: i32
    next: *ListNode
]

fn create_node(value: i32) -> ListNode [
    return ListNode { value: value, next: null }
]

[
    ; Create linked list nodes
    new node1 = create_node(10)
    new node2 = create_node(20)
    new node3 = create_node(30)

    ; Link nodes
    node1.next = &node2
    node2.next = &node3

    ; Traverse linked list
    new current = &node1
    while (current != null) [
        print("{current.value}")
        current = current.next
    ]

    return 0
]
```

## 5.10 Exercises

1. Define a `Book` struct with title, author, and price fields, and implement a method to calculate discounted price
2. Create a 3x3 matrix and implement a matrix transpose function
3. Use unsafe blocks to perform memory reinterpretation and print the bit pattern of float 1.0 as an integer
4. Define a generic struct `Pair<T, U>` with two fields of different types, and implement a swap method
5. Use arbitrary bit-width types to define a network packet struct

<details>
<summary>Reference Answers</summary>

```catlang
; Exercise 1: Book struct
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
    new book = Book { title: "CatLang Programming", author: "John Doe", price: 59.99 }
    new sale_price = book.discounted_price(0.2)
    print("Discounted price: {sale_price}")
    return 0
]

; Exercise 2: Matrix transpose
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] [
    new result [[i32; 3]; 3]
    for (new i = 0, i < 3, i += 1) [
        for (new j = 0, j < 3, j += 1) [
            result[i][j] = matrix[j][i]
        ]
    ]
    return result
]

; Exercise 3: Memory reinterpretation
[
    new float_val = 1.0
    new int_bits = m+i64 float_val
    print("Bit pattern of 1.0: {int_bits}")
    return 0
]

; Exercise 4: Generic Pair struct
struct Pair<T, U> [
    first: T
    second: U
]

impl Pair<T, U> [
    fn swap(self: Pair<T, U>) -> Pair<U, T> [
        return Pair { first: self.second, second: self.first }
    ]
]

[
    new p: Pair<i32, String> = Pair { first: 42, second: "Hello" }
    new swapped = p.swap()
    return 0
]

; Exercise 5: Network packet
struct NetworkPacket [
    version: a8        ; Version (4 bits) + Type (4 bits)
    flags: a8          ; Flags
    length: a16        ; Packet length
    checksum: a32      ; Checksum
    payload: [a8]      ; Dynamic payload
]

[
    new packet = NetworkPacket {
        version: 0x45,
        flags: 0x02,
        length: 1500,
        checksum: 0xDEADBEEF
    }
    return 0
]
```
</details>

## Next Steps

- [Chapter 06: Error Handling](06_error_handling.md) - try/catch, throw
- [Chapter 07: Memory Management](07_memory_management.md) - unsafe deep dive, memory operations
