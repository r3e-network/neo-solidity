---
title: "Native Contracts: Ledger Contract"
description: "Ledger Contract from Native Contracts."
---

# Ledger Contract

[Back to Native Contracts](/internals/native-contracts)

Provides read-only access to blockchain data — blocks, transactions, and their metadata.

### Methods

| Method                    | Signature                                  | Return        | Safe | Description                                                         |
| ------------------------- | ------------------------------------------ | ------------- | :--: | ------------------------------------------------------------------- |
| `currentIndex`            | `currentIndex()`                           | `uint256`     |  ✅  | Current block height. Maps from `block.number`.                     |
| `currentHash`             | `currentHash()`                            | `bytes32`     |  ✅  | Hash of the current block.                                          |
| `getBlock`                | `getBlock(uint256)`                        | `Block`       |  ✅  | Get block by index.                                                 |
| `getBlock`                | `getBlock(bytes32)`                        | `Block`       |  ✅  | Get block by hash.                                                  |
| `getTransaction`          | `getTransaction(bytes32)`                  | `Transaction` |  ✅  | Get transaction by hash.                                            |
| `getTransactionHeight`    | `getTransactionHeight(bytes32)`            | `int256`      |  ✅  | Block height containing the transaction. Returns `-1` if not found. |
| `getTransactionFromBlock` | `getTransactionFromBlock(uint256,uint256)` | `Transaction` |  ✅  | Get transaction by block index and tx index.                        |
| `getTransactionSigners`   | `getTransactionSigners(bytes32)`           | `Signer[]`    |  ✅  | Get signers of a transaction.                                       |
| `getTransactionVMState`   | `getTransactionVMState(bytes32)`           | `uint8`       |  ✅  | VM execution state of a transaction (`HALT`, `FAULT`, etc.).        |

::: info
The compiler auto-maps `block.number` to `Ledger.currentIndex()` and `blockhash(n)` to `Ledger.getBlock(n).hash`. See [Units and Globally Available Variables](/language-description/units-and-global-variables) for the full list of auto-mapped block context values.
:::

---
