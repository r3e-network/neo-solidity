impl ExecutionContext {
    fn invoke_native_oracle(&self, method: &str) -> StackItem {
        if method == "request" {
            // Return pseudo request id
            let req = Sha256::digest(self.invocation_counter.to_le_bytes());
            StackItem::byte_array(req[..4].to_vec())
        } else {
            StackItem::Null
        }
    }
}
