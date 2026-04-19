impl ExecutionContext {
    fn invoke_native_cryptolib(method: &str, params: StackItem) -> StackItem {
        match method {
            "sha256" => {
                if let StackItem::Array(args) = params {
                    let value = args
                        .borrow()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| StackItem::byte_array(Vec::new()));
                    let bytes = Self::stack_item_to_bytes(value);
                    let digest = Sha256::digest(&bytes);
                    StackItem::byte_array(digest[..].to_vec())
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            "ripemd160" => {
                if let StackItem::Array(args) = params {
                    let value = args
                        .borrow()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| StackItem::byte_array(Vec::new()));
                    let bytes = Self::stack_item_to_bytes(value);
                    let digest = Ripemd160::digest(&bytes);
                    StackItem::byte_array(digest[..].to_vec())
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            "keccak256" => {
                if let StackItem::Array(args) = params {
                    let value = args
                        .borrow()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| StackItem::byte_array(Vec::new()));
                    let bytes = Self::stack_item_to_bytes(value);
                    let digest = Keccak256::digest(&bytes);
                    StackItem::byte_array(digest[..].to_vec())
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            "murmur32" => {
                if let StackItem::Array(args) = params {
                    let value = args
                        .borrow()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| StackItem::byte_array(Vec::new()));
                    let seed = args
                        .borrow()
                        .get(1)
                        .cloned()
                        .unwrap_or(StackItem::UnsignedInteger(0));
                    let bytes = Self::stack_item_to_bytes(value);
                    let seed_u32 = match seed {
                        StackItem::UnsignedInteger(u) => u as u32,
                        StackItem::Integer(i) => i as u32,
                        StackItem::ByteArray(b) => {
                            let mut buf = [0u8; 4];
                            for (i, byte) in b.borrow().iter().take(4).enumerate() {
                                buf[i] = *byte;
                            }
                            u32::from_le_bytes(buf)
                        }
                        _ => 0,
                    };
                    let hash = Self::murmur3_32(&bytes, seed_u32);
                    StackItem::byte_array(hash.to_le_bytes().to_vec())
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            "verifywithecdsa" => StackItem::Boolean(false),
            "recoversecp256k1" => {
                // Neo N3 CryptoLib.RecoverSecp256K1(hash, signature) -> 65-byte
                // uncompressed pubkey (0x04 || x32 || y32) or Null on failure.
                // The Solidity `ecrecover` lowering (bytecode_builtins/builtin_call/
                // crypto.rs) packs `signature = r(32) || s(32) || v(1)` with v
                // normalized to 27..=30, matching Ethereum's recovery-id+27 form.
                let StackItem::Array(args) = params else {
                    return StackItem::Null;
                };
                let borrowed = args.borrow();
                let hash =
                    Self::stack_item_to_bytes(borrowed.first().cloned().unwrap_or(StackItem::Null));
                let signature =
                    Self::stack_item_to_bytes(borrowed.get(1).cloned().unwrap_or(StackItem::Null));
                if hash.len() < 32 || signature.len() != 65 {
                    return StackItem::Null;
                }

                use secp256k1::ecdsa::{RecoverableSignature, RecoveryId};
                use secp256k1::{Message, Secp256k1};

                let msg = match Message::from_slice(&hash[..32]) {
                    Ok(m) => m,
                    Err(_) => return StackItem::Null,
                };
                // v byte sits at signature[64]; accept 0/1 (raw) or 27..=30.
                let v = signature[64];
                let rec_i32 = match v {
                    0 | 1 => v as i32,
                    27..=30 => (v - 27) as i32,
                    _ => return StackItem::Null,
                };
                let rec_id = match RecoveryId::from_i32(rec_i32) {
                    Ok(id) => id,
                    Err(_) => return StackItem::Null,
                };
                let sig = match RecoverableSignature::from_compact(&signature[..64], rec_id) {
                    Ok(s) => s,
                    Err(_) => return StackItem::Null,
                };
                let secp = Secp256k1::new();
                match secp.recover_ecdsa(&msg, &sig) {
                    Ok(pk) => StackItem::byte_array(pk.serialize_uncompressed().to_vec()),
                    Err(_) => StackItem::Null,
                }
            }
            _ => StackItem::Null,
        }
    }
}
