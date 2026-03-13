# Types

Solidity is a statically typed language. Every variable (state and local) needs to have its type specified. Neo Solidity compiles these types down to native NeoVM representations.

## Value Types

### Booleans
`bool`: The possible values are constants `true` and `false`.
* **NeoVM Mapping:** Maps directly to NeoVM `Boolean`.

### Integers
`int` / `uint`: Signed and unsigned integers of various sizes. Keywords `uint8` to `uint256` in steps of 8 (unsigned of 8 up to 256 bits) and `int8` to `int256`. 

::: tip 💡 NeoVM Difference
NeoVM represents all integers as arbitrary-precision `BigInteger` values. This means overflow is impossible at runtime on NeoVM, though the compiler enforces type boundaries for signatures. 
::: 

### Fixed Point Numbers
`fixed` / `ufixed`: Not fully supported by mainline Solidity, and **not supported** by Neo Solidity.

### Address
`address`: Holds a 20 byte value.

::: tip 💡 NeoVM Difference
On NeoVM, `address` maps to a `UInt160` (the 20-byte script hash of an account or contract). While `address payable` is accepted by the compiler, direct value transfers via `.transfer()` and `.send()` are unsupported because Neo uses the NEP-17 token standard for value transfer.
:::

### Fixed-size byte arrays
`bytes1`, `bytes2`, `bytes3`, ..., `bytes32`. `byte` is an alias for `bytes1`.
* **NeoVM Mapping:** Maps to a fixed-length `ByteArray`. `bytes32` translates specifically to the Neo ABI type `Hash256`.

## Reference Types

### Data location
Variables of reference types have an additional annotation, the "data location".
* `memory`: Lifetime is limited to an external function call.
* `storage`: The location where the state variables are stored.
* `calldata`: Special data location that contains the function arguments. On NeoVM, this is treated identically to `memory`.

### Arrays
Arrays can have a compile-time fixed size or a dynamic size.
* **NeoVM Mapping:** Maps to the NeoVM `Array` type.

### Strings and Dynamic Byte Arrays
`bytes` and `string` are dynamic size arrays of bytes and characters.
* **NeoVM Mapping:** `string` maps to a UTF-8 `ByteString`. `bytes` maps to a dynamic `ByteArray`.

### Structs
Solidity provides a way to define new types in the form of structs.

::: tip 💡 NeoVM Difference
On NeoVM, structs map to an `Array` containing the struct's fields in memory. When written to `storage`, the entire struct is serialized as a single binary blob via `StdLib.serialize()`.
:::

## Mapping Types

Mapping types are declared as `mapping(_KeyType => _ValueType)`. 
* **NeoVM Mapping:** Maps to Neo's key-value Storage operations. Storage keys use deterministic, iterative hashing (`SHA256`) to produce collision-free access in Neo's global trie.

## Type Conversions

The compiler automatically handles upcasting between sizes. Explicit conversions are handled via `T(x)`. On NeoVM, many numerical conversions are conceptually no-ops because all numbers are `BigInteger` under the hood.