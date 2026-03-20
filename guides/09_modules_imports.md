# Chapter 09: Modules & Imports

This chapter introduces CatLang's module system and third-party library import mechanism.

## 9.1 Import Principles

### Zero Boilerplate Design

CatLang uses a zero boilerplate design where standard library features are automatically injected into the global scope:

```catlang
[
    ; The following built-in features can be used directly without imports
    print("Hello")           ; Built-in IO
    sleep(100)               ; Built-in concurrency
    len("string")            ; Built-in utility function
    sqrt(2.0)                ; Built-in math function

    return 0
]
```

### Third-Party Libraries on Demand

Only third-party libraries need explicit imports:

```catlang
; Import third-party libraries
import numpy as np
from my_lib.utils import helper_func

[
    ; Use third-party libraries
    new arr = np.array([1, 2, 3])
    new result = helper_func(arr)

    return 0
]
```

## 9.2 Import Syntax

### Simple Imports

```catlang
; Import entire module
import module_name

; Import with alias
import module_name as alias
import very_long_module_name as vlmn
```

### From-Module Imports

```catlang
; Import single name from module
from module_name import function_name

; Import multiple names from module
from module_name import func1, func2, func3

; Import from module with alias
from module_name import original_name as alias
from module_name import func1 as f1, func2 as f2
```

### Nested Module Paths

```catlang
; Dot-separated module paths
import package.subpackage.module

; Import from nested module
from package.subpackage.module import function
```

## 9.3 Import Examples

### Math Library

```catlang
; Assuming a third-party math library
import math as m

[
    new result = m.sin(3.14159 / 2)
    print("sin(π/2) = {result}")

    new sqrt_val = m.sqrt(16)
    print("√16 = {sqrt_val}")

    return 0
]
```

### Utility Library

```catlang
; Import specific functions from utility library
from string_utils import capitalize, reverse, trim

[
    new text = "  hello world  "
    new trimmed = trim(text)
    new capitalized = capitalize(trimmed)

    print("Original: '{text}'")
    print("Processed: '{capitalized}'")

    return 0
]
```

### Data Science Library

```catlang
import numpy as np
import pandas as pd

[
    ; Create array using numpy
    new arr = np.array([1, 2, 3, 4, 5])

    ; Create DataFrame using pandas
    new df = pd.DataFrame({
        "name": ["Alice", "Bob"],
        "age": [25, 30]
    })

    return 0
]
```

### Network Library

```catlang
from http.client import get, post
from json import parse as json_parse

[
    try [
        new response = get("https://api.example.com/data")
        new data = json_parse(response.body)
        print("Got data: {data}")
    ] catch (e Any) [
        print("Request failed: {e}")
    ]

    return 0
]
```

## 9.4 Module Organization

### Package Structure Example

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

### Import Methods

```catlang
; Import submodule
import my_package.core.engine

; Import from submodule
from my_package.core import engine
from my_package.utils.helpers import validate_input

; Use aliases to simplify
import my_package.core.engine as engine
from my_package.utils import helpers as h
```

## 9.5 Comprehensive Examples

### Example 1: Web Application

```catlang
; Import web framework
import web_framework as web
from web_framework import Route, Request, Response
from database import connect as db_connect
from auth import verify_token, require_auth

; Create application
new app = web.App("MyApp")

; Define routes
@app.route("/api/users")
async fn get_users(req: Request) -> Response [
    try [
        new db = await db_connect()
        new users = await db.query("SELECT * FROM users")
        return Response { status: 200, body: users }
    ] catch (e Any) [
        return Response { status: 500, body: "Error: {e}" }
    ]
]

; Protected route
@app.route("/api/admin")
@require_auth
async fn admin_panel(req: Request) -> Response [
    new token = req.headers["Authorization"]
    new user = await verify_token(token)
    return Response { status: 200, body: "Welcome, {user.name}" }
]

[
    await app.run(port: 8080)
    return 0
]
```

### Example 2: Data Processing Pipeline

```catlang
; Import data processing libraries
import pandas as pd
import numpy as np
from ml_library import train, predict, evaluate
from visualization import plot, histogram, scatter

; Load data
new data = pd.read_csv("data.csv")

; Data cleaning
new cleaned = data.drop_na()
new normalized = cleaned.normalize()

; Feature engineering
new features = normalized.select_columns(["age", "income", "score"])
new labels = normalized.select_column("target")

; Train model
new model = train(features, labels, model_type: "random_forest")

; Evaluate
new predictions = predict(model, features)
new metrics = evaluate(labels, predictions)

print("Accuracy: {metrics.accuracy}")
print("F1 Score: {metrics.f1}")

; Visualize
scatter(features, labels, title: "Data Distribution")
histogram(predictions, title: "Prediction Distribution")
plot.show()
```

### Example 3: CLI Tool

```catlang
; Import CLI framework
import cli_framework as cli
from cli_framework import Command, Option, Argument

; Define command
new cmd = Command("mytool", "My command line tool")

cmd.add_subcommand(
    Command("greet", "Greet")
        .add_argument(Argument("name", "Name"))
        .add_option(Option("--formal", "Formal mode"))
)

cmd.add_subcommand(
    Command("calculate", "Calculate")
        .add_argument(Argument("a", "First number"))
        .add_argument(Argument("b", "Second number"))
        .add_option(Option("--op", "Operator", default: "+"))
)

; Handle command
async fn handle_command(ctx: cli.Context) [
    switch (ctx.command) [
        case "greet":
            if (ctx.has_option("formal")) [
                print("Hello, {ctx.args.name}!")
            ] else [
                print("Hi, {ctx.args.name}!")
            ]
        case "calculate":
            new a = parse_int(ctx.args.a)
            new b = parse_int(ctx.args.b)
            new op = ctx.get_option("op")

            switch (op) [
                case "+": print("Result: {a + b}")
                case "-": print("Result: {a - b}")
                case "*": print("Result: {a * b}")
                case "/": print("Result: {a / b}")
            ]
    ]
]

[
    await cmd.run(handle_command)
    return 0
]
```

### Example 4: Game Development

```catlang
; Import game engine
import game_engine as ge
from game_engine import Scene, Sprite, Camera, Input
from physics import Physics2D, Collider
from audio import play_sound, set_volume

; Create game scene
new scene = ge.Scene("MainScene")

; Create player sprite
new player = Sprite {
    texture: "player.png",
    position: (100, 100),
    size: (32, 32)
}

; Add physics component
new collider = Collider { shape: "circle", radius: 16 }
player.add_component(collider)

scene.add_entity(player)

; Game main loop
async fn game_loop() [
    new camera = Camera { position: (0, 0), zoom: 1.0 }
    new physics = Physics2D { gravity: (0, -9.8) }

    while (true) [
        ; Handle input
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

        ; Update physics
        physics.update(player, delta_time: 0.016)

        ; Render
        scene.render(camera)

        ; Limit frame rate
        await sleep(16)
    ]
]

[
    set_volume(0.8)
    await game_loop()
    return 0
]
```

## 9.6 Third-Party Library Management

### Dependency Declaration

```catlang
; Assuming a package management configuration file package.catlang
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

### Install Dependencies

```bash
# Install all dependencies
catpkg install

# Install specific package
catpkg install numpy

# Update dependencies
catpkg update
```

## 9.7 Best Practices

### 1. Use Meaningful Aliases

```catlang
; Good practice
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

; Bad practice
import numpy as n  ; Too short, unclear
import pandas as pandas  ; Unnecessary
```

### 2. Avoid Over-Importing

```catlang
; Good practice - import only what's needed
from utils import validate_email, validate_phone

; Bad practice - import entire module when only a few functions needed
import utils
; Then only use utils.validate_email()
```

### 3. Organize Import Order

```catlang
; 1. Standard library (though CatLang standard library doesn't need imports)
; 2. Third-party libraries
import numpy as np
import pandas as pd

; 3. Local modules
from my_package.core import engine
from my_package.utils import helpers

; 4. Relative imports (if supported)
from .sibling_module import function
```

### 4. Conditional Imports

```catlang
; Import based on platform
if (platform == "windows") [
    import windows_specific as ws
] else [
    import unix_specific as us
]

; Optional imports
try [
    import optional_feature as of
    new has_feature = true
] catch (e Any) [
    new has_feature = false
]
```

## 9.8 Exercises

1. Assuming there's a `statistics` library, import it and calculate the mean, median, and standard deviation of a dataset
2. Import the `get` function from a hypothetical `http` library, fetch API data and parse JSON response
3. Organize a multi-file project containing main program, utility functions, and data processing modules

<details>
<summary>Reference Answers</summary>

```catlang
; Exercise 1: Statistical calculations
import statistics as stats

[
    new data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    new mean = stats.mean(data)
    new median = stats.median(data)
    new std_dev = stats.std_dev(data)

    print("Mean: {mean}")
    print("Median: {median}")
    print("Standard deviation: {std_dev}")

    return 0
]

; Exercise 2: HTTP request
from http.client import get
from json import parse as json_parse

[
    try [
        new response = get("https://api.example.com/users")
        new users = json_parse(response.body)

        for (new user in users) [
            print("User: {user.name} - {user.email}")
        ]
    ] catch (e Any) [
        print("Request failed: {e}")
    ]

    return 0
]

; Exercise 3: Project structure
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
    ; Implement file loading
]

fn save_file(path: str, content: str) [
    ; Implement file saving
]

; data_processor.catlang
fn validate(data: str) -> bool [
    ; Implement validation logic
]

fn process(data: str) -> str [
    ; Implement processing logic
]
```
</details>

## Next Steps

- [Chapter 10: Best Practices](10_best_practices.md) - Code style, performance tips, common pitfalls
