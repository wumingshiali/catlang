# Chapter 00: Environment Setup

This chapter introduces how to set up the CatLang development environment: download Zig and compile/download the CatLang compiler.

## 0.1 Download Zig

CatLang uses Zig as its backend for compilation, so Zig must be installed first.

### Windows

1. Visit the Zig official website: https://ziglang.org/download/
2. Download the package for Windows (e.g., `zig-windows-x86_64-0.11.0.zip`)
3. Extract to any directory, e.g., `C:\zig`
4. Add Zig to the PATH environment variable:
   - Right-click "This PC" → "Properties" → "Advanced System Settings"
   - Click "Environment Variables"
   - Find `Path` in "System variables" and click "Edit"
   - Add the Zig bin directory path (e.g., `C:\zig`)
5. Open a new command prompt and verify the installation:

```bash
zig version
```

### macOS

Install using Homebrew:

```bash
brew install zig
```

Verify the installation:

```bash
zig version
```

### Linux

Use package manager or download pre-built version:

```bash
# Ubuntu/Debian
sudo apt install zig

# Or download pre-built version
wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz
tar -xf zig-linux-x86_64-0.11.0.tar.xz
cd zig-linux-x86_64-0.11.0
sudo cp zig /usr/local/bin/
```

Verify the installation:

```bash
zig version
```

## 0.2 Compile the CatLang Compiler

The CatLang compiler is written in Rust and requires the Rust toolchain to compile.

### Install Rust

Visit https://rustup.rs/ to download and install Rust:

```bash
# Windows (PowerShell)
winget install Rustlang.Rustup

# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Compile CatLang

1. Clone or download the CatLang project

```bash
git clone https://github.com/your-org/catlang.git
cd catlang
```

2. Compile using Cargo

```bash
# Development version
cargo build

# Release version (optimized)
cargo build --release
```

3. Build output location

```
# Windows
target\debug\catlang.exe      # Development version
target\release\catlang.exe    # Release version

# macOS/Linux
target/debug/catlang          # Development version
target/release/catlang        # Release version
```

4. Verify the compiler

```bash
# Windows
target\release\catlang.exe --version

# macOS/Linux
./target/release/catlang --version
```

## 0.3 Download Pre-built Version

If you don't want to compile it yourself, you can download the pre-built CatLang compiler.

### Release Version (Not Available Yet)

Visit the project's Releases page to download the pre-built version for your platform:

```
N/A
```

### Extract and Use

```bash
# Windows - Extract and run catlang.exe directly
# macOS/Linux - May need to add execute permission
chmod +x catlang
```

## 0.4 Configure Editor (Optional)

### VS Code Configuration (Not Available Yet)

1. Install CatLang language support extension (if available)
2. Create `.vscode/settings.json`:

```json
{
    "files.associations": {
        "*.cat": "catlang"
    },
    "catlang.compilerPath": "./target/release/catlang.exe"
}
```

3. Create task configuration file `.vscode/tasks.json`: (This is available)

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Run CatLang",
            "type": "shell",
            "command": "./target/release/catlang.exe ${file}",
            "group": "build",
            "problemMatcher": []
        }
    ]
}
```

## 0.5 Verify Environment

Create test file `test.cat`:

```cat
[
    print("Environment setup successful!")
    return 0
]
```

Run the test:

```bash
# Windows
target\release\catlang.exe test.cat

# macOS/Linux
./target/release/catlang test.cat
```

If it outputs `Environment setup successful!`, the environment is configured.

## 0.6 Common Issues

### Zig Not Found

Ensure Zig is correctly added to the PATH environment variable and restart the terminal.

### Cargo Compilation Failed

Check Rust version:

```bash
rustc --version
cargo --version
```

To update Rust:

```bash
rustup update
```

### Permission Issues (macOS/Linux)

If you encounter permission errors:

```bash
chmod +x target/release/catlang
```

## Next Steps

After completing the environment setup, continue learning:

- [Chapter 01: Quick Start](01_getting_started.md) - Write your first CatLang program
