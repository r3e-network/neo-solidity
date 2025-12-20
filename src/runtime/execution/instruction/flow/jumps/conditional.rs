impl ExecutionContext {
    fn execute_flow_jumps_conditional(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0x24 => {
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
            0x25 => {
                // JMPIF_L
                let condition = self.pop_stack()?;
                let offset = self.read_i32_offset("JMPIF_L")?;
                if condition.is_truthy() {
                    let target = self.compute_offset_target(
                        "JMPIF_L",
                        self.instruction_pointer,
                        offset,
                    )?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 5;
                }
                Ok(true)
            }
            0x26 => {
                // JMPIFNOT
                let condition = self.pop_stack()?;
                let offset = self.read_i8_offset("JMPIFNOT")? as i32;
                if !condition.is_truthy() {
                    let target = self.compute_offset_target(
                        "JMPIFNOT",
                        self.instruction_pointer,
                        offset,
                    )?;
                    self.instruction_pointer = target;
                } else {
                    self.instruction_pointer += 2;
                }
                Ok(true)
            }
            0x27 => {
                // JMPIFNOT_L
                let condition = self.pop_stack()?;
                let offset = self.read_i32_offset("JMPIFNOT_L")?;
                if !condition.is_truthy() {
                    let target = self.compute_offset_target(
                        "JMPIFNOT_L",
                        self.instruction_pointer,
                        offset,
                    )?;
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
