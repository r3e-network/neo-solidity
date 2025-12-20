impl ExecutionContext {
    fn get_opcode_name(&self, opcode: u8) -> String {
        if let Some(name) = spec::opcode_name(opcode) {
            name.to_string()
        } else {
            format!("UNKNOWN_{:02X}", opcode)
        }
    }

    fn get_instruction_gas_cost(&self, opcode: u8) -> u64 {
        spec::opcode_gas(opcode).unwrap_or(1)
    }
}
