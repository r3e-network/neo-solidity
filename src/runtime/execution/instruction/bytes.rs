use super::*;

impl ExecutionContext {
    pub(crate) fn execute_bytes_instruction(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0x88 => {
                // NEWBUFFER
                self.new_buffer()?;
                self.instruction_pointer += 1;
            }
            0x89 => {
                // MEMCPY
                self.memcpy_bytes()?;
                self.instruction_pointer += 1;
            }
            0x8B => {
                // CAT
                self.concat_bytes()?;
                self.instruction_pointer += 1;
            }
            0x8C => {
                // SUBSTR
                self.substr_bytes()?;
                self.instruction_pointer += 1;
            }
            0x8D => {
                // LEFT
                self.left_bytes()?;
                self.instruction_pointer += 1;
            }
            0x8E => {
                // RIGHT
                self.right_bytes()?;
                self.instruction_pointer += 1;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
