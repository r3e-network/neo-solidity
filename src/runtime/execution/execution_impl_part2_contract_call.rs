impl ExecutionContext {
    fn handle_contract_call(&mut self) -> Result<(), RuntimeError> {
        // Neo N3 syscall convention: the first argument is at the top of the stack.
        // For System.Contract.Call(hash, method, flags, args), the evaluation stack
        // order is: [args, flags, method, hash].
        let contract_item = self.pop_stack()?;
        let method_item = self.pop_stack()?;
        let _flags = self.pop_stack()?; // call flags ignored in emulator
        let params = self.pop_stack()?;

        let method = String::from_utf8(Self::stack_item_to_bytes(method_item)).unwrap_or_default();
        let contract_bytes = Self::stack_item_to_bytes(contract_item);
        let mut hash = [0u8; 20];
        for (i, b) in contract_bytes.iter().take(20).enumerate() {
            hash[i] = *b;
        }

        let result = self.invoke_native_contract(&hash, &method, params);
        self.push_stack(result)?;
        Ok(())
    }

}
