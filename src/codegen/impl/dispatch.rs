impl CodeGenerator {
    pub fn generate(&mut self, ast: &AstNode) -> Result<CompilationResult, CompilerError> {
        let mut bytecode = Vec::new();
        let mut functions: Vec<FunctionMeta> = Vec::new();
        let mut events = Vec::new();
        let mut estimated_gas = 0;

        // Generate bytecode from AST
        self.generate_node(
            ast,
            &mut bytecode,
            &mut functions,
            &mut events,
            &mut estimated_gas,
        )?;

        // Add contract initialization: INITSLOT with 0 locals and 0 params
        // (prepended, shifts all offsets by INIT_MARKER_LEN = 3)
        let mut prefix = vec![0x56, 0x00, 0x00]; // INITSLOT + 0 locals + 0 params
        prefix.append(&mut bytecode); // move original bytecode after prefix
        bytecode = prefix;

        // Add final return
        bytecode.push(0x40); // RET

        // Deduplicate functions, keeping first occurrence (and its metadata)
        let mut unique_functions: Vec<FunctionMeta> = Vec::new();
        let mut seen_funcs = HashSet::new();
        for func in &functions {
            if seen_funcs.insert(func.name.clone()) {
                unique_functions.push(func.clone());
            }
        }

        // Derive contract name from input filename (strip extension, fallback to "Contract")
        let contract_name = self
            ._config
            .input_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Contract")
            .to_string();

        let abi_functions: Vec<_> = unique_functions
            .iter()
            .map(|func| {
                let inputs: Vec<_> = func
                    .params
                    .iter()
                    .map(|p| serde_json::json!({"name": p, "type": "Any"}))
                    .collect();
                let outputs: Vec<_> = func
                    .returns
                    .iter()
                    .map(|r| serde_json::json!({"name": r, "type": "Any"}))
                    .collect();
                serde_json::json!({
                    "name": func.name,
                    "inputs": inputs,
                    "outputs": outputs,
                    "stateMutability": "nonpayable"
                })
            })
            .collect();

        let abi_events: Vec<_> = events
            .iter()
            .map(|name| {
                serde_json::json!({
                    "name": name,
                    "inputs": []
                })
            })
            .collect();

        let abi = serde_json::json!({
            "functions": abi_functions,
            "events": abi_events,
        });

        let manifest_methods: Vec<_> = unique_functions
            .iter()
            .map(|func| {
                // Real offset = raw_offset + INIT_MARKER_LEN (init marker is prepended)
                let offset = func.raw_offset + INIT_MARKER_LEN;
                let parameters: Vec<_> = func
                    .params
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        serde_json::json!({
                            "name": p,
                            "type": "Any",
                            "index": i
                        })
                    })
                    .collect();
                let returntype = if func.returns.is_empty() {
                    "Void"
                } else {
                    "Any"
                };
                serde_json::json!({
                    "name": func.name,
                    "offset": offset,
                    "parameters": parameters,
                    "returntype": returntype,
                    "safe": false
                })
            })
            .collect();

        let manifest_events: Vec<_> = events
            .iter()
            .map(|name| {
                serde_json::json!({
                    "name": name,
                    "parameters": []
                })
            })
            .collect();

        let manifest = serde_json::json!({
            "name": contract_name,
            "groups": [],
            "features": {},
            "supportedstandards": [],
            "abi": {
                "methods": manifest_methods,
                "events": manifest_events,
            },
            // Least-privilege default: no permissions granted.
            // Contracts should explicitly declare needed permissions.
            "permissions": [],
            "trusts": [],
            "extra": {
                "Author": "Jimmy <jimmy@r3e.network>",
                "Description": "Contract compiled from Yul source",
                "Version": "1.0.0"
            }
        });

        let assembly = self.generate_assembly_representation(&bytecode);
        let source_map = self.generate_source_map(ast);
        let ast_node_count = self.count_ast_nodes(ast);

        // Collect function names for debug_info (backward-compatible)
        let function_names: Vec<String> = functions.iter().map(|f| f.name.clone()).collect();

        Ok(CompilationResult {
            bytecode,
            assembly,
            abi,
            manifest,
            estimated_gas,
            source_map,
            debug_info: serde_json::json!({
                "functions": function_names,
                "events": events,
                "ast_nodes": ast_node_count
            }),
        })
    }
}
