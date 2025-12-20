impl ExecutionContext {
    fn execute_flow_jumps_unconditional(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0x21 => {
                // NOP
                self.instruction_pointer += 1;
                Ok(true)
            }
            0x22 => {
                // JMP (1-byte signed relative offset from instruction start)
                let offset = self.read_i8_offset("JMP")? as i32;
                let target = self.compute_offset_target("JMP", self.instruction_pointer, offset)?;
                self.instruction_pointer = target;
                Ok(true)
            }
            0x23 => {
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
