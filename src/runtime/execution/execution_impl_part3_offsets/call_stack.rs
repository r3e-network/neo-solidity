impl ExecutionContext {
    /// Call function
    pub fn call_function(
        &mut self,
        address: u32,
        function_name: Option<String>,
    ) -> Result<(), RuntimeError> {
        if self.call_stack.len() as u32 >= self.call_stack_limit {
            // Call stack limit
            return Err(RuntimeError::ExecutionError {
                message: "Call stack overflow".to_string(),
            });
        }

        let frame = CallFrame {
            return_address: self.instruction_pointer + 1,
            function_name,
            local_variables: HashMap::new(),
            stack_base: self.stack.len(),
        };

        self.call_stack.push(frame);
        self.instruction_pointer = address;
        Ok(())
    }

    /// Return from function
    pub fn return_from_function(&mut self) -> Result<(), RuntimeError> {
        if let Some(frame) = self.call_stack.pop() {
            self.instruction_pointer = frame.return_address;
            // Restore stack to base level
            self.stack.truncate(frame.stack_base);
            Ok(())
        } else {
            Err(RuntimeError::ExecutionError {
                message: "No function to return from".to_string(),
            })
        }
    }
}

