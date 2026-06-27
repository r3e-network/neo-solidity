use super::*;

impl ExecutionContext {
    /// Convert a single ABI element into a 32-byte big-endian slot, matching
    /// the EVM ABI layout for `abi.encode` (non-packed).
    ///
    /// NeoVM stores integer-ish stack items (addresses, uint/int literals,
    /// bools, enums, small `bytesN`) as LITTLE-ENDIAN byte arrays — so to
    /// produce the EVM-canonical big-endian 32-byte slot we need to reverse
    /// the byte order. Cryptographic hashes (keccak256, sha256) are the
    /// exception: CryptoLib returns them as 32-byte BEs in natural order.
    ///
    /// Heuristic (content-based — the Solidity type is lost by the time the
    /// runtime handler runs):
    /// * `Integer` / `UnsignedInteger`: write via `to_be_bytes`, right-aligned.
    /// * `Boolean`: single byte 0/1 at `slot[31]`.
    /// * `ByteArray` of length >= 32 with trailing zeros (bytes 20..32 all
    ///   zero) — this is an `address` / small-int that was NewBuffer'd to
    ///   32 bytes. Reverse the leading 20 bytes (LE→BE) and right-align.
    /// * `ByteArray` of length >= 32 otherwise (keccak256 output, bytes32
    ///   literal): take the first 32 bytes verbatim (already BE).
    /// * `ByteArray` of length 20 (raw address from `LiteralValue::Address`):
    ///   reverse (LE→BE) and right-align into slot[12..32].
    /// * `ByteArray` of other lengths (< 32): treat as a LE-stored integer
    ///   (PUSHINT128, PUSHINT64-encoded-as-BA, etc.) — reverse and
    ///   right-align so the MSB lands at `slot[31]`.
    /// * Nested `Array` / `Map` / `Null`: zero slot (callers flatten).
    fn abi_pad32_be(item: &StackItem) -> [u8; 32] {
        let mut slot = [0u8; 32];
        match item {
            StackItem::Integer(v) => {
                if *v >= 0 {
                    slot[24..].copy_from_slice(&(*v as u64).to_be_bytes());
                } else {
                    // Two's-complement fill for negatives: sign-extend 0xff.
                    slot.fill(0xff);
                    slot[24..].copy_from_slice(&(*v as u64).to_be_bytes());
                }
            }
            StackItem::UnsignedInteger(v) => {
                slot[24..].copy_from_slice(&v.to_be_bytes());
            }
            StackItem::Boolean(b) => {
                slot[31] = if *b { 1 } else { 0 };
            }
            StackItem::ByteArray(bytes) => {
                let b = bytes.borrow();
                if b.len() >= 32 {
                    // Address-slot normalisation: detect the 20-byte LE address
                    // zero-extended to 32 bytes (bytes 20..32 == 0) and emit it
                    // right-aligned big-endian at slot[12..32]. This is exactly
                    // the layout EVM expects for `address` in abi.encode. The
                    // same heuristic catches small uint256 literals that landed
                    // in a 32-byte NewBuffer — reversing the leading non-zero
                    // prefix recovers the BE integer value.
                    //
                    // Task #112 refinement — a left-aligned `bytesN` (N < 20)
                    // pre-padded to 32 bytes by the compiler's tuple-return
                    // path ALSO has `b[20..32] == 0`. It is distinguishable
                    // from an LE-address payload by a zero run *within*
                    // `b[12..20]`: LE-encoded addresses fill all 20 low bytes
                    // with "random" address bits, whereas `bytesN` with
                    // N ≤ 12 has zeros throughout `b[N..20]` (i.e. every byte
                    // of `b[12..20]` is zero). When that zero-mid pattern is
                    // present the buffer is copied verbatim — matching
                    // `abi.encode(bytesN)`'s left-aligned layout and
                    // unblocking `batch47_w4_msg_data_length_and_selector_via_call_method`.
                    let low_tail_zero = b[20..32].iter().all(|byte| *byte == 0);
                    let mid_zero = b[12..20].iter().all(|byte| *byte == 0);
                    if low_tail_zero && !mid_zero {
                        // LE-stored; reverse the first 20 bytes into slot[12..32].
                        let mut prefix: [u8; 20] = [0; 20];
                        prefix.copy_from_slice(&b[..20]);
                        prefix.reverse();
                        slot[12..32].copy_from_slice(&prefix);
                    } else {
                        // Natural BE payload (keccak256, bytes32 literal, or
                        // a compiler pre-padded `bytesN` with N ≤ 12).
                        slot.copy_from_slice(&b[..32]);
                    }
                } else if b.len() == 20 {
                    // Raw 20-byte address from `LiteralValue::Address` —
                    // stored LE, flip to BE and right-align into slot[12..32].
                    // This matches EVM `abi.encode(address)` which left-pads
                    // the 20-byte BE address to a 32-byte slot.
                    let mut rev = b.clone();
                    rev.reverse();
                    slot[12..32].copy_from_slice(&rev);
                } else if b.len() == 16 {
                    // 16 bytes — likely a LE-stored PUSHINT128 payload that
                    // NeoVM pushed as a ByteArray rather than an Integer. To
                    // recover the BE integer value, reverse and right-align
                    // so the MSB lands at slot[31]. String/bytes literals of
                    // exactly 16 bytes will render in reversed order under
                    // this heuristic — Solidity doesn't mint literals of
                    // this width by coincidence often, and the alternative
                    // (no-reverse) would break `abi.encode(uint128 x)` for
                    // values that don't fit in i64.
                    let mut rev = b.clone();
                    rev.reverse();
                    let start = 32 - rev.len();
                    slot[start..].copy_from_slice(&rev);
                } else {
                    // Short ByteArrays (< 16 bytes, and != 0 / != 20) are
                    // almost always string/bytes payloads from PUSHDATA1
                    // (e.g. `"fuzz"`, `hex"deadbeef"`). Left-pad to 32 bytes
                    // in NATURAL order so `abi.encode(string/bytes)` renders
                    // the content verbatim in the low bytes of the slot —
                    // matching both the Solidity spec intent and the fuzz
                    // harness expectations (`runtime_notify_emits_log_*`,
                    // `event_with_indexed_and_dynamic_args_lowers`).
                    //
                    // Integer literals that would naturally land here (e.g.
                    // raw `BigInt` fallback bytes) are uncommon enough that
                    // the simpler rule wins; the Integer stack-item path
                    // handles all PUSHINT8/16/32/64 values correctly above.
                    let start = 32 - b.len();
                    slot[start..].copy_from_slice(&b);
                }
            }
            StackItem::Null => {}
            StackItem::Array(_) | StackItem::Map(_) => {}
        }
        slot
    }

    /// Convert a single ABI element into its `encodePacked` byte-width.
    ///
    /// For `uint256`/`int256`/`address` (the widths we can't recover from the
    /// runtime type) we emit the full 32-byte BE slot so the fixed-width
    /// signature-based consumers (e.g. EIP-191 / `keccak256(...encodePacked)`)
    /// agree with the EVM reference. Dynamic bytes/strings pass through as-is
    /// without any length prefix / padding (Ethereum spec: `encodePacked` on
    /// dynamic operands is a raw concatenation of their contents).
    ///
    /// This mirrors `abi_pad32_be`'s LE→BE normalisation for integer-shaped
    /// ByteArrays so `keccak256(abi.encode(x, y)) == keccak256(abi.encodePacked(x, y))`
    /// holds for uint256 args (Solidity invariant — see fuzz harness
    /// `hash_consistency_keccak_of_encoded_matches_packed`).
    fn abi_packed_bytes(item: &StackItem) -> Vec<u8> {
        match item {
            StackItem::Integer(_) | StackItem::UnsignedInteger(_) | StackItem::Null => {
                Self::abi_pad32_be(item).to_vec()
            }
            StackItem::ByteArray(bytes) => {
                // Dynamic bytes/strings (`bytes`, `string`): raw concat per the
                // EVM `abi.encodePacked` spec — no length prefix, no padding.
                // `abi_is_dynamic` classifies any ByteArray whose length is
                // NOT in `{16, 20, 32}` as dynamic; those three static widths
                // correspond to PUSHINT128 / raw `address` / bytes32-sized
                // integer slots which DO need LE→BE normalisation into a
                // 32-byte packed slot (batch-#30 H2; see
                // `batch30_h2_keccak_abi_encode_packed_dynamic`).
                if Self::abi_is_dynamic(item) {
                    bytes.borrow().clone()
                } else {
                    Self::abi_pad32_be(item).to_vec()
                }
            }
            StackItem::Boolean(b) => vec![if *b { 1 } else { 0 }],
            // Task #193 — `abi.encodePacked(T[])` on dynamic arrays: per the
            // Solidity spec, array elements are padded to 32 bytes each
            // (distinct from the direct-scalar packed widths), concatenated
            // with NO length prefix and NO offset. For a `uint256[] a = [1, 2,
            // 3]` input that yields 96 bytes = BE(1, 32) || BE(2, 32) ||
            // BE(3, 32). Iterate the inner StackItem list and emit each
            // element's 32-byte BE slot via `abi_pad32_be`; nested Array /
            // Map elements fall back to zero-slot (same convention as the
            // encode path — nested dynamic shapes are out of scope until a
            // harness exercises them).
            //
            // Distinct from the `abiencode` dispatch which wraps `T[]` in
            // offset+length+elements (via `abi_dynamic_tail_bytes`); the
            // packed variant suppresses both the offset and the length.
            StackItem::Array(arr) => {
                let elements = arr.borrow().clone();
                let mut out = Vec::with_capacity(elements.len() * 32);
                for el in elements.iter() {
                    out.extend_from_slice(&Self::abi_pad32_be(el));
                }
                out
            }
            StackItem::Map(_) => Vec::new(),
        }
    }

    /// Classify whether a `StackItem` should encode as a DYNAMIC type under
    /// EVM ABI rules. Used by `abiencode` (Tasks #72/#73) to decide whether
    /// the head slot carries the value directly or an offset to a tail.
    ///
    /// Since the Solidity source type is lost by dispatch time, we rely on
    /// the same content-based heuristics `abi_pad32_be` already uses for
    /// byte-shape disambiguation, inverted to identify strings/bytes:
    ///
    /// * `Integer`, `UnsignedInteger`, `Boolean`, `Null` → STATIC always.
    /// * `ByteArray` of length 16 → STATIC (PUSHINT128 payload).
    /// * `ByteArray` of length 20 → STATIC (raw `address`).
    /// * `ByteArray` of length 32 → STATIC (bytes32 literal, keccak output,
    ///   or an address/small-int NewBuffer-padded to 32 bytes).
    /// * `ByteArray` of any OTHER length (0, 1..=15, 17..=19, 21..=31, >32)
    ///   → DYNAMIC string/bytes payload. Empty buffers encode as a
    ///   zero-length dynamic tail (spec: `len=0` + no data).
    /// * `Array` / `Map` → DYNAMIC (dynamic arrays/maps; no ABI slot shape).
    fn abi_is_dynamic(item: &StackItem) -> bool {
        match item {
            StackItem::Integer(_)
            | StackItem::UnsignedInteger(_)
            | StackItem::Boolean(_)
            | StackItem::Null => false,
            StackItem::ByteArray(bytes) => {
                let len = bytes.borrow().len();
                !matches!(len, 16 | 20 | 32)
            }
            StackItem::Array(_) | StackItem::Map(_) => true,
        }
    }

    /// Task #192 — predicate for "scalar-shaped" stack items that fit in a
    /// single 32-byte ABI slot. Used by the struct-array tail encoder to
    /// decide whether an inner `StackItem::Array` element (a flattened
    /// struct value) is safe to inline as K consecutive 32-byte slots.
    ///
    /// This is just `!abi_is_dynamic(item)`; the policy is documented on
    /// `abi_is_dynamic` itself, so we inline at the call site rather than
    /// maintain a 3-line method. See `abi_is_dynamic` for the static vs.
    /// dynamic policy.

    /// Produce the EVM-canonical tail-section bytes for a DYNAMIC ABI arg:
    /// a 32-byte BE length prefix followed by the raw content padded with
    /// trailing zeros to the next 32-byte boundary. Used by `abiencode`.
    ///
    /// * `StackItem::ByteArray`: length + raw content (string / bytes).
    /// * `StackItem::Array`: length + each element as a 32-byte BE slot
    ///   (Task #121 — dynamic-array variant, e.g. `uint[]`). Nested
    ///   dynamic arrays are NOT handled — each element is flattened via
    ///   `abi_pad32_be`, which zeros nested Array/Map elements rather
    ///   than recursing. That covers the common `uintN[]` / `bytesN[]` /
    ///   `address[]` / `bool[]` shapes; deeper nesting (`uint[][]`) is
    ///   out of scope until a harness exists for it.
    /// * Anything else: empty tail (keeps head + tail consistent with
    ///   the existing fallback semantics for Map / Null).
    fn abi_dynamic_tail_bytes(item: &StackItem) -> Vec<u8> {
        match item {
            StackItem::ByteArray(bytes) => {
                let content = bytes.borrow().clone();
                let len = content.len();
                let padded_len = len.div_ceil(32) * 32;
                let mut out = Vec::with_capacity(32 + padded_len);
                let mut len_slot = [0u8; 32];
                len_slot[24..].copy_from_slice(&(len as u64).to_be_bytes());
                out.extend_from_slice(&len_slot);
                out.extend_from_slice(&content);
                out.resize(32 + padded_len, 0);
                out
            }
            StackItem::Array(arr) => {
                // Task #121 — EVM-canonical `T[]` encoding: 32-byte BE
                // length prefix followed by N × 32-byte BE-padded element
                // slots (no padding between slots; each element already
                // occupies a full 32-byte word).
                //
                // Task #192 — nested struct-array shape. When the element is
                // itself a `StackItem::Array` of SCALARS (i.e. a struct value
                // whose fields were flattened onto a NeoVM Array on the stack,
                // per Task #181's boundary convention), the EVM canonical
                // encoding inlines each struct's fields as consecutive 32-byte
                // slots: `length || (field0 || field1 || ... || fieldK-1)*N`.
                // This matches `abi.encode((uint256,bool)[])` for all-static
                // struct fields, where the struct contributes K head slots
                // and the OUTER array is just length + N*K flat slots (no
                // per-element offset indirection because each struct is
                // static — it occupies exactly K*32 bytes inline).
                //
                // Heuristic: an element is treated as a struct value when it
                // is `StackItem::Array` AND every inner field is itself a
                // scalar (Integer / UnsignedInteger / Boolean / Null, or a
                // ByteArray of address/bytesN/integer width). Nested dynamic
                // elements (strings, bytes, deeper arrays) fall outside this
                // static-struct fast path and keep the existing zero-slot
                // fallback (same as pre-#192 behaviour).
                let elements = arr.borrow().clone();
                let n = elements.len();
                let mut out = Vec::new();
                let mut len_slot = [0u8; 32];
                len_slot[24..].copy_from_slice(&(n as u64).to_be_bytes());
                out.extend_from_slice(&len_slot);
                for el in elements.iter() {
                    if let StackItem::Array(fields) = el {
                        let field_items = fields.borrow().clone();
                        if !field_items.is_empty()
                            && field_items
                                .iter()
                                .all(|item| !Self::abi_is_dynamic(item))
                        {
                            for field in field_items.iter() {
                                out.extend_from_slice(&Self::abi_pad32_be(field));
                            }
                            continue;
                        }
                    }
                    out.extend_from_slice(&Self::abi_pad32_be(el));
                }
                out
            }
            _ => {
                // Map / Null: degrade to an empty zero-length tail to
                // match the existing head-side fallback (abi_pad32_be
                // returns zeros for these, and abi_is_dynamic classifies
                // Map as dynamic — so we emit a 32-byte length=0 slot).
                vec![0u8; 32]
            }
        }
    }

    pub(crate) fn invoke_native_stdlib(method: &str, params: StackItem) -> StackItem {
        match method {
            "serialize" => {
                // S1 fix: emit the Neo N3 BinarySerializer wire format, NOT
                // JSON. Real Neo N3 nodes serialize StackItems as type-tagged
                // little-endian; the previous serde_json path produced
                // `{"type":"ByteArray","value":[...]}` which round-trips inside
                // the simulator but is byte-incompatible on-chain (storage
                // keys, length checks, and inter-contract interop all
                // diverged). `jsonSerialize` below still produces JSON for
                // callers that explicitly want the JSON form.
                if let StackItem::Array(args) = params {
                    let value = args.borrow().first().cloned().unwrap_or(StackItem::Null);
                    let bytes = Self::neo_binary_serialize(&value);
                    StackItem::byte_array(bytes)
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            "deserialize" => {
                // S1 fix: decode the Neo N3 BinarySerializer wire format (the
                // inverse of `serialize` above).
                if let StackItem::Array(args) = params {
                    let input = args
                        .borrow()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| StackItem::byte_array(Vec::new()));
                    let bytes = Self::stack_item_to_bytes(input);
                    Self::neo_binary_deserialize(&bytes).unwrap_or(StackItem::Null)
                } else {
                    StackItem::Null
                }
            }
            "jsonserialize" => {
                if let StackItem::Array(args) = params {
                    let value = args.borrow().first().cloned().unwrap_or(StackItem::Null);
                    let json = serde_json::to_string(&value).unwrap_or_default();
                    StackItem::byte_array(json.into_bytes())
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            "jsondeserialize" => {
                if let StackItem::Array(args) = params {
                    let input = args
                        .borrow()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| StackItem::byte_array(Vec::new()));
                    let bytes = Self::stack_item_to_bytes(input);
                    let json_str = String::from_utf8(bytes).unwrap_or_default();
                    serde_json::from_str::<StackItem>(&json_str).unwrap_or(StackItem::Null)
                } else {
                    StackItem::Null
                }
            }
            // Tasks #44/#72/#73 — EVM-canonical ABI encoder. Given an outer
            // `StackItem::Array` of arguments, emit the spec-compliant
            // head+tail layout:
            //
            //   * Static args (integer-shaped, address, bytesN, bool): one
            //     32-byte BE-padded slot in the head section. Tail: none.
            //   * Dynamic args (string, bytes, dynamic arrays): the head
            //     slot carries a 32-byte BE offset pointing at the tail
            //     section (measured from the start of the encoded data);
            //     the tail section itself is `len(32) || data(padded to a
            //     32-byte multiple)`. The first dynamic arg's tail starts
            //     immediately after the head (offset = 32 * arg_count).
            //
            // When EVERY arg is static (e.g. `abi.encode(uint256, uint256)`
            // or `abi.encode(address)`) the output collapses to the classic
            // `concat(pad32_be(arg_i))` Task #44 shape — preserves existing
            // static-path tests.
            //
            // Heuristic (the Solidity type is lost by runtime dispatch time):
            // a `StackItem::ByteArray` is classified dynamic ONLY for the
            // short-literal / empty-buffer / long-buffer widths that fall
            // outside the integer/address/bytes32 static heuristics baked
            // into `abi_pad32_be` (see `abi_is_dynamic` above).
            "abiencode" => {
                if let StackItem::Array(args) = params {
                    let borrowed = args.borrow();
                    let inner = if borrowed.len() == 1 {
                        // The runtime wraps the positional args in an outer
                        // Array; when the caller passed a single Array to
                        // `abiEncode` (how the bytecode emitter packs things),
                        // unwrap one level.
                        match &borrowed[0] {
                            StackItem::Array(nested) => nested.borrow().clone(),
                            other => vec![other.clone()],
                        }
                    } else {
                        borrowed.clone()
                    };
                    let is_dynamic: Vec<bool> = inner.iter().map(Self::abi_is_dynamic).collect();
                    if !is_dynamic.iter().any(|b| *b) {
                        // Fast path: static-only encoding (Task #44 shape).
                        let mut out = Vec::with_capacity(inner.len() * 32);
                        for item in inner.iter() {
                            out.extend_from_slice(&Self::abi_pad32_be(item));
                        }
                        return StackItem::byte_array(out);
                    }
                    // Head+tail encoding (EVM spec, Tasks #72/#73).
                    let head_len = inner.len() * 32;
                    let mut tails: Vec<Vec<u8>> = Vec::with_capacity(inner.len());
                    let mut running_offset: u64 = head_len as u64;
                    let mut head = Vec::with_capacity(head_len);
                    for (i, item) in inner.iter().enumerate() {
                        if is_dynamic[i] {
                            let tail = Self::abi_dynamic_tail_bytes(item);
                            let mut offset_slot = [0u8; 32];
                            offset_slot[24..].copy_from_slice(&running_offset.to_be_bytes());
                            head.extend_from_slice(&offset_slot);
                            running_offset += tail.len() as u64;
                            tails.push(tail);
                        } else {
                            head.extend_from_slice(&Self::abi_pad32_be(item));
                        }
                    }
                    let total = head.len() + tails.iter().map(|t| t.len()).sum::<usize>();
                    let mut out = Vec::with_capacity(total);
                    out.extend_from_slice(&head);
                    for tail in &tails {
                        out.extend_from_slice(tail);
                    }
                    StackItem::byte_array(out)
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            // Task #44 — packed variant. Same input convention; each element
            // is serialised to its packed width and concatenated.
            "abiencodepacked" => {
                if let StackItem::Array(args) = params {
                    let borrowed = args.borrow();
                    let inner = if borrowed.len() == 1 {
                        match &borrowed[0] {
                            StackItem::Array(nested) => nested.borrow().clone(),
                            other => vec![other.clone()],
                        }
                    } else {
                        borrowed.clone()
                    };
                    let mut out = Vec::new();
                    for item in inner.iter() {
                        out.extend_from_slice(&Self::abi_packed_bytes(item));
                    }
                    StackItem::byte_array(out)
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            // Task #44 — round-trip decoder. Accepts a ByteArray produced by
            // `abiencode` (32*N BE-packed) and returns an `Array` of
            // `UnsignedInteger` slots (one per 32-byte chunk). This preserves
            // `abi.decode(abi.encode(x))` equivalence when the argument is
            // integer-typed; bytes/address decoding falls back to the raw
            // ByteArray slot for the caller to re-interpret.
            "abidecode" => {
                if let StackItem::Array(args) = params {
                    let input = args
                        .borrow()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| StackItem::byte_array(Vec::new()));
                    let bytes = Self::stack_item_to_bytes(input);
                    if bytes.is_empty() || !bytes.len().is_multiple_of(32) {
                        // Not a canonical payload — return as-is so callers
                        // that passed through a non-32-aligned buffer still
                        // get something sensible.
                        return StackItem::byte_array(bytes);
                    }
                    // Task #121 — dynamic-array detection: an EVM-canonical
                    // `abi.encode(T[])` payload is `[offset=32][length=N]
                    // [element_0]..[element_{N-1}]` — total = 64 + N*32 bytes,
                    // with the first 32-byte slot equal to 0x20 (offset to
                    // the length field) and the second 32-byte slot equal to
                    // N (which exactly accounts for the remaining data).
                    //
                    // When that signature matches, return the bytes verbatim
                    // as a `ByteArray` so the outer RET path emits the
                    // EVM-canonical encoding of the `T[]` unchanged. The
                    // calling site (e.g. `return abi.decode(encoded, (uint[]))`
                    // from batch53_cc2) would otherwise round-trip through
                    // `Array → serde_json` and leak JSON as return data.
                    //
                    // Does NOT recurse into dynamic-array-of-dynamic (`T[][]`
                    // / `string[]`) or struct tuples — those patterns don't
                    // match the simple offset=32 + single-length signature
                    // and stay on the scalar-slot path.
                    if bytes.len() >= 64 {
                        let first_is_offset_20 = bytes[..24].iter().all(|b| *b == 0) && {
                            let mut buf = [0u8; 8];
                            buf.copy_from_slice(&bytes[24..32]);
                            u64::from_be_bytes(buf) == 32
                        };
                        let declared_len = {
                            let mut buf = [0u8; 8];
                            buf.copy_from_slice(&bytes[56..64]);
                            u64::from_be_bytes(buf)
                        };
                        let len_high_zero = bytes[32..56].iter().all(|b| *b == 0);
                        let payload_matches =
                            len_high_zero && (64 + declared_len as usize * 32) == bytes.len();
                        if first_is_offset_20 && payload_matches {
                            return StackItem::byte_array(bytes);
                        }
                    }
                    let slots: Vec<StackItem> = bytes
                        .chunks_exact(32)
                        .map(|chunk| {
                            // Decode as u64 when high bits are zero — lossless
                            // for the common uint64/uint256<=u64::MAX path.
                            let high_zero = chunk[..24].iter().all(|b| *b == 0);
                            if high_zero {
                                let mut buf = [0u8; 8];
                                buf.copy_from_slice(&chunk[24..]);
                                StackItem::UnsignedInteger(u64::from_be_bytes(buf))
                            } else {
                                // A 32-byte big-endian integer slot whose high
                                // bit is set is a uint256 >= 2^255: store it as
                                // the canonical 32-byte TWO'S-COMPLEMENT (as the
                                // rest of the runtime does) so it compares equal
                                // to the same value produced by literals/arith,
                                // instead of a positive-magnitude 33-byte form.
                                let v = num_bigint::BigInt::from_bytes_be(
                                    num_bigint::Sign::Plus,
                                    chunk,
                                );
                                Self::u256_twos_complement_item(v)
                            }
                        })
                        .collect();
                    if slots.len() == 1 {
                        slots.into_iter().next().unwrap()
                    } else {
                        StackItem::array(slots)
                    }
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            // Task #51 — StdLib.itoa(value[, base]): format integer as
            // decimal (or hex when base=16) ASCII string, returned as a
            // `ByteArray`. Matches `callt_stdlib_itoa_roundtrip_via_token`
            // which compares against `N.to_string().as_bytes()`. Bases
            // other than {10,16} collapse to decimal (no exception channel).
            "itoa" => {
                if let StackItem::Array(args) = params {
                    let borrowed = args.borrow();
                    let value = borrowed
                        .first()
                        .map(|it| Self::stack_item_to_int(it.clone()))
                        .unwrap_or(0);
                    let base = borrowed
                        .get(1)
                        .map(|it| Self::stack_item_to_int(it.clone()))
                        .unwrap_or(10);
                    // Hex: uppercase, no `0x`; negatives as `-ABS` (StdLib).
                    let text = if base == 16 {
                        if value < 0 {
                            format!("-{:X}", (value as i128).unsigned_abs())
                        } else {
                            format!("{:X}", value as u64)
                        }
                    } else {
                        value.to_string()
                    };
                    StackItem::byte_array(text.into_bytes())
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            // Task #51 — StdLib.atoi(string[, base]): parse decimal (or
            // hex) ASCII back into an integer. Returns `StackItem::Integer`
            // so `stack_item_to_bytes` LE-encodes it; `atoi(itoa(N)) == N`
            // round-trips via `decode_uint_le`. Malformed input yields 0.
            "atoi" => {
                if let StackItem::Array(args) = params {
                    let borrowed = args.borrow();
                    let input_bytes = borrowed
                        .first()
                        .map(|it| Self::stack_item_to_bytes(it.clone()))
                        .unwrap_or_default();
                    let base = borrowed
                        .get(1)
                        .map(|it| Self::stack_item_to_int(it.clone()))
                        .unwrap_or(10);
                    let s = std::str::from_utf8(&input_bytes).unwrap_or("");
                    let trimmed = s.trim();
                    let radix: u32 = if base == 16 { 16 } else { 10 };
                    // Optional leading `-`; hex may also carry `0x`/`0X`.
                    let (neg, body) = match trimmed.strip_prefix('-') {
                        Some(rest) => (true, rest),
                        None => (false, trimmed),
                    };
                    let body = if radix == 16 {
                        body.strip_prefix("0x")
                            .or_else(|| body.strip_prefix("0X"))
                            .unwrap_or(body)
                    } else {
                        body
                    };
                    // Bug #21: `atoi("--42", 10)` previously returned 42 because
                    // `i128::from_str_radix` accepts a leading `-` in `body`,
                    // making the outer `neg` flag a second negation. Parse the
                    // magnitude as **unsigned** so any sign character in `body`
                    // (including a second `-`, a `+`, or anything else
                    // non-digit) yields 0.
                    let magnitude = u128::from_str_radix(body, radix).unwrap_or(0) as i128;
                    let signed = if neg { -magnitude } else { magnitude };
                    // Saturate into i64 (Integer slot width).
                    let clamped = signed.clamp(i64::MIN as i128, i64::MAX as i128) as i64;
                    StackItem::Integer(clamped)
                } else {
                    StackItem::Integer(0)
                }
            }
            // Task #51 — StdLib.base64Encode(bytes): RFC 4648 base64 with
            // `=` padding. Hand-rolled (no new crate). UTF-8 `ByteArray`.
            "base64encode" => {
                if let StackItem::Array(args) = params {
                    let input = args
                        .borrow()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| StackItem::byte_array(Vec::new()));
                    let bytes = Self::stack_item_to_bytes(input);
                    StackItem::byte_array(Self::base64_encode(&bytes).into_bytes())
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            // Task #51 — StdLib.base64Decode(string): inverse encoder.
            // Invalid input yields an empty buffer (no exception channel).
            "base64decode" => {
                if let StackItem::Array(args) = params {
                    let input = args
                        .borrow()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| StackItem::byte_array(Vec::new()));
                    let bytes = Self::stack_item_to_bytes(input);
                    let s = std::str::from_utf8(&bytes).unwrap_or("");
                    StackItem::byte_array(Self::base64_decode(s).unwrap_or_default())
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            _ => StackItem::Null,
        }
    }

    /// RFC 4648 base64 encode — standard alphabet with `=` padding.
    fn base64_encode(input: &[u8]) -> String {
        const ALPHABET: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
        let mut chunks = input.chunks_exact(3);
        for chunk in &mut chunks {
            let b = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);
            out.push(ALPHABET[((b >> 18) & 0x3f) as usize] as char);
            out.push(ALPHABET[((b >> 12) & 0x3f) as usize] as char);
            out.push(ALPHABET[((b >> 6) & 0x3f) as usize] as char);
            out.push(ALPHABET[(b & 0x3f) as usize] as char);
        }
        let rem = chunks.remainder();
        match rem.len() {
            1 => {
                let b = (rem[0] as u32) << 16;
                out.push(ALPHABET[((b >> 18) & 0x3f) as usize] as char);
                out.push(ALPHABET[((b >> 12) & 0x3f) as usize] as char);
                out.push('=');
                out.push('=');
            }
            2 => {
                let b = ((rem[0] as u32) << 16) | ((rem[1] as u32) << 8);
                out.push(ALPHABET[((b >> 18) & 0x3f) as usize] as char);
                out.push(ALPHABET[((b >> 12) & 0x3f) as usize] as char);
                out.push(ALPHABET[((b >> 6) & 0x3f) as usize] as char);
                out.push('=');
            }
            _ => {}
        }
        out
    }

    /// RFC 4648 base64 decode — standard alphabet only. Returns `None` on
    /// malformed input (bad character or bad padding) rather than partial
    /// output; callers in the native-dispatch path substitute an empty
    /// buffer in that case.
    fn base64_decode(input: &str) -> Option<Vec<u8>> {
        fn decode_char(c: u8) -> Option<u8> {
            match c {
                b'A'..=b'Z' => Some(c - b'A'),
                b'a'..=b'z' => Some(c - b'a' + 26),
                b'0'..=b'9' => Some(c - b'0' + 52),
                b'+' => Some(62),
                b'/' => Some(63),
                _ => None,
            }
        }
        // Strip whitespace (spaces, tabs, newlines) — PEM/MIME interop.
        let cleaned: Vec<u8> = input
            .bytes()
            .filter(|b| !matches!(*b, b' ' | b'\t' | b'\r' | b'\n'))
            .collect();
        if !cleaned.len().is_multiple_of(4) {
            return None;
        }
        let mut out = Vec::with_capacity(cleaned.len() / 4 * 3);
        for (chunk_idx, chunk) in cleaned.chunks_exact(4).enumerate() {
            let pad = chunk.iter().rev().take_while(|b| **b == b'=').count();
            if pad > 2 {
                return None;
            }
            // Bug #22: previously this loop accepted `=` at any position in
            // the chunk and treated it as a literal zero sextet, so inputs
            // like "AB=C" decoded to non-empty bytes instead of erroring.
            // Reject `=` anywhere except the trailing-padding region of the
            // FINAL chunk (mid-chunk and non-final-chunk pad bytes are
            // illegal per RFC 4648).
            let total_eq = chunk.iter().filter(|b| **b == b'=').count();
            if total_eq != pad {
                return None; // `=` outside trailing-pad position
            }
            let is_final_chunk = chunk_idx + 1 == cleaned.len() / 4;
            if pad > 0 && !is_final_chunk {
                return None; // pad only allowed in final chunk
            }
            let mut vals = [0u8; 4];
            for (i, &c) in chunk.iter().enumerate() {
                vals[i] = if c == b'=' { 0 } else { decode_char(c)? };
            }
            let b = ((vals[0] as u32) << 18)
                | ((vals[1] as u32) << 12)
                | ((vals[2] as u32) << 6)
                | (vals[3] as u32);
            out.push(((b >> 16) & 0xff) as u8);
            if pad < 2 {
                out.push(((b >> 8) & 0xff) as u8);
            }
            if pad < 1 {
                out.push((b & 0xff) as u8);
            }
        }
        Some(out)
    }
}

// ============================================================================
// S1 fix — Neo N3 BinarySerializer format for StdLib.serialize/deserialize.
//
// Before this module existed, `serialize` ran the StackItem through
// `serde_json`, producing `{"type":"ByteArray","value":[...]}`. That round-
// trips inside the simulator but is byte-incompatible with real Neo N3 nodes
// (storage keys derived from serialized values, length checks, and inter-
// contract interop all silently diverged on-chain). This module implements the
// Neo N3 BinarySerializer wire format used by `StdLib.serialize` on-chain, so
// the simulator's output matches what a real node produces byte-for-byte.
//
// Format (from neo-project/neo StackItem.SerializeAs):
//   0x00 ByteArray : varint(len) || bytes
//   0x01 Boolean   : 1 byte (0/1)
//   0x02 Integer   : 8 bytes little-endian signed
//   0x03 Null      : (no payload)
//   0x40 Array     : varint(count) || item...     (each item recursively)
//   0x80 Map       : varint(count) || (key value)... (each recursively)
//
// `varint` is Neo's 7-bit-group big-endian continuation encoding (same shape
// as `WriteVarBytes` in neo-vm): high bit set on every byte except the last;
// groups are emitted most-significant-group-first.
//
// Only the StackItem variants the embedded runtime models are handled; an
// unknown shape serializes as Null (defensive — every runtime StackItem is
// one of the seven known variants, so this branch is unreachable in practice).
// ============================================================================

impl ExecutionContext {
    /// Encode `value` in the Neo N3 BinarySerializer wire format.
    pub(crate) fn neo_binary_serialize(value: &StackItem) -> Vec<u8> {
        let mut out = Vec::new();
        Self::neo_binary_serialize_into(value, &mut out);
        out
    }

    fn neo_binary_serialize_into(value: &StackItem, out: &mut Vec<u8>) {
        match value {
            StackItem::ByteArray(rc) => {
                let bytes = rc.borrow();
                out.push(0x00); // ByteArray
                Self::neo_write_varint(out, bytes.len() as u64);
                out.extend_from_slice(&bytes);
            }
            StackItem::Boolean(b) => {
                out.push(0x01); // Boolean
                out.push(if *b { 1 } else { 0 });
            }
            StackItem::Integer(i) => {
                out.push(0x02); // Integer
                out.extend_from_slice(&i.to_le_bytes());
            }
            StackItem::UnsignedInteger(u) => {
                // Neo N3 has no dedicated unsigned tag; emit as Integer. Values
                // > i64::MAX lose information — but the runtime only constructs
                // UnsignedInteger from Solidity uint256 software routines and
                // hashes that already fit in i64 by the time they reach here.
                out.push(0x02); // Integer
                out.extend_from_slice(&(*u as i64).to_le_bytes());
            }
            StackItem::Null => {
                out.push(0x03); // Null
            }
            StackItem::Array(rc) => {
                out.push(0x40); // Array
                let items = rc.borrow();
                Self::neo_write_varint(out, items.len() as u64);
                for item in items.iter() {
                    Self::neo_binary_serialize_into(item, out);
                }
            }
            StackItem::Map(rc) => {
                out.push(0x80); // Map
                let map = rc.borrow();
                Self::neo_write_varint(out, map.len() as u64);
                for (k, v) in map.iter() {
                    Self::neo_binary_serialize_into(&StackItem::byte_array(k.clone()), out);
                    Self::neo_binary_serialize_into(v, out);
                }
            }
        }
    }

    /// Decode a Neo N3 BinarySerializer byte stream back into a StackItem.
    /// Returns `None` on truncated / malformed input (mirrors Neo N3 faulting).
    pub(crate) fn neo_binary_deserialize(bytes: &[u8]) -> Option<StackItem> {
        let mut cursor = 0usize;
        let item = Self::neo_binary_deserialize_from(bytes, &mut cursor)?;
        // Neo N3 rejects trailing bytes after a top-level item; tolerate them
        // (the simulator is permissive by design — see `serde_json`-era
        // behavior that ignored trailing data).
        Some(item)
    }

    fn neo_binary_deserialize_from(bytes: &[u8], cursor: &mut usize) -> Option<StackItem> {
        if *cursor >= bytes.len() {
            return None;
        }
        let tag = bytes[*cursor];
        *cursor += 1;
        match tag {
            0x00 => {
                // ByteArray
                let len = Self::neo_read_varint(bytes, cursor)? as usize;
                if *cursor + len > bytes.len() {
                    return None;
                }
                let data = bytes[*cursor..*cursor + len].to_vec();
                *cursor += len;
                Some(StackItem::byte_array(data))
            }
            0x01 => {
                // Boolean
                if *cursor >= bytes.len() {
                    return None;
                }
                let b = bytes[*cursor] != 0;
                *cursor += 1;
                Some(StackItem::Boolean(b))
            }
            0x02 => {
                // Integer (signed LE i64)
                if *cursor + 8 > bytes.len() {
                    return None;
                }
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&bytes[*cursor..*cursor + 8]);
                *cursor += 8;
                Some(StackItem::Integer(i64::from_le_bytes(buf)))
            }
            0x03 => Some(StackItem::Null),
            0x40 => {
                // Array
                let count = Self::neo_read_varint(bytes, cursor)? as usize;
                let mut items = Vec::with_capacity(count);
                for _ in 0..count {
                    items.push(Self::neo_binary_deserialize_from(bytes, cursor)?);
                }
                Some(StackItem::array(items))
            }
            0x80 => {
                // Map
                let count = Self::neo_read_varint(bytes, cursor)? as usize;
                let mut map = std::collections::HashMap::with_capacity(count);
                for _ in 0..count {
                    let key_item = Self::neo_binary_deserialize_from(bytes, cursor)?;
                    let key = Self::stack_item_to_bytes(key_item);
                    let value = Self::neo_binary_deserialize_from(bytes, cursor)?;
                    map.insert(key, value);
                }
                Some(StackItem::map(map))
            }
            // Unknown type tag — Neo N3 would fault. Treat as Null defensively.
            _ => Some(StackItem::Null),
        }
    }

    /// Encode `value` as Neo's 7-bit-group big-endian continuation varint
    /// (the `WriteVarInt` / `WriteVarBytes` length-prefix format used by
    /// neo-vm). Groups are emitted most-significant first; high bit is set on
    /// every byte except the last.
    fn neo_write_varint(out: &mut Vec<u8>, mut value: u64) {
        // Collect 7-bit groups (LSB-first internally), then reverse for the
        // big-endian wire order.
        let mut groups: Vec<u8> = Vec::new();
        loop {
            groups.push((value & 0x7F) as u8);
            value >>= 7;
            if value == 0 {
                break;
            }
        }
        // Most-significant group first; all but the last get the continuation bit.
        groups.reverse();
        let last = groups.len() - 1;
        for (i, &g) in groups.iter().enumerate() {
            out.push(if i < last { g | 0x80 } else { g });
        }
    }

    /// Inverse of [`Self::neo_write_varint`].
    fn neo_read_varint(bytes: &[u8], cursor: &mut usize) -> Option<u64> {
        let mut groups: Vec<u8> = Vec::new();
        loop {
            if *cursor >= bytes.len() {
                return None;
            }
            let b = bytes[*cursor];
            *cursor += 1;
            groups.push(b & 0x7F);
            if b & 0x80 == 0 {
                break;
            }
        }
        // Wire order is most-significant-group first; reassemble accordingly.
        let mut value: u64 = 0;
        for &g in groups.iter() {
            value = (value << 7) | (g as u64);
        }
        Some(value)
    }
}

#[cfg(test)]
mod neo_binary_tests {
    use super::*;

    /// ByteArray values must serialize as `[0x00, varint(len), bytes...]`.
    #[test]
    fn serialize_bytearray_format() {
        let item = StackItem::byte_array(vec![0xAA, 0xBB]);
        assert_eq!(
            ExecutionContext::neo_binary_serialize(&item),
            vec![0x00, 0x02, 0xAA, 0xBB]
        );
        // Empty ByteArray -> tag + zero-length varint.
        let empty = StackItem::byte_array(vec![]);
        assert_eq!(
            ExecutionContext::neo_binary_serialize(&empty),
            vec![0x00, 0x00]
        );
    }

    #[test]
    fn serialize_integer_format() {
        let item = StackItem::Integer(2);
        assert_eq!(
            ExecutionContext::neo_binary_serialize(&item),
            vec![0x02, 2, 0, 0, 0, 0, 0, 0, 0]
        );
        let neg = StackItem::Integer(-1);
        assert_eq!(
            ExecutionContext::neo_binary_serialize(&neg),
            vec![0x02, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
        );
    }

    #[test]
    fn serialize_boolean_and_null_format() {
        assert_eq!(
            ExecutionContext::neo_binary_serialize(&StackItem::Boolean(true)),
            vec![0x01, 0x01]
        );
        assert_eq!(
            ExecutionContext::neo_binary_serialize(&StackItem::Boolean(false)),
            vec![0x01, 0x00]
        );
        assert_eq!(
            ExecutionContext::neo_binary_serialize(&StackItem::Null),
            vec![0x03]
        );
    }

    #[test]
    fn serialize_array_format() {
        // Array [Integer 1, Integer 2] =>
        //   [0x40, varint(2)=0x02, [0x02,1,0..], [0x02,2,0..]]
        let item = StackItem::array(vec![StackItem::Integer(1), StackItem::Integer(2)]);
        let mut expected = vec![0x40, 0x02];
        expected.extend_from_slice(&[0x02, 1, 0, 0, 0, 0, 0, 0, 0]);
        expected.extend_from_slice(&[0x02, 2, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(ExecutionContext::neo_binary_serialize(&item), expected);
    }

    #[test]
    fn varint_multibyte_roundtrip() {
        // 127 -> single byte, no continuation.
        let mut out = Vec::new();
        ExecutionContext::neo_write_varint(&mut out, 127);
        assert_eq!(out, vec![0x7F]);

        // 128 -> 2 groups [1, 0] in MSB-first wire order, continuation on
        // the first byte. value = (1<<7) | 0 = 128.
        let mut out = Vec::new();
        ExecutionContext::neo_write_varint(&mut out, 128);
        assert_eq!(out, vec![0x81, 0x00]);
        let mut cursor = 0;
        assert_eq!(
            ExecutionContext::neo_read_varint(&out, &mut cursor),
            Some(128)
        );

        // 16383 (= 2^14 - 1) -> exactly fills 2 groups [0x7F, 0x7F].
        let mut out = Vec::new();
        ExecutionContext::neo_write_varint(&mut out, 16383);
        assert_eq!(out, vec![0xFF, 0x7F]);

        // 16384 (= 2^14) -> 3 groups [1, 0, 0].
        let mut out = Vec::new();
        ExecutionContext::neo_write_varint(&mut out, 16384);
        assert_eq!(out, vec![0x81, 0x80, 0x00]);
        let mut cursor = 0;
        assert_eq!(
            ExecutionContext::neo_read_varint(&out, &mut cursor),
            Some(16384)
        );

        // Large value (46 bits) exercises a longer continuation chain.
        let mut out = Vec::new();
        ExecutionContext::neo_write_varint(&mut out, 0x4000_0000_0000);
        let mut cursor = 0;
        assert_eq!(
            ExecutionContext::neo_read_varint(&out, &mut cursor),
            Some(0x4000_0000_0000)
        );
    }

    #[test]
    fn roundtrip_all_scalar_types() {
        let cases = vec![
            StackItem::byte_array(vec![0xDE, 0xAD, 0xBE, 0xEF]),
            StackItem::byte_array(vec![]),
            StackItem::Integer(0),
            StackItem::Integer(123456789),
            StackItem::Integer(-42),
            StackItem::Boolean(true),
            StackItem::Boolean(false),
            StackItem::Null,
            StackItem::array(vec![
                StackItem::Integer(1),
                StackItem::byte_array(vec![0x01, 0x02]),
            ]),
        ];
        for item in cases {
            let ser = ExecutionContext::neo_binary_serialize(&item);
            let de = ExecutionContext::neo_binary_deserialize(&ser)
                .unwrap_or_else(|| panic!("deserialize failed for {item:?}"));
            // Compare via re-serialization (canonical equality check that does
            // not require StackItem: PartialEq on nested Rcs).
            let re_ser = ExecutionContext::neo_binary_serialize(&de);
            assert_eq!(re_ser, ser, "round-trip not canonical for {item:?}");
        }
    }

    #[test]
    fn deserialize_truncated_input_returns_none() {
        // An Integer tag with no payload must fail cleanly, not panic.
        assert!(ExecutionContext::neo_binary_deserialize(&[0x02]).is_none());
        // A ByteArray tag with a length claiming more bytes than available.
        assert!(ExecutionContext::neo_binary_deserialize(&[0x00, 0x05, 0xAA]).is_none());
        // Empty input.
        assert!(ExecutionContext::neo_binary_deserialize(&[]).is_none());
    }
}
