impl SemanticAnalyzer {
    fn visit_node<F>(&self, node: &AstNode, visitor: &mut F)
    where
        F: FnMut(&AstNode, u32),
    {
        self.visit_node_recursive(node, visitor, 0);
    }

    #[allow(clippy::only_used_in_recursion)]
    fn visit_node_recursive<F>(&self, node: &AstNode, visitor: &mut F, depth: u32)
    where
        F: FnMut(&AstNode, u32),
    {
        visitor(node, depth);

        match &node.node_type {
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                for stmt in statements {
                    self.visit_node_recursive(stmt, visitor, depth + 1);
                }
            }
            AstNodeType::Function { body, .. } => {
                self.visit_node_recursive(body, visitor, depth + 1);
            }
            AstNodeType::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_node_recursive(condition, visitor, depth + 1);
                self.visit_node_recursive(then_branch, visitor, depth + 1);
                if let Some(else_stmt) = else_branch {
                    self.visit_node_recursive(else_stmt, visitor, depth + 1);
                }
            }
            AstNodeType::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init_stmt) = init {
                    self.visit_node_recursive(init_stmt, visitor, depth + 1);
                }
                self.visit_node_recursive(condition, visitor, depth + 1);
                if let Some(update_stmt) = update {
                    self.visit_node_recursive(update_stmt, visitor, depth + 1);
                }
                self.visit_node_recursive(body, visitor, depth + 1);
            }
            AstNodeType::Switch {
                expression,
                cases,
                default,
            } => {
                self.visit_node_recursive(expression, visitor, depth + 1);
                for case in cases {
                    self.visit_node_recursive(&case.value, visitor, depth + 1);
                    self.visit_node_recursive(&case.body, visitor, depth + 1);
                }
                if let Some(default_stmt) = default {
                    self.visit_node_recursive(default_stmt, visitor, depth + 1);
                }
            }
            AstNodeType::FunctionCall { arguments, .. } => {
                for arg in arguments {
                    self.visit_node_recursive(arg, visitor, depth + 1);
                }
            }
            AstNodeType::Assignment { value, .. } => {
                self.visit_node_recursive(value, visitor, depth + 1);
            }
            _ => {}
        }
    }
}

