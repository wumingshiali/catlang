# Security Policy

## Supported Versions

| Version | Supported                   |
| ------- | --------------------------- |
| LTS     | :white_check_mark:          |
| ESM     | Security updates only       |
| > 0.x   | :white_check_mark:          |
| ≤ 0.x   | :x:                         |

## Reporting a Vulnerability

Vulnerability Severity Classification:
| Vulnerability Type              | Impact                          | Severity |
| ------------------------------- | ------------------------------- | -------- |
| Compiler/Interpreter Logic Flaw | Denial of Service / Data Leak   | Medium   |
| Stdlib File/Path Handling       | Path Traversal / Unauthorized Access | High     |
| String/Encoding Mishandling     | Memory Corruption / RCE         | Critical |
| Sandbox/Unsafe Feature Escape   | Arbitrary Command Execution     | Critical |
| Toolchain/Dependency Supply Chain | Limited Scope / Local Impact  | Low      |
