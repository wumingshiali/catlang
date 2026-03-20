# 第 03 章：控制流

本章介绍 CatLang 的控制流结构：条件语句、循环和模式匹配。

## 3.1 if 条件语句

### 基本语法

```catlang
[
    new age = 18
    
    if (age >= 18) [
        print("你已成年")
    ]
    
    return 0
]
```

### if-else

```catlang
[
    new score = 85
    
    if (score >= 60) [
        print("及格")
    ] else [
        print("不及格")
    ]
    
    return 0
]
```

### if-else if-else

```catlang
[
    new score = 85
    
    if (score >= 90) [
        print("优秀")
    ] else if (score >= 80) [
        print("良好")
    ] else if (score >= 60) [
        print("及格")
    ] else [
        print("不及格")
    ]
    
    return 0
]
```

### 嵌套 if

```catlang
[
    new age = 20
    new has_license = true
    
    if (age >= 18) [
        if (has_license) [
            print("你可以开车")
        ] else [
            print("你需要先考驾照")
        ]
    ] else [
        print("你还未成年")
    ]
    
    return 0
]
```

## 3.2 while 循环

### 基本语法

```catlang
[
    new count = 0
    
    while (count < 5) [
        print("计数：{count}")
        count = count + 1
    ]
    
    return 0
]
```

### 带条件的 while

```catlang
[
    new sum = 0
    new i = 1
    
    while (i <= 100) [
        sum = sum + i
        i = i + 1
    ]
    
    print("1 到 100 的和：{sum}")  ; 输出：5050
    
    return 0
]
```

### 无限循环（使用 break 退出）

```catlang
[
    new counter = 0
    
    while (true) [
        if (counter >= 5) [
            break
        ]
        print("计数：{counter}")
        counter = counter + 1
    ]
    
    return 0
]
```

## 3.3 for 循环

CatLang 的 for 循环采用 C 风格语法：

```catlang
for (初始化，条件，更新) 循环体
```

### 基本语法

```catlang
[
    for (new i = 0, i < 5, i += 1) [
        print("i = {i}")
    ]
    
    return 0
]
```

### 计算阶乘

```catlang
[
    new n = 5
    new factorial = 1
    
    for (new i = 1, i <= n, i += 1) [
        factorial = factorial * i
    ]
    
    print("{n}! = {factorial}")  ; 输出：120
    
    return 0
]
```

### 遍历数组（索引方式）

```catlang
[
    new arr = [10, 20, 30, 40, 50]
    new len = 5
    new sum = 0
    
    for (new i = 0, i < len, i += 1) [
        sum = sum + arr[i]
    ]
    
    print("数组和：{sum}")  ; 输出：150
    
    return 0
]
```

## 3.4 switch 模式匹配

switch 语句提供强大的模式匹配功能：

### 基本语法

```catlang
[
    new day = 3
    
    switch (day) [
        case 1:
            print("星期一")
        case 2:
            print("星期二")
        case 3:
            print("星期三")
        case 4:
            print("星期四")
        case 5:
            print("星期五")
        case 6:
            print("星期六")
        case 7:
            print("星期日")
        default:
            print("无效的日期")
    ]
    
    return 0
]
```

### 匹配字符串

```catlang
[
    new command = "start"
    
    switch (command) [
        case "start":
            print("启动服务")
        case "stop":
            print("停止服务")
        case "restart":
            print("重启服务")
        default:
            print("未知命令")
    ]
    
    return 0
]
```

### 匹配布尔值

```catlang
[
    new is_ready = true
    
    switch (is_ready) [
        case true:
            print("准备就绪")
        case false:
            print("尚未准备")
    ]
    
    return 0
]
```

### 使用标识符模式

```catlazng
[
    new value = 42
    
    switch (value) [
        case 0:
            print("零值")
        case negative:
            ; negative 绑定到值
            print("负数：{negative}")
        case positive:
            print("正数：{positive}")
    ]
    
    return 0
]
```

## 3.5 控制流表达式

### 逻辑运算符短路

```catlang
[
    new x = 5
    
    ; && 短路：如果左边为 false，右边不执行
    if (x > 0 && x < 10) [
        print("x 在 0 到 10 之间")
    ]
    
    ; || 短路：如果左边为 true，右边不执行
    if (x < 0 || x > 100) [
        print("x 超出正常范围")
    ]
    
    return 0
]
```

### 三元运算符替代方案

CatLang 没有三元运算符，但可以用 if-else 表达相同逻辑：

```catlang
[
    new age = 20
    new status
    
    if (age >= 18) [
        status = "成年"
    ] else [
        status = "未成年"
    ]
    
    print("状态：{status}")
    
    return 0
]
```

## 3.6 综合示例

### 示例 1：打印九九乘法表

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

### 示例 2：判断闰年

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
        print("{year} 是闰年")
    ] else [
        print("{year} 是平年")
    ]
    
    return 0
]
```

### 示例 3：简单计算器

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
                print("错误：除数不能为零")
                result = 0
            ]
        default:
            print("未知运算符")
            result = 0
    ]
    
    print("结果：{result}")
    
    return 0
]
```

### 示例 4：斐波那契数列

```catlang
[
    new n = 10
    new a = 0
    new b = 1
    
    print("斐波那契数列前{n}项：")
    
    for (new i = 0, i < n, i += 1) [
        print("{a} ")
        new temp = a + b
        a = b
        b = temp
    ]
    
    return 0
]
```

## 3.7 练习

1. 使用 while 循环计算 1 到 100 之间所有偶数的和
2. 使用 switch 语句根据分数输出等级（A: 90-100, B: 80-89, C: 70-79, D: 60-69, F: <60）
3. 使用 for 循环打印以下图案：
   ```
   *
   **
   ***
   ****
   *****
   ```

<details>
<summary>参考答案</summary>

```catlang
; 练习 1：偶数和
[
    new sum = 0
    new i = 2
    
    while (i <= 100) [
        sum = sum + i
        i = i + 2
    ]
    
    print("偶数和：{sum}")
    
    return 0
]

; 练习 2：成绩等级
[
    new score = 85
    
    if (score >= 90) [
        print("等级：A")
    ] else if (score >= 80) [
        print("等级：B")
    ] else if (score >= 70) [
        print("等级：C")
    ] else if (score >= 60) [
        print("等级：D")
    ] else [
        print("等级：F")
    ]
    
    return 0
]

; 练习 3：打印图案
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

## 下一步

- [第 04 章：函数](04_functions.md) - 函数定义、参数、返回值
- [第 06 章：错误处理](06_error_handling.md) - try/catch、异常处理
