impl ExecutionContext {
    fn pack_items(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("PACK")?;
        if count > self.stack.len() {
            return Err(RuntimeError::ExecutionError {
                message: "PACK: insufficient stack items".to_string(),
            });
        }
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(self.pop_stack()?);
        }
        self.push_stack(StackItem::array(items))
    }

    fn pack_map(&mut self) -> Result<(), RuntimeError> {
        let count = self.pop_usize("PACKMAP")?;
        if count.saturating_mul(2) > self.stack.len() {
            return Err(RuntimeError::ExecutionError {
                message: "PACKMAP: insufficient stack items".to_string(),
            });
        }
        let mut map = std::collections::HashMap::new();
        for _ in 0..count {
            let value = self.pop_stack()?;
            let key = self.pop_stack()?;
            let key_bytes = Self::stack_item_to_bytes(key);
            map.insert(key_bytes, value);
        }
        self.push_stack(StackItem::map(map))
    }
}
