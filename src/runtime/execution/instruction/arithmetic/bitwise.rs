impl ExecutionContext {
    fn execute_arithmetic_bitwise(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0x90 => {
                // INVERT
                let value = self.pop_stack()?;
                let result = self.bitwise_not(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0x91 => {
                // AND
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.bitwise_and(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0x92 => {
                // OR
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.bitwise_or(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0x93 => {
                // XOR
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.bitwise_xor(a, b)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0x97 => {
                // EQUAL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(result))?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0x98 => {
                // NOTEQUAL
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let result = self.stack_items_equal(&a, &b)?;
                self.push_stack(StackItem::Boolean(!result))?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
