# ArcScript

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/yourusername/arcscript/rust.yml?branch=main)](https://github.com/yourusername/arcscript/actions)

**ArcScript** is a modern, domain-specific scripting language designed for game logic and interactive applications. Built with Rust for performance and safety, ArcScript offers an intuitive syntax that's perfect for game designers, scripters, and developers.

## ✨ Features

- 🚀 **Fast Execution**: Built on Rust for high-performance script execution
- 🎯 **Simple Syntax**: Clean, readable syntax designed for game logic
- 🔧 **First-Class Functions**: Closures with lexical scoping
- 📦 **Tables & Objects**: Flexible data structures for game entities
- 🛠️ **Rich Standard Library**: Built-in functions for common operations
- 🔍 **Strong Error Handling**: Detailed error messages with line numbers
- 💻 **Interactive REPL**: Test code instantly with the built-in REPL
- 🎨 **Game-Focused**: Designed specifically for game scripting workflows

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/arcscript.git
cd arcscript

# Build the project
cargo build --release

# Run the demo
cargo run
```

### Running ArcScript

```bash
# Run a script file
cargo run path/to/script.arc

# Start the interactive REPL
cargo run repl

# Run built-in demo
cargo run
```

## 📖 Language Tour

### Variables and Types

```arcscript
var x = 42;              // Integer
var pi = 3.14159;        // Float
var name = "Alice";      // String
var active = true;       // Boolean
var nothing = nil;       // Nil
```

### Functions

```arcscript
func greet(name): {
    println("Hello,", name);
} end

func add(a, b): {
    return a + b;
} end

var result = add(5, 3);  // 8
```

### Closures

```arcscript
var x = 10;

func makeAdder(n): {
    func inner(val): {
        return x + n + val;
    } end
    return inner;
} end

var adder = makeAdder(5);
println(adder(3));  // 18
```

### Tables and Objects

```arcscript
// Table literal
var player = {
    name: "Hero",
    hp: 100,
    level: 5
};

println(player.name);     // "Hero"
println(player["hp"]);    // 100

// Object declaration
object Enemy: {
    var hp = 50;
    var damage = 10;
    
    func attack(): {
        return damage;
    } end
} end
```

### Control Flow

```arcscript
// If-Elif-Else
if x > 10 then {
    println("Large");
} elif x > 5 then {
    println("Medium");
} else {
    println("Small");
} end

// While loop
var i = 0;
while i < 5 do {
    println(i);
    i = i + 1;
} end

// For loop
for i = 1, 10, 1 do {
    println(i);
} end

// Loop control
for i = 1, 100, 1 do {
    if i > 5 then {
        break;
    } end
    println(i);
} end
```

### Built-in Functions

```arcscript
// Output
print("Hello");          // Print without newline
println("World!");       // Print with newline

// Type operations
type(42);                // "int"
len("Hello");            // 5
len({a: 1, b: 2});      // 2

// Type conversions
int("42");               // 42
float(10);               // 10.0
str(42);                 // "42"

// Math functions
abs(-10);                // 10
min(5, 10);              // 5
max(5, 10);              // 10
sqrt(16);                // 4.0
pow(2, 8);               // 256.0
floor(3.7);              // 3
ceil(3.2);               // 4
round(3.5);              // 4

// String functions
substring("Hello", 0, 2);    // "He"
contains("test", "es");      // true
toUpper("hello");            // "HELLO"
toLower("WORLD");            // "world"
```

## 📚 Examples

Check out the `examples/` directory for comprehensive examples:

- **[basic.arc](examples/basic.arc)** - Variables, functions, and arithmetic
- **[control_flow.arc](examples/control_flow.arc)** - Conditionals and loops
- **[loops.arc](examples/loops.arc)** - For loops with break/continue
- **[closures.arc](examples/closures.arc)** - Closure capture and nested functions
- **[tables_objects.arc](examples/tables_objects.arc)** - Tables and object declarations
- **[builtins.arc](examples/builtins.arc)** - Built-in functions showcase
- **[math.arc](examples/math.arc)** - Math functions (abs, sqrt, pow, etc.)
- **[strings.arc](examples/strings.arc)** - String operations (substring, contains, etc.)
- **[advanced.arc](examples/advanced.arc)** - Advanced features and patterns

Run any example with:
```bash
cargo run examples/basic.arc
```

## 🏗️ Architecture

ArcScript follows a classic interpreter pipeline:

```
Source Code → Lexer → Tokens → Parser → AST → Interpreter → Execution
```

- **Lexer** ([src/lexer.rs](src/lexer.rs)): Tokenization with support for keywords, operators, literals, and comments
- **Parser** ([src/parser.rs](src/parser.rs)): Recursive descent parser with error recovery
- **AST** ([src/ast.rs](src/ast.rs)): Clean abstract syntax tree representation
- **Interpreter** ([src/interpreter.rs](src/interpreter.rs)): Tree-walking interpreter with environment-based scoping

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_closures
```

## 🛣️ Roadmap

- [x] Core language features (variables, functions, control flow)
- [x] **For loops** with customizable step
- [x] **Break/Continue** for loop control
- [x] Closures with lexical scoping
- [x] Tables and object declarations
- [x] **Math functions** (abs, min, max, sqrt, pow, floor, ceil, round)
- [x] **String functions** (substring, contains, toUpper, toLower)
- [x] Standard library (print, type, len, conversions)
- [x] REPL and file execution
- [x] Error handling with line numbers
- [ ] Event system for game lifecycle hooks
- [ ] Bytecode VM for improved performance
- [ ] Game-specific API (vectors, input, timing)
- [ ] LSP server for IDE support
- [ ] Debugger integration
- [ ] Module system

## 📝 Recent Updates (January 2026)

Latest improvements include:
- ✅ **For Loops**: `for var = start, end, step do { ... } end`
- ✅ **Loop Control**: `break` and `continue` statements
- ✅ **Math Functions**: `abs`, `min`, `max`, `floor`, `ceil`, `round`, `sqrt`, `pow`
- ✅ **String Functions**: `substring`, `contains`, `toUpper`, `toLower`
- ✅ **Built-in Functions**: `print`, `println`, `type`, `len`, `str`, `int`, `float`
- ✅ **File Execution**: Run ArcScript files directly from the command line
- ✅ **Enhanced Errors**: Runtime error messages now include line numbers
- ✅ **New Examples**: 9 example files showcasing all features
- ✅ **Comprehensive Tests**: 10+ unit tests covering all language features
- ✅ **Professional Setup**: MIT License, CI/CD with GitHub Actions

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📖 Documentation

For detailed language reference, see [docs/ArcScript_Guide.txt](docs/ArcScript_Guide.txt).

## 🙏 Acknowledgments

- Inspired by Lua, JavaScript, and Python's simplicity
- Built with Rust for performance and safety
- Designed for game developers and interactive applications

---

**Made with ❤️ for game developers**
