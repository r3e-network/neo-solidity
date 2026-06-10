use super::*;

impl ExecutionContext {
    pub(crate) fn execute_arithmetic_binary(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0x9E => {
                // ADD
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.add_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0x9F => {
                // SUB
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.sub_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xA0 => {
                // MUL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.mul_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xA1 => {
                // DIV
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.div_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xA2 => {
                // MOD
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.mod_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xA3 => {
                // POW
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.pow_stack_items(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xA5 => {
                // MODMUL
                let modulus = self.pop_stack()?;
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.modmul_stack_items(a, b, modulus)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xA6 => {
                // MODPOW
                let modulus = self.pop_stack()?;
                let exponent = self.pop_stack()?;
                let base = self.pop_stack()?;
                let result = self.modpow_stack_items(base, exponent, modulus)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
