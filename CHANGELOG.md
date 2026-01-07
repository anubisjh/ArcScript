# ArcScript Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-07

### Added
- **Core Language Features**
  - Variables with multiple types (int, float, bool, string, nil)
  - Arithmetic and logical operators
  - First-class functions with closure support
  - Control flow (if/elif/else, while loops)
  - Tables (key-value data structures)
  - Object declarations for game entity prototypes
  - Member access (`obj.field`) and index access (`obj["key"]`)

- **Built-in Functions**
  - `print()` - Output without newline
  - `println()` - Output with newline
  - `type()` - Get type of value
  - `len()` - Get length of string or table
  - `str()` - Convert to string
  - `int()` - Convert to integer
  - `float()` - Convert to float

- **Error Handling**
  - Structured parser errors with line/column information
  - Parser error recovery for better error reporting
  - Runtime errors with descriptive messages
  - Optional line number tracking in runtime errors
  - Division by zero detection
  - Type error checking

- **Developer Tools**
  - Interactive REPL mode (`cargo run repl`)
  - File execution (`cargo run script.arc`)
  - Comprehensive unit test suite (10+ tests)
  - GitHub Actions CI/CD pipeline

- **Documentation**
  - Comprehensive README with examples
  - Language guide (ArcScript_Guide.txt)
  - CONTRIBUTING.md with guidelines
  - Example scripts showcasing features
  - Grammar specification (EBNF)

- **Examples**
  - basic.arc - Variables, functions, arithmetic
  - control_flow.arc - Conditionals and loops
  - closures.arc - Closure capture
  - tables_objects.arc - Tables and objects
  - builtins.arc - Built-in functions
  - advanced.arc - Advanced patterns

- **Project Infrastructure**
  - MIT License
  - .gitignore for Rust projects
  - Test scripts (test.sh, test.ps1)
  - Professional README with badges

### Implementation Details
- **Lexer**: Token stream generation with keywords, operators, literals, comments
- **Parser**: Recursive descent with operator precedence and error recovery
- **AST**: Clean abstract syntax tree representation
- **Interpreter**: Tree-walking with environment-based scoping
- **Closures**: Functions capture their defining environment
- **Scoping**: Nested scopes with parent chaining, variable shadowing

### Future Plans
- Event system for game lifecycle hooks
- Bytecode VM for improved performance
- Game-specific API (vectors, input, timing)
- LSP server for IDE support
- Debugger integration
- Module system

## [0.0.1] - Initial Development

### Added
- Basic lexer implementation
- Simple parser for expressions
- Minimal interpreter
- Initial AST definitions

---

[0.1.0]: https://github.com/yourusername/arcscript/releases/tag/v0.1.0
[0.0.1]: https://github.com/yourusername/arcscript/releases/tag/v0.0.1
