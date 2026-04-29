---
title: "Native Contracts: Policy Contract"
description: "Policy Contract from Native Contracts."
---

# Policy Contract

[Back to Native Contracts](/internals/native-contracts)

Controls network-wide policy parameters. Most setter methods require committee multi-signature authorization.

### Methods

| Method             | Signature                  | Return    | Safe | Description                                          |
| ------------------ | -------------------------- | --------- | :--: | ---------------------------------------------------- |
| `getFeePerByte`    | `getFeePerByte()`          | `uint256` |  ✅  | Network fee per transaction byte (in GAS fractions). |
| `setFeePerByte`    | `setFeePerByte(uint256)`   | `void`    |  ❌  | Set fee per byte (committee only).                   |
| `getExecFeeFactor` | `getExecFeeFactor()`       | `uint32`  |  ✅  | Execution fee multiplier for opcode costs.           |
| `setExecFeeFactor` | `setExecFeeFactor(uint32)` | `void`    |  ❌  | Set execution fee factor (committee only).           |
| `getStoragePrice`  | `getStoragePrice()`        | `uint256` |  ✅  | GAS cost per byte of contract storage.               |
| `setStoragePrice`  | `setStoragePrice(uint256)` | `void`    |  ❌  | Set storage price (committee only).                  |
| `isBlocked`        | `isBlocked(address)`       | `bool`    |  ✅  | Check if an account is blocked from transacting.     |
| `blockAccount`     | `blockAccount(address)`    | `void`    |  ❌  | Block an account (committee only).                   |
| `unblockAccount`   | `unblockAccount(address)`  | `void`    |  ❌  | Unblock an account (committee only).                 |

### Code Example

```solidity
import "devpack/contracts/NativeCalls.sol";

contract FeeEstimator {
    function estimateStorageCost(uint256 bytesCount) public view returns (uint256) {
        uint256 pricePerByte = NativeCalls.getStoragePrice();
        return pricePerByte * bytesCount;
    }

    function getNetworkFeeParams() public view returns (uint256 feePerByte, uint32 execFactor) {
        feePerByte = NativeCalls.getFeePerByte();
        execFactor = NativeCalls.getExecFeeFactor();
    }

    function requireNotBlocked(address account) internal view {
        require(!NativeCalls.isBlocked(account), "account is blocked");
    }
}
```

---
