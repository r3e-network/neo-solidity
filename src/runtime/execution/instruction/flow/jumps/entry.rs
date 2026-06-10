use super::*;

impl ExecutionContext {
    pub(crate) fn execute_flow_jumps(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        if self.execute_flow_jumps_unconditional(opcode)? {
            return Ok(true);
        }
        if self.execute_flow_jumps_conditional(opcode)? {
            return Ok(true);
        }
        if self.execute_flow_jumps_compare(opcode)? {
            return Ok(true);
        }

        Ok(false)
    }
}
