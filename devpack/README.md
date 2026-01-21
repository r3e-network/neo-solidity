# Neo N3 Devpack for Solidity

**Neo N3 smart contract devpack for the `neo-solidity` compiler**

> Important: `neo-solidity` treats `Runtime`, `Storage`, `Syscalls`, `Neo`, `NativeCalls` and `abi`
> as **compiler intrinsics** (built-in libraries). Their Solidity source is primarily for editor
> tooling (signatures/docs). The compiler lowers supported members directly to Neo N3 syscalls and
> native contract calls. Unsupported members will fail compilation with a diagnostic listing the
> supported intrinsics.

This devpack provides Solidity-facing interfaces for the Neo N3 features that `neo-solidity`
currently supports, plus complete example contracts (NEP-17/NEP-11).

## 🎯 Features

### ✅ Core Neo N3 Integration
- **Syscalls**: Runtime, crypto, storage, iterator, and contract syscalls (plus convenience helpers)
- **Native Contracts**: NEO, GAS, ContractManagement, Policy, Oracle, RoleManagement
- **Storage**: `Storage.get/put/remove/find` lowered to `System.Storage.*`
- **Events**: Solidity `emit` lowered to `System.Runtime.Notify` (Neo enforces ABI event name + parameter types)

### ✅ NEP Standards Support
- **NEP-17**: Fungible token standard (enhanced ERC-20)
- **NEP-11**: Non-fungible token standard (enhanced ERC-721)
- **NEP-24**: NFT royalty standard (`royaltyInfo`)
- **Custom NEPs**: Framework for implementing additional standards

### ✅ Advanced Features
- **Cross-Contract Calls**: `Syscalls.contractCall{WithFlags}` and optional `CALLT` token emission
- **Upgradeable Contracts**: ContractManagement wrappers via `NativeCalls.*`

## 📚 Quick Start

### Installation

```bash
# Install Neo Solidity Compiler with devpack
git clone https://github.com/r3e-network/neo-solidity.git
cd neo-solidity
make install

# Or use npm package
npm install -g @r3e-network/neo-solidity-devpack
```

### Basic Usage

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@r3e-network/neo-solidity-devpack/standards/NEP17.sol";

contract MyToken is NEP17 {
    constructor() NEP17("My Token", "MTK", 8, 0, 1_000_000 * 10**8) {}

    event Minted(address indexed to, uint256 amount);

    function mint(address to, uint256 amount) public onlyOwner withWitness {
        _mint(to, amount);
        emit Minted(to, amount);
    }
}
```

## 🏗️ Architecture

### Core Components

```
devpack/
├── contracts/           # Core framework contracts
│   ├── FrameworkBase.sol # Base framework (minimally-permissioned)
│   ├── Framework.sol    # Extended framework (dynamic calls; requires wildcard permissions)
│   ├── OracleService.sol # Oracle native-contract wrapper (optional)
│   ├── NativeCalls.sol # Native contract interfaces
│   └── Syscalls.sol    # System call mappings
├── standards/          # NEP standard implementations
│   ├── NEP17.sol      # Fungible tokens
│   ├── NEP11.sol      # Non-fungible tokens
│   └── NEP24.sol      # NFT royalty standard
├── libraries/          # Utility libraries
│   ├── Neo.sol        # Neo blockchain utilities
│   ├── Storage.sol    # Advanced storage operations
│   └── Runtime.sol    # Runtime services
└── examples/           # Complete contract examples
    ├── CompleteNEP17Token.sol # Full NEP-17 token
    └── CompleteNEP11NFT.sol   # Full NEP-11 NFT
```

## 📖 API Reference

### Supported Intrinsics

`neo-solidity` lowers calls to a supported subset of these libraries:

- `Runtime`: `notify`, `notifyIndexed`, `checkWitness`, `gasLeft`, `burnGas`, `log`, `getTime`, `getTrigger`
- `Storage`: `get`, `put`, `remove`, `find`
- `Syscalls`: `contractCall{WithFlags}`, `createStandardAccount`, `createMultisigAccount`, `System.Runtime.*`, `System.Storage.*`, crypto/json/base64, iterator helpers
- `NativeCalls`: NEO/GAS token calls, ContractManagement, Policy, Oracle, RoleManagement helpers

For exact Solidity signatures, see:
- `devpack/contracts/Syscalls.sol`
- `devpack/contracts/NativeCalls.sol`
- `devpack/libraries/Runtime.sol`
- `devpack/libraries/Storage.sol`

## 🔐 Manifest Permissions (Dynamic Calls)

Neo N3 manifests must declare cross-contract call permissions. The compiler can infer many direct calls
(native contracts like `GAS.transfer`, fixed `System.Contract.Call` sites), but **fully dynamic calls**
(unknown contract hash and/or method name at compile time) may require wildcard permissions.

**Recommended options:**
- Prefer `FrameworkBase.sol` / `NativeCalls.*` helpers to avoid dynamic calls in production code.
- Enforce strict manifests in CI with:
  - `--deny-wildcard-permissions` (blocks `{"contract":"*","methods":"*"}`)
  - `--deny-wildcard-contracts` (blocks any `contract:"*"`)
  - `--deny-wildcard-methods` (blocks any `methods:"*"`)
- If you intentionally need dynamic calls, provide an explicit allowlist:

```json
// permissions.json
[
  { "contract": "0x0000000000000000000000000000000000000000", "methods": ["transfer"] }
]
```

```bash
neo-solc MyContract.sol -I devpack -o build/MyContract \\
  --manifest-permissions permissions.json \\
  --manifest-permissions-mode replace-wildcards \\
  --deny-wildcard-contracts --deny-wildcard-methods
```

## 🏗️ Constructors & Deployment Data

Neo N3 deployment invokes the contract’s `_deploy(data, update)` method.
`neo-solidity` injects `_deploy` automatically.

- If your Solidity contract has **no constructor parameters**, you can deploy normally.
- If your Solidity contract has a **parameterised constructor**, pass constructor arguments via
  `_deploy.data` as an **array of arguments**:
  - Neo-Express / CLI tooling: pass a **JSON-encoded array string** (e.g. `[7]`, `["hello", 1]`).
  - SDKs that support StackItems directly may pass an **Array** value.
  - Contract-to-contract deploy flows may pass **StdLib.serialize(...) bytes** (for example `abi.encode(...)`).

This compiler attempts to decode constructor deployment data via `StdLib.jsonDeserialize`, then
falls back to `StdLib.deserialize` (binary serialization), and finally uses `data` as-is when native
calls throw. The generated manifest will include (and on-chain execution will require) permissions
for the StdLib methods used (`jsonDeserialize` / `deserialize`).
