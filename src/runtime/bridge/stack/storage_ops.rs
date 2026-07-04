use super::*;

impl VMBridge {
    pub(crate) fn handle_sload(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("SLOAD", 3)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let key = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        // Use default account address for storage operations
        // Note: Override via ExecutionContext.set_storage_account() for custom accounts
        let account = "0x0000000000000000000000000000000000000000";
        let key_bytes = key.to_bytes();

        let value = storage.get(account, &key_bytes).map_err(|e| {
            VMBridgeError::StorageOperationFailed {
                message: e.to_string(),
            }
        })?;

        let result = match value {
            Some(data) => StackItem::byte_array(data),
            None => StackItem::UnsignedInteger(0),
        };

        context
            .push_stack(result)
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    pub(crate) fn handle_sstore(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("SSTORE", 3)
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let value = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let key = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let account = "0x0000000000000000000000000000000000000000";
        let key_bytes = key.to_bytes();
        let value_bytes = value.to_bytes();

        storage
            .set(account, &key_bytes, &value_bytes)
            .map_err(|e| VMBridgeError::StorageOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }
}
