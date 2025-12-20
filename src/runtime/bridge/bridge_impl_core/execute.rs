impl VMBridge {
    /// Execute bytecode through the bridge
    pub fn execute(
        &mut self,
        context: &mut execution::ExecutionContext,
        _state: &mut state::StateManager,
        storage: &mut storage::StorageManager,
        gas: &mut execution::GasTracker,
    ) -> Result<ExecutionResult, RuntimeError> {
        context.bind_storage(&self.contract_account, storage)?;

        let mut state_changes = Vec::new();
        let mut stack_trace = Vec::new();

        loop {
            gas.sync_from_execution(context.gas_used());

            // Check gas limit using execution context accounting
            if gas.out_of_gas() {
                let result = ExecutionResult {
                    success: false,
                    return_data: Vec::new(),
                    gas_used: gas.used(),
                    gas_limit: gas.limit(),
                    exception: Some(RuntimeException {
                        exception_type: ExceptionType::OutOfGas,
                        message: "Execution ran out of gas".to_string(),
                        instruction_pointer: Some(context.instruction_count() as u32),
                        stack_trace: stack_trace.clone(),
                    }),
                    state_changes,
                    logs: context.logs().to_vec(),
                    stack_trace: Some(stack_trace),
                    metadata: ExecutionMetadata::default(),
                };
                context.unbind_storage();
                return Ok(result);
            }

            // Execute single step
            match context.step() {
                Ok(step_result) => {
                    gas.sync_from_execution(context.gas_used());

                    if step_result.halted {
                        let modified_accounts = self.apply_storage_overlay(context, storage)?;
                        for account in modified_accounts {
                            let changes = storage.commit(&account)?;
                            for change in changes {
                                state_changes.push(StateChange {
                                    change_type: super::StateChangeType::StorageChange,
                                    account: account.clone(),
                                    key: Some(change.key),
                                    old_value: change.old_value,
                                    new_value: change.new_value.unwrap_or_default(),
                                });
                            }
                        }

                        gas.sync_from_execution(context.gas_used());

                        let result = ExecutionResult {
                            success: true,
                            return_data: self.extract_return_data(context)?,
                            gas_used: gas.used(),
                            gas_limit: gas.limit(),
                            exception: None,
                            state_changes,
                            logs: context.logs().to_vec(),
                            stack_trace: None,
                            metadata: ExecutionMetadata::default(),
                        };
                        context.unbind_storage();
                        return Ok(result);
                    }

                    // Add to stack trace if debugging enabled
                    if self.config.enable_debugging {
                        stack_trace.push(StackFrame {
                            function_name: None,
                            instruction_pointer: step_result.instruction_pointer,
                            opcode: step_result.opcode,
                            stack_items: step_result.stack_items,
                            local_variables: HashMap::new(),
                        });
                    }
                }
                Err(RuntimeError::OutOfGas { .. }) => {
                    gas.sync_from_execution(context.gas_used());

                    let result = ExecutionResult {
                        success: false,
                        return_data: Vec::new(),
                        gas_used: gas.used(),
                        gas_limit: gas.limit(),
                        exception: Some(RuntimeException {
                            exception_type: ExceptionType::OutOfGas,
                            message: "Execution ran out of gas".to_string(),
                            instruction_pointer: Some(context.instruction_count() as u32),
                            stack_trace: stack_trace.clone(),
                        }),
                        state_changes,
                        logs: context.logs().to_vec(),
                        stack_trace: Some(stack_trace),
                        metadata: ExecutionMetadata::default(),
                    };
                    context.unbind_storage();
                    return Ok(result);
                }
                Err(e) => {
                    gas.sync_from_execution(context.gas_used());

                    let result = ExecutionResult {
                        success: false,
                        return_data: Vec::new(),
                        gas_used: gas.used(),
                        gas_limit: gas.limit(),
                        exception: Some(RuntimeException {
                            exception_type: ExceptionType::Fault,
                            message: e.to_string(),
                            instruction_pointer: Some(context.instruction_count() as u32),
                            stack_trace: stack_trace.clone(),
                        }),
                        state_changes,
                        logs: context.logs().to_vec(),
                        stack_trace: Some(stack_trace),
                        metadata: ExecutionMetadata::default(),
                    };
                    context.unbind_storage();
                    return Ok(result);
                }
            }
        }
    }
}
