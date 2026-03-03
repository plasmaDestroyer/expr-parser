# Expression Parser

An expression parser and evaluator with variable support, built from scratch in Rust.

## Architecture
```
Input → Lexer → [Tokens] → Parser → AST → Evaluator → Result
```

| Stage     | Description                                                      |
| --------- | ---------------------------------------------------------------- |
| Lexer     | Char-by-char tokenization into numbers, operators, and variables |
| Parser    | Recursive descent parser that builds an AST with precedence      |
| Evaluator | Tree-walking evaluator with a variable store (`HashMap`)         |

## Features
| Feature              | Description                                          |
| -------------------- | ---------------------------------------------------- |
| Basic math           | `+`, `-`, `*`, `/` operations                        |
| Operator precedence  | `*` and `/` bind tighter than `+` and `-`            |
| Variables            | Assign with `x = 10`, use in expressions like `x + 5`|
| REPL                 | Interactive read-eval-print loop                     |
| Error handling       | Graceful `Result`-based error propagation            |

## Usage
### Building
```
cargo build
```

### Running
```
cargo run
```

## Example
```
Enter an expression to evaluate: (type 'exit' to quit)
> x = 10
10

> y = 20
20

> x + y * 2
50

> z = x * y + 5
205

> exit
Exiting program!
```

## Error Handling
All errors are handled gracefully using Rust's `Result` type — division by zero, unknown characters, and undefined variables return descriptive error messages without crashing the REPL.
```
> 10 / 0
Division by Zero not allowed!

> x.y
Unknown Character: .

> unknown_var + 1
Undefined Variable!
```