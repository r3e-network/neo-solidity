use super::*;

impl VMBridge {
    pub(crate) fn handle_push0(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("PUSH", Some(1))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        context
            .push_stack(StackItem::UnsignedInteger(0))
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    pub(crate) fn handle_push1(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("PUSH", Some(1))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        context
            .push_stack(StackItem::UnsignedInteger(1))
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }
}
