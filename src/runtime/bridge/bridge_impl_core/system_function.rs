impl VMBridge {
    /// Call system function
    pub fn call_system_function(
        &mut self,
        name: &str,
        args: &[StackItem],
    ) -> Result<Vec<StackItem>, VMBridgeError> {
        if let Some(syscall) = self.system_calls.get(name) {
            syscall(self, args)
        } else {
            Err(VMBridgeError::SystemCallFailed {
                name: name.to_string(),
                message: "System call not found".to_string(),
            })
        }
    }
}
