impl Optimizer {
    pub fn new(level: u8) -> Self {
        Self {
            level,
            stats: OptimizationStats {
                eliminated_instructions: 0,
                inlined_functions: 0,
                folded_constants: 0,
            },
        }
    }

    pub fn optimize(&mut self, mut ast: AstNode) -> Result<AstNode, CompilerError> {
        match self.level {
            0 => Ok(ast), // No optimization
            1 => {
                ast = self.constant_folding(ast)?;
                Ok(ast)
            }
            2 => {
                ast = self.constant_folding(ast)?;
                ast = self.dead_code_elimination(ast)?;
                Ok(ast)
            }
            3 => {
                ast = self.constant_folding(ast)?;
                ast = self.dead_code_elimination(ast)?;
                ast = self.function_inlining(ast)?;
                Ok(ast)
            }
            _ => Err(CompilerError::CodegenError(
                "Invalid optimization level".to_string(),
            )),
        }
    }

    pub fn get_stats(&self) -> &OptimizationStats {
        &self.stats
    }
}

