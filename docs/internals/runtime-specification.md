# Runtime Specification

Complete reference for the embedded NeoVM runtime used by the Neo Solidity compiler for testing and execution. This documents the execution model, opcode support, syscall implementation, gas accounting, and storage emulation.

## NeoVM Execution Model

The Neo Virtual Machine (NeoVM) is a stack-based virtual machine designed for executing smart contracts on the Neo N3 blockchain. The embedded runtime in `neo-solc` faithfully emulates this execution model for local testing.

### Stack Machine Architecture

NeoVM uses two stacks for computation:

- **Evaluation Stack** -- the primary operand stack. All instructions consume operands from and push results to this stack. Stack items can be integers (arbitrary precision), byte arrays, booleans, arrays, structs, maps, or interop handles.
- **Alt Stack** -- an auxiliary stack for temporary storage. Items can be moved between the evaluation stack and alt stack using `TOALTSTACK` and `FROMALTSTACK`.

### Slots

NeoVM provides three categories of indexed variable slots, initialized per execution context:

| Slot Type    | Opcodes                                                      | Purpose                       |
| ------------ | ------------------------------------------------------------ | ----------------------------- |
| **Local**    | `LDLOC0`-`LDLOC6`, `LDLOC`, `STLOC0`-`STLOC6`, `STLOC`       | Function-local variables      |
| **Argument** | `LDARG0`-`LDARG6`, `LDARG`, `STARG0`-`STARG6`, `STARG`       | Function parameters           |
| **Static**   | `LDSFLD0`-`LDSFLD6`, `LDSFLD`, `STSFLD0`-`STSFLD6`, `STSFLD` | Contract-level static storage |

Slots are allocated with `INITSLOT` (locals + arguments) or `INITSSLOT` (static slots). Each slot holds a single stack item of any type.

### Execution Contexts

Each function call creates a new execution context containing:

- Its own evaluation stack frame
- Local and argument slots (sized by `INITSLOT`)
- An instruction pointer into the script bytecode
- Exception handling state (try/catch/finally frames)

The `CALL` instruction pushes a new context; `RET` pops it and returns control to the caller.

## Opcode Support

The embedded runtime implements the full Neo N3 opcode set. Opcodes are organized by category below with their hex codes and gas costs.

### Constants

| Opcode           | Hex           | Gas | Description                      |
| ---------------- | ------------- | --- | -------------------------------- |
| `PUSHINT8`       | `0x00`        | 1   | Push 1-byte signed integer       |
| `PUSHINT16`      | `0x01`        | 1   | Push 2-byte signed integer       |
| `PUSHINT32`      | `0x02`        | 1   | Push 4-byte signed integer       |
| `PUSHINT64`      | `0x03`        | 1   | Push 8-byte signed integer       |
| `PUSHINT128`     | `0x04`        | 1   | Push 16-byte signed integer      |
| `PUSHINT256`     | `0x05`        | 1   | Push 32-byte signed integer      |
| `PUSHT`          | `0x08`        | 1   | Push `true`                      |
| `PUSHF`          | `0x09`        | 1   | Push `false`                     |
| `PUSHA`          | `0x0A`        | 1   | Push address (pointer)           |
| `PUSHNULL`       | `0x0B`        | 1   | Push null                        |
| `PUSHDATA1`      | `0x0C`        | 2   | Push data (1-byte length prefix) |
| `PUSHDATA2`      | `0x0D`        | 2   | Push data (2-byte length prefix) |
| `PUSHDATA4`      | `0x0E`        | 2   | Push data (4-byte length prefix) |
| `PUSHM1`         | `0x0F`        | 1   | Push -1                          |
| `PUSH0`-`PUSH16` | `0x10`-`0x20` | 1   | Push small integers 0 through 16 |

### Flow Control

| Opcode                    | Hex             | Gas | Description                                      |
| ------------------------- | --------------- | --- | ------------------------------------------------ |
| `NOP`                     | `0x21`          | 1   | No operation                                     |
| `JMP` / `JMP_L`           | `0x22` / `0x23` | 2   | Unconditional jump (short/long offset)           |
| `JMPIF` / `JMPIF_L`       | `0x24` / `0x25` | 2   | Jump if top is true                              |
| `JMPIFNOT` / `JMPIFNOT_L` | `0x26` / `0x27` | 2   | Jump if top is false                             |
| `JMPEQ` / `JMPEQ_L`       | `0x28` / `0x29` | 2   | Jump if top two are equal                        |
| `JMPNE` / `JMPNE_L`       | `0x2A` / `0x2B` | 2   | Jump if top two are not equal                    |
| `JMPGT` / `JMPGT_L`       | `0x2C` / `0x2D` | 2   | Jump if greater than                             |
| `JMPGE` / `JMPGE_L`       | `0x2E` / `0x2F` | 2   | Jump if greater or equal                         |
| `JMPLT` / `JMPLT_L`       | `0x30` / `0x31` | 2   | Jump if less than                                |
| `JMPLE` / `JMPLE_L`       | `0x32` / `0x33` | 2   | Jump if less or equal                            |
| `CALL` / `CALL_L`         | `0x34` / `0x35` | 512 | Call function (short/long offset)                |
| `CALLA`                   | `0x36`          | 512 | Call address on stack                            |
| `CALLT`                   | `0x37`          | 512 | Call method token (native contract optimization) |
| `ABORT`                   | `0x38`          | 1   | Abort execution unconditionally                  |
| `ASSERT`                  | `0x39`          | 1   | Abort if top is false                            |
| `THROW`                   | `0x3A`          | 1   | Throw exception                                  |
| `TRY` / `TRY_L`           | `0x3B` / `0x3C` | 1   | Begin try block (catch/finally offsets)          |
| `ENDTRY` / `ENDTRY_L`     | `0x3D` / `0x3E` | 1   | End try block                                    |
| `ENDFINALLY`              | `0x3F`          | 1   | End finally block (rethrows pending exception)   |
| `RET`                     | `0x40`          | 0   | Return from current context                      |
| `SYSCALL`                 | `0x41`          | 10  | Invoke system interop (4-byte hash follows)      |

### Stack Operations

| Opcode     | Hex    | Gas | Description                  |
| ---------- | ------ | --- | ---------------------------- |
| `DEPTH`    | `0x43` | 2   | Push stack depth             |
| `DROP`     | `0x45` | 2   | Remove top item              |
| `NIP`      | `0x46` | 2   | Remove second item           |
| `XDROP`    | `0x48` | 2   | Remove item at index N       |
| `CLEAR`    | `0x49` | 2   | Clear the stack              |
| `DUP`      | `0x4A` | 2   | Duplicate top item           |
| `OVER`     | `0x4B` | 2   | Copy second item to top      |
| `PICK`     | `0x4D` | 2   | Copy item at index N to top  |
| `TUCK`     | `0x4E` | 2   | Insert top item below second |
| `SWAP`     | `0x50` | 2   | Swap top two items           |
| `ROT`      | `0x51` | 2   | Rotate top three items       |
| `ROLL`     | `0x52` | 2   | Move item at index N to top  |
| `REVERSE3` | `0x53` | 2   | Reverse top 3 items          |
| `REVERSE4` | `0x54` | 2   | Reverse top 4 items          |
| `REVERSEN` | `0x55` | 2   | Reverse top N items          |

### Splice and Buffer

| Opcode      | Hex    | Gas | Description                     |
| ----------- | ------ | --- | ------------------------------- |
| `NEWBUFFER` | `0x88` | 4   | Create new buffer of given size |
| `MEMCPY`    | `0x89` | 4   | Copy bytes between buffers      |
| `CAT`       | `0x8B` | 4   | Concatenate two byte arrays     |
| `SUBSTR`    | `0x8C` | 4   | Extract substring               |
| `LEFT`      | `0x8D` | 4   | Take left N bytes               |
| `RIGHT`     | `0x8E` | 4   | Take right N bytes              |

### Bitwise and Logic

| Opcode     | Hex    | Gas | Description           |
| ---------- | ------ | --- | --------------------- |
| `INVERT`   | `0x90` | 3   | Bitwise NOT           |
| `AND`      | `0x91` | 3   | Bitwise AND           |
| `OR`       | `0x92` | 3   | Bitwise OR            |
| `XOR`      | `0x93` | 3   | Bitwise XOR           |
| `EQUAL`    | `0x97` | 3   | Equality comparison   |
| `NOTEQUAL` | `0x98` | 3   | Inequality comparison |

### Numeric

| Opcode                     | Hex                      | Gas | Description                        |
| -------------------------- | ------------------------ | --- | ---------------------------------- |
| `SIGN`                     | `0x99`                   | 3   | Push sign (-1, 0, 1)               |
| `ABS`                      | `0x9A`                   | 3   | Absolute value                     |
| `NEGATE`                   | `0x9B`                   | 3   | Negate                             |
| `INC` / `DEC`              | `0x9C` / `0x9D`          | 3   | Increment / decrement              |
| `ADD` / `SUB`              | `0x9E` / `0x9F`          | 3   | Addition / subtraction             |
| `MUL` / `DIV` / `MOD`      | `0xA0` / `0xA1` / `0xA2` | 5   | Multiplication / division / modulo |
| `POW`                      | `0xA3`                   | 8   | Exponentiation                     |
| `SQRT`                     | `0xA4`                   | 6   | Integer square root                |
| `MODMUL` / `MODPOW`        | `0xA5` / `0xA6`          | 8   | Modular multiply / modular power   |
| `SHL` / `SHR`              | `0xA8` / `0xA9`          | 3   | Shift left / shift right           |
| `NOT`                      | `0xAA`                   | 2   | Logical NOT                        |
| `BOOLAND` / `BOOLOR`       | `0xAB` / `0xAC`          | 2   | Logical AND / OR                   |
| `NZ`                       | `0xB1`                   | 2   | Non-zero test                      |
| `NUMEQUAL` / `NUMNOTEQUAL` | `0xB3` / `0xB4`          | 3   | Numeric equality                   |
| `LT` / `LE` / `GT` / `GE`  | `0xB5`-`0xB8`            | 3   | Comparison operators               |
| `MIN` / `MAX`              | `0xB9` / `0xBA`          | 3   | Minimum / maximum                  |
| `WITHIN`                   | `0xBB`                   | 3   | Range check (a <= x < b)           |

### Compound and Collections

| Opcode                                  | Hex                      | Gas | Description                   |
| --------------------------------------- | ------------------------ | --- | ----------------------------- |
| `PACKMAP`                               | `0xBE`                   | 4   | Pack key-value pairs into map |
| `PACKSTRUCT`                            | `0xBF`                   | 4   | Pack items into struct        |
| `PACK` / `UNPACK`                       | `0xC0` / `0xC1`          | 4   | Pack/unpack array             |
| `NEWARRAY0` / `NEWARRAY` / `NEWARRAY_T` | `0xC2` / `0xC3` / `0xC4` | 4   | Create arrays                 |
| `NEWSTRUCT0` / `NEWSTRUCT`              | `0xC5` / `0xC6`          | 4   | Create structs                |
| `NEWMAP`                                | `0xC8`                   | 4   | Create empty map              |
| `SIZE`                                  | `0xCA`                   | 4   | Get collection/string size    |
| `HASKEY`                                | `0xCB`                   | 4   | Check if key exists           |
| `KEYS` / `VALUES`                       | `0xCC` / `0xCD`          | 4   | Get map keys / values         |
| `PICKITEM`                              | `0xCE`                   | 4   | Get item by index/key         |
| `APPEND`                                | `0xCF`                   | 4   | Append to array               |
| `SETITEM`                               | `0xD0`                   | 4   | Set item by index/key         |
| `REVERSEITEMS`                          | `0xD1`                   | 4   | Reverse collection in place   |
| `REMOVE`                                | `0xD2`                   | 4   | Remove item by index/key      |
| `CLEARITEMS`                            | `0xD3`                   | 4   | Clear all items               |
| `POPITEM`                               | `0xD4`                   | 4   | Pop last item from array      |

### Type Operations

| Opcode    | Hex    | Gas | Description                    |
| --------- | ------ | --- | ------------------------------ |
| `ISNULL`  | `0xD8` | 2   | Check if top is null           |
| `ISTYPE`  | `0xD9` | 2   | Check if top matches type code |
| `CONVERT` | `0xDB` | 2   | Convert top to specified type  |

Supported type codes:

| Code   | Type             |
| ------ | ---------------- |
| `0x20` | Boolean          |
| `0x21` | Integer          |
| `0x28` | ByteString       |
| `0x30` | Buffer           |
| `0x40` | Array            |
| `0x41` | Struct           |
| `0x48` | Map              |
| `0x60` | InteropInterface |

### Extensions

| Opcode      | Hex    | Gas | Description         |
| ----------- | ------ | --- | ------------------- |
| `ABORTMSG`  | `0xE0` | 1   | Abort with message  |
| `ASSERTMSG` | `0xE1` | 1   | Assert with message |

## Syscall Implementation

Syscalls are the bridge between NeoVM bytecode and the Neo N3 platform services. They are invoked via the `SYSCALL` opcode followed by a 4-byte interop ID.

### Syscall ID Computation

Syscall IDs are the first 4 bytes of the SHA-256 hash of the syscall name string:

```
ID = SHA256("System.Storage.Get")[0..4]
```

This is computed at compile time and embedded in the bytecode.

### Storage Syscalls

| Syscall                             | Description                                           |
| ----------------------------------- | ----------------------------------------------------- |
| `System.Storage.GetContext`         | Get the current contract's read-write storage context |
| `System.Storage.GetReadOnlyContext` | Get a read-only storage context                       |
| `System.Storage.AsReadOnly`         | Convert a context to read-only                        |
| `System.Storage.Get`                | Read a value by key from storage                      |
| `System.Storage.Put`                | Write a key-value pair to storage                     |
| `System.Storage.Delete`             | Delete a key from storage                             |
| `System.Storage.Find`               | Find entries by key prefix, returns iterator          |
| `System.Storage.Local.Get`          | Read from local storage                               |
| `System.Storage.Local.Put`          | Write to local storage                                |
| `System.Storage.Local.Delete`       | Delete from local storage                             |
| `System.Storage.Local.Find`         | Find in local storage by prefix                       |

### Iterator Syscalls

| Syscall                   | Description                                     |
| ------------------------- | ----------------------------------------------- |
| `System.Iterator.Next`    | Advance iterator to next entry; returns boolean |
| `System.Iterator.Value`   | Get current iterator entry value                |
| `System.Iterator.Dispose` | Free the iterator handle and release resources  |

Iterator handles are created by `Storage.Find` and are validated by `ISTYPE` (type code `0x80`) while alive. Disposed handles fail `ISTYPE` checks.

::: info
`Iterator.Dispose` returns a boolean and invalidates the handle. After disposal, `ISTYPE` checks against type code `0x80` will return false.
:::

### Runtime Syscalls

| Syscall                                 | Description                                                     |
| --------------------------------------- | --------------------------------------------------------------- |
| `System.Runtime.GetTrigger`             | Get execution trigger type                                      |
| `System.Runtime.Platform`               | Get platform identifier string                                  |
| `System.Runtime.GetNetwork`             | Get network magic number                                        |
| `System.Runtime.GetAddressVersion`      | Get address version byte                                        |
| `System.Runtime.GetTime`                | Get current block timestamp                                     |
| `System.Runtime.GetScriptContainer`     | Get the transaction that triggered execution                    |
| `System.Runtime.GetExecutingScriptHash` | Get hash of currently executing contract                        |
| `System.Runtime.GetCallingScriptHash`   | Get hash of the calling contract                                |
| `System.Runtime.GetEntryScriptHash`     | Get hash of the entry-point contract                            |
| `System.Runtime.LoadScript`             | Load and execute a script                                       |
| `System.Runtime.CheckWitness`           | Verify that the specified account has witnessed the transaction |
| `System.Runtime.GetInvocationCounter`   | Get number of times current contract was called                 |
| `System.Runtime.GetRandom`              | Get a pseudo-random number                                      |
| `System.Runtime.Log`                    | Emit a log message                                              |
| `System.Runtime.Notify`                 | Emit a notification event (name + state array)                  |
| `System.Runtime.GetNotifications`       | Get notifications emitted so far                                |
| `System.Runtime.GasLeft`                | Get remaining gas                                               |
| `System.Runtime.BurnGas`                | Burn specified amount of gas                                    |
| `System.Runtime.CurrentSigners`         | Get current transaction signers                                 |
| `System.Runtime.Serialize`              | Serialize a stack item to byte array                            |
| `System.Runtime.Deserialize`            | Deserialize a byte array back to a stack item                   |

### Crypto Syscalls

| Syscall                       | Description                           |
| ----------------------------- | ------------------------------------- |
| `System.Crypto.CheckSig`      | Verify a single signature (secp256k1) |
| `System.Crypto.CheckMultisig` | Verify multiple signatures            |

Additional crypto operations are available through the CryptoLib native contract: SHA256, RIPEMD160, Keccak256, Murmur32, Hash160, Hash256.

### Contract Syscalls

| Syscall                                 | Description                                         |
| --------------------------------------- | --------------------------------------------------- |
| `System.Contract.Call`                  | Call another contract by hash                       |
| `System.Contract.GetCallFlags`          | Get current call flags                              |
| `System.Contract.CreateStandardAccount` | Create standard account script hash from public key |
| `System.Contract.CreateMultisigAccount` | Create multisig account script hash                 |

## Gas Accounting

The embedded runtime tracks gas consumption to approximate production Neo N3 behavior.

### How Gas Works

Every opcode and syscall has an associated gas cost. The runtime deducts gas before executing each instruction. If gas reaches zero, execution halts with an out-of-gas error.

### Opcode Gas Costs

Gas costs by opcode category:

| Category             | Typical Gas | Notes                    |
| -------------------- | ----------- | ------------------------ |
| Constants (PUSH\*)   | 1-2         | PUSHDATA variants cost 2 |
| Flow control (JMP\*) | 2           | CALL variants cost 512   |
| Stack operations     | 2           | All stack manipulation   |
| Slot operations      | 2-3         | INITSLOT costs 3         |
| Splice/buffer        | 4           | All buffer operations    |
| Bitwise/logic        | 2-3         |                          |
| Numeric (basic)      | 3           | ADD, SUB, comparisons    |
| Numeric (complex)    | 5-8         | MUL, DIV, POW, MODPOW    |
| Collections          | 4           | All compound operations  |
| Type operations      | 2           | ISNULL, ISTYPE, CONVERT  |
| RET                  | 0           | Free                     |

### Syscall Gas Costs

| Syscall Category                           | Gas Cost | Notes                           |
| ------------------------------------------ | -------- | ------------------------------- |
| Storage context (GetContext, AsReadOnly)   | 1        | Cheap metadata operations       |
| Storage read (Get, Find)                   | 100      | Per-operation                   |
| Storage write (Put)                        | 1,000    | Most expensive common operation |
| Storage delete (Delete)                    | 100      |                                 |
| Iterator (Next, Value)                     | 1        |                                 |
| Runtime metadata (Platform, GetTime, etc.) | 1        |                                 |
| Runtime.CheckWitness                       | 200      | Signature verification          |
| Runtime.GetRandom                          | 50       |                                 |
| Runtime.Log / Notify                       | 1        |                                 |
| Crypto.CheckSig / CheckMultisig            | 1,000    | Cryptographic verification      |
| Contract.Call                              | 10       |                                 |
| Contract.Create\*Account                   | 10       |                                 |

### Current Accuracy

Gas accounting is approximately 85% accurate compared to the production Neo N3 reference implementation. Known gaps:

- Dynamic costs for large integer operations are approximated
- Some complex operations use fixed costs instead of size-dependent costs
- Edge cases in exception handling gas effects need spec verification

::: tip
For precise gas estimation, deploy to Neo-Express and measure actual consumption. The embedded runtime provides a good approximation for development but should not be used as the sole source of gas budgeting for mainnet.
:::

## Storage Emulation

The embedded runtime provides a full in-memory storage emulation layer.

### Storage Context Model

Each contract has its own isolated storage context. Storage contexts can be:

- **Read-write** -- obtained via `System.Storage.GetContext`
- **Read-only** -- obtained via `System.Storage.GetReadOnlyContext` or `AsReadOnly`

Write operations (`Put`, `Delete`) on a read-only context will fail.

### Get / Put / Delete Semantics

- **Get**: Returns the byte array value for a given key, or null if the key does not exist.
- **Put**: Writes a key-value pair. Both key and value are byte arrays. Overwrites existing values.
- **Delete**: Removes a key-value pair. No error if the key does not exist.

### Find and Iterator Operations

`Storage.Find` takes a key prefix and returns an iterator handle over all matching entries:

1. The runtime materializes all entries matching the prefix into an ordered list
2. An iterator token (byte handle) is created and pushed onto the stack
3. `Iterator.Next` advances the cursor and returns `true` if an entry is available
4. `Iterator.Value` returns the current entry as a key-value struct
5. Iterator handles are automatically invalidated when disposed

::: warning
The current implementation materializes all matching entries at `Find` time. This works correctly but is less efficient than streaming for large datasets. See [Parity and Limitations](/internals/parity-and-limitations) for details.
:::

## Event and Notification System

### Runtime.Notify Semantics

The `System.Runtime.Notify` syscall emits a notification event consisting of:

- **Event name**: A string identifying the event type
- **State array**: An array of stack items containing the event data

In Solidity, `emit` statements are compiled to `Runtime.Notify` calls. The event name corresponds to the Solidity event name, and the indexed/non-indexed parameters are packed into the state array.

```solidity
// Solidity
emit Transfer(from, to, amount);

// Compiles to NeoVM:
// PUSH3 args onto stack, PACK into array
// PUSHDATA "Transfer"
// SYSCALL System.Runtime.Notify
```

### Runtime.Log

`System.Runtime.Log` emits a simple string log message. This is used for debugging and does not appear in on-chain event logs.

## Exception Handling

### TRY / CATCH / FINALLY Opcodes

NeoVM supports structured exception handling:

- `TRY` begins a try block, specifying catch and/or finally offsets
- `THROW` raises an exception (top of stack becomes the exception object)
- `ENDTRY` exits the try block normally, jumping to the end offset
- `ENDFINALLY` exits the finally block; if an exception is pending, it is rethrown

### Exception Propagation

1. When `THROW` is executed, the VM searches the current context's exception handler stack
2. If a catch handler is found, execution jumps to the catch offset with the exception on the stack
3. If a finally handler exists, it executes before propagation continues
4. If no handler is found in the current context, the exception propagates to the caller
5. `ENDFINALLY` rethrows any pending exception that was not handled by the catch block

### Gas Effects

Gas continues to be consumed during exception handling. The `TRY`, `ENDTRY`, and `ENDFINALLY` opcodes each cost 1 gas unit. Stack unwinding during propagation does not have additional gas cost beyond the opcodes executed.

## Native Contract Integration

The embedded runtime includes a registry of Neo N3 native contracts:

| Contract           | Description                         |
| ------------------ | ----------------------------------- |
| NEO                | NEO governance token                |
| GAS                | GAS utility token                   |
| ContractManagement | Deploy, update, and query contracts |
| Policy             | Network fee policies                |
| Oracle             | Oracle request service              |
| RoleManagement     | Node role management                |
| Notary             | Notary service                      |
| Treasury           | Treasury management                 |
| Ledger             | Blockchain data access              |
| CryptoLib          | Cryptographic functions             |
| StdLib             | Standard library utilities          |

### ContractManagement

The embedded runtime maintains a stateful in-memory contract registry that supports:

- `Deploy` -- register a new contract with NEF and manifest
- `Update` -- update an existing contract (increments update counter)
- `GetContract` -- query contract metadata by hash

This registry persists within a single test execution and is used by both syscall-based and native contract calls.

## Embedded Runtime vs Production Neo N3

### What the Embedded Runtime Provides

- Full NeoVM opcode execution with correct stack semantics
- Storage read/write/find with iterator support
- Event notification capture
- Gas tracking (approximate)
- Exception handling with try/catch/finally
- Native contract call routing
- Crypto operations (SHA256, RIPEMD160, Keccak256, CheckSig)

### Differences from Production

| Aspect                 | Embedded Runtime                | Production Neo N3           |
| ---------------------- | ------------------------------- | --------------------------- |
| Storage                | In-memory, ephemeral            | Persistent on-chain         |
| Gas costs              | ~85% accurate                   | Exact per specification     |
| Blockchain data        | Stub/placeholder values         | Real chain state            |
| Signature verification | Best-effort secp256k1           | Full Neo crypto stack       |
| Iterator behavior      | Materialized (eager)            | Streaming (lazy)            |
| ByteString vs Buffer   | Both treated as byte array      | Distinct mutation semantics |
| Oracle                 | Returns deterministic pseudo ID | Real oracle network         |
| Network state          | Fixed test values               | Live network data           |

### When to Use Neo-Express

Use the embedded runtime for:

- Unit testing individual contract functions
- Verifying compilation output correctness
- Quick iteration during development

Use Neo-Express or testnet for:

- Integration testing with multiple contracts
- Accurate gas measurement
- Testing with real blockchain state
- Verifying oracle and cross-contract interactions
- Pre-mainnet validation

## See Also

- [Architecture](/internals/architecture) -- compiler pipeline that generates the bytecode
- [Parity and Limitations](/internals/parity-and-limitations) -- known runtime gaps
- [Error Reference](/advisory-content/error-reference) -- runtime-related error codes
- [CLI Reference](/compiler/using-the-compiler) -- compiler options affecting code generation
- [Troubleshooting](/advisory-content/troubleshooting) -- common issues and solutions
