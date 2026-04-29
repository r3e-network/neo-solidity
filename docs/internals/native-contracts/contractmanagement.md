---
title: "Native Contracts: ContractManagement"
description: "ContractManagement from Native Contracts."
---

# ContractManagement

[Back to Native Contracts](/internals/native-contracts)

Manages the lifecycle of all deployed contracts on Neo N3 — deployment, upgrades, destruction, and introspection.

### Methods

| Method                    | Signature                          | Return          | Safe | Description                                               |
| ------------------------- | ---------------------------------- | --------------- | :--: | --------------------------------------------------------- |
| `deploy`                  | `deploy(bytes,bytes)`              | `ContractState` |  ❌  | Deploy a new contract from NEF + manifest.                |
| `deploy`                  | `deploy(bytes,bytes,bytes)`        | `ContractState` |  ❌  | Deploy with initialization data passed to `_deploy`.      |
| `update`                  | `update(bytes,bytes)`              | `void`          |  ❌  | Update calling contract's NEF and/or manifest.            |
| `update`                  | `update(bytes,bytes,bytes)`        | `void`          |  ❌  | Update with migration data passed to `_deploy`.           |
| `destroy`                 | `destroy()`                        | `void`          |  ❌  | Permanently destroy the calling contract and its storage. |
| `getContract`             | `getContract(address)`             | `ContractState` |  ✅  | Get contract state by script hash.                        |
| `getContractById`         | `getContractById(int256)`          | `ContractState` |  ✅  | Get contract state by numeric ID.                         |
| `getMinimumDeploymentFee` | `getMinimumDeploymentFee()`        | `uint256`       |  ✅  | Minimum GAS required to deploy a contract.                |
| `setMinimumDeploymentFee` | `setMinimumDeploymentFee(uint256)` | `void`          |  ❌  | Set minimum deployment fee (committee only).              |
| `hasMethod`               | `hasMethod(address,string,uint8)`  | `bool`          |  ✅  | Check if a contract exposes a specific method.            |
| `listContracts`           | `listContracts()`                  | `Iterator`      |  ✅  | Iterator over all deployed contracts.                     |

### Code Example

```solidity
import "devpack/contracts/NativeCalls.sol";

contract Upgradeable {
    address private _owner;

    constructor() {
        _owner = msg.sender;
    }

    /// @dev Upgrade this contract in-place
    function upgrade(bytes memory newNef, bytes memory newManifest) public {
        require(Runtime.checkWitness(_owner), "not owner");
        NativeCalls.updateContract(newNef, newManifest);
        // After update, the new code executes immediately
    }

    /// @dev Permanently destroy this contract
    function kill() public {
        require(Runtime.checkWitness(_owner), "not owner");
        NativeCalls.destroyContract();
    }

    /// @dev Check if another contract exists
    function contractExists(address target) public view returns (bool) {
        return NativeCalls.hasMethod(target, "name", 0);
    }
}
```

::: warning
`destroy()` is permanent and irreversible. All contract storage is deleted. There is no refund mechanism like EVM's deprecated `selfdestruct`. Always gate destruction behind strict authorization.
:::

---
