# ArcScript

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/yourusername/arcscript/rust.yml?branch=main)](https://github.com/yourusername/arcscript/actions)

**ArcScript** is a scripting language I built specifically for game development and interactive applications. Think of it as a blend of Lua's simplicity and Python's readability, but tailored for game logic. It's written in Rust, so it's fast and reliable, perfect for when you need to script complex game behavior without worrying about crashes or performance issues.

Whether you're building an RPG with complex character systems, a puzzle game with intricate mechanics, or just need a clean way to script your game's logic, ArcScript has you covered.

## Why ArcScript?

I wanted a scripting language that feels natural to write and doesn't get in your way when you're prototyping game ideas. Here's what makes ArcScript different:

- **Genuinely Fast**: Built on Rust's foundation, so your scripts run smoothly even in complex game loops
- **Actually Readable**: No weird syntax quirks. If you can read basic code, you can read ArcScript
- **Real Closures**: First-class functions that actually work the way you'd expect them to
- **Flexible Data Structures**: Tables and objects that make sense for game entities and state management
- **Complete Standard Library**: All the math and string functions you need, already built in
- **Helpful Error Messages**: When something breaks, you'll know exactly where and why
- **Interactive REPL**: Test your game logic on the fly without recompiling anything
- **Built for Games**: Every feature was chosen with game development in mind

## Getting Started

### Installation

First, make sure you have Rust installed (you'll need version 1.70 or newer). Then:

```bash
# Clone the repository
git clone https://github.com/yourusername/arcscript.git
cd arcscript

# Build it (this might take a minute the first time)
cargo build --release

# Try the demo to see it in action
cargo run
```

### Your First Script

Let's run something simple:

```bash
# Execute a script file
cargo run path/to/script.arc

# Or jump into the interactive REPL to experiment
cargo run repl

# The default demo is pretty cool too
cargo run
```

## Learn the Language

### Variables Are Simple

ArcScript keeps types straightforward. Just use `var` and let the language figure it out:

```arcscript
var health = 100;        // Integers for whole numbers
var speed = 2.5;         // Floats for decimals
var name = "Warrior";    // Strings for text
var alive = true;        // Booleans for logic
var treasure = nil;      // Nil when you need "nothing"
```

### Functions Work How You'd Expect

No complicated syntax here, just write what you mean:

```arcscript
func greet(name): {
    println("Hello,", name);
} end

func add(a, b): {
    return a + b;
} end

var result = add(5, 3);  // Result is 8
```

### Closures Are Actually Useful

This is where things get interesting. Closures let you create functions that remember their environment:

```arcscript
var multiplier = 10;

func makeScaler(factor): {
    func scale(value): {
        return multiplier * factor * value;
    } end
    return scale;
} end

var doubleScale = makeScaler(2);
println(doubleScale(5));  // Outputs: 100
```

This is perfect for creating custom game logic, like damage calculators that scale with player stats.

### Tables and Objects for Game Data

Tables are your go-to for storing related data:

```arcscript
// Create a player with properties
var player = {
    name: "Hero",
    hp: 100,
    level: 5,
    gold: 250
};

println(player.name);     // "Hero"
println(player["hp"]);    // 100 (both notations work)

// Objects let you define reusable templates
object Enemy: {
    var hp = 50;
    var damage = 10;
    
    func attack(): {
        return damage;
    } end
} end

// Now every enemy starts with these stats
var enemyDamage = Enemy.attack();
```

### Control Flow That Makes Sense

ArcScript's control flow reads like plain English:

```arcscript
// Conditionals with elif for multiple branches
if health > 80 then {
    println("Feeling strong!");
} elif health > 40 then {
    println("Getting hurt...");
} else {
    println("Critical condition!");
} end

// While loops for ongoing checks
var countdown = 5;
while countdown > 0 do {
    println("T-minus", countdown);
    countdown -= 1;  // Yes, we have compound assignments!
} end

// For loops when you know the range
for i = 1, 10, 1 do {
    println("Wave", i);
} end

// Loop control when you need it
for i = 1, 100, 1 do {
    if i > 5 then {
        break;  // Exit the loop early
    } end
    if i % 2 == 0 then {
        continue;  // Skip even numbers
    } end
    println(i);
} end
```

### Built-in Functions You'll Actually Use

I included the functions I found myself needing over and over:

```arcscript
// Output to console
print("HP: ");           // Prints without newline
println("100/100");      // Adds newline at the end

// Check types and lengths
type(42);                // Returns "int"
len("Hello");            // Returns 5
len({a: 1, b: 2});      // Returns 2 (table entries)

// Convert between types
int("42");               // String to integer: 42
float(10);               // Integer to float: 10.0
str(42);                 // Integer to string: "42"

// Math operations (super useful for game calculations)
abs(-10);                // Absolute value: 10
min(5, 10);              // Smaller value: 5
max(5, 10);              // Larger value: 10
sqrt(16);                // Square root: 4.0
pow(2, 8);               // Power: 256.0
floor(3.7);              // Round down: 3
ceil(3.2);               // Round up: 4
round(3.5);              // Round nearest: 4

// String manipulation
substring("Hello", 0, 2);     // Extract: "He"
contains("test", "es");       // Check if contains: true
toUpper("hello");             // Uppercase: "HELLO"
toLower("WORLD");             // Lowercase: "world"
```

## Real Examples

I've included a bunch of working examples in the `examples/` directory. Here's what each one shows:

- **[basic.arc](examples/basic.arc)** - Start here: variables, functions, and basic arithmetic
- **[control_flow.arc](examples/control_flow.arc)** - If statements, loops, and branching logic
- **[loops.arc](examples/loops.arc)** - For loops with break and continue
- **[closures.arc](examples/closures.arc)** - How closures work and why they're useful
- **[tables_objects.arc](examples/tables_objects.arc)** - Working with data structures
- **[builtins.arc](examples/builtins.arc)** - Tour of all built-in functions
- **[math.arc](examples/math.arc)** - Math operations for game calculations
- **[strings.arc](examples/strings.arc)** - String manipulation techniques
- **[operators.arc](examples/operators.arc)** - Modulo and compound assignments in action
- **[advanced.arc](examples/advanced.arc)** - Putting it all together

Try them out:
```bash
cargo run examples/basic.arc
```

## How It Works

ArcScript uses a straightforward interpreter architecture that's easy to understand and extend:

```
Your Code → Lexer → Tokens → Parser → AST → Interpreter → Results
```

Each component does one job well:

- **Lexer** ([src/lexer.rs](src/lexer.rs)): Breaks your code into tokens (keywords, numbers, operators, etc.)
- **Parser** ([src/parser.rs](src/parser.rs)): Builds a syntax tree from tokens, with error recovery if something's wrong
- **AST** ([src/ast.rs](src/ast.rs)): Clean representation of your program structure
- **Interpreter** ([src/interpreter.rs](src/interpreter.rs)): Walks the tree and executes your code

No magic, no hidden complexity, just a clean pipeline that's easy to debug and extend.

## Testing

The test suite covers all major features:

```bash
# Run all tests
cargo test

# See detailed output
cargo test -- --nocapture

# Run a specific test
cargo test test_closures
```

## What's Next

Here's what's already done and what's coming:

**Completed:**
- [x] Core language (variables, functions, control flow)
- [x] For loops with customizable step values
- [x] Break and continue statements
- [x] Modulo operator (%) for remainders
- [x] Compound assignments (+=, -=, *=, /=)
- [x] Closures with proper lexical scoping
- [x] Tables and object declarations
- [x] Complete math library (abs, min, max, sqrt, pow, floor, ceil, round)
- [x] String manipulation (substring, contains, toUpper, toLower)
- [x] Standard library (print, type, len, conversions)
- [x] REPL and file execution
- [x] Error handling with line numbers

**On the Roadmap:**
- [ ] Event system for game lifecycle (update, collision, etc.)
- [ ] Bytecode VM for better performance
- [ ] Game-specific APIs (vectors, timers, input handling)
- [ ] Language server for IDE support
- [ ] Debugger integration
- [ ] Module system for organizing larger projects

## Recent Changes

Just wrapped up some major improvements (January 2026):

**New Language Features:**
- For loops that let you control start, end, and step: `for i = 0, 100, 5 do`
- Break and continue for fine control over loops
- Modulo operator (%) for things like even/odd checks
- Compound assignments (+=, -=, *=, /=) to write cleaner code

**New Built-ins:**
- Math functions: abs, min, max, floor, ceil, round, sqrt, pow
- String functions: substring, contains, toUpper, toLower
- Type operations: print, println, type, len, str, int, float

**Better Developer Experience:**
- Run scripts directly from files
- Error messages now show line numbers
- 10 example files covering every feature
- Comprehensive test suite
- Professional setup with CI/CD

## Want to Contribute?

I'd love to see what you build with this! Contributions are absolutely welcome.

Here's how to get started:

1. Fork the repo
2. Create a feature branch: `git checkout -b feature/cool-new-thing`
3. Make your changes and test them
4. Commit with a clear message: `git commit -m 'Add cool new thing'`
5. Push: `git push origin feature/cool-new-thing`
6. Open a pull request

Check out [CONTRIBUTING.md](CONTRIBUTING.md) for more detailed guidelines on code style, testing, and what makes a good PR.

## License

MIT License, use it for whatever you want. See [LICENSE](LICENSE) for the legal details.

## Documentation

Want to dive deeper? The complete language reference is in [docs/ArcScript_Guide.txt](docs/ArcScript_Guide.txt).

## Credits

This project draws inspiration from some of my favorite languages:

- **Lua** for showing that simplicity is powerful
- **Python** for proving that readability matters
- **JavaScript** for making closures accessible

Built with **Rust** because when you're running game scripts at 60fps, performance and safety aren't optional.

---

**Built for game developers who want to focus on creating, not debugging.**
