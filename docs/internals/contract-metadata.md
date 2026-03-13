# Manifest Specification

Every Neo N3 smart contract is deployed as two artifacts: a `.nef` file (NeoVM bytecode) and a `.manifest.json` file (contract metadata). The manifest describes the contract's ABI, permissions, supported standards, and trust relationships. The neo-solidity compiler generates both artifacts automatically from your Solidity source code — no manual JSON authoring required.

## NEF Format

The Neo Executable Format (NEF) is the binary container for NeoVM bytecode. The compiler produces a `.nef` file alongside the manifest for every compiled contract.

| Field         | Size     | Description                                                     |
| ------------- | -------- | --------------------------------------------------------------- |
| Magic         | 4 bytes  | `NEF3` — identifies the file as a Neo N3 executable             |
| Compiler ID   | 64 bytes | UTF-8 string, zero-padded (e.g. `neo-solidity-0.1.0`)           |
| Source URL    | variable | Length-prefixed string, max 256 bytes. Set via `--nef-source`   |
| Reserved      | 1 byte   | Must be `0x00`                                                  |
| Method Tokens | variable | Varint count + up to 128 token entries for `CALLT` optimization |
| Reserved      | 2 bytes  | Must be `0x0000`                                                |
| Script        | variable | Length-prefixed NeoVM bytecode (max 512 KB)                     |
| Checksum      | 4 bytes  | First 4 bytes of `SHA256(SHA256(header + script))`              |

::: tip
The `--nef-source` flag embeds a URL or identifier into the NEF header. This is useful for linking deployed contracts back to their source repository. If the value exceeds 256 bytes it is silently truncated to the nearest valid UTF-8 boundary and a `NEF_SOURCE_TRUNCATED` warning is emitted.
:::

## Manifest Structure

The manifest is a JSON file containing all metadata the Neo N3 runtime needs to validate and dispatch calls to your contract. The compiler emits every required top-level field:

```json
{
    "name": "ContractName",
    "groups": [],
    "features": {},
    "supportedstandards": [],
    "abi": {
        "methods": [],
        "events": []
    },
    "permissions": [],
    "trusts": [],
    "extra": {
        "Author": "Jimmy <jimmy@r3e.network>",
        "Description": "Solidity contract 'ContractName' compiled to NeoVM",
        "Version": "0.1.0.0",
        "Compiler": "neo-solidity-0.1.0"
    }
}
```

### name

The contract name used for on-chain identification. Defaults to the Solidity contract name.

Override with NatSpec:

```solidity
/// @custom:neo.manifest.name MyCustomName
contract Token { }
```

### groups

Array of group objects for contract grouping and multi-signature verification. Defaults to an empty array.

Override with NatSpec:

```solidity
/// @custom:neo.manifest.groups [{"pubkey":"03b209...","signature":"abc123..."}]
contract Token { }
```

### features

Must be an empty object `{}` for Neo N3. The Neo runtime rejects any contract with populated feature keys. The compiler enforces this constraint — non-empty custom values are ignored with a warning.

::: warning
Do not attempt to set features to a non-empty object. Neo N3 reserves this field for future use and will reject deployment of contracts with populated features.
:::

### supportedstandards

Array of standard identifier strings (e.g. `["NEP-17"]`). The compiler auto-detects supported standards from your contract's method signatures and events. See [Standards Auto-Detection](#standards-auto-detection) for the full detection logic.

Override with NatSpec:

```solidity
/// @custom:neo.manifest.supportedstandards ["NEP-17"]
contract Token { }
```

### abi

The contract's Application Binary Interface, containing method and event declarations.

#### Methods

Each public or external function in your Solidity contract becomes a method entry in the manifest ABI:

```json
{
    "name": "transfer",
    "parameters": [
        { "name": "from", "type": "Hash160" },
        { "name": "to", "type": "Hash160" },
        { "name": "amount", "type": "Integer" },
        { "name": "data", "type": "Any" }
    ],
    "returntype": "Boolean",
    "offset": 0,
    "safe": false
}
```

The compiler maps Solidity types to Neo manifest types according to this table:

| Solidity Type       | Manifest Type | Notes                          |
| ------------------- | ------------- | ------------------------------ |
| `address`           | `Hash160`     | 20-byte Neo address            |
| `uint8`..`uint256`  | `Integer`     | All unsigned integer widths    |
| `int8`..`int256`    | `Integer`     | All signed integer widths      |
| `bool`              | `Boolean`     |                                |
| `string`            | `String`      |                                |
| `bytes`             | `ByteArray`   | Dynamic byte array             |
| `bytes1`..`bytes31` | `ByteArray`   | Fixed-size byte arrays         |
| `bytes32`           | `Hash256`     | 32-byte hash                   |
| `bytes20`           | `Hash160`     | Treated as address-width       |
| `T[]`               | `Array`       | Any dynamic array              |
| `mapping(K => V)`   | `Map`         | Key-value mapping              |
| `struct`            | `Array`       | Structs serialize as arrays    |
| `enum`              | `Integer`     | Enums are integer-backed       |
| Neo `Any` type      | `Any`         | Explicit any-type from devpack |

::: info
When a NeoType annotation is present (via the devpack), it takes precedence over the Solidity type inference. For example, a parameter annotated with `NeoType.Address` always maps to `Hash160` regardless of its Solidity declaration.
:::

The `safe` flag is set to `true` for `view` and `pure` functions. Safe methods can be invoked without a transaction (read-only calls) and do not consume GAS on-chain.

When a function returns multiple values, the manifest return type is `Array`. Functions with no return value use `Void`.

**Overloaded functions:** Neo N3 dispatches methods by name and does not support Solidity-style overload resolution. The compiler emits the canonical name for the "primary" overload (highest arity) and uses a mangled `neo_name` for secondary overloads, guaranteeing unique ABI names.

#### Events

Each Solidity `event` becomes an event entry with a name and typed parameter array:

```json
{
    "name": "Transfer",
    "parameters": [
        { "name": "from", "type": "Hash160" },
        { "name": "to", "type": "Hash160" },
        { "name": "amount", "type": "Integer" }
    ]
}
```

Event parameters use the same type mapping table as method parameters.

### permissions

Array of permission entries declaring which contracts and methods this contract may call. Neo N3 enforces these at runtime — any call not covered by a permission entry will fault.

Explicit permission (specific contract and method):

```json
{
    "contract": "0xd2a4cff31913016155e38e474a2c06d08be276cf",
    "methods": ["transfer"]
}
```

Wildcard permissions (use with caution):

```json
{ "contract": "*", "methods": "*" }
```

```json
{ "contract": "0xd2a4cff31913016155e38e474a2c06d08be276cf", "methods": "*" }
```

```json
{ "contract": "*", "methods": ["ping"] }
```

See [Permission Inference](#permission-inference) and [Permission Hardening](#permission-hardening) for details on how the compiler generates and restricts these entries.

### trusts

Declares which contracts are trusted to call this contract. Defaults to an empty array `[]` (no external callers trusted).

Override with NatSpec:

```solidity
/// @custom:neo.manifest.trusts "*"
contract OpenContract { }
```

::: warning
Setting `trusts` to `"*"` means any contract on the network can call yours. Use this only when your contract is designed as a public utility. For most contracts, leave the default empty array or specify explicit contract hashes.
:::

### extra

Arbitrary metadata attached to the contract. The compiler populates baseline fields automatically:

| Key           | Value                                            |
| ------------- | ------------------------------------------------ |
| `Author`      | Compiler author identifier                       |
| `Description` | `"Solidity contract '<Name>' compiled to NeoVM"` |
| `Version`     | Compiler version in `major.minor.patch.0` format |
| `Compiler`    | Compiler ID string (e.g. `neo-solidity-0.1.0`)   |

Add custom fields via NatSpec `@custom:neo.manifest.extra.*` tags. Values are parsed as JSON when valid, otherwise stored as plain strings.

## Standards Auto-Detection

The compiler analyzes your contract's public method signatures and events to automatically populate the `supportedstandards` array. Detection is case-insensitive.

### NEP-17 Detection (Fungible Token)

| Requirement      | Detail                                                              |
| ---------------- | ------------------------------------------------------------------- |
| Required methods | `symbol`, `decimals`, `totalSupply`, `balanceOf`, `transfer`        |
| Exclusion        | `ownerOf` must NOT be present (indicates NFT)                       |
| Transfer event   | `Transfer` event with 3 parameters                                  |
| Hint             | `transfer` should have 4 parameters: `from`, `to`, `amount`, `data` |
| Near-miss        | Warning emitted if 3+ of 5 required methods are present             |

### NEP-11 Detection (Non-Fungible Token)

| Requirement        | Detail                                                       |
| ------------------ | ------------------------------------------------------------ |
| Core methods       | `balanceOf` AND `ownerOf`                                    |
| Transfer mechanism | At least one of: `transfer`, `transferFrom`, `tokensOf`      |
| Transfer event     | `Transfer` event with 4 parameters                           |
| Hint               | `transfer` should have 3 parameters: `to`, `tokenId`, `data` |
| Near-miss          | Warning if `ownerOf` present but no transfer mechanism       |

### NEP-24 Detection (Token Discovery / Royalty)

| Requirement | Detail                                     |
| ----------- | ------------------------------------------ |
| Trigger     | `tokenUri` OR `royaltyInfo` method present |

### NEP-26 Detection (Contract Upgrade)

| Requirement      | Detail                                            |
| ---------------- | ------------------------------------------------- |
| Required methods | Both `update` AND `destroy`                       |
| Near-miss        | Info diagnostic if only one of the two is present |

::: tip
The compiler emits diagnostics when your contract almost matches a standard. Check compiler output for `[warning][NEP-17]` or `[info][NEP-26]` messages to catch missing methods before deployment.
:::

## Permission Inference

The compiler performs static analysis on the compiled IR to infer the minimal set of permissions your contract needs. The goal is to emit the narrowest possible permissions — wildcards are used only when the compiler cannot determine the call target or method at compile time.

### How it works

1. **Native contract calls** — When your code calls a native contract (GAS, NEO, StdLib, CryptoLib, etc.), the compiler emits an explicit permission with the native contract's deterministic hash and the specific method name.

2. **Known contract address calls** — When `Syscalls.contractCall()` is invoked with a compile-time constant address (literal or `constant` variable), the compiler emits a permission for that specific contract hash.

3. **Known method name** — When the method name is a string literal, the compiler restricts the permission to that specific method.

4. **Dynamic target or method** — When the contract address or method name is a runtime variable, the compiler falls back to a wildcard for the unknown component.

5. **Self-calls** — Calls to `address(this)` are recognized and do not generate wildcard permissions, since Neo N3 always allows contracts to call themselves.

### Example: native contract call

A Solidity call to `NativeCalls.gasTransfer(from, to, amount, data)` generates:

```json
{
    "contract": "0xd2a4cff31913016155e38e474a2c06d08be276cf",
    "methods": ["transfer"]
}
```

### Example: dynamic target, static method

```solidity
interface IFace {
    function ping(uint256 value) external returns (uint256);
}

contract Caller {
    function callPing(address target, uint256 value) public returns (uint256) {
        return IFace(target).ping(value);
    }
}
```

Generates a wildcard contract restricted to the known method:

```json
{ "contract": "*", "methods": ["ping"] }
```

### Example: static target, dynamic method

```solidity
contract Caller {
    function callGas(string memory method) public returns (bytes memory) {
        return Syscalls.contractCall(NativeCalls.GAS_CONTRACT, method, abi.encode());
    }
}
```

Generates a specific contract with wildcard methods:

```json
{
    "contract": "0xd2a4cff31913016155e38e474a2c06d08be276cf",
    "methods": "*"
}
```

### Implicit native permissions

Certain Solidity constructs require native contract calls that are not visible in your source code:

| Construct                             | Native Permission                                             |
| ------------------------------------- | ------------------------------------------------------------- |
| `mapping` storage access              | `StdLib.serialize` + `CryptoLib.keccak256`                    |
| Struct field storage                  | `CryptoLib.keccak256` (+ `StdLib.serialize` for mapping keys) |
| `keccak256()`                         | `CryptoLib.keccak256`                                         |
| `ecrecover()`                         | `CryptoLib.recoverSecp256K1`                                  |
| `abi.encode()` / `abi.encodePacked()` | `StdLib.serialize`                                            |
| `abi.decode()`                        | `StdLib.deserialize`                                          |
| Cross-contract calls                  | `StdLib.serialize` + `StdLib.deserialize`                     |
| `block.number`                        | `Ledger.currentIndex`                                         |
| Contract deployment                   | `ContractManagement.deploy`                                   |
| `getContract()`                       | `ContractManagement.getContract` + `StdLib.serialize`         |

## Permission Hardening

### CLI Flags

Use these flags to reject manifests containing wildcard permissions at compile time:

| Flag                          | Effect                                                  |
| ----------------------------- | ------------------------------------------------------- |
| `--deny-wildcard-permissions` | Reject full wildcard (`contract="*"` AND `methods="*"`) |
| `--deny-wildcard-contracts`   | Reject any entry with `contract="*"`                    |
| `--deny-wildcard-methods`     | Reject any entry with `methods="*"`                     |

```bash
# Production build — reject all wildcards
neo-solc contract.sol \
  --deny-wildcard-permissions \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -I devpack -o build/
```

::: warning
When a deny flag is triggered, compilation fails with a `Manifest` error. The error message identifies which wildcard type was detected. Use permission override files to replace wildcards with explicit entries.
:::

### Permission Override Files

Override files let you replace compiler-inferred wildcard permissions with explicit entries:

```bash
neo-solc contract.sol \
  --manifest-permissions permissions.json \
  --manifest-permissions-mode replace-wildcards \
  -I devpack -o build/
```

The override file is a JSON array of permission entries:

```json
[
    {
        "contract": "0xd2a4cff31913016155e38e474a2c06d08be276cf",
        "methods": ["transfer", "balanceOf"]
    },
    {
        "contract": "0x0102030405060708090a0b0c0d0e0f1011121314",
        "methods": ["ping"]
    }
]
```

Alternatively, wrap the array in an object with a `permissions` key:

```json
{
    "permissions": [
        {
            "contract": "0xd2a4cff31913016155e38e474a2c06d08be276cf",
            "methods": ["transfer"]
        }
    ]
}
```

**Override modes:**

| Mode                | Behavior                                                                                              |
| ------------------- | ----------------------------------------------------------------------------------------------------- |
| `replace-wildcards` | Replace only wildcard entries with override entries. Non-wildcard inferred permissions are preserved. |
| `merge`             | Merge override entries into the inferred permissions. Existing entries are extended, not replaced.    |

::: tip
Combine `--deny-wildcard-contracts` with `--manifest-permissions-mode replace-wildcards` to enforce that all wildcard contracts are replaced by your override file while keeping precise native contract permissions intact.
:::

## NatSpec Manifest Overrides

The compiler supports NatSpec custom tags to override manifest fields directly from Solidity source. Both `@custom:neo.manifest.*` and `@custom:manifest.*` prefixes are recognized.

### Supported Tags

| Tag                                       | Expected Value             | Description                           |
| ----------------------------------------- | -------------------------- | ------------------------------------- |
| `@custom:neo.manifest.name`               | String or JSON string      | Override contract name                |
| `@custom:neo.manifest.groups`             | JSON array                 | Override groups array                 |
| `@custom:neo.manifest.features`           | JSON object (must be `{}`) | Override features (must remain empty) |
| `@custom:neo.manifest.supportedstandards` | JSON array                 | Override detected standards           |
| `@custom:neo.manifest.trusts`             | JSON array or `"*"`        | Override trust list                   |
| `@custom:neo.manifest.extra.<Key>`        | JSON value or plain string | Add custom extra metadata field       |

### Full Example

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title GoldToken
 * @custom:neo.manifest.name GoldToken
 * @custom:neo.manifest.supportedstandards ["NEP-17"]
 * @custom:neo.manifest.trusts ["0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"]
 * @custom:neo.manifest.extra.Author "Acme Corp"
 * @custom:neo.manifest.extra.Repository "https://github.com/acme/gold-token"
 * @custom:neo.manifest.extra.Build {"commit":"abc123","branch":"main"}
 */
contract GoldToken {
    mapping(address => uint256) private _balances;
    uint256 private _totalSupply;

    event Transfer(address indexed from, address indexed to, uint256 amount);

    function symbol() public pure returns (string memory) { return "GOLD"; }
    function decimals() public pure returns (uint256) { return 8; }
    function totalSupply() public view returns (uint256) { return _totalSupply; }
    function balanceOf(address account) public view returns (uint256) { return _balances[account]; }

    function transfer(address from, address to, uint256 amount, bytes memory data)
        public returns (bool)
    {
        require(_balances[from] >= amount, "insufficient balance");
        _balances[from] -= amount;
        _balances[to] += amount;
        emit Transfer(from, to, amount);
        return true;
    }
}
```

::: info
Values for override tags are parsed as JSON first. If JSON parsing fails, the raw string is used as-is. This means `@custom:neo.manifest.name MyToken` and `@custom:neo.manifest.name "MyToken"` both work.
:::

## Manifest Warnings

The compiler emits structured warnings for manifest-related issues. In standard JSON mode, these appear in the `errors` array with `severity: "warning"`.

| Code                         | Type                 | Trigger                                                 |
| ---------------------------- | -------------------- | ------------------------------------------------------- |
| `COMPILER_WARNING`           | `CompilerWarning`    | General compiler warning (default code)                 |
| `NEF_SOURCE_TRUNCATED`       | `NefSourceTruncated` | `--nef-source` value exceeds 256 bytes                  |
| `MANIFEST_FULL_WILDCARD`     | `Validation`         | Both `contract="*"` and `methods="*"` in a single entry |
| `MANIFEST_WILDCARD_CONTRACT` | `Validation`         | `contract="*"` in a permission entry                    |
| `MANIFEST_WILDCARD_METHODS`  | `Validation`         | `methods="*"` in a permission entry                     |

::: info
Wildcard warnings are informational by default. They become hard errors when the corresponding `--deny-wildcard-*` flag is set.
:::

## Complete Example

Below is the full `manifest.json` output for the `GoldToken` contract above:

```json
{
    "name": "GoldToken",
    "groups": [],
    "features": {},
    "supportedstandards": ["NEP-17"],
    "abi": {
        "methods": [
            {
                "name": "symbol",
                "parameters": [],
                "returntype": "String",
                "offset": 0,
                "safe": true
            },
            {
                "name": "decimals",
                "parameters": [],
                "returntype": "Integer",
                "offset": 0,
                "safe": true
            },
            {
                "name": "totalSupply",
                "parameters": [],
                "returntype": "Integer",
                "offset": 0,
                "safe": true
            },
            {
                "name": "balanceOf",
                "parameters": [{ "name": "account", "type": "Hash160" }],
                "returntype": "Integer",
                "offset": 0,
                "safe": true
            },
            {
                "name": "transfer",
                "parameters": [
                    { "name": "from", "type": "Hash160" },
                    { "name": "to", "type": "Hash160" },
                    { "name": "amount", "type": "Integer" },
                    { "name": "data", "type": "Any" }
                ],
                "returntype": "Boolean",
                "offset": 0,
                "safe": false
            }
        ],
        "events": [
            {
                "name": "Transfer",
                "parameters": [
                    { "name": "from", "type": "Hash160" },
                    { "name": "to", "type": "Hash160" },
                    { "name": "amount", "type": "Integer" }
                ]
            }
        ]
    },
    "permissions": [
        {
            "contract": "0xacce6fd80d44e1796aa0c2c625e9e4e0ce39efc0",
            "methods": ["serialize", "deserialize"]
        },
        {
            "contract": "0x726cb6e0cd8628a1350a611384688911ab75f51b",
            "methods": ["keccak256"]
        }
    ],
    "trusts": ["0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"],
    "extra": {
        "Author": "Acme Corp",
        "Description": "Solidity contract 'GoldToken' compiled to NeoVM",
        "Version": "0.1.0.0",
        "Compiler": "neo-solidity-0.1.0",
        "Repository": "https://github.com/acme/gold-token",
        "Build": { "commit": "abc123", "branch": "main" }
    }
}
```

Key observations:

- `supportedstandards` is `["NEP-17"]` — the compiler detected all five required methods and the `Transfer` event.
- `permissions` includes `StdLib.serialize` and `CryptoLib.keccak256` — these are required by the `mapping` storage access pattern, not by any explicit cross-contract call in the source.
- `trusts` reflects the NatSpec override, restricting callers to the NEO native contract.
- `safe: true` on `symbol`, `decimals`, `totalSupply`, and `balanceOf` because they are `pure` or `view` functions.
- `extra` merges compiler defaults with NatSpec overrides. The `Author` field is overridden by the NatSpec tag.

## Security Considerations

- **Wildcard permissions** — `contract="*"` with `methods="*"` allows your contract to call any method on any contract. This is the maximum attack surface. Use `--deny-wildcard-permissions` in production builds.
- **Wildcard contracts** — `contract="*"` with specific methods still allows calling that method on any contract, which may include malicious ones. Use `--deny-wildcard-contracts` when possible.
- **Wildcard methods** — Specific contract with `methods="*"` allows calling any method on that contract. Prefer explicit method lists.
- **Trust field** — `trusts: "*"` means any contract on the network can invoke your contract's methods. Leave as `[]` unless your contract is a public utility.
- **Supported standards** — Verify that `supportedstandards` matches your intent. Declaring NEP-17 compliance without implementing the full spec can cause wallet and explorer integration failures.
- **Safe methods** — Ensure methods marked `safe: true` are genuinely read-only. A `view` function that modifies state through assembly or low-level calls will pass compilation but may behave unexpectedly on-chain.
- **Permission override files** — Treat override files as security-critical configuration. Review them in code review alongside your Solidity source.

## See Also

- [Native Contracts](/internals/native-contracts) — Deterministic hashes and devpack wrappers for NEO, GAS, StdLib, CryptoLib, and other native contracts.
- [Syscalls](/internals/syscalls) — NeoVM syscall reference and how the compiler maps Solidity constructs to syscall sequences.
- [Devpack Standards](/additional-material/neo-standards) — NEP-17, NEP-11, and other standard interface definitions in the devpack.
