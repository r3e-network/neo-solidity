use super::*;

impl ExecutionContext {
    pub(crate) fn execute_arithmetic_numeric_unary(
        &mut self,
        opcode: u8,
    ) -> Result<bool, RuntimeError> {
        match opcode {
            0x99 => {
                // SIGN
                let value = self.pop_stack()?;
                let result = self.sign_stack_item(value)?;
                self.push_stack(StackItem::Integer(result))?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0x9A => {
                // ABS
                let value = self.pop_stack()?;
                let result = self.abs_stack_item(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0x9B => {
                // NEGATE
                let value = self.pop_stack()?;
                let result = self.negate_stack_item(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0x9C => {
                // INC
                let value = self.pop_stack()?;
                let result = self.inc_stack_item(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0x9D => {
                // DEC
                let value = self.pop_stack()?;
                let result = self.dec_stack_item(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xA4 => {
                // SQRT
                let value = self.pop_stack()?;
                let result = self.sqrt_stack_item(value)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
