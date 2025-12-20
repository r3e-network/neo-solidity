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
            _ => StackItem::Null,
        }
    }
}
