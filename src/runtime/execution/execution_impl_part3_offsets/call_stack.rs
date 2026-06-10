use super::*;

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
            saved_locals: std::mem::take(&mut self.locals),
            saved_args: std::mem::take(&mut self.args),
            msg_sender_override: None,
            syscall_result_expected: false,
        };

        self.call_stack.push(frame);
        self.instruction_pointer = address;
        Ok(())
    }

    /// Return from function
    pub fn return_from_function(&mut self) -> Result<(), RuntimeError> {
        if let Some(frame) = self.call_stack.pop() {
            self.instruction_pointer = frame.return_address;
            // Save return value (top of stack) before restoring stack
            let mut return_value = if self.stack.len() > frame.stack_base {
                Some(self.stack.last().cloned().unwrap())
            } else {
                None
            };
            // Task #160 — the self-offsets dispatch branch of
            // `handle_contract_call` (Task #83 sibling-merge routing through the
            // 20-byte zero placeholder) sets `syscall_result_expected` on the
            // frame it pushes. The compiler's external-call lowering (see
            // `src/ir/statements/dispatch/try_catch.rs` and
            // `src/ir/statements/dispatch/expressions.rs`) always assumes the
            // syscall leaves exactly one result on the caller's evaluation
            // stack — matching `invoke_native_contract`'s contract, which
            // pushes `Null` for empty returns. When the callee here is `void`
            // (its `ReturnVoid` emits `0x40` without pushing anything), we
            // synthesise the same `Null` so the caller's `DROP` finds its
            // expected slot rather than underflowing into a synthetic fault
            // that routes a happy-path `try Target(t).voidFn()` into the catch
            // arm.
            if return_value.is_none() && frame.syscall_result_expected {
                return_value = Some(StackItem::Null);
            }
            // Restore stack to base level
            self.stack.truncate(frame.stack_base);
            // Push return value back onto caller's stack
            if let Some(val) = return_value {
                self.stack.push(val);
            }
            // Restore caller's local and argument slots
            self.locals = frame.saved_locals;
            self.args = frame.saved_args;
            Ok(())
        } else {
            Err(RuntimeError::ExecutionError {
                message: "No function to return from".to_string(),
            })
        }
    }
}
