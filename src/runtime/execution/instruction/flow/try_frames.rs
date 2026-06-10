use super::*;

impl ExecutionContext {
    pub(crate) fn dispatch_exception(&mut self, message: String) -> Result<(), RuntimeError> {
        self.uncaught_exception = Some(message.clone());

        loop {
            let Some(frame) = self.try_stack.last().cloned() else {
                return Err(RuntimeError::ExecutionError { message });
            };

            // Exceptions thrown inside FINALLY blocks, or inside CATCH blocks without a FINALLY,
            // are not handled by that frame and must propagate to outer frames.
            if frame.state == TryFrameState::Finally
                || (frame.state == TryFrameState::Catch && frame.finally_target.is_none())
            {
                self.try_stack.pop();
                continue;
            }

            // TRY state + catch handler => enter catch and clear the exception.
            if frame.state == TryFrameState::Try {
                if let Some(catch_target) = frame.catch_target {
                    // If the throw happened inside a callee (i.e. there are
                    // call_stack frames pushed after this TRY frame's owner),
                    // unwind them. Otherwise the catch body's RET would pop
                    // the callee's frame and resume the TRY body's post-call
                    // continuation — causing the catch's return value to leak
                    // into the success path's expression (batch31 H2b).
                    while self.call_stack.len() > frame.owner_call_depth {
                        if let Some(callee_frame) = self.call_stack.pop() {
                            // Restore the owner's locals and args. Don't push
                            // a return value — the stack unwound to the catch
                            // handler pushes the exception message itself.
                            self.stack.truncate(callee_frame.stack_base);
                            self.locals = callee_frame.saved_locals;
                            self.args = callee_frame.saved_args;
                        }
                    }
                    if let Some(top) = self.try_stack.last_mut() {
                        top.state = TryFrameState::Catch;
                    }
                    // Task #86 — the catch handler receives the raw revert
                    // payload so `catch (bytes memory data)` sees the
                    // EVM-canonical envelope (`selector || abi.encode(args)`)
                    // produced by THROW. Falling back to the UTF-8 rendering
                    // of the exception message (the pre-Task-#86 behavior)
                    // would leak the `"THROW: …"` prefix and the lossy U+FFFD
                    // replacements for non-UTF-8 payload bytes into the
                    // caller, breaking ABI decoding.
                    let payload = if !self.revert_payload.is_empty() {
                        self.revert_payload.clone()
                    } else {
                        message.as_bytes().to_vec()
                    };
                    self.push_stack(StackItem::byte_array(payload))?;
                    self.uncaught_exception = None;
                    self.revert_payload.clear();
                    self.instruction_pointer = catch_target;
                    return Ok(());
                }
            }

            // Otherwise, route into FINALLY (must exist).
            if let Some(finally_target) = frame.finally_target {
                if let Some(top) = self.try_stack.last_mut() {
                    top.state = TryFrameState::Finally;
                }
                self.instruction_pointer = finally_target;
                return Ok(());
            }

            // Defensive: malformed try frame (no catch or finally).
            self.try_stack.pop();
        }
    }

    pub(crate) fn execute_flow_try_frames(&mut self, opcode: u8) -> Result<bool, RuntimeError> {
        match opcode {
            0x3B => {
                // TRY CatchOffset(sbyte) FinallyOffset(sbyte)
                let catch_offset = self.read_i8_offset("TRY")? as i32;
                let finally_offset = {
                    let idx = self.instruction_pointer as usize + 2;
                    if idx >= self.bytecode.len() {
                        return Err(RuntimeError::ExecutionError {
                            message: "TRY: insufficient bytecode for offset".to_string(),
                        });
                    }
                    (self.bytecode[idx] as i8) as i32
                };

                if catch_offset == 0 && finally_offset == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "TRY: catchOffset and finallyOffset cannot both be 0".to_string(),
                    });
                }

                let catch_target = if catch_offset == 0 {
                    None
                } else {
                    Some(self.compute_offset_target(
                        "TRY",
                        self.instruction_pointer,
                        catch_offset,
                    )?)
                };

                let finally_target = if finally_offset == 0 {
                    None
                } else {
                    Some(self.compute_offset_target(
                        "TRY",
                        self.instruction_pointer,
                        finally_offset,
                    )?)
                };

                self.try_stack.push(TryFrame {
                    catch_target,
                    finally_target,
                    end_target: None,
                    state: TryFrameState::Try,
                    owner_call_depth: self.call_stack.len(),
                });

                self.instruction_pointer += 3;
            }
            0x3C => {
                // TRY_L CatchOffset(int) FinallyOffset(int)
                let catch_offset = self.read_i32_offset("TRY_L")?;
                let finally_offset = {
                    let start = self.instruction_pointer as usize + 5;
                    let end = start + 4;
                    if end > self.bytecode.len() {
                        return Err(RuntimeError::ExecutionError {
                            message: "TRY_L: insufficient bytecode for offset".to_string(),
                        });
                    }
                    let mut buf = [0u8; 4];
                    buf.copy_from_slice(&self.bytecode[start..end]);
                    i32::from_le_bytes(buf)
                };

                if catch_offset == 0 && finally_offset == 0 {
                    return Err(RuntimeError::ExecutionError {
                        message: "TRY_L: catchOffset and finallyOffset cannot both be 0"
                            .to_string(),
                    });
                }

                let catch_target = if catch_offset == 0 {
                    None
                } else {
                    Some(self.compute_offset_target(
                        "TRY_L",
                        self.instruction_pointer,
                        catch_offset,
                    )?)
                };

                let finally_target = if finally_offset == 0 {
                    None
                } else {
                    Some(self.compute_offset_target(
                        "TRY_L",
                        self.instruction_pointer,
                        finally_offset,
                    )?)
                };

                self.try_stack.push(TryFrame {
                    catch_target,
                    finally_target,
                    end_target: None,
                    state: TryFrameState::Try,
                    owner_call_depth: self.call_stack.len(),
                });

                self.instruction_pointer += 9;
            }
            0x3D => {
                // ENDTRY: endOffset(sbyte)
                if self.try_stack.is_empty() {
                    return Err(RuntimeError::ExecutionError {
                        message: "ENDTRY: corresponding TRY block not found".to_string(),
                    });
                }

                if self
                    .try_stack
                    .last()
                    .is_some_and(|frame| frame.state == TryFrameState::Finally)
                {
                    return Err(RuntimeError::ExecutionError {
                        message: "ENDTRY: cannot execute inside FINALLY".to_string(),
                    });
                }

                let end_offset = self.read_i8_offset("ENDTRY")? as i32;
                let end_target =
                    self.compute_offset_target("ENDTRY", self.instruction_pointer, end_offset)?;

                let finally_target = self.try_stack.last().and_then(|frame| frame.finally_target);

                if let Some(finally_target) = finally_target {
                    if let Some(frame) = self.try_stack.last_mut() {
                        frame.state = TryFrameState::Finally;
                        frame.end_target = Some(end_target);
                    }
                    self.instruction_pointer = finally_target;
                } else {
                    self.try_stack.pop();
                    self.instruction_pointer = end_target;
                }
            }
            0x3E => {
                // ENDTRY_L: endOffset(int)
                if self.try_stack.is_empty() {
                    return Err(RuntimeError::ExecutionError {
                        message: "ENDTRY_L: corresponding TRY block not found".to_string(),
                    });
                }

                if self
                    .try_stack
                    .last()
                    .is_some_and(|frame| frame.state == TryFrameState::Finally)
                {
                    return Err(RuntimeError::ExecutionError {
                        message: "ENDTRY_L: cannot execute inside FINALLY".to_string(),
                    });
                }

                let end_offset = self.read_i32_offset("ENDTRY_L")?;
                let end_target =
                    self.compute_offset_target("ENDTRY_L", self.instruction_pointer, end_offset)?;

                let finally_target = self.try_stack.last().and_then(|frame| frame.finally_target);

                if let Some(finally_target) = finally_target {
                    if let Some(frame) = self.try_stack.last_mut() {
                        frame.state = TryFrameState::Finally;
                        frame.end_target = Some(end_target);
                    }
                    self.instruction_pointer = finally_target;
                } else {
                    self.try_stack.pop();
                    self.instruction_pointer = end_target;
                }
            }
            0x3F => {
                // ENDFINALLY
                let Some(frame) = self.try_stack.pop() else {
                    return Err(RuntimeError::ExecutionError {
                        message: "ENDFINALLY: corresponding TRY block not found".to_string(),
                    });
                };

                if self.uncaught_exception.is_none() {
                    let Some(end_target) = frame.end_target else {
                        return Err(RuntimeError::ExecutionError {
                            message: "ENDFINALLY: ENDTRY target not set".to_string(),
                        });
                    };

                    self.instruction_pointer = end_target;
                } else {
                    let message = self
                        .uncaught_exception
                        .clone()
                        .unwrap_or_else(|| "Unhandled exception".to_string());
                    self.dispatch_exception(message)?;
                }
            }
            _ => return Ok(false),
        }

        Ok(true)
    }
}
