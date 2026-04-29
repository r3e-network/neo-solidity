---
title: "Troubleshooting: Unsupported EVM Features"
description: "Unsupported EVM Features from Troubleshooting."
---

# Unsupported EVM Features

[Back to Troubleshooting](/advisory-content/troubleshooting)

### E3001: UnsupportedFeature

```
error[E3001]: unsupported feature: delegatecall
  --> MyContract.sol:15:9
   |
15 |         address(impl).delegatecall(data);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: NeoVM has no delegate call mechanism
   = help: use ContractManagement.update() for upgradeable contracts
```

**Blocked constructs and their Neo alternatives:**

| Blocked Feature        | Error | Neo Alternative                                 |
| ---------------------- | ----- | ----------------------------------------------- |
| `delegatecall`         | E3001 | `ContractManagement.update()` for upgrades      |
| `assembly { ... }`     | E3001 | Use devpack libraries instead                   |
| `new Contract(...)`    | E3001 | `ContractManagement.deploy(...)`                |
| `type(X).creationCode` | E3001 | Deploy via `ContractManagement.deploy(...)`     |
| `type(X).runtimeCode`  | E3001 | No Neo equivalent                               |

::: warning
These constructs have no safe 1:1 NeoVM equivalent. Refactor to the listed Neo-native patterns.
:::

---
