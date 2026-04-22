impl ExecutionContext {
    fn invoke_native_contract_management(&mut self, method: &str, params: StackItem) -> StackItem {
        match method {
            "getcontract" => {
                if let StackItem::Array(args) = params {
                    if let Some(StackItem::ByteArray(hash_bytes)) = args.borrow().first() {
                        let hash_slice = hash_bytes.borrow();
                        if let Some(state) = self.lookup_contract(hash_slice.as_slice()) {
                            return self.contract_to_stackitem(&state);
                        }
                        // Task #189 — self-registry fallback. The isolated
                        // test-harness runtime does not call `deploy` before
                        // invoking user methods, so `contract_registry` is
                        // empty. When a compiled `address(this).code` lowering
                        // routes through `ContractManagement.getContract(addr)`
                        // with `addr = default_account_bytes` (the executing
                        // script hash derived from Hash160(bytecode) in
                        // `initialize`), returning Null causes the downstream
                        // PICKITEM on .nef to fault with
                        // "PICKITEM: unsupported target Null". Synthesize a
                        // minimal ContractState backed by the actually-executing
                        // bytecode so the PICKITEM(index=3 / .nef) returns the
                        // script bytes — matching the EVM expectation that
                        // `address(this).code` yields the current contract's
                        // runtime bytecode.
                        if hash_slice.as_slice() == self.default_account_bytes.as_slice()
                            && !self.bytecode.is_empty()
                        {
                            let mut hash_le = [0u8; 20];
                            let copy_len = self.default_account_bytes.len().min(20);
                            hash_le[..copy_len]
                                .copy_from_slice(&self.default_account_bytes[..copy_len]);
                            let synthetic = ContractState {
                                id: 0,
                                hash: hash_le,
                                nef: self.bytecode.clone(),
                                manifest: Vec::new(),
                                update_counter: 0,
                            };
                            return self.contract_to_stackitem(&synthetic);
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
