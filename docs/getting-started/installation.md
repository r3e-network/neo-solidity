# Installation

This page covers building the Neo Solidity compiler from source and setting up optional tooling for development, testing, and local deployment.

## Prerequisites

| Requirement        | Minimum Version | Purpose                                                 |
| ------------------ | --------------- | ------------------------------------------------------- |
| **Rust toolchain** | 1.70+ (stable)  | Compiler is written in Rust                             |
| **Git**            | 2.x             | Clone the repository                                    |
| **Node.js**        | 16.0+           | TypeScript tooling, docs site, Hardhat/Foundry packages |
| **npm**            | 8.0+            | Package management for tooling workspace                |
| **.NET SDK**       | 8.0+            | Optional: C# runtime library, Neo-Express               |
| **jq**             | 1.6+            | Optional: manifest inspection, smoke test scripts       |

### Verify Rust Installation

```bash
rustc --version
# Expected: rustc 1.70.0 or higher

cargo --version
# Expected: cargo 1.70.0 or higher
```

If Rust is not installed, use [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Verify Node.js Installation

```bash
node --version
# Expected: v16.0.0 or higher

npm --version
# Expected: 8.0.0 or higher
```

Node.js is only required for the TypeScript tooling packages and the VitePress documentation site. The core compiler (`neo-solc`) has no Node.js dependency.

## Build from Source

### 1. Clone the Repository

```bash
git clone https://github.com/r3e-network/neo-solidity.git
cd neo-solidity
```

### 2. Build the Compiler (Release)

```bash
cargo build --release
```

Expected output:

```
   Compiling neo-solidity v0.12.0 (/path/to/neo-solidity)
    Finished `release` profile [optimized] target(s) in 45.23s
```

The release binary is at:

```
target/release/neo-solc
```

::: tip Debug Builds
For faster iteration during development, use `cargo build` (without `--release`). The debug binary is at `target/debug/neo-solc`. Debug builds compile faster but produce a slower compiler binary.
:::

### 3. Build via Make

The Makefile wraps common commands:

```bash
make build
```

This runs `cargo build --release` and prints a confirmation message.

## Verify Installation

### Check the Binary

```bash
./target/release/neo-solc --help
```

Expected output (abbreviated):

```
neo-solc 0.12.0
Compiles Solidity to Neo N3 smart contracts (.nef + .manifest.json)

Usage: neo-solc [OPTIONS] [source]...

Arguments:
  [source]...  Input Solidity file(s)

Options:
  -o, --output <FILE>       Output prefix or directory
  -O, --optimize <LEVEL>    Optimization level (0-3) [default: 2]
  -f, --format <FORMAT>     Output format [default: complete]
      --callt               Emit CALLT + method tokens
  -I, --include-path <DIR>  Additional import search path
  -v, --verbose             Verbose output
  ...
```

### Compile a Test Contract

```bash
./target/release/neo-solc examples/SimpleStorage.sol -I devpack -O2 -o /tmp/SimpleStorage
```

Verify the output files exist:

```bash
ls -la /tmp/SimpleStorage.nef /tmp/SimpleStorage.manifest.json
```

### Run the Test Suite

```bash
cargo test --workspace
```

This runs 620+ tests covering the lexer, parser, semantic analysis, IR generation, code generation, runtime, and E2E compilation. All tests should pass on a clean build.

## Install the Binary System-Wide

To install `neo-solc` into your Cargo bin directory (typically `~/.cargo/bin`):

```bash
cargo install --path .
```

After installation, `neo-solc` is available from any directory:

```bash
neo-solc --version
# neo-solc 0.12.0
```

### Add Cargo Bin to PATH

If `~/.cargo/bin` is not already in your PATH, add it:

```bash
# bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# zsh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# fish
fish_add_path ~/.cargo/bin
```

## Optional: TypeScript Tooling

The repository includes TypeScript packages for Hardhat integration, Foundry adapters, ABI routing, and CLI tools. These live in the `tooling/` workspace.

### Install and Build

```bash
# Install dependencies
npm --prefix tooling install

# Build all packages
npm --prefix tooling run build
```

Or via Make:

```bash
make tooling-install
make tooling-build
```

### Verify Tooling

```bash
# Run tooling tests
npm --prefix tooling test

# Lint tooling code
npm --prefix tooling run lint
```

Or via Make:

```bash
make tooling-test
make tooling-lint
```

## Optional: C# Runtime Library

The C# runtime library provides EVM-compatible runtime primitives for Neo N3. It requires the .NET SDK.

```bash
dotnet build src/Neo.Sol.Runtime/Neo.Sol.Runtime.csproj --configuration Release
```

Run the runtime tests:

```bash
dotnet test tests/Neo.Sol.Runtime.Tests/Neo.Sol.Runtime.Tests.csproj --configuration Release
```

Or via Make:

```bash
make runtime-build
make runtime-test
```

::: info
The C# runtime is optional. The core compiler (`neo-solc`) does not depend on .NET. The runtime is used for testing EVM-compatible behavior and is not required for contract compilation or deployment.
:::

## Optional: Neo-Express (Local Deployment)

[Neo-Express](https://github.com/neo-project/neo-express) is a local Neo N3 blockchain for development and testing. It lets you deploy and invoke contracts without connecting to TestNet or MainNet.

### Install Neo-Express

```bash
# Install as a local .NET tool
dotnet tool install Neo.Express --tool-path ./build/dotnet-tools
```

### Verify Neo-Express

```bash
./build/dotnet-tools/neoxp --version
```

### Run a Deployment Smoke Test

```bash
make test-deploy-smoke
```

This script:

1. Builds `neo-solc` if needed
2. Compiles a test contract
3. Creates a fresh Neo-Express chain
4. Deploys the contract
5. Invokes methods and validates results

See [Deploy Workflow](/workflows/deploy) for detailed deployment instructions.

## Optional: Documentation Site

The documentation site uses [VitePress](https://vitepress.dev). To build it locally:

```bash
# Install dependencies (from repo root)
npm install

# Development server (run manually in your terminal)
# npm run docs:dev

# Production build
npm run docs:build
```

The built site is output to `docs/.vitepress/dist`.

## Environment Verification

Run the full environment check:

```bash
# Build + unit tests
make build
make test

# Full workspace tests including tooling
make test-all

# Production readiness gate (formatting, linting, release build, all tests, smoke suites)
make production-gate
```

If `make production-gate` passes, your environment is fully set up and the compiler is working correctly.

## Common Issues and Troubleshooting

### Rust Compilation Errors

**Problem:** `error[E0658]: use of unstable library feature`

**Solution:** Update your Rust toolchain. The compiler requires Rust 1.70+:

```bash
rustup update stable
```

### Linker Errors on Linux

**Problem:** `error: linker 'cc' not found`

**Solution:** Install build essentials:

```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# Fedora/RHEL
sudo dnf install gcc
```

### OpenSSL Errors

**Problem:** `Could not find directory of OpenSSL installation`

**Solution:** Install OpenSSL development headers:

```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# Fedora/RHEL
sudo dnf install openssl-devel

# macOS
brew install openssl
```

### Neo-Express Not Found

**Problem:** `error: neoxp not found`

**Solution:** Ensure .NET SDK 8.0+ is installed and install Neo-Express:

```bash
dotnet --version
dotnet tool install Neo.Express --tool-path ./build/dotnet-tools
```

### Cargo Install Fails with Permission Error

**Problem:** `Permission denied` when running `cargo install --path .`

**Solution:** Ensure `~/.cargo/bin` exists and is writable:

```bash
mkdir -p ~/.cargo/bin
cargo install --path .
```

## Next Steps

- [Quick Start](/getting-started/quickstart) -- Compile your first contract step by step.
- [Compile Workflow](/workflows/compile) -- Full CLI reference.
- [Overview](/getting-started/overview) -- Architecture and feature coverage.
