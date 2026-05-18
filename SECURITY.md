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
- **Low / Medium / High:** Submit via [GitHub Private Vulnerability Reporting](./security/advisories/new).
- **Critical:**  
  1. Fetch our public key from `keys.openpgp.org`:  
     ```bash
     gpg --keyserver keys.openpgp.org --recv-keys B701965E2A5646227F1CC5EC8878A07DEE708DCE
     ```
  2. Encrypt your report and send to `ZWj1154142014@hotmail.com`.  
  ⚠️ **Do not disclose details publicly** until a fix is released.

We follow a coordinated disclosure policy: patches are prioritized for the next patch/LTS release, and public CVE publication is delayed until downstream users have had time to update.
