impl CodeGenerator {
    fn generate_source_map(&self, ast: &AstNode) -> String {
        let mut source_map = String::new();

        self.visit_ast_for_source_map(ast, &mut source_map, 0);

        source_map
    }

    #[allow(clippy::only_used_in_recursion)]
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

    #[allow(clippy::only_used_in_recursion)]
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
            AstNodeType::If {
                condition,
                then_branch,
                else_branch,
            } => {
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

