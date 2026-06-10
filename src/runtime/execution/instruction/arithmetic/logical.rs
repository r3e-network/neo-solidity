use super::*;

impl ExecutionContext {
    pub(crate) fn execute_arithmetic_logical(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0xAA => {
                // NOT (logical)
                self.logical_not()?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xAB => {
                // BOOLAND
                self.logical_and()?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xAC => {
                // BOOLOR
                self.logical_or()?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xB1 => {
                // NZ
                let value = self.pop_stack()?;
                self.push_stack(StackItem::Boolean(value.is_truthy()))?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
