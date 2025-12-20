impl ExecutionContext {
    fn execute_collection_instruction(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0xBE => {
                // PACKMAP
                self.pack_map()?;
                self.instruction_pointer += 1;
            }
            0xBF => {
                // PACKSTRUCT (array-backed)
                self.pack_items()?;
                self.instruction_pointer += 1;
            }
            0xC0 => {
                // PACK
                self.pack_items()?;
                self.instruction_pointer += 1;
            }
            0xC1 => {
                // UNPACK
                self.unpack()?;
                self.instruction_pointer += 1;
            }
            0xC2 => {
                // NEWARRAY0
                self.new_array0()?;
                self.instruction_pointer += 1;
            }
            0xC3 => {
                // NEWARRAY
                self.new_array()?;
                self.instruction_pointer += 1;
            }
            0xC4 => {
                // NEWARRAY_T
                self.new_array()?;
                self.instruction_pointer += 1;
            }
            0xC5 => {
                // NEWSTRUCT0
                self.new_struct0()?;
                self.instruction_pointer += 1;
            }
            0xC6 => {
                // NEWSTRUCT
                self.new_struct()?;
                self.instruction_pointer += 1;
            }
            0xC8 => {
                // NEWMAP
                self.new_map()?;
                self.instruction_pointer += 1;
            }
            0xCA => {
                // SIZE
                self.size_of()?;
                self.instruction_pointer += 1;
            }
            0xCB => {
                // HASKEY
                self.haskey()?;
                self.instruction_pointer += 1;
            }
            0xCC => {
                // KEYS
                self.keys()?;
                self.instruction_pointer += 1;
            }
            0xCD => {
                // VALUES
                self.values()?;
                self.instruction_pointer += 1;
            }
            0xCE => {
                // PICKITEM
                self.pick_item()?;
                self.instruction_pointer += 1;
            }
            0xCF => {
                // APPEND
                self.append_item()?;
                self.instruction_pointer += 1;
            }
            0xD0 => {
                // SETITEM
                self.set_item()?;
                self.instruction_pointer += 1;
            }
            0xD1 => {
                // REVERSEITEMS
                self.reverse_items()?;
                self.instruction_pointer += 1;
            }
            0xD2 => {
                // REMOVE
                self.remove_item()?;
                self.instruction_pointer += 1;
            }
            0xD3 => {
                // CLEARITEMS
                self.clear_items()?;
                self.instruction_pointer += 1;
            }
            0xD4 => {
                // POPITEM
                self.pop_item_from_collection()?;
                self.instruction_pointer += 1;
            }
            0xD8 => {
                // ISNULL
                let item = self.pop_stack()?;
                self.push_stack(StackItem::Boolean(matches!(item, StackItem::Null)))?;
                self.instruction_pointer += 1;
            }
            0xD9 => {
                // ISTYPE (best-effort): NeoVM takes a 1-byte immediate operand (StackItemType).
                let operand_index = self.instruction_pointer as usize + 1;
                if operand_index >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "ISTYPE: insufficient bytecode for operand".to_string(),
                    });
                }
                let expected = self.bytecode[operand_index];
                let item = self.pop_stack()?;
                let matches = match expected {
                    0x00 => true, // Any
                    0x20 => matches!(item, StackItem::Boolean(_)),
                    0x21 | 0x22 => {
                        matches!(item, StackItem::Integer(_) | StackItem::UnsignedInteger(_))
                    }
                    0x28 | 0x30 => matches!(item, StackItem::ByteArray(_)),
                    0x40 | 0x41 => matches!(item, StackItem::Array(_)),
                    0x48 => matches!(item, StackItem::Map(_)),
                    0x60 => matches!(item, StackItem::ByteArray(_)), // interop handles as byte tokens
                    0x80 => self.is_iterator_token(&item),
                    _ => false,
                };
                self.push_stack(StackItem::Boolean(matches))?;
                self.instruction_pointer += 2;
            }
            0xDB => {
                // CONVERT (best-effort coercion): NeoVM takes a 1-byte immediate operand (StackItemType).
                let operand_index = self.instruction_pointer as usize + 1;
                if operand_index >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "CONVERT: insufficient bytecode for operand".to_string(),
                    });
                }
                let target_code = self.bytecode[operand_index];
                let item = self.pop_stack()?;
                let converted = self.convert_item(item, target_code)?;
                self.push_stack(converted)?;
                self.instruction_pointer += 2;
            }
            0xE0 => {
                // ABORTMSG
                let message = Self::stack_item_to_bytes(self.pop_stack()?);
                return Err(RuntimeError::ExecutionError {
                    message: format!("ABORTMSG: {}", String::from_utf8_lossy(&message)),
                });
            }
            0xE1 => {
                // ASSERTMSG
                let message = Self::stack_item_to_bytes(self.pop_stack()?);
                let condition = self.pop_stack()?;
                if condition.is_truthy() {
                    self.instruction_pointer += 1;
                } else {
                    return Err(RuntimeError::ExecutionError {
                        message: format!("ASSERTMSG failed: {}", String::from_utf8_lossy(&message)),
                    });
                }
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
