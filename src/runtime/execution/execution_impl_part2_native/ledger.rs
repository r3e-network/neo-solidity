impl ExecutionContext {
    fn invoke_native_ledger(&mut self, method: &str) -> StackItem {
        match method {
            "currentindex" => {
                let height = *self.block_height.get_or_insert(self.default_block_height);
                StackItem::UnsignedInteger(height)
            }
            _ => StackItem::Null,
        }
    }
}
