impl ExecutionContext {
    fn execute_slots_instruction(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0x57 => {
                // INITSLOT locals, args
                if self.instruction_pointer + 2 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "INITSLOT: insufficient bytecode".to_string(),
                    });
                }
                let local_count = self.bytecode[self.instruction_pointer as usize + 1] as usize;
                let arg_count = self.bytecode[self.instruction_pointer as usize + 2] as usize;
                self.locals = vec![StackItem::Null; local_count];
                self.args = vec![StackItem::Null; arg_count];
                self.instruction_pointer += 3;
            }
            0x56 => {
                // INITSSLOT static slots
                if self.instruction_pointer + 1 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "INITSSLOT: insufficient bytecode".to_string(),
                    });
                }
                let static_count = self.bytecode[self.instruction_pointer as usize + 1] as usize;
                self.static_fields = vec![StackItem::Null; static_count];
                self.instruction_pointer += 2;
            }
            0x58..=0x5E => {
                // LDSFLD0-6
                let index = (opcode - 0x58) as usize;
                let value = self.get_static(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 1;
            }
            0x5F => {
                // LDSFLD
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "LDSFLD: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.get_static(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 2;
            }
            0x60..=0x66 => {
                // STSFLD0-6
                let index = (opcode - 0x60) as usize;
                let value = self.pop_stack()?;
                self.set_static(index, value)?;
                self.instruction_pointer += 1;
            }
            0x67 => {
                // STSFLD
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "STSFLD: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.pop_stack()?;
                self.set_static(index, value)?;
                self.instruction_pointer += 2;
            }
            0x68..=0x6E => {
                // LDLOC0-6
                let index = (opcode - 0x68) as usize;
                let value = self.get_local(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 1;
            }
            0x6F => {
                // LDLOC
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "LDLOC: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.get_local(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 2;
            }
            0x70..=0x76 => {
                // STLOC0-6
                let index = (opcode - 0x70) as usize;
                let value = self.pop_stack()?;
                self.set_local(index, value)?;
                self.instruction_pointer += 1;
            }
            0x77 => {
                // STLOC
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "STLOC: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.pop_stack()?;
                self.set_local(index, value)?;
                self.instruction_pointer += 2;
            }
            0x78..=0x7E => {
                // LDARG0-6
                let index = (opcode - 0x78) as usize;
                let value = self.get_arg(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 1;
            }
            0x7F => {
                // LDARG
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "LDARG: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.get_arg(index)?;
                self.push_stack(value)?;
                self.instruction_pointer += 2;
            }
            0x80..=0x86 => {
                // STARG0-6
                let index = (opcode - 0x80) as usize;
                let value = self.pop_stack()?;
                self.set_arg(index, value)?;
                self.instruction_pointer += 1;
            }
            0x87 => {
                // STARG
                let idx_pos = self.instruction_pointer as usize + 1;
                if idx_pos >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "STARG: missing index".to_string(),
                    });
                }
                let index = self.bytecode[idx_pos] as usize;
                let value = self.pop_stack()?;
                self.set_arg(index, value)?;
                self.instruction_pointer += 2;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}

