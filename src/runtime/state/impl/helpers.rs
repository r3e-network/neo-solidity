impl StateManager {
    fn record_change(&mut self, change: StateChange) {
        self.change_log.push(change);
        self.change_count += 1;
    }

    fn calculate_hash(&self, data: &[u8]) -> String {
        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(data);
        hex::encode(hash)
    }

    fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

