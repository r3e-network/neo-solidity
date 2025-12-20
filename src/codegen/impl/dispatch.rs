impl CodeGenerator {
    pub fn new(config: &CompilerConfig) -> Self {
        Self {
            _config: config.clone(),
        }
    }

    pub fn generate(&mut self, ast: &AstNode) -> Result<CompilationResult, CompilerError> {
        let mut bytecode = Vec::new();
        let mut functions = Vec::new();
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

        // Add contract initialization
        bytecode.insert(0, 0x0C); // PUSHDATA1
        bytecode.insert(1, 0x04); // Length
        bytecode.extend_from_slice(b"init"); // Initialization marker

        // Add final return
        bytecode.push(0x40); // RET

        let mut unique_functions = Vec::new();
        let mut seen_funcs = HashSet::new();
        for name in &functions {
            if seen_funcs.insert(name.clone()) {
                unique_functions.push(name.clone());
            }
        }

        let abi_functions: Vec<_> = unique_functions
            .iter()
            .map(|name| {
                serde_json::json!({
                    "name": name,
                    "inputs": [],
                    "outputs": [],
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
            .enumerate()
            .map(|(idx, name)| {
                serde_json::json!({
                    "name": name,
                    "offset": (idx * 16) as u32,
                    "parameters": [],
                    "returntype": "Any",
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
            "name": "GeneratedContract",
            "groups": [],
            "features": {},
            "supportedstandards": [],
            "abi": {
                "methods": manifest_methods,
                "events": manifest_events,
            },
            "permissions": [{"contract": "*", "methods": "*"}],
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

        Ok(CompilationResult {
            bytecode,
            assembly,
            abi,
            manifest,
            estimated_gas,
            source_map,
            debug_info: serde_json::json!({
                "functions": functions,
                "events": events,
                "ast_nodes": ast_node_count
            }),
        })
    }
}

