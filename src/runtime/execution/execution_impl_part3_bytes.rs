impl ExecutionContext {
    fn new_buffer(&mut self) -> Result<(), RuntimeError> {
        let len = self.pop_usize("NEWBUFFER")?;
        self.push_stack(StackItem::byte_array(vec![0u8; len]))
    }

    fn memcpy_bytes(&mut self) -> Result<(), RuntimeError> {
        // NeoVM MEMCPY stack order: [dst, dst_offset, src, src_offset, count]
        // (top-of-stack is `count`).
        let count = self.pop_usize("MEMCPY count")?;
        let src_offset = self.pop_usize("MEMCPY src_offset")?;
        let src = Self::stack_item_to_bytes(self.pop_stack()?);
        let dst_offset = self.pop_usize("MEMCPY dst_offset")?;
        let dst_item = self.pop_stack()?;

        match dst_item {
            StackItem::ByteArray(dst) => {
                let src_end =
                    src_offset
                        .checked_add(count)
                        .ok_or(RuntimeError::ExecutionError {
                            message: "MEMCPY: source range overflow".to_string(),
                        })?;
                let dst_end =
                    dst_offset
                        .checked_add(count)
                        .ok_or(RuntimeError::ExecutionError {
                            message: "MEMCPY: destination range overflow".to_string(),
                        })?;

                {
                    let mut dst_ref = dst.borrow_mut();
                    if src_end > src.len() || dst_end > dst_ref.len() {
                        return Err(RuntimeError::ExecutionError {
                            message: "MEMCPY: range out of bounds".to_string(),
                        });
                    }

                    let src_slice = &src[src_offset..src_end];
                    dst_ref[dst_offset..dst_end].copy_from_slice(src_slice);
                }

                // Implementation note: this runtime deliberately leaves `dst`
                // on the stack after MEMCPY (C-style `memcpy` return).
                // Multiple compiler callers — `coerce_to_fixed_bytes` (and
                // its `bytesN(..)`/`address(..)` clients), the Task #66
                // narrow-integer `abi.encodePacked` CAT chain, event indexed
                // arg lowering, and the binary-op fixed-bytes coercion —
                // rely on this: they follow the MEMCPY with a `Swap; Drop`
                // to discard the leaked buffer and keep the canonical
                // ByteString result. Changing this to not-push breaks all
                // of those callers (stack underflow when the Swap;Drop then
                // consumes the canonical result). Task #112 therefore
                // keeps the legacy push and threads the workaround through
                // `emit_pad_bytesn_to_32` as well.
                self.push_stack(StackItem::ByteArray(dst))
            }
            other => Err(RuntimeError::ExecutionError {
                message: format!("MEMCPY: unsupported destination {other:?}"),
            }),
        }
    }

    fn concat_bytes(&mut self) -> Result<(), RuntimeError> {
        let b = Self::stack_item_to_bytes(self.pop_stack()?);
        let mut a = Self::stack_item_to_bytes(self.pop_stack()?);
        a.extend_from_slice(&b);
        self.push_stack(StackItem::byte_array(a))
    }

    fn substr_bytes(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("SUBSTR")?;
        let index = self.pop_usize("SUBSTR")?;
        let data = Self::stack_item_to_bytes(self.pop_stack()?);
        if index + count > data.len() {
            return Err(RuntimeError::ExecutionError {
                message: "SUBSTR: out of bounds".to_string(),
            });
        }
        self.push_stack(StackItem::byte_array(data[index..index + count].to_vec()))
    }

    fn left_bytes(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("LEFT")?;
        let data = Self::stack_item_to_bytes(self.pop_stack()?);
        if count > data.len() {
            return Err(RuntimeError::ExecutionError {
                message: "LEFT: out of bounds".to_string(),
            });
        }
        self.push_stack(StackItem::byte_array(data[..count].to_vec()))
    }

    fn right_bytes(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("RIGHT")?;
        let data = Self::stack_item_to_bytes(self.pop_stack()?);
        if count > data.len() {
            return Err(RuntimeError::ExecutionError {
                message: "RIGHT: out of bounds".to_string(),
            });
        }
        self.push_stack(StackItem::byte_array(data[data.len() - count..].to_vec()))
    }

}
