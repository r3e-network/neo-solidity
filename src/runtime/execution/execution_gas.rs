use super::*;

/// Named-operation fallback fee (in GAS fractions) for the legacy
/// `consume_gas(op, None)` path. The primary interpreter charges per-opcode
/// via `spec::opcode_gas`, so this table is only a compatibility fallback —
/// a static `match` instead of the previous per-instance
/// `HashMap<String, u64>` that allocated 9 key Strings on every
/// `GasTracker::new`.
fn named_operation_cost(operation: &str) -> u64 {
    match operation {
        // NeoVM opcode fees
        "ADD" | "SUB" | "MUL" | "DIV" => 8,
        "PUSH" | "POP" => 1,
        // Syscall fees
        "CALL" => 512,
        "SSTORE" => 200,
        "SLOAD" => 100,
        _ => 1,
    }
}

impl GasTracker {
    /// Create new gas tracker
    pub fn new(limit: u64) -> Self {
        Self {
            limit,
            used: 0,
            base_cost: 0, // NeoVM has no base transaction cost in execution context
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
        let cost = amount.unwrap_or_else(|| named_operation_cost(operation));

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
