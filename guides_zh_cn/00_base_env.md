# 第 00 章：环境配置

本章介绍如何配置 CatLang 开发环境：下载 Zig 和编译/下载 CatLang 编译器。

## 0.1 下载 Zig

CatLang 使用 Zig 作为后端进行编译，因此需要先安装 Zig。

### Windows

1. 访问 Zig 官网：https://ziglang.org/download/
2. 下载适用于 Windows 的压缩包（如 `zig-windows-x86_64-0.11.0.zip`）
3. 解压到任意目录，例如 `C:\zig`
4. 将 Zig 添加到 PATH 环境变量：
   - 右键"此电脑" → "属性" → "高级系统设置"
   - 点击"环境变量"
   - 在"系统变量"中找到 `Path`，点击"编辑"
   - 添加 Zig 的 bin 目录路径（如 `C:\zig`）
5. 打开新的命令提示符，验证安装：

```bash
zig version
```

### macOS

使用 Homebrew 安装：

```bash
brew install zig
```

验证安装：

```bash
zig version
```

### Linux

使用包管理器或下载预编译版本：

```bash
# Ubuntu/Debian
sudo apt install zig

# 或者下载预编译版本
wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz
tar -xf zig-linux-x86_64-0.11.0.tar.xz
cd zig-linux-x86_64-0.11.0
sudo cp zig /usr/local/bin/
```

验证安装：

```bash
zig version
```

## 0.2 编译 CatLang 编译器

你可以直接从Releases界面下载最新的稳定版本，也可以自己在电脑上构建测试版。

## 0.4 配置编辑器（可选）

### VS Code 配置

1. 创建 `.vscode/settings.json`：

```json
{
    "files.associations": {
        "*.cat": "catlang"
    },
    "catlang.compilerPath": "./target/release/catlang.exe"
}
```

2. 创建任务配置文件 `.vscode/tasks.json`：

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "运行 CatLang",
            "type": "shell",
            "command": "./target/release/catlang.exe ${file}",
            "group": "build",
            "problemMatcher": []
        }
    ]
}
```

## 0.5 验证环境

创建测试文件 `test.cat`：

```cat
[
    print("环境配置成功！")
    return 0
]
```

运行测试：

```bash
# Windows
target\release\catlang.exe test.cat

# macOS/Linux
./target/release/catlang test.cat
```

如果输出 `环境配置成功！`，则环境配置完成。

## 0.6 常见问题

### Zig 找不到

确保 Zig 已正确添加到 PATH 环境变量，并重启终端。

### Cargo 编译失败

检查 Rust 版本：

```bash
rustc --version
cargo --version
```

如需更新 Rust：

```bash
rustup update
```

### 权限问题（macOS/Linux）

如果遇到权限错误：

```bash
chmod +x target/release/catlang
```

## 下一步

环境配置完成后，继续学习：

- [第 01 章：快速开始](01_getting_started.md) - 编写第一个 CatLang 程序
