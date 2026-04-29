---
title: "Test Contracts: Workspace-Wide Test Suites"
description: "Workspace-Wide Test Suites from Test Contracts."
---

# Workspace-Wide Test Suites

[Back to Test Contracts](/basics/testing-contracts)

### Core Tests

```bash
# Rust tests only
make test

# Rust tests + tooling tests + tooling lint
make test-all

# Rust tests + tooling tests + C# runtime tests
make test-all-full
```

### Tooling Tests

```bash
# TypeScript package tests (Hardhat, Foundry, ABI router, CLI tools)
make tooling-test

# TypeScript linting
make tooling-lint
```

### C# Runtime Tests

```bash
# Requires .NET 8 SDK
make runtime-test
```
