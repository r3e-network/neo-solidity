impl ExecutionContext {
    fn execute_arithmetic_shifts(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0xA8 => {
                // SHL
                let shift = self.pop_stack()?;
                let value = self.pop_stack()?;
                let result = self.shift_left(value, shift)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            0xA9 => {
                // SHR
                let shift = self.pop_stack()?;
                let value = self.pop_stack()?;
                let result = self.shift_right(value, shift)?;
                self.push_stack(result)?;
                self.instruction_pointer += 1;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
