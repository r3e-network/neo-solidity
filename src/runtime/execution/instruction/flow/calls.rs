impl ExecutionContext {
    fn execute_flow_calls(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0x34 => {
                // CALL (1-byte signed relative offset from instruction start)
                let offset = self.read_i8_offset("CALL")? as i32;
                let base = self.instruction_pointer;
                let target = self.compute_offset_target("CALL", base, offset)?;
                self.push_call_frame(base + 2)?;
                self.instruction_pointer = target;
            }
            0x35 => {
                // CALL_L (4-byte signed relative offset from instruction start)
                let offset = self.read_i32_offset("CALL_L")?;
                let base = self.instruction_pointer;
                let target = self.compute_offset_target("CALL_L", base, offset)?;
                self.push_call_frame(base + 5)?;
                self.instruction_pointer = target;
            }
            0x36 => {
                // CALLA (absolute address from stack)
                let target_usize = self.pop_usize("CALLA")?;
                let target = u32::try_from(target_usize).map_err(|_| RuntimeError::ExecutionError {
                    message: "CALLA: target out of bounds".to_string(),
                })?;
                if target as usize >= self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "CALLA: target out of bounds".to_string(),
                    });
                }
                let return_address = self.instruction_pointer + 1;
                self.push_call_frame(return_address)?;
                self.instruction_pointer = target;
            }
            0x37 => {
                // CALLT (token-based contract call; u16 token index)
                let start = self.instruction_pointer as usize + 1;
                let end = start + 2;
                if end > self.bytecode.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: "CALLT: insufficient bytecode for token index".to_string(),
                    });
                }

                let index = u16::from_le_bytes([self.bytecode[start], self.bytecode[start + 1]]);
                let token = self
                    .method_tokens
                    .get(index as usize)
                    .ok_or_else(|| RuntimeError::ExecutionError {
                        message: format!(
                            "CALLT: token index {} out of bounds ({} token(s) loaded)",
                            index,
                            self.method_tokens.len()
                        ),
                    })?
                    .clone();

                let arg_count = token.parameters_count as usize;
                if arg_count > self.stack.len() {
                    return Err(RuntimeError::ExecutionError {
                        message: format!(
                            "CALLT: insufficient stack items for {arg_count} argument(s)"
                        ),
                    });
                }

                let mut args = Vec::with_capacity(arg_count);
                for _ in 0..arg_count {
                    args.push(self.pop_stack()?);
                }
                args.reverse();

                let params = StackItem::array(args);
                let result = self.invoke_native_contract(&token.hash, &token.method, params);
                if token.has_return_value {
                    self.push_stack(result)?;
                }

                self.instruction_pointer += 3;
            }
            _ => return Ok(false),
        }

        Ok(true)
    }

    fn push_call_frame(&mut self, return_address: u32) -> Result<(), RuntimeError> {
        if self.call_stack.len() as u32 >= self.call_stack_limit {
            return Err(RuntimeError::ExecutionError {
                message: "Call stack overflow".to_string(),
            });
        }

        let frame = CallFrame {
            return_address,
            function_name: None,
            local_variables: HashMap::new(),
            stack_base: self.stack.len(),
            saved_locals: std::mem::take(&mut self.locals),
            saved_args: std::mem::take(&mut self.args),
            msg_sender_override: None,
            syscall_result_expected: false,
        };
        self.call_stack.push(frame);
        Ok(())
    }
}
