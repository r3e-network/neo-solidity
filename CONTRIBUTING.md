# Contributing to Neo Solidity Compiler

We welcome contributions to the Neo Solidity Compiler project! This guide will help you get started.

## 🎯 Getting Started

### Prerequisites

- **Rust**: 1.70 or higher
- **Node.js**: 16.0 or higher (for tooling)
- **.NET**: 6.0 or higher (for runtime)
- **Git**: Latest version

### Development Setup

```bash
# 1. Fork and clone the repository
git clone https://github.com/yourusername/neo-solidity.git
cd neo-solidity

# 2. Install dependencies
make install-deps

# 3. Build the project
make build-all

# 4. Run tests to ensure everything works
make test-all
```

## 📋 Ways to Contribute

### 🐛 Bug Reports

Found a bug? Please create an issue with:

- **Description**: Clear description of the bug
- **Steps to Reproduce**: How to reproduce the issue
- **Expected vs Actual**: What you expected vs what happened
- **Environment**: OS, Rust version, Neo version
- **Code Sample**: Minimal example that reproduces the bug

### 🚀 Feature Requests

Have an idea? Create an issue with:

- **Problem Statement**: What problem does this solve?
- **Proposed Solution**: How should it work?
- **Alternatives**: What other approaches were considered?
- **Use Cases**: Who would benefit from this feature?

### 💻 Code Contributions

#### Good First Issues

Look for issues labeled [`good first issue`](https://github.com/r3e-network/neo-solidity/labels/good%20first%20issue) - these are perfect for new contributors.

#### Development Workflow

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Changes**
   - Follow our coding standards (see below)
   - Add tests for new functionality
   - Update documentation if needed

3. **Test Your Changes**
   ```bash
   make test-all
   make lint
   make format
   ```

4. **Commit Your Changes**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a Pull Request on GitHub.

## 🎨 Coding Standards

### Rust Code

- Follow [Rust style guidelines](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write comprehensive tests
- Document public APIs

```rust
/// Compiles Yul code to NeoVM bytecode
/// 
/// # Arguments
/// * `input` - The Yul source code to compile
/// * `config` - Compilation configuration
/// 
/// # Returns
/// * `Result<CompilationResult, CompilerError>` - Compilation result or error
pub fn compile(input: &str, config: &CompilerConfig) -> Result<CompilationResult, CompilerError> {
    // Implementation here
}
```

### C# Code

- Follow [Microsoft C# conventions](https://docs.microsoft.com/en-us/dotnet/csharp/programming-guide/inside-a-program/coding-conventions)
- Use XML documentation comments
- Write unit tests for all public methods
- Follow async/await patterns

```csharp
/// <summary>
/// Stores data in EVM-compatible memory
/// </summary>
/// <param name="offset">Memory offset</param>
/// <param name="data">Data to store</param>
/// <returns>Task representing the async operation</returns>
public async Task MStore(uint offset, byte[] data)
{
    // Implementation here
}
```

### TypeScript Code

- Follow [Airbnb TypeScript Style Guide](https://github.com/airbnb/javascript/tree/master/packages/eslint-config-airbnb-typescript)
- Use strict TypeScript configuration
- Write comprehensive JSDoc comments
- Include unit tests

```typescript
/**
 * Compiles Solidity contract for Neo blockchain
 * @param source - Solidity source code
 * @param options - Compilation options
 * @returns Promise resolving to compilation result
 */
export async function compile(
  source: string, 
  options: CompileOptions
): Promise<CompileResult> {
  // Implementation here
}
```

## 🧪 Testing Guidelines

### Test Requirements

- **Unit Tests**: All new functions must have unit tests
- **Integration Tests**: Add integration tests for new features
- **Performance Tests**: Include benchmarks for performance-critical code
- **Documentation Tests**: Ensure examples in docs work

### Running Tests

```bash
# Run all tests
make test-all

# Run specific test suites
cargo test lexer_tests
cargo test parser_tests
dotnet test runtime/

# Run performance benchmarks
cargo bench

# Run with coverage
make test-coverage
```

### Writing Good Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_compilation() {
        let input = "{ let x := add(1, 2) }";
        let config = CompilerConfig::default();
        
        let result = compile(input, &config).unwrap();
        
        assert!(!result.bytecode.is_empty());
        assert!(result.estimated_gas > 0);
    }

    #[test]
    fn test_error_handling() {
        let invalid_input = "{ let x := }"; // Missing value
        let config = CompilerConfig::default();
        
        let result = compile(invalid_input, &config);
        
        assert!(result.is_err());
    }
}
```

## 📚 Documentation

### Documentation Requirements

- **README**: Update if you change installation or usage
- **API Docs**: Document all public APIs
- **Examples**: Provide working examples
- **Changelog**: Add entry for breaking changes

### Documentation Style

- Use clear, concise language
- Include code examples
- Provide context and motivation
- Link to related concepts

## 🔍 Code Review Process

### What We Look For

- **Correctness**: Does the code work as intended?
- **Test Coverage**: Are there adequate tests?
- **Performance**: Any performance implications?
- **Security**: Are there security considerations?
- **Documentation**: Is it well documented?
- **Style**: Does it follow our coding standards?

### Getting Your PR Merged

1. **All Tests Pass**: CI must be green
2. **Code Review**: At least one maintainer approval
3. **Documentation**: Updated if needed
4. **No Conflicts**: Rebase if needed

## 🎯 Project Areas

### Compiler Core

- **Lexer**: Tokenizing Yul input
- **Parser**: Building AST from tokens  
- **Semantic Analysis**: Type checking and validation
- **Optimizer**: Code optimization passes
- **Code Generator**: NeoVM bytecode generation

### Runtime Library

- **Memory Manager**: EVM memory emulation
- **Storage Manager**: Solidity storage layout
- **ABI Encoder**: Function call encoding/decoding
- **Crypto Library**: Hash functions and signatures
- **Event System**: Event emission and filtering

### Developer Tools

- **Hardhat Plugin**: Hardhat integration
- **Foundry Adapter**: Foundry compatibility
- **CLI Tools**: Command-line interface
- **Debug Tools**: Debugging and profiling

## 🏷️ Commit Message Convention

We use conventional commits:

```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Build/tooling changes

**Examples:**
```
feat(compiler): add support for Yul functions
fix(runtime): resolve memory allocation bug
docs(readme): update installation instructions
```

## 🚀 Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **Major** (1.0.0): Breaking changes
- **Minor** (0.1.0): New features, backward compatible
- **Patch** (0.0.1): Bug fixes, backward compatible

### Creating Releases

1. Update version numbers
2. Update CHANGELOG.md
3. Create release branch
4. Run full test suite
5. Create GitHub release
6. Publish packages

## 💬 Communication

### Where to Get Help

- **Discord**: [R3E Network Discord](https://discord.gg/r3e-network)
- **GitHub Issues**: For bugs and feature requests
- **Email**: jimmy@r3e.network for private issues

### Community Guidelines

- **Be Respectful**: Treat everyone with respect
- **Be Constructive**: Provide helpful feedback
- **Be Patient**: Allow time for responses
- **Follow Code of Conduct**: See CODE_OF_CONDUCT.md

## 🙏 Recognition

Contributors will be:

- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Invited to contributor Discord channel
- Eligible for contributor rewards

## ❓ Questions?

Don't hesitate to ask! We're here to help:

- Open a GitHub issue
- Join our Discord
- Email jimmy@r3e.network

Thank you for contributing to Neo Solidity Compiler! 🚀