# CatLang Tutorial Index

Welcome to the CatLang programming language tutorial! This tutorial will take you from zero to mastery in CatLang.

## Tutorial Directory

| Chapter | Title | Description |
|------|------|------|
| [00](00_base_env.md) | Environment Setup | Download Zig, compile/download CatLang |
| [01](01_getting_started.md) | Quick Start | First program, compile and run |
| [02](02_basic_syntax.md) | Basic Syntax | Variables, types, operators, comments |
| [03](03_control_flow.md) | Control Flow | Conditions, loops, pattern matching |
| [04](04_functions.md) | Functions | Definition, parameters, return values, async functions |
| [05](05_data_structures.md) | Data Structures | Structs, arrays, pointers, generics |
| [06](06_error_handling.md) | Error Handling | try/catch, throw, custom errors |
| [07](07_memory_management.md) | Memory Management | unsafe blocks, memory reinterpretation, cpy |
| [08](08_concurrency.md) | Concurrency | async/await, spawn tasks |
| [09](09_modules_imports.md) | Modules & Imports | Third-party library imports, module organization |
| [10](10_best_practices.md) | Best Practices | Code style, common pitfalls, performance tips |
| [11](11_type_system.md) | Deep Dive into Type System | Generic types, arbitrary bit-width types |

## Quick Navigation

- **Getting Started**: Start with [Chapter 00](00_base_env.md) to set up your environment
- **Look up Syntax**: Refer to [Basic Syntax](02_basic_syntax.md) or [Syntax Specification](../syntax.txt)
- **Type System**: Check [Deep Dive into Type System](11_type_system.md) for generics and arbitrary bit-width types
- **Example Code**: See `benchmark/` and `test/` directories

## About CatLang

CatLang is a systems programming language that focuses on the following design principles:

1. **Zero Boilerplate** - Standard library features are automatically injected, no imports needed
2. **On-Demand Imports** - Only third-party libraries require explicit imports
3. **Safety-Control Balance** - Safe by default, with low-level control via `unsafe`
4. **Distinctive Syntax** - Square bracket code blocks `[]`, semicolon comments `;`
5. **Flexible Type System** - Support for generic types and arbitrary bit-width types
