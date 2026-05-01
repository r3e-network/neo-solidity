---
title: "Testing Contracts: Writing Your Own Tests"
description: "Writing Your Own Tests from Testing Contracts."
---

# Writing Your Own Tests

[Back to Testing Contracts](/basics/testing-contracts)

## Adding a Compilation Test

1. Create a new Solidity file in `examples/` or `examples/new/`.
2. The E2E test suite automatically picks up files in these directories.
3. Run `cargo test --test e2e_compilation_tests` to verify.

## Adding a Smoke Test

1. Create a new script in `examples/` following the pattern of existing scripts.
2. The script should:
    - Create a temporary directory and clean up on exit
    - Resolve `neo-solc` and `neoxp` binaries
    - Write a test contract inline
    - Compile, deploy, invoke, and validate
3. Add a Make target in the `Makefile`.
4. Add the target to `test-deploy-smoke-full` dependencies.

## Adding a Unit Test

Add `#[test]` functions in the relevant module or create a new test file in `tests/`:

```rust
#[test]
fn test_my_feature() {
    let source = r#"
        // SPDX-License-Identifier: MIT
        pragma solidity ^0.8.20;
        contract Test {
            function foo() public pure returns (uint256) {
                return 42;
            }
        }
    "#;
    // Use the compiler API to compile and validate
    // ...
}
```
