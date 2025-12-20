impl ExecutionContext {
    fn invoke_native_contract_management(&mut self, method: &str, params: StackItem) -> StackItem {
        match method {
            "getcontract" => {
                if let StackItem::Array(args) = params {
                    if let Some(StackItem::ByteArray(hash_bytes)) = args.borrow().first() {
                        if let Some(state) =
                            self.lookup_contract(hash_bytes.borrow().as_slice())
                        {
                            return self.contract_to_stackitem(&state);
                        }
                    }
                }
                StackItem::Null
            }
            "deploy" => {
                if let StackItem::Array(args) = params {
                    let args = args.borrow();
                    if args.len() >= 2 {
                        let nef = Self::stack_item_to_bytes(args[0].clone());
                        let manifest = Self::stack_item_to_bytes(args[1].clone());
                        let state = self.register_contract(nef, manifest);
                        return self.contract_to_stackitem(&state);
                    }
                }
                StackItem::Null
            }
            "update" => {
                if let StackItem::Array(args) = params {
                    let args = args.borrow();
                    if args.len() >= 2 {
                        let nef = Self::stack_item_to_bytes(args[0].clone());
                        let manifest = Self::stack_item_to_bytes(args[1].clone());
                        if let Some(hash) = self.contract_registry.keys().next().cloned() {
                            if let Some(state) =
                                self.update_contract(&hash, nef.clone(), manifest.clone())
                            {
                                return self.contract_to_stackitem(&state);
                            }
                        }
                        let state = self.register_contract(nef, manifest);
                        return self.contract_to_stackitem(&state);
                    }
                }
                StackItem::Null
            }
            _ => StackItem::Null,
        }
    }
}
