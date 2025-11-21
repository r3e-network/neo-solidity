# Recent test additions (Delta)

- CLI / Standard JSON
  - Negative standard detection for incomplete method sets
  - State mutability label mapping
  - Metadata keccak defaults and hex prefixing
- Metadata/Manifest
  - NEP-24 detection, output prefix sanitization
  - Storage map slot/name assertions
  - ABI content checks for constructors, functions, and events
- Bytecode
  - Empty-contract RET fallback
  - CALL offset patching between functions
- Runtime execution
  - RET behavior with/without return data
  - JMP/JMPIF/JMPIFNOT control flow
  - Caller account validation
  - Stack overflow guard and memory bounds errors

Use `cargo test` (already green) to run everything. EOF
