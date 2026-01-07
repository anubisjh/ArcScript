# Contributing to ArcScript

Thank you for your interest in contributing to ArcScript! This document provides guidelines and instructions for contributing to the project.

## 🌟 Ways to Contribute

- **Bug Reports**: Found a bug? Please open an issue with detailed reproduction steps
- **Feature Requests**: Have an idea? Open an issue to discuss it
- **Code Contributions**: Submit pull requests with bug fixes or new features
- **Documentation**: Improve docs, add examples, or write tutorials
- **Testing**: Add test cases or improve existing tests

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- Familiarity with Rust and language implementation concepts

### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/arcscript.git
   cd arcscript
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/arcscript.git
   ```
4. Build and test:
   ```bash
   cargo build
   cargo test
   ```

## 📝 Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b bugfix/issue-number
```

### 2. Make Your Changes

- Write clean, idiomatic Rust code
- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed
- Keep commits atomic and well-described

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific tests
cargo test test_name

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

### 4. Commit Your Changes

Write clear commit messages:

```
feat: Add support for array literals

- Implemented array parsing in parser.rs
- Added Array value type in interpreter.rs
- Added tests for array operations
- Updated documentation
```

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then open a pull request on GitHub with:
- Clear title describing the change
- Description of what was changed and why
- Link to related issues (if applicable)
- Screenshots/examples (if applicable)

## 🎨 Code Style Guidelines

### Rust Conventions

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` to format code
- Address `cargo clippy` warnings
- Use meaningful variable and function names
- Add comments for complex logic

### Project Structure

```
src/
  lexer.rs        - Tokenization
  parser.rs       - Parsing and AST construction
  ast.rs          - AST node definitions
  interpreter.rs  - Execution and runtime
  main.rs         - CLI entry point

examples/         - Example ArcScript programs
docs/             - Documentation
spec/             - Language specification
```

### Testing

- Add unit tests in the same file using `#[cfg(test)]` modules
- Add integration tests in `tests/` directory
- Use descriptive test names: `test_closure_captures_outer_variable`
- Test both success and error cases

Example test:

```rust
#[test]
fn test_built_in_len() {
    let source = r#"
        var s = "Hello";
        var result = len(s);
    "#;
    let mut interp = Interpreter::new();
    // ... rest of test
}
```

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Description**: Clear description of the issue
2. **Reproduction**: Minimal code example that reproduces the bug
3. **Expected vs Actual**: What you expected vs what happened
4. **Environment**: OS, Rust version, ArcScript version
5. **Stack Trace**: If applicable, include error messages

Example:

```markdown
## Bug: Division by zero not handled in REPL

**Description**: When dividing by zero in REPL mode, the program panics instead of showing a runtime error.

**Reproduction**:
cargo run repl
> var x = 10 / 0;

**Expected**: Runtime error message
**Actual**: Panic with stack trace

**Environment**: Windows 11, Rust 1.75, ArcScript main branch
```

## ✨ Feature Requests

When requesting features, please:

1. Check if the feature already exists or is planned
2. Describe the use case clearly
3. Provide examples of how it would work
4. Consider alternatives and trade-offs

## 📚 Documentation

When updating documentation:

- Update `README.md` for user-facing changes
- Update `docs/ArcScript_Guide.txt` for language features
- Add inline comments for complex code
- Include examples in documentation
- Keep grammar specification (`spec/grammar.ebnf`) in sync

## 🔍 Code Review Process

Pull requests are reviewed for:

- **Correctness**: Does it work as intended?
- **Tests**: Are there adequate tests?
- **Style**: Does it follow project conventions?
- **Documentation**: Is it properly documented?
- **Performance**: Are there any performance concerns?
- **Breaking Changes**: Are breaking changes justified and documented?

## 📜 Commit Message Guidelines

Format:
```
<type>: <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

Example:
```
feat: Add array type support

- Implemented array literals in parser
- Added Array variant to Value enum
- Added array indexing and length operations
- Added comprehensive tests

Closes #123
```

## 🎯 Areas Needing Help

Current priorities:

1. **Standard Library**: More built-in functions
2. **Error Messages**: More detailed and helpful error messages
3. **Examples**: More comprehensive example scripts
4. **Performance**: Optimization opportunities
5. **Documentation**: Tutorials and guides
6. **Tooling**: LSP server, syntax highlighting

## 💡 Questions?

- Open a GitHub issue with the `question` label
- Check existing issues and discussions

## 🙏 Thank You!

Every contribution helps make ArcScript better. Thank you for taking the time to contribute!

---

**Happy Coding! 🚀**
