impl VMBridge {
    /// Create new VM bridge
    pub fn new(config: &RuntimeConfig) -> Result<Self, RuntimeError> {
        let mut bridge = Self {
            config: config.clone(),
            instruction_mapping: HashMap::new(),
            system_calls: HashMap::new(),
            contract_account: config.contract_account.clone(),
        };

        bridge.initialize_instruction_mapping();
        bridge.initialize_system_calls();
        Ok(bridge)
    }
}
