use super::*;

impl ExecutionContext {
    pub(crate) fn get_local(&self, index: usize) -> Result<StackItem, RuntimeError> {
        self.locals
            .get(index)
            .cloned()
            .ok_or(RuntimeError::ExecutionError {
                message: format!("Local index {index} out of bounds"),
            })
    }

    pub(crate) fn set_local(&mut self, index: usize, value: StackItem) -> Result<(), RuntimeError> {
        if let Some(slot) = self.locals.get_mut(index) {
            *slot = value;
            Ok(())
        } else {
            Err(RuntimeError::ExecutionError {
                message: format!("Local index {index} out of bounds"),
            })
        }
    }

    pub(crate) fn get_arg(&self, index: usize) -> Result<StackItem, RuntimeError> {
        self.args
            .get(index)
            .cloned()
            .ok_or(RuntimeError::ExecutionError {
                message: format!("Argument index {index} out of bounds"),
            })
    }

    pub(crate) fn set_arg(&mut self, index: usize, value: StackItem) -> Result<(), RuntimeError> {
        if let Some(slot) = self.args.get_mut(index) {
            *slot = value;
            Ok(())
        } else {
            Err(RuntimeError::ExecutionError {
                message: format!("Argument index {index} out of bounds"),
            })
        }
    }

    pub(crate) fn get_static(&self, index: usize) -> Result<StackItem, RuntimeError> {
        self.static_fields
            .get(index)
            .cloned()
            .ok_or(RuntimeError::ExecutionError {
                message: format!("Static field index {index} out of bounds"),
            })
    }

    pub(crate) fn set_static(
        &mut self,
        index: usize,
        value: StackItem,
    ) -> Result<(), RuntimeError> {
        if let Some(slot) = self.static_fields.get_mut(index) {
            *slot = value;
            Ok(())
        } else {
            Err(RuntimeError::ExecutionError {
                message: format!("Static field index {index} out of bounds"),
            })
        }
    }
}
