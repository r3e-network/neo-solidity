use super::*;

impl VMBridge {
    /// Handle EVM instruction in NeoVM context
    pub fn handle_instruction(
        &mut self,
        opcode: u8,
        context: &mut execution::ExecutionContext,
        state: &mut state::StateManager,
        storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        if let Some(handler) = self.instruction_mapping.get(&opcode) {
            handler(self, context, state, storage, gas)
        } else {
            Err(VMBridgeError::UnsupportedInstruction { opcode })
        }
    }
}
