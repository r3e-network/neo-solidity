use super::*;

impl ExecutionContext {
    pub(crate) fn murmur3_32(data: &[u8], seed: u32) -> u32 {
        const C1: u32 = 0xcc9e2d51;
        const C2: u32 = 0x1b873593;
        let mut h1: u32 = seed;
        let len = data.len();

        // Process 4-byte chunks
        let chunks = data.chunks_exact(4);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let k1 = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let k1 = k1.wrapping_mul(C1);
            let k1 = k1.rotate_left(15);
            let k1 = k1.wrapping_mul(C2);

            h1 ^= k1;
            h1 = h1.rotate_left(13);
            h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
        }

        // Process remaining bytes
        if !remainder.is_empty() {
            let mut k1: u32 = 0;
            for (i, &byte) in remainder.iter().enumerate() {
                k1 |= (byte as u32) << (i * 8);
            }
            let k1 = k1.wrapping_mul(C1);
            let k1 = k1.rotate_left(15);
            let k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }

        // Finalization
        h1 ^= len as u32;
        h1 ^= h1 >> 16;
        h1 = h1.wrapping_mul(0x85ebca6b);
        h1 ^= h1 >> 13;
        h1 = h1.wrapping_mul(0xc2b2ae35);
        h1 ^= h1 >> 16;

        h1
    }

    /// Verify secp256k1 ECDSA signature
    ///
    /// # Arguments
    /// * `message` - The message hash (32 bytes) that was signed
    /// * `pubkey` - The public key (33 or 65 bytes)
    /// * `signature` - The signature (64 bytes compact or DER encoded)
    ///
    /// # Returns
    /// `true` if the signature is valid, `false` otherwise
    pub(crate) fn verify_secp256k1_with_message(
        message: &[u8],
        pubkey: &[u8],
        signature: &[u8],
    ) -> bool {
        // Validate input lengths
        if message.len() != 32 {
            return false;
        }
        if pubkey.is_empty() || (pubkey.len() != 33 && pubkey.len() != 65) {
            return false;
        }
        if signature.len() < 64 || signature.len() > 72 {
            // 64 bytes for compact, up to 72 for DER
            return false;
        }

        // Parse public key
        let pk = match secp256k1::PublicKey::from_slice(pubkey) {
            Ok(pk) => pk,
            Err(_) => return false,
        };

        // Parse signature (try DER first, then compact)
        let sig = match secp256k1::ecdsa::Signature::from_der(signature)
            .or_else(|_| secp256k1::ecdsa::Signature::from_compact(signature))
        {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        // Parse message - use proper error handling instead of unwrap
        let msg = match secp256k1::Message::from_slice(message) {
            Ok(msg) => msg,
            Err(_) => return false,
        };

        // Verify signature
        let secp = secp256k1::Secp256k1::verification_only();
        secp.verify_ecdsa(&msg, &sig, &pk).is_ok()
    }

    /// Get the current message hash for signature verification.
    ///
    /// Returns the host-injected transaction signing hash when one was armed
    /// via [`Self::override_signing_hash`] for this execution (S3 fix),
    /// otherwise falls back to a deterministic synthetic hash derived from the
    /// execution context (bytecode hash + storage account + invocation
    /// counter). The fallback preserves the behavior of every test written
    /// before the injectable-hash API existed.
    pub(crate) fn get_current_message_hash(&self) -> [u8; 32] {
        // S3 fix — prefer the host-injected transaction signing hash. Neo N3
        // verifies signatures against the script container's verifiable
        // transaction digest; the embedded runtime has no real script
        // container, so hosts that need real correctness inject the digest
        // explicitly. `None` here means "no override armed for this
        // execution" → fall through to the synthetic hash.
        if let Some(injected) = self.active_signing_hash {
            return injected;
        }

        // Backward-compatible fallback: a deterministic message hash derived
        // from the execution context. Includes bytecode hash + current account
        // + invocation counter.
        let mut hasher_input = Vec::new();

        // Include bytecode hash
        let bytecode_hash = Sha256::digest(&self.bytecode);
        hasher_input.extend_from_slice(&bytecode_hash);

        // Include storage account if available
        if let Some(ref account) = self.storage_account {
            hasher_input.extend_from_slice(account.as_bytes());
        }

        // Include invocation counter for uniqueness
        hasher_input.extend_from_slice(&self.invocation_counter.to_le_bytes());

        // Final hash
        let result = Sha256::digest(&hasher_input);
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}
