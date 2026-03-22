impl ExecutionContext {
    fn execute_push_instruction(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            // Push operations (0x00-0x4F)
            0x05 => {
                // PUSHINT256
                if self.instruction_pointer + 32 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT256: insufficient bytecode".to_string(),
                    });
                }
                let start = self.instruction_pointer as usize + 1;
                let end = start + 32;
                let value = self.bytecode[start..end].to_vec();
                self.push_stack(StackItem::byte_array(value))?;
                self.instruction_pointer += 33;
            }
            0x04 => {
                // PUSHINT128
                if self.instruction_pointer + 16 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT128: insufficient bytecode".to_string(),
                    });
                }
                let start = self.instruction_pointer as usize + 1;
                let end = start + 16;
                let value = self.bytecode[start..end].to_vec();
                self.push_stack(StackItem::byte_array(value))?;
                self.instruction_pointer += 17;
            }
            0x08 => {
                // PUSHT
                self.push_stack(StackItem::Boolean(true))?;
                self.instruction_pointer += 1;
            }
            0x09 => {
                // PUSHF
                self.push_stack(StackItem::Boolean(false))?;
                self.instruction_pointer += 1;
            }
            0x0F => {
                // PUSHM1
                self.push_stack(StackItem::Integer(-1))?;
                self.instruction_pointer += 1;
            }
            0x00 => {
                // PUSHINT8
                if self.instruction_pointer + 1 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT8: insufficient bytecode".to_string(),
                    });
                }
                let value = self.bytecode[self.instruction_pointer as usize + 1] as i8 as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 2;
            }
            0x01 => {
                // PUSHINT16
                if self.instruction_pointer + 2 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT16: insufficient bytecode".to_string(),
                    });
                }
                let bytes = &self.bytecode
                    [self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 3];
                let value = i16::from_le_bytes([bytes[0], bytes[1]]) as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 3;
            }
            0x02 => {
                // PUSHINT32
                if self.instruction_pointer + 4 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT32: insufficient bytecode".to_string(),
                    });
                }
                let bytes = &self.bytecode
                    [self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 5];
                let value = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 5;
            }
            0x03 => {
                // PUSHINT64
                if self.instruction_pointer + 8 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHINT64: insufficient bytecode".to_string(),
                    });
                }
                let bytes = &self.bytecode
                    [self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 9];
                let mut array = [0u8; 8];
                array.copy_from_slice(bytes);
                let value = i64::from_le_bytes(array);
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 9;
            }
            0x0A => {
                // PUSHA (absolute address)
                let target = self.read_u32_offset("PUSHA")?;
                self.push_stack(StackItem::UnsignedInteger(target as u64))?;
                self.instruction_pointer += 5;
            }
            0x0B => {
                // PUSHNULL
                self.push_stack(StackItem::Null)?;
                self.instruction_pointer += 1;
            }
            0x0C => {
                // PUSHDATA1
                if self.instruction_pointer + 1 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA1: insufficient bytecode for length".to_string(),
                    });
                }
                let length = self.bytecode[self.instruction_pointer as usize + 1] as usize;
                if self.instruction_pointer as usize + 2 + length > self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA1: insufficient bytecode for data".to_string(),
                    });
                }
                let data = self.bytecode[self.instruction_pointer as usize + 2
                    ..self.instruction_pointer as usize + 2 + length]
                    .to_vec();
                self.push_stack(StackItem::byte_array(data))?;
                self.instruction_pointer += 2 + length as u32;
            }
            0x0D => {
                // PUSHDATA2
                if self.instruction_pointer + 2 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA2: insufficient bytecode for length".to_string(),
                    });
                }
                let len_bytes = &self.bytecode
                    [self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 3];
                let length = u16::from_le_bytes([len_bytes[0], len_bytes[1]]) as usize;
                if self.instruction_pointer as usize + 3 + length > self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA2: insufficient bytecode for data".to_string(),
                    });
                }
                let data = self.bytecode[self.instruction_pointer as usize + 3
                    ..self.instruction_pointer as usize + 3 + length]
                    .to_vec();
                self.push_stack(StackItem::byte_array(data))?;
                self.instruction_pointer += 3 + length as u32;
            }
            0x0E => {
                // PUSHDATA4
                if self.instruction_pointer + 4 >= self.bytecode.len() as u32 {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA4: insufficient bytecode for length".to_string(),
                    });
                }
                let len_bytes = &self.bytecode
                    [self.instruction_pointer as usize + 1..self.instruction_pointer as usize + 5];
                let length =
                    u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]])
                        as usize;
                if length > self.memory_limit {
                    return Err(RuntimeError::ExecutionError {
                        message: format!(
                            "PUSHDATA4: data length {} exceeds memory limit {}",
                            length, self.memory_limit
                        ),
                    });
                }
                if self.instruction_pointer as usize + 5 + length > self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "PUSHDATA4: insufficient bytecode for data".to_string(),
                    });
                }
                let data = self.bytecode[self.instruction_pointer as usize + 5
                    ..self.instruction_pointer as usize + 5 + length]
                    .to_vec();
                self.push_stack(StackItem::byte_array(data))?;
                self.instruction_pointer += 5 + length as u32;
            }
            0x10 => {
                // PUSH0
                self.push_stack(StackItem::Integer(0))?;
                self.instruction_pointer += 1;
            }
            0x11..=0x20 => {
                // PUSH1-PUSH16
                let value = (opcode - 0x10) as i64;
                self.push_stack(StackItem::Integer(value))?;
                self.instruction_pointer += 1;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
