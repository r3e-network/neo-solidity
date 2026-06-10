use super::*;

impl VMBridge {
    pub(crate) fn handle_mload(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("MLOAD", Some(3))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let address = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        // Convert address to usize
        let addr = match address {
            StackItem::UnsignedInteger(a) => a as usize,
            StackItem::Integer(a) => a as usize,
            _ => {
                return Err(VMBridgeError::MemoryOperationFailed {
                    message: "Invalid memory address".to_string(),
                })
            }
        };

        // Read 32 bytes from memory
        let data = {
            let slice = context.read_memory(addr, 32).map_err(|e| {
                VMBridgeError::MemoryOperationFailed {
                    message: e.to_string(),
                }
            })?;
            slice.to_vec()
        };
        context
            .push_stack(StackItem::byte_array(data))
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        Ok(())
    }

    pub(crate) fn handle_mstore(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("MSTORE", Some(3))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let value = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let address = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let addr = match address {
            StackItem::UnsignedInteger(a) => a as usize,
            StackItem::Integer(a) => a as usize,
            _ => {
                return Err(VMBridgeError::MemoryOperationFailed {
                    message: "Invalid memory address".to_string(),
                })
            }
        };

        let data = value.to_bytes();
        let mut padded_data = data;
        padded_data.resize(32, 0); // Pad to 32 bytes

        context.write_memory(addr, &padded_data).map_err(|e| {
            VMBridgeError::MemoryOperationFailed {
                message: e.to_string(),
            }
        })?;

        Ok(())
    }

    pub(crate) fn handle_mstore8(
        _bridge: &mut VMBridge,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        _storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<(), VMBridgeError> {
        gas.consume_gas("MSTORE8", Some(3))
            .map_err(|e| VMBridgeError::BridgeError {
                message: e.to_string(),
            })?;

        let value = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;
        let address = context
            .pop_stack()
            .map_err(|e| VMBridgeError::StackOperationFailed {
                message: e.to_string(),
            })?;

        let addr = match address {
            StackItem::UnsignedInteger(a) => a as usize,
            StackItem::Integer(a) => a as usize,
            _ => {
                return Err(VMBridgeError::MemoryOperationFailed {
                    message: "Invalid memory address".to_string(),
                })
            }
        };

        let byte_value = match value {
            StackItem::UnsignedInteger(v) => (v & 0xFF) as u8,
            StackItem::Integer(v) => (v & 0xFF) as u8,
            StackItem::ByteArray(bytes) => bytes.borrow().first().copied().unwrap_or(0),
            _ => 0,
        };

        context.write_memory(addr, &[byte_value]).map_err(|e| {
            VMBridgeError::MemoryOperationFailed {
                message: e.to_string(),
            }
        })?;

        Ok(())
    }
}
