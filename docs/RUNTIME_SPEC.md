# NeoVM Runtime Specification (Current Emulator State)

This document captures the fidelity of the embedded Neo N3 runtime at
`src/runtime/execution.rs` against the official VM/syscall surface.

## Opcodes
- Opcode table is derived from the canonical Neo N3 enum (see `src/runtime/spec.rs`).
- Supported families: constants, flow control (JMP/JMPIF*/JMPEQ/JMPNE/JMPGT/JMPLT/JMPGE/JMPLE, CALL/A/T/T, TRY/ENDTRY/ENDFINALLY), stack ops, slots (LD*/ST* for args/locals/static), splice/buffer, bitwise, numeric, compound (PACK/MAP/STRUCT/NEWMAP/NEWARRAY_T/NEWSTRUCT), types (ISNULL/ISTYPE/CONVERT), extensions (ABORTMSG/ASSERTMSG).
- Structured exceptions: try/catch/finally stack with rethrow semantics; ENDFINALLY rethrows pending errors when required.
- Type codes: ISTYPE/CONVERT understand booleans (0x20), integers (0x21/0x22), ByteString/Buffer (0x28/0x30 treated as bytes), arrays/structs (0x40/0x41), maps (0x48), interop/iterator handles (0x60/0x80 best-effort).

## Syscalls
- Registry and IDs are generated via SHA-256(name) (see `src/runtime/spec.rs`).
- Implemented categories:
  - Storage: GetContext/GetReadOnlyContext, Get/Put/Delete, Find, Iterator.Next/Value/Dispose.
  - Runtime: Platform/GetNetwork/GetTime/GetBlockTime/GetGasLeft/GetInvocationCounter/CallingScriptHash/EntryScriptHash/ExecutingScriptHash/CheckWitness/Log/Notify/Serialize/Deserialize/GetRandom.
  - Crypto: SHA256/RIPEMD160/Keccak256/Murmur32/Hash160/Hash256/CheckSig/CheckMultisig (best-effort secp256k1).
  - Blockchain: GetHeight, GetBlock/GetTransaction*/GetContract/GetCommittee/GetValidators/GetBlockHash (stub data).
  - Contract: Call/CallEx/GetCallFlags/CreateStandardAccount/CreateMultisigAccount.
  - ContractManagement: Deploy/Update/GetContract (stateful in-memory registry with NEF/manifest + update counter).
  - Oracle: Request (returns deterministic pseudo ID).
  - Policy: GetFeePerByte/GetExecFeeFactor/GetStoragePrice (stub values).
- Syscall gas hints: per-syscall table approximating Neo pricing (storage/iterator/runtime/crypto/oracle/contract) applied before execution; out-of-gas halts.

## Iterators
- `Storage.Find` produces a handle (byte token) tied to materialized entries; `Iterator.Next` advances the index, `Iterator.Value` reads the current entry, `Iterator.Dispose` frees the handle and returns a boolean.
- Iterator handles are validated by ISTYPE (type code 0x80) while alive; disposed handles fail ISTYPE.

## Native Contracts
- Hash table in `spec.rs` covers NEO, GAS, ContractManagement, Policy, Oracle, RoleManagement, Ledger placeholder.
- In-memory ContractManagement registry backs Deploy/Update/GetContract and is reused by native `contractmanagement.*` calls.

## Gas & Stack Effects
- Syscall gas: per-syscall hints as above.
- Opcode gas: table in `spec.rs` mirrors Neo opcode surface (values are emulator-scaled; tighten as needed).

## Known Gaps (see `docs/NEO_VM_PARITY_TODO.md` for detail)
- Exact opcode gas/stack effects, ByteString vs Buffer mutation semantics, streaming iterators, richer native contract surfaces, and strict CheckSig/CheckMultisig parity remain to be implemented.
