impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            gas_limit: 10_000_000,
            call_stack_limit: 1024,
            memory_limit: 1024 * 1024,       // 1MB
            storage_limit: 10 * 1024 * 1024, // 10MB
            network_magic: 0x4F454E,         // "NEO" magic
            enable_debugging: false,
            enable_tracing: false,
            strict_mode: true,
            neo_version: "3.5.0".to_string(),
            contract_account: "0x0000000000000000000000000000000000000000".to_string(),
            default_block_height: 0,
            default_timestamp: 0,
        }
    }
}

