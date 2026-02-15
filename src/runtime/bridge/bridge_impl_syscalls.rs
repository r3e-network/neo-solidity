impl VMBridge {
    fn syscall_keccak256(
        bridge: &mut VMBridge,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
        if args.len() != 1 {
            return Err(VMBridgeError::InvalidArguments {
                expected: 1,
                got: args.len(),
            });
        }

        let input = extract_bytes(&args[0])?;
        let hash = bridge.keccak256(&input);

        Ok(vec![StackItem::byte_array(hash.to_vec())])
    }

    fn syscall_sha256(
        _bridge: &mut VMBridge,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
        if args.len() != 1 {
            return Err(VMBridgeError::InvalidArguments {
                expected: 1,
                got: args.len(),
            });
        }

        use sha2::{Digest, Sha256};

        let input = extract_bytes(&args[0])?;
        let hash = Sha256::digest(&input);

        Ok(vec![StackItem::byte_array(hash.to_vec())])
    }

    fn syscall_ecrecover(
        bridge: &mut VMBridge,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
        // Complete ECDSA signature recovery implementation
        if args.len() != 4 {
            return Err(VMBridgeError::InvalidArguments {
                expected: 4,
                got: args.len(),
            });
        }

        let hash = extract_bytes(&args[0])?;
        let v = extract_integer(&args[1])? as u8;
        let r = extract_bytes(&args[2])?;
        let s = extract_bytes(&args[3])?;

        if hash.len() < 32 || r.len() != 32 || s.len() != 32 {
            return Err(VMBridgeError::SystemCallFailed {
                name: "ecrecover".to_string(),
                message: "Invalid hash or signature length".to_string(),
            });
        }

        use secp256k1::ecdsa::{RecoverableSignature, RecoveryId};
        use secp256k1::{Message, Secp256k1};

        let secp = Secp256k1::new();
        let message =
            Message::from_slice(&hash[..32]).map_err(|e| VMBridgeError::SystemCallFailed {
                name: "ecrecover".to_string(),
                message: format!("Invalid hash: {e}"),
            })?;

        let rec_id = match v {
            27 | 28 => RecoveryId::from_i32((v - 27) as i32).map_err(|e| {
                VMBridgeError::SystemCallFailed {
                    name: "ecrecover".to_string(),
                    message: format!("Invalid recovery id: {e}"),
                }
            })?,
            _ => {
                return Err(VMBridgeError::SystemCallFailed {
                    name: "ecrecover".to_string(),
                    message: "Recovery id must be 27 or 28".to_string(),
                })
            }
        };

        let mut sig_bytes = [0u8; 64];
        sig_bytes[..32].copy_from_slice(&r[..32]);
        sig_bytes[32..].copy_from_slice(&s[..32]);

        let signature = RecoverableSignature::from_compact(&sig_bytes, rec_id).map_err(|e| {
            VMBridgeError::SystemCallFailed {
                name: "ecrecover".to_string(),
                message: format!("Invalid signature: {e}"),
            }
        })?;

        let public_key = secp.recover_ecdsa(&message, &signature).map_err(|e| {
            VMBridgeError::SystemCallFailed {
                name: "ecrecover".to_string(),
                message: format!("Recovery failed: {e}"),
            }
        })?;

        let pub_bytes = public_key.serialize_uncompressed();
        let hash = bridge.keccak256(&pub_bytes[1..]); // remove prefix byte
        let address = hash[12..].to_vec(); // last 20 bytes

        Ok(vec![StackItem::byte_array(address)])
    }

    fn syscall_verify(
        _bridge: &mut VMBridge,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
        if args.len() != 3 {
            return Err(VMBridgeError::InvalidArguments {
                expected: 3,
                got: args.len(),
            });
        }

        let hash = extract_bytes(&args[0])?;
        let signature = extract_bytes(&args[1])?;
        let public_key = extract_bytes(&args[2])?;

        if hash.len() < 32 || signature.len() != 64 {
            return Err(VMBridgeError::SystemCallFailed {
                name: "verify".to_string(),
                message: "Invalid hash or signature length".to_string(),
            });
        }

        use secp256k1::ecdsa::Signature;
        use secp256k1::{Message, PublicKey, Secp256k1};

        let secp = Secp256k1::new();

        // Create message from hash
        let message =
            Message::from_slice(&hash[..32]).map_err(|e| VMBridgeError::SystemCallFailed {
                name: "verify".to_string(),
                message: format!("Invalid hash: {e}"),
            })?;

        // Parse signature
        let sig = Signature::from_compact(&signature[..64]).map_err(|e| {
            VMBridgeError::SystemCallFailed {
                name: "verify".to_string(),
                message: format!("Invalid signature: {e}"),
            }
        })?;

        // Parse public key
        let pubkey =
            PublicKey::from_slice(&public_key).map_err(|e| VMBridgeError::SystemCallFailed {
                name: "verify".to_string(),
                message: format!("Invalid public key: {e}"),
            })?;

        // Verify signature
        let verification_result = secp.verify_ecdsa(&message, &sig, &pubkey).is_ok();

        Ok(vec![StackItem::Boolean(verification_result)])
    }

    fn syscall_blake2f(
        _bridge: &mut VMBridge,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
        if args.len() != 1 {
            return Err(VMBridgeError::InvalidArguments {
                expected: 1,
                got: args.len(),
            });
        }

        use blake2::{Blake2b512, Digest};

        let input = extract_bytes(&args[0])?;
        let hash = Blake2b512::digest(&input);

        Ok(vec![StackItem::byte_array(hash[..].to_vec())])
    }

    fn syscall_modexp(
        _bridge: &mut VMBridge,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
        if args.len() != 3 {
            return Err(VMBridgeError::InvalidArguments {
                expected: 3,
                got: args.len(),
            });
        }

        use num_bigint::BigUint;

        let base_bytes = extract_bytes(&args[0])?;
        let exp_bytes = extract_bytes(&args[1])?;
        let mod_bytes = extract_bytes(&args[2])?;

        let base = BigUint::from_bytes_be(&base_bytes);
        let exp = BigUint::from_bytes_be(&exp_bytes);
        let modulus = BigUint::from_bytes_be(&mod_bytes);

        if modulus == BigUint::ZERO {
            return Ok(vec![StackItem::byte_array(vec![0u8; 32])]);
        }

        let result = base.modpow(&exp, &modulus);
        let result_bytes = result.to_bytes_be();

        // Pad to 32 bytes (left-padded with zeros)
        let mut padded = vec![0u8; 32];
        let start = 32usize.saturating_sub(result_bytes.len());
        let copy_len = result_bytes.len().min(32);
        padded[start..start + copy_len].copy_from_slice(&result_bytes[..copy_len]);

        Ok(vec![StackItem::byte_array(padded)])
    }

    fn keccak256(&self, data: &[u8]) -> [u8; 32] {
        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(data);
        let mut out = [0u8; 32];
        out.copy_from_slice(&hash);
        out
    }
}
