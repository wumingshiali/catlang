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

### Submission Guidelines
- **All Severity Levels:** Submit via [GitHub Private Vulnerability Reporting](https://github.com/wumingshiali/catlang/security/advisories/new).  
  ⚠️ **Do not disclose details publicly** until a fix is released.

We follow a coordinated disclosure policy: patches are prioritized for the next patch/LTS release, and public CVE publication is delayed until downstream users have had time to update.
