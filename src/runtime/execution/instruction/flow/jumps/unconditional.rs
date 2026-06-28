use super::*;
use crate::opcode::OpCode;

impl ExecutionContext {
    pub(crate) fn execute_flow_jumps_unconditional(
        &mut self,
        opcode: u8,
    ) -> Result<bool, RuntimeError> {
        const NOP: u8 = OpCode::NOP.byte();
        const JMP: u8 = OpCode::JMP.byte();
        const JMP_L: u8 = OpCode::JMP_L.byte();

        match opcode {
            NOP => {
                // NOP
                self.instruction_pointer += 1;
                Ok(true)
            }
            JMP => {
                // JMP (1-byte signed relative offset from instruction start)
                let offset = self.read_i8_offset("JMP")? as i32;
                let target = self.compute_offset_target("JMP", self.instruction_pointer, offset)?;
                self.instruction_pointer = target;
                Ok(true)
            }
            JMP_L => {
                // JMP_L (4-byte signed relative offset from instruction start)
                let offset = self.read_i32_offset("JMP_L")?;
                let target =
                    self.compute_offset_target("JMP_L", self.instruction_pointer, offset)?;
                self.instruction_pointer = target;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
