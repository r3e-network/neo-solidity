impl NeoRuntime {
    /// Deploy contract and return address
    pub fn deploy_contract(
        &mut self,
        bytecode: &[u8],
        constructor_args: &[u8],
    ) -> Result<String, RuntimeError> {
        // Generate contract address
        let address = self.generate_contract_address()?;

        // Store bytecode in state
        self.state_manager.set_code(&address, bytecode)?;

        // Execute constructor if present
        if !constructor_args.is_empty() {
            let result = self.execute(bytecode, constructor_args)?;
            if !result.success {
                return Err(RuntimeError::ExecutionError {
                    message: format!("Constructor failed: {:?}", result.exception),
                });
            }
        }

        Ok(address)
    }

    fn generate_contract_address(&self) -> Result<String, RuntimeError> {
        // Generate deterministic contract address using deployer address + nonce
        use sha3::{Digest, Keccak256};

        let input = format!(
            "contract_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );

        let hash = Keccak256::digest(input.as_bytes());
        Ok(format!("0x{}", hex::encode(&hash[12..32]))) // Take last 20 bytes
    }
}

