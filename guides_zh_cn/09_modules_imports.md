# 第 09 章：模块与导入

本章介绍 CatLang 的模块系统和第三方库导入机制。

## 9.1 导入原则

### 零样板设计

CatLang 采用零样板设计，标准库功能自动注入全局作用域：

```catlang
[
    ; 以下内置功能无需导入，直接使用
    print("Hello")           ; 内置 IO
    sleep(100)               ; 内置并发
    len("string")            ; 内置工具函数
    sqrt(2.0)                ; 内置数学函数
    
    return 0
]
```

### 第三方库按需导入

只有使用第三方库时才需要显式导入：

```catlang
; 导入第三方库
import numpy as np
from my_lib.utils import helper_func

[
    ; 使用第三方库
    new arr = np.array([1, 2, 3])
    new result = helper_func(arr)
    
    return 0
]
```

## 9.2 导入语法

### 简单导入

```catlang
; 导入整个模块
import module_name

; 导入并设置别名
import module_name as alias
import very_long_module_name as vlmn
```

### 从模块导入

```catlang
; 从模块导入单个名称
from module_name import function_name

; 从模块导入多个名称
from module_name import func1, func2, func3

; 从模块导入并设置别名
from module_name import original_name as alias
from module_name import func1 as f1, func2 as f2
```

### 嵌套模块路径

```catlang
; 点号分隔的模块路径
import package.subpackage.module

; 从嵌套模块导入
from package.subpackage.module import function
```

## 9.3 导入示例

### 数学库

```catlang
; 假设有一个第三方数学库
import math as m

[
    new result = m.sin(3.14159 / 2)
    print("sin(π/2) = {result}")
    
    new sqrt_val = m.sqrt(16)
    print("√16 = {sqrt_val}")
    
    return 0
]
```

### 工具库

```catlang
; 从工具库导入特定函数
from string_utils import capitalize, reverse, trim

[
    new text = "  hello world  "
    new trimmed = trim(text)
    new capitalized = capitalize(trimmed)
    
    print("原始：'{text}'")
    print("处理后：'{capitalized}'")
    
    return 0
]
```

### 数据科学库

```catlang
import numpy as np
import pandas as pd

[
    ; 使用 numpy 创建数组
    new arr = np.array([1, 2, 3, 4, 5])
    
    ; 使用 pandas 创建 DataFrame
    new df = pd.DataFrame({
        "name": ["Alice", "Bob"],
        "age": [25, 30]
    })
    
    return 0
]
```

### 网络库

```catlang
from http.client import get, post
from json import parse as json_parse

[
    try [
        new response = get("https://api.example.com/data")
        new data = json_parse(response.body)
        print("获取数据：{data}")
    ] catch (e Any) [
        print("请求失败：{e}")
    ]
    
    return 0
]
```

## 9.4 模块组织

### 包结构示例

```
my_package/
├── __init__.catlang
├── core/
│   ├── __init__.catlang
│   ├── engine.catlang
│   └── processor.catlang
├── utils/
│   ├── __init__.catlang
│   ├── helpers.catlang
│   └── validators.catlang
└── api/
    ├── __init__.catlang
    └── client.catlang
```

### 导入方式

```catlang
; 导入子模块
import my_package.core.engine

; 从子模块导入
from my_package.core import engine
from my_package.utils.helpers import validate_input

; 使用别名简化
import my_package.core.engine as engine
from my_package.utils import helpers as h
```

## 9.5 综合示例

### 示例 1：Web 应用

```catlang
; 导入 Web 框架
import web_framework as web
from web_framework import Route, Request, Response
from database import connect as db_connect
from auth import verify_token, require_auth

; 创建应用
new app = web.App("MyApp")

; 定义路由
@app.route("/api/users")
async fn get_users(req: Request) -> Response [
    try [
        new db = await db_connect()
        new users = await db.query("SELECT * FROM users")
        return Response { status: 200, body: users }
    ] catch (e Any) [
        return Response { status: 500, body: "错误：{e}" }
    ]
]

; 受保护的路由
@app.route("/api/admin")
@require_auth
async fn admin_panel(req: Request) -> Response [
    new token = req.headers["Authorization"]
    new user = await verify_token(token)
    return Response { status: 200, body: "欢迎，{user.name}" }
]

[
    await app.run(port: 8080)
    return 0
]
```

### 示例 2：数据处理管道

```catlang
; 导入数据处理库
import pandas as pd
import numpy as np
from ml_library import train, predict, evaluate
from visualization import plot, histogram, scatter

; 加载数据
new data = pd.read_csv("data.csv")

; 数据清洗
new cleaned = data.drop_na()
new normalized = cleaned.normalize()

; 特征工程
new features = normalized.select_columns(["age", "income", "score"])
new labels = normalized.select_column("target")

; 训练模型
new model = train(features, labels, model_type: "random_forest")

; 评估
new predictions = predict(model, features)
new metrics = evaluate(labels, predictions)

print("准确率：{metrics.accuracy}")
print("F1 分数：{metrics.f1}")

; 可视化
scatter(features, labels, title: "数据分布")
histogram(predictions, title: "预测分布")
plot.show()
```

### 示例 3：CLI 工具

```catlang
; 导入 CLI 框架
import cli_framework as cli
from cli_framework import Command, Option, Argument

; 定义命令
new cmd = Command("mytool", "我的命令行工具")

cmd.add_subcommand(
    Command("greet", "打招呼")
        .add_argument(Argument("name", "姓名"))
        .add_option(Option("--formal", "正式模式"))
)

cmd.add_subcommand(
    Command("calculate", "计算")
        .add_argument(Argument("a", "第一个数"))
        .add_argument(Argument("b", "第二个数"))
        .add_option(Option("--op", "运算符", default: "+"))
)

; 处理命令
async fn handle_command(ctx: cli.Context) [
    switch (ctx.command) [
        case "greet":
            if (ctx.has_option("formal")) [
                print("您好，{ctx.args.name}!")
            ] else [
                print("嗨，{ctx.args.name}!")
            ]
        case "calculate":
            new a = parse_int(ctx.args.a)
            new b = parse_int(ctx.args.b)
            new op = ctx.get_option("op")
            
            switch (op) [
                case "+": print("结果：{a + b}")
                case "-": print("结果：{a - b}")
                case "*": print("结果：{a * b}")
                case "/": print("结果：{a / b}")
            ]
    ]
]

[
    await cmd.run(handle_command)
    return 0
]
```

### 示例 4：游戏开发

```catlang
; 导入游戏引擎
import game_engine as ge
from game_engine import Scene, Sprite, Camera, Input
from physics import Physics2D, Collider
from audio import play_sound, set_volume

; 创建游戏场景
new scene = ge.Scene("MainScene")

; 创建玩家精灵
new player = Sprite {
    texture: "player.png",
    position: (100, 100),
    size: (32, 32)
}

; 添加物理组件
new collider = Collider { shape: "circle", radius: 16 }
player.add_component(collider)

scene.add_entity(player)

; 游戏主循环
async fn game_loop() [
    new camera = Camera { position: (0, 0), zoom: 1.0 }
    new physics = Physics2D { gravity: (0, -9.8) }
    
    while (true) [
        ; 处理输入
        if (Input.is_pressed("LEFT")) [
            player.position.x = player.position.x - 5
        ]
        if (Input.is_pressed("RIGHT")) [
            player.position.x = player.position.x + 5
        ]
        if (Input.is_pressed("SPACE")) [
            player.velocity.y = 10
            play_sound("jump.wav")
        ]
        
        ; 更新物理
        physics.update(player, delta_time: 0.016)
        
        ; 渲染
        scene.render(camera)
        
        ; 限制帧率
        await sleep(16)
    ]
]

[
    set_volume(0.8)
    await game_loop()
    return 0
]
```

## 9.6 第三方库管理

### 依赖声明

```catlang
; 假设有一个包管理配置文件 package.catlang
[package]
name = "my_app"
version = "1.0.0"

[dependencies]
numpy = "1.24.0"
pandas = "2.0.0"
web_framework = "0.5.0"

[dev-dependencies]
test_framework = "1.0.0"
```

### 安装依赖

```bash
# 安装所有依赖
catpkg install

# 安装特定包
catpkg install numpy

# 更新依赖
catpkg update
```

## 9.7 最佳实践

### 1. 使用有意义的别名

```catlang
; 好的做法
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

; 不好的做法
import numpy as n  ; 太短，不清晰
import pandas as pandas  ; 没必要
```

### 2. 避免过度导入

```catlang
; 好的做法 - 只导入需要的
from utils import validate_email, validate_phone

; 不好的做法 - 导入整个模块当只需要几个函数
import utils
; 然后只用 utils.validate_email()
```

### 3. 组织导入顺序

```catlang
; 1. 标准库（虽然 CatLang 标准库无需导入）
; 2. 第三方库
import numpy as np
import pandas as pd

; 3. 本地模块
from my_package.core import engine
from my_package.utils import helpers

; 4. 相对导入（如果支持）
from .sibling_module import function
```

### 4. 条件导入

```catlang
; 根据平台导入
if (platform == "windows") [
    import windows_specific as ws
] else [
    import unix_specific as us
]

; 可选导入
try [
    import optional_feature as of
    new has_feature = true
] catch (e Any) [
    new has_feature = false
]
```

## 9.8 练习

1. 假设有一个 `statistics` 库，导入并计算一组数据的平均值、中位数和标准差
2. 从一个假设的 `http` 库导入 `get` 函数，获取 API 数据并解析 JSON 响应
3. 组织一个多文件项目，包含主程序、工具函数和数据处理模块

<details>
<summary>参考答案</summary>

```catlang
; 练习 1：统计计算
import statistics as stats

[
    new data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    
    new mean = stats.mean(data)
    new median = stats.median(data)
    new std_dev = stats.std_dev(data)
    
    print("平均值：{mean}")
    print("中位数：{median}")
    print("标准差：{std_dev}")
    
    return 0
]

; 练习 2：HTTP 请求
from http.client import get
from json import parse as json_parse

[
    try [
        new response = get("https://api.example.com/users")
        new users = json_parse(response.body)
        
        for (new user in users) [
            print("用户：{user.name} - {user.email}")
        ]
    ] catch (e Any) [
        print("请求失败：{e}")
    ]
    
    return 0
]

; 练习 3：项目结构
; main.catlang
import utils
from data_processor import process, validate

[
    new raw_data = utils.load_file("input.txt")
    
    if (validate(raw_data)) [
        new result = process(raw_data)
        utils.save_file("output.txt", result)
    ]
    
    return 0
]

; utils.catlang
fn load_file(path: str) -> str [
    ; 实现加载文件
]

fn save_file(path: str, content: str) [
    ; 实现保存文件
]

; data_processor.catlang
fn validate(data: str) -> bool [
    ; 实现验证逻辑
]

fn process(data: str) -> str [
    ; 实现处理逻辑
]
```
</details>

## 下一步

- [第 10 章：最佳实践](10_best_practices.md) - 代码风格、性能提示、常见陷阱
