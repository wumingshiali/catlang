# 第 01 章：快速开始

欢迎来到 CatLang！本章将带你运行第一个 CatLang 程序。

## 1.1 前提条件

在开始之前，请确保已完成环境配置：

- ✅ Zig 已安装
- ✅ CatLang 编译器已编译或下载

如果尚未配置，请先阅读 [第 00 章：环境配置](00_base_env.md)。

## 1.2 第一个 CatLang 程序

创建文件 `hello.cat`：

```catlang
; 我的第一个 CatLang 程序
[
    print("Hello, CatLang!")
    print("欢迎来到系统级编程的世界")
    
    ; 变量声明与字符串插值
    new name = "CatLang"
    new version = 1.0
    print("正在运行：{name} v{version}")
    
    return 0
]
```

## 1.3 运行程序

```bash
# 运行 CatLang 程序
./target/release/catlang hello.cat
```

## 1.4 程序结构解析

让我们看看上面的程序：

```catlang
; 我的第一个 CatLang 程序     ; 分号开始注释
[                          ; 方括号开始代码块（主入口）
    print("Hello")         ; 调用内置函数，无需导入
    new name = "CatLang"   ; 声明变量并初始化
    return 0               ; 返回值
]                          ; 结束代码块
```

### 核心特点

| 特点 | 说明 |
|------|------|
| `;` 注释 | 从分号到行尾都是注释 |
| `[]` 代码块 | 所有代码块使用方括号 |
| 零样板 | `print` 直接使用，无需 `import` |
| `new` 声明变量 | 统一的变量声明关键字 |

## 1.5 尝试修改

试着修改程序，添加以下内容：

```catlang
[
    ; 尝试不同的字面量类型
    new int_val = 42
    new float_val = 3.14159
    new hex_val = 0xFF
    new bool_val = true
    new string_val = "Hello"
    
    ; 输出它们
    print("整数：{int_val}")
    print("浮点：{float_val}")
    print("十六进制：{hex_val}")
    print("布尔：{bool_val}")
    print("字符串：{string_val}")
    
    return 0
]
```

## 1.6 下一步

现在你已经运行了第一个程序，继续学习：

- [第 00 章：环境配置](00_base_env.md) - 查看如何配置环境（如果还没配置）
- [第 02 章：基础语法](02_basic_syntax.md) - 深入学习变量、类型和运算符
- [语法规范](../syntax.txt) - 完整的语法规则参考

## 小测验

1. CatLang 使用什么符号表示注释？
2. 代码块使用什么括号？
3. 需要 `import` 才能使用 `print` 吗？

<details>
<summary>点击查看答案</summary>

1. 分号 `;`
2. 方括号 `[]`
3. 不需要，标准库功能自动注入
</details>
