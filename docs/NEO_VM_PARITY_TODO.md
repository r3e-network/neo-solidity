## NeoVM Parity TODO

This file tracks remaining gaps between the runtime in `src/runtime/execution.rs` and the full Neo N3 VM/syscall surface. Current implementation covers:
- Full jump suite (JMP/JMPIF*/JMPEQ/JMPNE/JMPGT/JMPLT/JMPGE/JMPLE), CALL/CALLA/CALLT, TRY/ENDTRY/ENDFINALLY with catch/finally rethrow semantics, ASSERT/ABORT/THROW handling.
- Stack ops (DEPTH/DROP/NIP/XDROP/CLEAR/DUP/OVER/PICK/TUCK/SWAP/ROT/ROLL/REVERSE3/REVERSE4/REVERSEN), locals/args + static slots (INITSSLOT/LDSFLD/STSFLD/LDARG/STARG), type ops (ISNULL/ISTYPE/CONVERT), arrays/maps/buffers, packing/pick/set, SIZE aliases, buffer ops (CAT/SUBSTR/LEFT/RIGHT).
- Arithmetic/logic (SIGN/ABS/NEGATE/INC/DEC/MODMUL/MODPOW/POW/SQRT/min/max/within), logical ops, SHA256/HASH256 opcode handlers, and hash/crypto helpers via syscalls.
- Syscalls routed through a central registry with coverage for storage (Get/Put/Delete/Find/contexts + Iterator Next/Value/Dispose), runtime (platform/network/time/gas/random/invocation counter/log/notify/serialize/deserialize/checkWitness), crypto (SHA256/RIPEMD160/Keccak256/Murmur32/Hash160/Hash256/CheckSig/CheckMultisig), blockchain metadata (height/block/tx/contract placeholders), contract calls (Call/CallEx/GetCallFlags/create account), oracle/policy stubs, and native contract mapping (NEO/GAS/Policy/Oracle/ContractManagement/RoleManagement). A stateful in-memory ContractManagement registry tracks deployed contracts and update counters.
- Iterator handles are true tokens with disposal; Storage.Find materializes prefix-matched entries plus an iterator token.
- Syscall gas table uses per-syscall values (storage/iterator/runtime/crypto/oracle/contract) instead of coarse buckets.
- Collection helpers include NEWARRAY0/NEWSTRUCT/NEWSTRUCT0/NEWMAP, HASKEY/KEYS/VALUES at 0xCB/0xCC/0xCD, PICKITEM0/1, APPEND, SETITEM0/1, REMOVE, CLEARITEMS, POPITEM.

Remaining items (non-exhaustive – align with the official Neo N3 spec):

### Opcodes
- Structured exception handling needs full spec verification (stack unwinding, handler selection, gas effects).
- Iterator helpers and RANGE/REVERSE-specific collection ops are still missing; NEWBUFFER may need spec-aligned opcode/value instead of aliases.
- Gas costs should be updated to match NeoVM tables for every opcode (opcode-level fees are still approximate).
- Distinguish ByteString vs Buffer type codes (0x28 vs 0x30) and add buffer mutation semantics.

### Syscalls / Native Contracts
- Runtime: flesh out `CheckWitness` semantics, `GetRandom` nondeterminism, proper Notify/Log handling (event emission).
- Crypto: implement real `CheckSig`/`CheckMultisig` over secp256k1, support additional hash functions if required.
- Blockchain: block/transaction accessors beyond height/time/random (e.g., `GetTransaction`, `GetBlock`, `GetContract`) and iterator semantics for `Storage.Find` that stream rather than materialize.
- Policy/ContractManagement/Ledger native contracts: method surfaces, stateful behavior, gas; contract registry should expose full manifest/NEF shape with hashes/checksums.
- Gas accounting for syscalls aligned to spec (now partially modeled; still approximate) and iterator disposal.

### Testing
- Add conformance tests per opcode/syscall against official NeoVM vectors.
- Add gas accounting regression tests once the gas table is implemented.
- Add conformance fixtures for ContractManagement (deploy/update/get), iterator disposal, ByteString vs Buffer type codes, and iterator streaming semantics.

Implementation of the above should follow the authoritative Neo N3 VM and native contract specification (opcode values/stack effects/gas, syscall IDs, method signatures). Replace current stubs with real semantics and remove placeholders once the spec-aligned behavior is defined.
