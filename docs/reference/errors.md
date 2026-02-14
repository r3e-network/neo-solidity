# Error Reference

Canonical error/warning references:

- [`docs/ERROR_REFERENCE.md`](../ERROR_REFERENCE.md)
- compiler diagnostics in CLI output (`--json-errors`, `--json-warnings`)

## Common failure categories

- Source/import resolution failures
- Solidity semantic/type errors
- Unsupported/blocked EVM features
- Manifest wildcard policy failures
- Deployment data/constructor mismatch issues

## Practical debugging loop

1. Compile with verbose diagnostics: `neo-solc contract.sol -I devpack --json-errors --json-warnings`
2. Fix highest-severity diagnostics first.
3. Re-run strict manifest flags.
4. Validate with Neo-Express smoke tests.

## Manifest-specific diagnostics

Most production blockers involve wildcard permission requirements for dynamic contract calls.

Use explicit permissions or refactor dynamic call sites to static targets/methods.
