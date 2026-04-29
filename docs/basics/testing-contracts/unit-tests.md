---
title: "Test Contracts: Unit Tests"
description: "Unit Tests from Test Contracts."
---

# Unit Tests

[Back to Test Contracts](/basics/testing-contracts)

Unit tests cover individual compiler components in isolation. They live in `tests/` and inline `#[cfg(test)]` modules.

```bash
# Run all unit tests
cargo test --workspace

# Run tests for a specific module
cargo test lexer_tests
cargo test parser_tests
cargo test semantic_tests
cargo test runtime_tests

# Run with output visible (useful for debugging)
cargo test -- --nocapture

# Run a specific test by name
cargo test test_simple_storage_compilation

# Run tests in parallel (default) or single-threaded
cargo test -- --test-threads=1
```

### Runtime Tests

The embedded NeoVM runtime has extensive unit tests covering opcodes, syscalls, storage, iterators, and gas accounting:

```bash
# All runtime tests
cargo test runtime_

# Specific runtime areas
cargo test runtime_arithmetic
cargo test runtime_storage
cargo test runtime_crypto
```
