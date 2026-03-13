# Security Considerations

While Solidity itself provides various security guarantees, writing smart contracts requires a deep understanding of the underlying execution environment.

::: tip 💡 NeoVM Difference
The most critical security differences between EVM and NeoVM lie in **Authorization (Witnesses vs `msg.sender`)** and **Contract Permissions (Manifests)**.
:::

## Authorization: Witnesses over msg.sender

On Ethereum, authorization is implicit through `msg.sender`. The caller's address is automatically set by the EVM. 

On Neo, authorization is explicit. The transaction includes cryptographic witnesses (signatures) that prove the caller controls specific addresses. 

While Neo Solidity maps `msg.sender` to `Runtime.getCallingScriptHash()` (which securely returns the hash of the contract calling your contract), **it is an anti-pattern to use this for user authorization**. You must use `Runtime.checkWitness()`.

```solidity
// ❌ EVM pattern — relies on msg.sender implicitly
function withdraw(uint256 amount) public {
    require(msg.sender == owner, "not owner");
    // ...
}

// ✅ Neo-idiomatic — explicit witness check
import {Runtime} from "@neo/Runtime.sol";

function withdraw(uint256 amount) public {
    // Cryptographically verifies the transaction was signed by `owner`
    require(Runtime.checkWitness(owner), "not authorized");
    // ...
}
```

## Manifest Permissions

Neo N3 enforces contract call permissions at the VM level via the contract's manifest. If your contract attempts to call a target contract or method not explicitly allowed in your manifest, execution will fault.

By default, dynamic calls in Solidity (like `address(target).call(...)`) force the compiler to emit wildcard permissions (`"contract": "*", "methods": "*"`). **This is a severe security risk**, as it allows your contract to be hijacked to interact with malicious contracts.

**Recommendation:**
Always compile production contracts with strict wildcard denial flags:
```bash
neo-solc MyContract.sol --deny-wildcard-contracts --deny-wildcard-methods
```
If dynamic routing is strictly required by your architecture, use NatSpec overrides to explicitly allowlist specific trusted hashes.

## No Overflows / Underflows

Because NeoVM uses arbitrary-precision `BigInteger` representation, mathematical operations do not overflow or underflow at the 256-bit boundary. 

If your contract's security model relies on expected overflow wrapping (such as circular counters), you must implement explicit modulo arithmetic (`% 256`).

## Upgradeability

Ethereum relies on `delegatecall` and complex proxy architectures to make contracts upgradeable. NeoVM inherently blocks `delegatecall`.

However, Neo natively supports upgrading contracts using `ContractManagement.update()`. This replaces the executing code while retaining the existing storage. Ensure that access to your update method is heavily protected, preferably behind a multisig or DAO governance check.