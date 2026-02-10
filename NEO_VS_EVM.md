# Neo N3 vs EVM: Migration Guide for Ethereum Developers

This guide covers the key differences between Ethereum (EVM) and Neo N3 that
matter when porting Solidity contracts using the Neo Solidity Compiler.

---

## Execution Model

| Aspect          | Ethereum (EVM)                     | Neo N3 (NeoVM)                                    |
| --------------- | ---------------------------------- | ------------------------------------------------- |
| Virtual machine | EVM (stack-based, 256-bit words)   | NeoVM (stack-based, arbitrary-precision integers) |
| Consensus       | Proof of Stake                     | dBFT (delegated Byzantine Fault Tolerance)        |
| Block finality  | Probabilistic (~12 min for safety) | Immediate (single-block finality)                 |
| Block time      | ~12 seconds                        | ~15 seconds                                       |
| Account model   | EOA + Contract accounts            | Universal contract accounts                       |

## Gas Model

On Ethereum, gas is paid in ETH and metered per opcode. On Neo N3, gas is a
separate NEP-17 token (GAS) and fees are charged per transaction, not per
opcode within a call.

**Key differences:**

- There is no `gasleft()` equivalent on Neo. The entire transaction has a
  system fee budget set by the sender.
- `{gas: ...}` call options are accepted by the compiler but ignored at
  runtime.
- `{value: ...}` has no direct equivalent. Value transfer uses NEP-17
  `transfer` calls (see Token Standards below).

```solidity
// EVM: send ETH with a call
target.call{value: 1 ether, gas: 50000}(data);

// Neo: transfer GAS explicitly, then call
NativeCalls.gasTransfer(address(this), target, amount);
target.call(data);
```

## Storage Model

Ethereum uses a slot-based storage model where each 256-bit slot is addressed
by a `keccak256` hash. Neo N3 uses a key-value store accessed through
`System.Storage.*` syscalls.

| Aspect         | Ethereum                    | Neo N3                                   |
| -------------- | --------------------------- | ---------------------------------------- |
| Addressing     | 256-bit slot index          | Arbitrary-length byte key                |
| Hash function  | keccak256                   | Prefix concatenation                     |
| Mappings       | `keccak256(key . slot)`     | `prefix + key` byte concatenation        |
| Dynamic arrays | `keccak256(slot) + index`   | `prefix + length_key` / `prefix + index` |
| Cost model     | Cold/warm SLOAD distinction | Flat per-byte read/write fee             |

The compiler handles this translation automatically. Solidity `mapping` and
array declarations produce the correct Neo storage key layout without manual
intervention.

```solidity
// Both platforms: same Solidity source
mapping(address => uint256) public balances;

// EVM storage: keccak256(abi.encode(addr, 0))
// Neo storage: prefix_0 + addr_bytes -> BigInteger value
```

## Token Standards

| Ethereum | Neo N3                 | Purpose              |
| -------- | ---------------------- | -------------------- |
| ERC-20   | NEP-17                 | Fungible tokens      |
| ERC-721  | NEP-11 (non-divisible) | Non-fungible tokens  |
| ERC-1155 | NEP-11 (divisible)     | Semi-fungible tokens |

The compiler detects ERC interface patterns and maps them to NEP equivalents
in the generated manifest.

**Key API differences:**

```solidity
// ERC-20 transfer (caller-initiated)
function transfer(address to, uint256 amount) external returns (bool);

// NEP-17 transfer (three-party: from, to, amount, data)
function transfer(address from, address to, uint256 amount, bytes calldata data)
    external returns (bool);
```

On Neo, the `from` parameter is explicit and the runtime verifies the caller
is authorized via `Runtime.checkWitness(from)`. There is no implicit
`msg.sender` deduction for token transfers.

## Authorization: checkWitness vs msg.sender

On Ethereum, `msg.sender` identifies the immediate caller. On Neo N3, the
equivalent is `Runtime.checkWitness(address)`, which verifies that the given
account has signed the transaction or that the calling contract's hash matches.

```solidity
// EVM pattern
function withdraw(uint256 amount) external {
    require(msg.sender == owner, "Not owner");
    // ...
}

// Neo pattern (using devpack)
import {Runtime} from "devpack/libraries/Runtime.sol";

function withdraw(uint256 amount) external {
    require(Runtime.checkWitness(owner), "Not owner");
    // ...
}
```

The compiler maps `msg.sender` reads to the calling script hash where
possible, but `checkWitness` is the idiomatic Neo authorization primitive.

## Native Contracts vs Precompiles

Ethereum has precompiled contracts at fixed addresses (e.g., `0x01` for
ecrecover). Neo N3 has **native contracts** -- first-class system contracts
with their own script hashes, callable via `System.Contract.Call`.

| Ethereum Precompile | Neo N3 Native Contract | Purpose                           |
| ------------------- | ---------------------- | --------------------------------- |
| `0x01` ecrecover    | `CryptoLib`            | Signature verification            |
| `0x02` SHA-256      | `CryptoLib`            | Hashing                           |
| N/A                 | `GasToken`             | GAS (fee token) management        |
| N/A                 | `NeoToken`             | NEO (governance token) management |
| N/A                 | `PolicyContract`       | Network fee and account policies  |
| N/A                 | `OracleContract`       | Off-chain data requests           |
| N/A                 | `RoleManagement`       | Designated node role queries      |
| N/A                 | `LedgerContract`       | Block and transaction queries     |
| N/A                 | `ContractManagement`   | Deploy, update, destroy contracts |

The devpack provides Solidity wrappers in `NativeCalls.sol` and
`OracleService.sol` so that native contract calls look like regular function
calls in Solidity source.

## Contract Deployment

On Ethereum, contracts are deployed by sending a transaction with bytecode in
the `data` field. On Neo N3, deployment goes through the `ContractManagement`
native contract.

| Aspect               | Ethereum                            | Neo N3                                     |
| -------------------- | ----------------------------------- | ------------------------------------------ |
| Deployment mechanism | Transaction with init bytecode      | `ContractManagement.deploy(nef, manifest)` |
| Constructor          | Runs once during deploy tx          | `_deploy(data, update)` called by runtime  |
| Upgrade              | Deploy new contract + proxy pattern | `ContractManagement.update(nef, manifest)` |
| Destruction          | `selfdestruct` (deprecated)         | `ContractManagement.destroy()`             |
| Output artifacts     | ABI JSON + bytecode                 | `.nef` + `.manifest.json`                  |

```solidity
// Neo constructor pattern
// The compiler maps your Solidity constructor to _deploy
constructor(uint256 initialSupply) {
    totalSupply = initialSupply;
    balances[msg.sender] = initialSupply;
}
// Compiles to: _deploy(data, update) where data carries constructor args
```

## Events and Notifications

Ethereum events use `LOG0`-`LOG4` opcodes with indexed topics. Neo N3 uses
`System.Runtime.Notify` with a notification name and state array.

| Aspect         | Ethereum                      | Neo N3                              |
| -------------- | ----------------------------- | ----------------------------------- |
| Opcode         | `LOG0` - `LOG4`               | `System.Runtime.Notify`             |
| Indexed params | Up to 3 (topics)              | Mapped to notification state fields |
| Event name     | `keccak256` hash in topic[0]  | String name in notification         |
| Subscription   | `eth_subscribe` / logs filter | ApplicationLog RPC queries          |

The compiler translates `emit` statements to `Runtime.Notify` calls
automatically. Indexed parameters are preserved in the manifest event
definition for client-side filtering.

```solidity
// Same Solidity source works on both platforms
event Transfer(address indexed from, address indexed to, uint256 value);

function _transfer(address from, address to, uint256 amount) internal {
    // ...
    emit Transfer(from, to, amount);
    // EVM: LOG3 with keccak256("Transfer(address,address,uint256)") as topic[0]
    // Neo: Runtime.Notify("Transfer", [from, to, amount])
}
```

## Migration Checklist

When porting an Ethereum Solidity contract to Neo N3 using `neo-solc`:

1. **Replace `msg.value` patterns** with explicit NEP-17 transfers via
   `NativeCalls.gasTransfer` or `NativeCalls.neoTransfer`. Implement
   `onNEP17Payment` to receive tokens.

2. **Replace `msg.sender` authorization** with `Runtime.checkWitness(addr)`
   where the caller's identity must be verified cryptographically.

3. **Remove inline assembly** blocks. Use devpack syscall wrappers for any
   low-level operations (storage, crypto, runtime queries).

4. **Replace `selfdestruct`** with `ContractManagement.destroy()`.

5. **Replace `create` / `create2`** with `ContractManagement.deploy()`.

6. **Replace `delegatecall`** patterns. Neo contracts have isolated storage;
   use direct cross-contract calls via `System.Contract.Call` instead.

7. **Map ERC interfaces to NEP standards**. The compiler does this
   automatically when it detects ERC-20/721 patterns, but verify the
   generated manifest `supportedstandards` field.

8. **Test on Neo-Express** before deploying to TestNet. Use the smoke test
   scripts in `examples/` as a starting point.

---

For the full feature support matrix, see [FEATURE_MATRIX.md](./FEATURE_MATRIX.md).
For recent changes, see [CHANGELOG.md](./CHANGELOG.md).
