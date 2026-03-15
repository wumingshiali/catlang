# CatLang 教程索引

欢迎来到 CatLang 编程语言教程！本教程将带你从零开始学习 CatLang。

## 教程目录

| 章节 | 标题 | 描述 |
|------|------|------|
| [00](00_base_env.md) | 环境配置 | 下载 Zig、编译/下载 CatLang |
| [01](01_getting_started.md) | 快速开始 | 第一个程序、编译运行 |
| [02](02_basic_syntax.md) | 基础语法 | 变量、类型、运算符、注释 |
| [03](03_control_flow.md) | 控制流 | 条件、循环、模式匹配 |
| [04](04_functions.md) | 函数 | 定义、参数、返回值、异步函数 |
| [05](05_data_structures.md) | 数据结构 | 结构体、数组、指针 |
| [06](06_error_handling.md) | 错误处理 | try/catch、throw、自定义错误 |
| [07](07_memory_management.md) | 内存管理 | unsafe 块、内存重解释、cpy |
| [08](08_concurrency.md) | 并发编程 | async/await、spawn 任务 |
| [09](09_modules_imports.md) | 模块与导入 | 第三方库导入、模块组织 |
| [10](10_best_practices.md) | 最佳实践 | 代码风格、常见陷阱、性能提示 |

## 快速导航

- **新手入门**：从 [第 00 章](00_base_env.md) 开始配置环境
- **查阅语法**：参考 [基础语法](02_basic_syntax.md) 或 [语法规范](../syntax.txt)
- **示例代码**：查看 `benchmark/` 和 `test/` 目录

## 关于 CatLang

CatLang 是一门注重以下设计原则的系统级编程语言：

1. **零样板** - 标准库功能自动注入，无需导入
2. **按需导入** - 仅第三方库需要显式导入
3. **安全与控制平衡** - 默认安全，通过 `unsafe` 提供底层控制
4. **高辨识度语法** - 方括号代码块 `[]`，分号注释 `;`
