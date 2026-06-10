use super::*;

impl ExecutionContext {
    pub(crate) fn execute_stack_instruction(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            // Stack operations (0x39-0x4F)
            0x43 => {
                // DEPTH
                self.push_stack(StackItem::Integer(self.stack.len() as i64))?;
                self.instruction_pointer += 1;
            }
            0x45 => {
                // DROP
                self.pop_stack()?;
                self.instruction_pointer += 1;
            }
            0x46 => {
                // NIP
                self.nip()?;
                self.instruction_pointer += 1;
            }
            0x48 => {
                // XDROP
                self.xdrop()?;
                self.instruction_pointer += 1;
            }
            0x49 => {
                // CLEAR
                self.clear_stack();
                self.instruction_pointer += 1;
            }
            0x4A => {
                // DUP
                let top = self.peek_stack()?.clone();
                self.push_stack(top)?;
                self.instruction_pointer += 1;
            }
            0x4B => {
                // OVER
                self.over()?;
                self.instruction_pointer += 1;
            }
            0x4D => {
                // PICK (index on top)
                self.pick_n()?;
                self.instruction_pointer += 1;
            }
            0x4E => {
                // TUCK
                self.tuck()?;
                self.instruction_pointer += 1;
            }
            0x50 => {
                // SWAP
                let top = self.pop_stack()?;
                let second = self.pop_stack()?;
                self.push_stack(top)?;
                self.push_stack(second)?;
                self.instruction_pointer += 1;
            }
            0x51 => {
                // ROT
                if self.stack.len() < 3 {
                    return Err(RuntimeError::ExecutionError {
                        message: "ROT: insufficient stack items".to_string(),
                    });
                }
                let c = self.pop_stack()?;
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                self.push_stack(b)?;
                self.push_stack(c)?;
                self.push_stack(a)?;
                self.instruction_pointer += 1;
            }
            0x52 => {
                // ROLL (index on top)
                self.roll()?;
                self.instruction_pointer += 1;
            }
            0x53 => {
                // REVERSE3
                self.reverse_top_n(3)?;
                self.instruction_pointer += 1;
            }
            0x54 => {
                // REVERSE4
                self.reverse_top_n(4)?;
                self.instruction_pointer += 1;
            }
            0x55 => {
                // REVERSEN (count on top)
                let count = self.pop_usize("REVERSEN")?;
                self.reverse_top_n(count)?;
                self.instruction_pointer += 1;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
