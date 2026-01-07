# ArcScript

ArcScript is a domain-specific scripting language for game logic.

This repository contains a complete implementation (lexer, parser, AST, interpreter) in Rust with support for functions, closures, objects, tables, and control flow.

## Features

- **Variables & Types**: Int, Float, Bool, String, Tables, Functions
- **Control Flow**: if/elif/else, while loops
- **Functions**: First-class functions with closures
- **Tables & Objects**: Dictionary-like tables with member/index access
- **Object Declarations**: Game entity prototypes with methods and properties
- **Error Handling**: Structured parse and runtime errors with recovery
- **REPL**: Interactive mode for experimentation

## Recent changes (Jan 2026)

- ✅ Implemented nested scopes for blocks and function calls (shadowing supported).
- ✅ Functions now return values and can access outer (parent) scopes.
- ✅ Added closures: functions capture their defining environment.
- ✅ Implemented table literals `{key: value}` and member/index access (`obj.field`, `obj["key"]`).
- ✅ Added object declarations with `object Name: { ... } end`.
- ✅ Lexer: added `!` (logical not) token, string escape sequences (\n, \t, \", etc.), and block comments `/* */`.
- ✅ Structured parser errors with recovery and runtime errors (division by zero, type errors, etc.).
- ✅ Added REPL mode (`cargo run repl`) for interactive scripting.
- ✅ Created example scripts in `examples/` directory.
- ✅ Added comprehensive unit tests and GitHub Actions CI workflow.

## Running

```bash
# Run demo script
cargo run

# Run REPL
cargo run repl

# Run tests
cargo test
```

## Examples

See `examples/` for sample scripts demonstrating:
- `basic.arc` - Variables, functions, arithmetic
- `tables_objects.arc` - Tables and object declarations
- `closures.arc` - Closure capture and nested functions
- `control_flow.arc` - Loops and conditionals

## Documentation

See `docs/ArcScript_Guide.txt` for detailed language reference.
