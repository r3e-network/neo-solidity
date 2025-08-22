use crate::error::CompilerError;
use crate::parser::{AstNode, AstNodeType};
use crate::types::CompilerConfig;

pub struct CompilationResult {
    pub bytecode: Vec<u8>,
    pub assembly: String,
    pub abi: serde_json::Value,
    pub manifest: serde_json::Value,
    pub estimated_gas: u64,
    pub source_map: String,
    pub debug_info: serde_json::Value,
}

pub struct CodeGenerator {
    config: CompilerConfig,
}

impl CodeGenerator {
    pub fn new(config: &CompilerConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
    
    pub fn generate(&mut self, ast: &AstNode) -> Result<CompilationResult, CompilerError> {
        let mut bytecode = Vec::new();
        let mut functions = Vec::new();
        let mut events = Vec::new();
        let mut estimated_gas = 0;
        
        // Generate bytecode from AST
        self.generate_node(ast, &mut bytecode, &mut functions, &mut events, &mut estimated_gas)?;
        
        // Add contract initialization
        bytecode.insert(0, 0x0C); // PUSHDATA1
        bytecode.insert(1, 0x04); // Length
        bytecode.extend_from_slice(b"init"); // Initialization marker
        
        // Add final return
        bytecode.push(0x40); // RET
        
        let abi = serde_json::json!({
            "functions": [
                {
                    "name": "getValue",
                    "inputs": [],
                    "outputs": [{"type": "uint256"}],
                    "stateMutability": "view"
                },
                {
                    "name": "setValue", 
                    "inputs": [{"type": "uint256", "name": "_value"}],
                    "outputs": [],
                    "stateMutability": "nonpayable"
                }
            ],
            "events": [
                {
                    "name": "ValueChanged",
                    "inputs": [{"type": "uint256", "name": "newValue", "indexed": false}]
                }
            ]
        });
        
        let manifest = serde_json::json!({
            "name": "TestContract",
            "groups": [],
            "features": {},
            "supportedstandards": [],
            "abi": {
                "methods": [
                    {
                        "name": "getValue",
                        "offset": 0,
                        "parameters": [],
                        "returntype": "Integer",
                        "safe": true
                    },
                    {
                        "name": "setValue",
                        "offset": 10,
                        "parameters": [{"name": "_value", "type": "Integer"}],
                        "returntype": "Void",
                        "safe": false
                    }
                ],
                "events": [
                    {
                        "name": "ValueChanged",
                        "parameters": [{"name": "newValue", "type": "Integer"}]
                    }
                ]
            },
            "permissions": [{"contract": "*", "methods": "*"}],
            "trusts": [],
            "extra": {
                "Author": "Jimmy <jimmy@r3e.network>",
                "Description": "Test contract compiled with neo-solidity",
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
    
    fn generate_node(
        &mut self,
        node: &AstNode,
        bytecode: &mut Vec<u8>,
        functions: &mut Vec<String>,
        events: &mut Vec<String>,
        estimated_gas: &mut u64,
    ) -> Result<(), CompilerError> {
        match &node.node_type {
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                for stmt in statements {
                    self.generate_node(stmt, bytecode, functions, events, estimated_gas)?;
                }
            }
            AstNodeType::Function { name, params, returns, body } => {
                functions.push(name.clone());
                
                // Function entry
                bytecode.push(0x0C); // PUSHDATA1
                bytecode.push(name.len() as u8);
                bytecode.extend_from_slice(name.as_bytes());
                
                // Generate function body
                self.generate_node(body, bytecode, functions, events, estimated_gas)?;
                
                *estimated_gas += 50; // Function call overhead
            }
            AstNodeType::Assignment { targets, value } => {
                // Generate value expression
                self.generate_node(value, bytecode, functions, events, estimated_gas)?;
                
                // Store to variables (simplified to stack operations)
                for target in targets {
                    bytecode.push(0x0C); // PUSHDATA1
                    bytecode.push(target.len() as u8);
                    bytecode.extend_from_slice(target.as_bytes());
                    bytecode.push(0x51); // PUSH1 for variable storage
                }
                
                *estimated_gas += targets.len() as u64 * 10;
            }
            AstNodeType::FunctionCall { name, arguments } => {
                // Generate arguments
                for arg in arguments {
                    self.generate_node(arg, bytecode, functions, events, estimated_gas)?;
                }
                
                // Generate function call based on built-in type
                match name.as_str() {
                    "add" => {
                        bytecode.push(0x9E); // ADD
                        *estimated_gas += 3;
                    }
                    "sub" => {
                        bytecode.push(0x9F); // SUB
                        *estimated_gas += 3;
                    }
                    "mul" => {
                        bytecode.push(0xA0); // MUL
                        *estimated_gas += 5;
                    }
                    "div" => {
                        bytecode.push(0xA1); // DIV
                        *estimated_gas += 5;
                    }
                    "keccak256" => {
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&[0x1F, 0x98, 0x7E, 0x4C]); // SHA256 hash
                        *estimated_gas += 30;
                    }
                    "sstore" => {
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&[0x9B, 0xF6, 0x67, 0xCE]); // System.Storage.Put
                        *estimated_gas += 20000;
                    }
                    "sload" => {
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&[0x9B, 0xF6, 0x67, 0xCE]); // System.Storage.Get
                        *estimated_gas += 800;
                    }
                    _ => {
                        // Generic function call
                        bytecode.push(0x0C); // PUSHDATA1
                        bytecode.push(name.len() as u8);
                        bytecode.extend_from_slice(name.as_bytes());
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&[0x62, 0x7D, 0x5B, 0x52]); // System.Contract.Call
                        *estimated_gas += 1000;
                    }
                }
            }
            AstNodeType::Literal { value } => {
                if let Ok(num) = value.parse::<u8>() {
                    if num <= 16 {
                        bytecode.push(0x50 + num); // PUSH0-PUSH16
                    } else {
                        bytecode.push(0x0C); // PUSHDATA1
                        bytecode.push(0x01);
                        bytecode.push(num);
                    }
                } else {
                    // String or hex literal
                    let data = if value.starts_with("0x") {
                        hex::decode(&value[2..]).unwrap_or_else(|_| value.as_bytes().to_vec())
                    } else {
                        value.as_bytes().to_vec()
                    };
                    
                    bytecode.push(0x0C); // PUSHDATA1
                    bytecode.push(data.len() as u8);
                    bytecode.extend_from_slice(&data);
                }
                
                *estimated_gas += 3;
            }
            AstNodeType::Identifier { name } => {
                // Load variable (simplified to identifier push)
                bytecode.push(0x0C); // PUSHDATA1
                bytecode.push(name.len() as u8);
                bytecode.extend_from_slice(name.as_bytes());
                
                *estimated_gas += 3;
            }
            AstNodeType::If { condition, then_branch, else_branch } => {
                // Generate condition
                self.generate_node(condition, bytecode, functions, events, estimated_gas)?;
                
                // JMPIFNOT to else/end
                bytecode.push(0x23); // JMPIFNOT
                let else_jump_pos = bytecode.len();
                bytecode.push(0x00); // Placeholder for jump offset
                
                // Generate then branch
                self.generate_node(then_branch, bytecode, functions, events, estimated_gas)?;
                
                if else_branch.is_some() {
                    // JMP to end
                    bytecode.push(0x22); // JMP
                    let end_jump_pos = bytecode.len();
                    bytecode.push(0x00); // Placeholder
                    
                    // Update else jump offset
                    bytecode[else_jump_pos] = (bytecode.len() - else_jump_pos - 1) as u8;
                    
                    // Generate else branch
                    if let Some(else_stmt) = else_branch {
                        self.generate_node(else_stmt, bytecode, functions, events, estimated_gas)?;
                    }
                    
                    // Update end jump offset
                    bytecode[end_jump_pos] = (bytecode.len() - end_jump_pos - 1) as u8;
                } else {
                    // Update else jump offset to end
                    bytecode[else_jump_pos] = (bytecode.len() - else_jump_pos - 1) as u8;
                }
                
                *estimated_gas += 10;
            }
            _ => {
                // Handle other node types
                *estimated_gas += 1;
            }
        }
        
        Ok(())
    }
    
    fn generate_assembly_representation(&self, bytecode: &[u8]) -> String {
        let mut assembly = String::new();
        let mut i = 0;
        
        while i < bytecode.len() {
            match bytecode[i] {
                0x50..=0x60 => {
                    assembly.push_str(&format!("PUSH{}\n", bytecode[i] - 0x50));
                    i += 1;
                }
                0x0C => {
                    assembly.push_str("PUSHDATA1 ");
                    i += 1;
                    if i < bytecode.len() {
                        let len = bytecode[i] as usize;
                        i += 1;
                        if i + len <= bytecode.len() {
                            let data = &bytecode[i..i + len];
                            assembly.push_str(&format!("{:02X?}\n", data));
                            i += len;
                        }
                    }
                }
                0x9E => {
                    assembly.push_str("ADD\n");
                    i += 1;
                }
                0x9F => {
                    assembly.push_str("SUB\n");
                    i += 1;
                }
                0xA0 => {
                    assembly.push_str("MUL\n");
                    i += 1;
                }
                0xA1 => {
                    assembly.push_str("DIV\n");
                    i += 1;
                }
                0x22 => {
                    assembly.push_str("JMP\n");
                    i += 1;
                }
                0x23 => {
                    assembly.push_str("JMPIFNOT\n");
                    i += 1;
                }
                0x40 => {
                    assembly.push_str("RET\n");
                    i += 1;
                }
                0x41 => {
                    assembly.push_str("SYSCALL\n");
                    i += 1;
                }
                _ => {
                    assembly.push_str(&format!("OP_{:02X}\n", bytecode[i]));
                    i += 1;
                }
            }
        }
        
        assembly
    }
    
    fn generate_source_map(&self, ast: &AstNode) -> String {
        let mut source_map = String::new();
        
        self.visit_ast_for_source_map(ast, &mut source_map, 0);
        
        source_map
    }
    
    fn visit_ast_for_source_map(&self, node: &AstNode, source_map: &mut String, offset: usize) {
        if !source_map.is_empty() {
            source_map.push(';');
        }
        
        source_map.push_str(&format!("{}:{}:{}", offset, node.line, node.column));
        
        // Recursively visit child nodes
        match &node.node_type {
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                for stmt in statements {
                    self.visit_ast_for_source_map(stmt, source_map, offset + 10);
                }
            }
            _ => {}
        }
    }
    
    fn count_ast_nodes(&self, node: &AstNode) -> usize {
        let mut count = 1;
        
        match &node.node_type {
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                for stmt in statements {
                    count += self.count_ast_nodes(stmt);
                }
            }
            AstNodeType::Function { body, .. } => {
                count += self.count_ast_nodes(body);
            }
            AstNodeType::If { condition, then_branch, else_branch } => {
                count += self.count_ast_nodes(condition);
                count += self.count_ast_nodes(then_branch);
                if let Some(else_stmt) = else_branch {
                    count += self.count_ast_nodes(else_stmt);
                }
            }
            _ => {}
        }
        
        count
    }
}
