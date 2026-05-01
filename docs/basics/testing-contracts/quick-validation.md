---
title: "Testing Contracts: Quick Validation"
description: "Quick Validation from Testing Contracts."
---

# Quick Validation

[Back to Testing Contracts](/basics/testing-contracts)

## Overview

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
