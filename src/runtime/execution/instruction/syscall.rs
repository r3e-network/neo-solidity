impl ExecutionContext {
    fn execute_syscall_instruction(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        if opcode != 0x41 {
            return Ok(false);
        }

        if self.instruction_pointer + 4 >= self.bytecode.len() as u32 {
            return Err(RuntimeError::ExecutionError {
                message: "SYSCALL: insufficient bytecode".to_string(),
            });
        }

        let mut syscall_id = [0u8; 4];
        syscall_id.copy_from_slice(
            &self.bytecode[self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 5],
        );
        // Consume syscall-specific gas if known
        // NOTE: This is ADDITIONAL gas on top of the base opcode cost (already charged
        // at the start of execute_instruction). The syscall_gas map contains only the
        // extra cost for each syscall, not the total cost.
        if let Some(extra_cost) = self.syscall_gas.get(&syscall_id) {
            // Use checked_add to detect overflow instead of saturating silently
            let projected = match self.gas_used.checked_add(*extra_cost) {
                Some(p) => p,
                None => {
                    return Err(RuntimeError::OutOfGas {
                        used: u64::MAX,
                        limit: self.gas_limit,
                    });
                }
            };
            if projected > self.gas_limit {
                return Err(RuntimeError::OutOfGas {
                    used: projected,
                    limit: self.gas_limit,
                });
            }
            self.gas_used = projected;
        }
        self.handle_syscall(syscall_id)?;
        // Task #70: `handle_contract_call` may route a `this.someFn()` self
        // external call by rewiring `instruction_pointer` directly to the
        // target method's compiled offset. In that case the SYSCALL's usual
        // `+= 5` post-increment would land past the target's INITSLOT
        // prologue, so skip it.
        if self.syscall_suppress_ip_advance {
            self.syscall_suppress_ip_advance = false;
        } else {
            self.instruction_pointer += 5;
        }

        Ok(true)
    }
}

