impl VMBridge {
    fn handle_drop(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("DROP", Some(2))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    fn handle_dup(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("DUP", Some(2))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let item = context
            .peek_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?
            .clone();

        context
            .push_stack(item)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    fn handle_swap(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("SWAP", Some(2))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        context
            .push_stack(a)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        context
            .push_stack(b)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }
}

