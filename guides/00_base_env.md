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

## 0.2 Install the CatLang Compiler

You can get lastet stable catlang from release or you can build nightly version in your computer.
Then,you must add catc to PATH.

## 0.4 Configure Editor (Optional)

### VS Code Configuration

1. Create `.vscode/settings.json`:

```json
{
    "files.associations": {
        "*.cat": "catlang"
    },
    "catlang.compilerPath": "catc"
}
```

2. Create task configuration file `.vscode/tasks.json`:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Run CatLang",
            "type": "shell",
            "command": "catc ${file}",
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
catc test.cat
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
