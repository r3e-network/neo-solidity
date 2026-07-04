use super::*;

impl VMBridge {
    pub(crate) fn handle_lt(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("LT", 3)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::lt_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    pub(crate) fn handle_gt(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("GT", 3)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::gt_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    pub(crate) fn handle_eq(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("EQ", 3)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::eq_stack_items(a, b)?;
        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    pub(crate) fn handle_ne(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("NE", 3)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let b = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let a = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let result = Self::eq_stack_items(a, b)?;
        let inverted = match result {
            StackItem::Boolean(flag) => StackItem::Boolean(!flag),
            other => other,
        };
        context
            .push_stack(inverted)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }
}
