use super::*;
use crate::opcode::OpCode;

impl ExecutionContext {
    pub(crate) fn execute_flow_jumps_compare(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        const JMPEQ: u8 = OpCode::JMPEQ.byte();
        const JMPEQ_L: u8 = OpCode::JMPEQ_L.byte();
        const JMPNE: u8 = OpCode::JMPNE.byte();
        const JMPNE_L: u8 = OpCode::JMPNE_L.byte();
        const JMPGT: u8 = OpCode::JMPGT.byte();
        const JMPGT_L: u8 = OpCode::JMPGT_L.byte();
        const JMPGE: u8 = OpCode::JMPGE.byte();
        const JMPGE_L: u8 = OpCode::JMPGE_L.byte();
        const JMPLT: u8 = OpCode::JMPLT.byte();
        const JMPLT_L: u8 = OpCode::JMPLT_L.byte();
        const JMPLE: u8 = OpCode::JMPLE.byte();
        const JMPLE_L: u8 = OpCode::JMPLE_L.byte();

        match opcode {
            JMPEQ => {
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
            JMPEQ_L => {
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
            JMPNE => {
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
            JMPNE_L => {
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
            JMPGT => {
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
            JMPGT_L => {
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
            JMPGE => {
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
            JMPGE_L => {
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
            JMPLT => {
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
            JMPLT_L => {
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
            JMPLE => {
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
            JMPLE_L => {
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
