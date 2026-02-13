impl GasTracker {
    /// Create new gas tracker
    pub fn new(limit: u64) -> Self {
        let mut operation_costs = HashMap::new();
        // NeoVM opcode fees (in GAS fractions)
        operation_costs.insert("ADD".to_string(), 8);
        operation_costs.insert("SUB".to_string(), 8);
        operation_costs.insert("MUL".to_string(), 8);
        operation_costs.insert("DIV".to_string(), 8);
        operation_costs.insert("PUSH".to_string(), 1);
        operation_costs.insert("POP".to_string(), 1);
        // Syscall fees
        operation_costs.insert("CALL".to_string(), 512);
        operation_costs.insert("SSTORE".to_string(), 200);
        operation_costs.insert("SLOAD".to_string(), 100);

        Self {
            limit,
            used: 0,
            base_cost: 0, // NeoVM has no base transaction cost in execution context
            operation_costs,
        }
    }

    /// Reset gas tracker
    pub fn reset(&mut self, new_limit: u64) {
        self.limit = new_limit;
        self.used = 0;
    }

    /// Consume gas for operation
    pub fn consume_gas(
        &mut self,
        operation: &str,
        amount: Option<u64>,
    ) -> Result<(), RuntimeError> {
        let cost = amount.unwrap_or_else(|| *self.operation_costs.get(operation).unwrap_or(&1));

        if self.used + cost > self.limit {
            return Err(RuntimeError::OutOfGas {
                used: self.used + cost,
                limit: self.limit,
            });
        }

        self.used += cost;
        Ok(())
    }

    /// Get remaining gas
    pub fn remaining(&self) -> u64 {
        self.limit.saturating_sub(self.used)
    }

    /// Get used gas
    pub fn used(&self) -> u64 {
        self.used
    }

    /// Sync gas usage with execution context accounting.
    pub fn sync_from_execution(&mut self, execution_used: u64) {
        self.used = execution_used;
    }

    /// Base transaction cost that seeds gas tracking.
    pub fn base_cost(&self) -> u64 {
        self.base_cost
    }

    /// Get gas limit
    pub fn limit(&self) -> u64 {
        self.limit
    }

    /// Check if out of gas
    pub fn out_of_gas(&self) -> bool {
        self.used >= self.limit
    }

    /// Get usage percentage
    pub fn usage_percent(&self) -> f64 {
        if self.limit == 0 {
            return 0.0;
        }
        (self.used as f64 / self.limit as f64) * 100.0
    }
}

