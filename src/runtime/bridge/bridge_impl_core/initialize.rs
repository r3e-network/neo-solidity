use super::*;

impl VMBridge {
    // Private helper methods

    pub(crate) fn initialize_instruction_mapping(&mut self) {
        // Arithmetic instructions (aligned to spec)
        self.instruction_mapping.insert(0x9E, Self::handle_add);
        self.instruction_mapping.insert(0x9F, Self::handle_sub);
        self.instruction_mapping.insert(0xA0, Self::handle_mul);
        self.instruction_mapping.insert(0xA1, Self::handle_div);
        self.instruction_mapping.insert(0xA2, Self::handle_mod);
        self.instruction_mapping.insert(0xA5, Self::handle_modmul);
        self.instruction_mapping.insert(0xA6, Self::handle_modpow);
        self.instruction_mapping.insert(0xA8, Self::handle_shl);
        self.instruction_mapping.insert(0xA9, Self::handle_shr);
        self.instruction_mapping.insert(0x91, Self::handle_and);
        self.instruction_mapping.insert(0x92, Self::handle_or);
        self.instruction_mapping.insert(0x93, Self::handle_xor);

        // Comparison instructions
        self.instruction_mapping.insert(0xB5, Self::handle_lt);
        self.instruction_mapping.insert(0xB7, Self::handle_gt);
        // Support both deep equality and numeric equality opcodes
        self.instruction_mapping.insert(0x97, Self::handle_eq);
        self.instruction_mapping.insert(0x98, Self::handle_ne);
        self.instruction_mapping.insert(0xB3, Self::handle_eq);
        self.instruction_mapping.insert(0xB4, Self::handle_ne);

        // Stack instructions
        self.instruction_mapping.insert(0x10, Self::handle_push0);
        self.instruction_mapping.insert(0x11, Self::handle_push1);
        self.instruction_mapping.insert(0x45, Self::handle_drop);
        self.instruction_mapping.insert(0x4A, Self::handle_dup);
        self.instruction_mapping.insert(0x50, Self::handle_swap);

        // Control flow
        self.instruction_mapping.insert(0x40, Self::handle_ret);
        self.instruction_mapping.insert(0x22, Self::handle_jmp);
        self.instruction_mapping.insert(0x24, Self::handle_jmpif);
        self.instruction_mapping.insert(0x26, Self::handle_jmpifnot);
        self.instruction_mapping.insert(0x25, Self::handle_jmpif); // long variant
        self.instruction_mapping.insert(0x27, Self::handle_jmpifnot); // long variant

        // Memory operations (EVM compatibility)
        self.instruction_mapping.insert(0x51, Self::handle_mload);
        self.instruction_mapping.insert(0x52, Self::handle_mstore);
        self.instruction_mapping.insert(0x53, Self::handle_mstore8);

        // Storage operations (EVM compatibility)
        self.instruction_mapping.insert(0x54, Self::handle_sload);
        self.instruction_mapping.insert(0x55, Self::handle_sstore);
    }

    pub(crate) fn initialize_system_calls(&mut self) {
        self.system_calls
            .insert("keccak256".to_string(), Self::syscall_keccak256);
        self.system_calls
            .insert("sha256".to_string(), Self::syscall_sha256);
        self.system_calls
            .insert("ecrecover".to_string(), Self::syscall_ecrecover);
        self.system_calls
            .insert("verify".to_string(), Self::syscall_verify);
        self.system_calls
            .insert("blake2f".to_string(), Self::syscall_blake2f);
        self.system_calls
            .insert("modexp".to_string(), Self::syscall_modexp);
    }
}
