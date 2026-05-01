# SMTChecker and Formal Verification

The Solidity compiler has a built-in formal verification module called the SMTChecker. This module can be used to automatically prove properties of smart contracts. 

::: tip 💡 NeoVM Difference
The standard `solc` SMTChecker module operates specifically on the behavioral constraints of the EVM (like 256-bit arithmetic boundaries and sequential storage slots).

**Neo Solidity does not currently include the SMTChecker.** 

Because NeoVM uses arbitrary-precision integers (`BigInteger`) internally, Neo Solidity implements fixed-width overflow behavior with compiler/runtime guards rather than native EVM word overflow. Standard EVM SMTChecker results are therefore not a direct proof for generated NeoVM artifacts; use them as source-level hints and validate arithmetic behavior with Neo Solidity tests.
:::

## Verification on Neo

Currently, verifying Neo Solidity contracts requires external auditing tools or testing frameworks. We recommend utilizing the `tooling/` packages in this repository (like `@neo-solidity/hardhat-solc-neo`) to run extensive unit and integration tests against local Neo-Express nodes.

Future iterations of the compiler may introduce Neo-specific constraint checkers for static analysis, specifically regarding:
1. **Manifest Permission Wildcards:** Validating that all cross-contract calls are strictly bounded.
2. **Witness Checks:** Statically analyzing functions to ensure `Runtime.checkWitness` is executed prior to any state-mutating operations.
