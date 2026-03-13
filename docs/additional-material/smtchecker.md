# SMTChecker and Formal Verification

The Solidity compiler has a built-in formal verification module called the SMTChecker. This module can be used to automatically prove properties of smart contracts. 

::: tip 💡 NeoVM Difference
The standard `solc` SMTChecker module operates specifically on the behavioral constraints of the EVM (like 256-bit arithmetic boundaries and sequential storage slots).

**Neo Solidity does not currently include the SMTChecker.** 

Because NeoVM uses arbitrary-precision integers (`BigInteger`) and a dynamic Key-Value storage model, many of the constraints the standard SMTChecker attempts to verify (like arithmetic overflows) are not applicable to the NeoVM runtime.
:::

## Verification on Neo

Currently, verifying Neo Solidity contracts requires external auditing tools or testing frameworks. We recommend utilizing the `tooling/` packages in this repository (like `@r3e-network/hardhat-solc-neo`) to run extensive unit and integration tests against local Neo-Express nodes.

Future iterations of the compiler may introduce Neo-specific constraint checkers for static analysis, specifically regarding:
1. **Manifest Permission Wildcards:** Validating that all cross-contract calls are strictly bounded.
2. **Witness Checks:** Statically analyzing functions to ensure `Runtime.checkWitness` is executed prior to any state-mutating operations.