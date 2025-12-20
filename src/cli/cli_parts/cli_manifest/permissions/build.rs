fn collect_contract_call_permissions(
    ir_module: &ir::Module,
) -> (BTreeMap<String, PermissionMethods>, bool) {
    let mut needs_wildcard = false;
    let mut contract_methods: BTreeMap<String, PermissionMethods> = BTreeMap::new();

    for function in &ir_module.functions {
        if needs_wildcard {
            break;
        }

        for req in analyze_contract_calls(function) {
            let contract = req.contract.unwrap_or_else(|| "*".to_string());
            let methods = if let Some(method) = req.method {
                let mut set = std::collections::BTreeSet::new();
                set.insert(method);
                PermissionMethods::Some(set)
            } else {
                PermissionMethods::All
            };

            contract_methods
                .entry(contract.clone())
                .and_modify(|existing| existing.merge_in(methods.clone()))
                .or_insert(methods);

            if contract == "*" && matches!(contract_methods.get("*"), Some(PermissionMethods::All))
            {
                needs_wildcard = true;
                break;
            }
        }
    }

    (contract_methods, needs_wildcard)
}

/// Infer contract permissions based on method signatures and behavior
fn infer_permissions(
    _metadata: &ContractMetadata,
    ir_module: &ir::Module,
) -> Vec<serde_json::Value> {
    // Neo N3 enforces contract call permissions via the manifest. Native contract
    // calls (StdLib/CryptoLib/etc) are executed through `System.Contract.Call` and
    // therefore also require explicit permissions, otherwise the contract will
    // fault on-chain.
    //
    // We keep manifests minimal by emitting:
    // - Exact permissions for native contracts when the contract only calls native contracts.
    // - Precise permissions for contract calls (`Syscalls.contractCall`) when the
    //   contract hash and/or method name are compile-time constants.
    // - Wildcard permissions only when the contract performs fully dynamic
    //   contract calls (both target + method unknown at compile time), because the
    //   destination can be user-controlled.
    let (contract_methods, needs_wildcard) = collect_contract_call_permissions(ir_module);
    let native_methods = collect_native_permissions(ir_module);

    if needs_wildcard {
        return vec![json!({ "contract": "*", "methods": "*" })];
    }

    let mut merged = contract_methods;

    for (contract, methods) in native_methods {
        let methods = PermissionMethods::Some(methods);
        merged
            .entry(contract)
            .and_modify(|existing| existing.merge_in(methods.clone()))
            .or_insert(methods);
    }

    merged
        .into_iter()
        .map(|(contract, methods)| {
            let methods_json = match methods {
                PermissionMethods::All => json!("*"),
                PermissionMethods::Some(set) => json!(set.into_iter().collect::<Vec<_>>()),
            };
            json!({
                "contract": contract,
                "methods": methods_json,
            })
        })
        .collect()
}
