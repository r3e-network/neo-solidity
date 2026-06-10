use super::*;

impl ExecutionContext {
    pub(crate) fn execute_flow_jumps_compare(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0x28 => {
                // JMPEQ
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i8_offset("JMPEQ")? as i32;
                if self.stack_items_equal(&a, &b)? {
                    let target =
                        self.compute_offset_target("JMPEQ", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 2;
                }
                Ok(true)
            }
            0x29 => {
                // JMPEQ_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i32_offset("JMPEQ_L")?;
                if self.stack_items_equal(&a, &b)? {
                    let target =
                        self.compute_offset_target("JMPEQ_L", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 5;
                }
                Ok(true)
            }
            0x2A => {
                // JMPNE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i8_offset("JMPNE")? as i32;
                if !self.stack_items_equal(&a, &b)? {
                    let target =
                        self.compute_offset_target("JMPNE", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 2;
                }
                Ok(true)
            }
            0x2B => {
                // JMPNE_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i32_offset("JMPNE_L")?;
                if !self.stack_items_equal(&a, &b)? {
                    let target =
                        self.compute_offset_target("JMPNE_L", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 5;
                }
                Ok(true)
            }
            0x2C => {
                // JMPGT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i8_offset("JMPGT")? as i32;
                if self.greater_than(&a, &b)? {
                    let target =
                        self.compute_offset_target("JMPGT", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 2;
                }
                Ok(true)
            }
            0x2D => {
                // JMPGT_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i32_offset("JMPGT_L")?;
                if self.greater_than(&a, &b)? {
                    let target =
                        self.compute_offset_target("JMPGT_L", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 5;
                }
                Ok(true)
            }
            0x2E => {
                // JMPGE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i8_offset("JMPGE")? as i32;
                let ge = self.greater_than(&a, &b)? || self.stack_items_equal(&a, &b)?;
                if ge {
                    let target =
                        self.compute_offset_target("JMPGE", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 2;
                }
                Ok(true)
            }
            0x2F => {
                // JMPGE_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i32_offset("JMPGE_L")?;
                let ge = self.greater_than(&a, &b)? || self.stack_items_equal(&a, &b)?;
                if ge {
                    let target =
                        self.compute_offset_target("JMPGE_L", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 5;
                }
                Ok(true)
            }
            0x30 => {
                // JMPLT
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i8_offset("JMPLT")? as i32;
                if self.less_than(&a, &b)? {
                    let target =
                        self.compute_offset_target("JMPLT", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 2;
                }
                Ok(true)
            }
            0x31 => {
                // JMPLT_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i32_offset("JMPLT_L")?;
                if self.less_than(&a, &b)? {
                    let target =
                        self.compute_offset_target("JMPLT_L", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 5;
                }
                Ok(true)
            }
            0x32 => {
                // JMPLE
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i8_offset("JMPLE")? as i32;
                let le = self.less_than(&a, &b)? || self.stack_items_equal(&a, &b)?;
                if le {
                    let target =
                        self.compute_offset_target("JMPLE", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 2;
                }
                Ok(true)
            }
            0x33 => {
                // JMPLE_L
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;
                let offset = self.read_i32_offset("JMPLE_L")?;
                let le = self.less_than(&a, &b)? || self.stack_items_equal(&a, &b)?;
                if le {
                    let target =
                        self.compute_offset_target("JMPLE_L", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 5;
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
