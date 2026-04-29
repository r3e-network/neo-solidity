---
title: "Test Contracts: Quick Validation"
description: "Quick Validation from Test Contracts."
---

# Quick Validation

[Back to Test Contracts](/basics/testing-contracts)

For fast feedback during development:

```bash
# Run all Rust tests (unit + integration + E2E + conformance)
make test
# or equivalently:
cargo test --workspace
```

For a quick compilation sanity check (verifies NEF magic bytes and manifest structure):

```bash
bash examples/test_compilation.sh
```
