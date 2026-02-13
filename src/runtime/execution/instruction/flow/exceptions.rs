impl ExecutionContext {
    fn execute_flow_exceptions(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0x38 => {
                // ABORT
                return Err(RuntimeError::ExecutionError {
                    message: "ABORT instruction executed".to_string(),
                });
            }
            0x39 => {
                // ASSERT
                let condition = self.pop_stack()?;
                if condition.is_truthy() {
                    self.instruction_pointer += 1;
                } else {
                    return Err(RuntimeError::ExecutionError {
                        message: "ASSERT failed".to_string(),
                    });
                }
            }
            0x3A => {
                // THROW
                let message = Self::stack_item_to_bytes(self.pop_stack()?);
                let message = String::from_utf8_lossy(&message);
                let message = if message.is_empty() {
                    "THROW".to_string()
                } else {
                    format!("THROW: {message}")
                };
                self.dispatch_exception(message)?;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
