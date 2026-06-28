use super::*;
use crate::opcode::OpCode;

impl ExecutionContext {
    pub(crate) fn execute_flow_jumps_conditional(
        &mut self,
        opcode: u8,
    ) -> Result<bool, RuntimeError> {
        const JMPIF: u8 = OpCode::JMPIF.byte();
        const JMPIF_L: u8 = OpCode::JMPIF_L.byte();
        const JMPIFNOT: u8 = OpCode::JMPIFNOT.byte();
        const JMPIFNOT_L: u8 = OpCode::JMPIFNOT_L.byte();

        match opcode {
            JMPIF => {
                // JMPIF
                let condition = self.pop_stack()?;
                let offset = self.read_i8_offset("JMPIF")? as i32;
                if condition.is_truthy() {
                    let target =
                        self.compute_offset_target("JMPIF", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 2;
                }
                Ok(true)
            }
            JMPIF_L => {
                // JMPIF_L
                let condition = self.pop_stack()?;
                let offset = self.read_i32_offset("JMPIF_L")?;
                if condition.is_truthy() {
                    let target =
                        self.compute_offset_target("JMPIF_L", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 5;
                }
                Ok(true)
            }
            JMPIFNOT => {
                // JMPIFNOT
                let condition = self.pop_stack()?;
                let offset = self.read_i8_offset("JMPIFNOT")? as i32;
                if !condition.is_truthy() {
                    let target =
                        self.compute_offset_target("JMPIFNOT", self.instruction_pointer, offset)?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 2;
                }
                Ok(true)
            }
            JMPIFNOT_L => {
                // JMPIFNOT_L
                let condition = self.pop_stack()?;
                let offset = self.read_i32_offset("JMPIFNOT_L")?;
                if !condition.is_truthy() {
                    let target =
                        self.compute_offset_target("JMPIFNOT_L", self.instruction_pointer, offset)?;
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
