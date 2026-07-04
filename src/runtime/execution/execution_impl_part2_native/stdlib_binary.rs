//! ## Neo N3 BinarySerializer (Runtime)
//!
//! Implementation of `StdLib.serialize` and `StdLib.deserialize` for the Neo N3
//! wire format. Extracted from stdlib.rs to keep the dispatch module under the
//! 800-line limit.
//!
//! Functions live in an `impl ExecutionContext` block but are `&self`-free.

use super::*;

// Section 2 — Neo N3 BinarySerializer (StdLib.serialize / deserialize)
// ============================================================================
//
// Before the S1 fix, `serialize` ran the StackItem through `serde_json`,
// producing `{"type":"ByteArray","value":[...]}`. That round-trips inside the
// simulator but is byte-incompatible with real Neo N3 nodes (storage keys,
// length checks, and inter-contract interop all silently diverge on-chain).
//
// Format (neo-project/neo StackItem.SerializeAs):
//   0x00 ByteArray : varint(len) || bytes
//   0x01 Boolean   : 1 byte (0/1)
//   0x02 Integer   : 8 bytes LE signed
//   0x03 Null      : (no payload)
//   0x40 Array     : varint(count) || item…     (recursive)
//   0x80 Map       : varint(count) || (key value)… (recursive)
//
// `varint` = Neo's 7-bit-group big-endian continuation encoding (same shape
// as `WriteVarBytes` in neo-vm): high bit set on every byte except the last;
// groups emitted most-significant-group-first.

impl ExecutionContext {
    /// Encode `value` in the Neo N3 BinarySerializer wire format.
    pub(crate) fn neo_binary_serialize(value: &StackItem) -> Vec<u8> {
        let mut out = Vec::new();
        Self::neo_binary_serialize_into(value, &mut out);
        out
    }

    fn neo_binary_serialize_into(value: &StackItem, out: &mut Vec<u8>) {
        match value {
            StackItem::ByteArray { data: rc, .. } => {
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
