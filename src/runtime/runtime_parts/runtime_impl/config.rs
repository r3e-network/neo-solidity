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

impl RuntimeConfig {
    /// Create a new builder
    pub fn builder() -> RuntimeConfigBuilder {
        RuntimeConfigBuilder::default()
    }

    /// Create config for testing with debugging enabled
    pub fn for_testing() -> Self {
        Self {
            enable_debugging: true,
            enable_tracing: true,
            strict_mode: false,
            ..Default::default()
        }
    }
}

/// Builder for RuntimeConfig
#[derive(Debug, Clone, Default)]
pub struct RuntimeConfigBuilder {
    config: RuntimeConfig,
}

impl RuntimeConfigBuilder {
    pub fn gas_limit(mut self, limit: u64) -> Self {
        self.config.gas_limit = limit;
        self
    }

    pub fn debugging(mut self, enabled: bool) -> Self {
        self.config.enable_debugging = enabled;
        self
    }

    pub fn tracing(mut self, enabled: bool) -> Self {
        self.config.enable_tracing = enabled;
        self
    }

    pub fn build(self) -> RuntimeConfig {
        self.config
    }
}

