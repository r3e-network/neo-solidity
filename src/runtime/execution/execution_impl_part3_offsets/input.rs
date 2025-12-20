impl ExecutionContext {
    /// Return the length of the calldata buffer
    pub fn input_size(&self) -> usize {
        self.input_data.len()
    }

    /// Read a slice of calldata, zero-padding when the requested range exceeds the buffer
    pub fn read_input_slice(&self, offset: usize, length: usize) -> Vec<u8> {
        let mut buffer = vec![0u8; length];
        if offset >= self.input_data.len() {
            return buffer;
        }

        let available = (offset + length).min(self.input_data.len());
        let copy_len = available.saturating_sub(offset);
        buffer[..copy_len].copy_from_slice(&self.input_data[offset..offset + copy_len]);
        buffer
    }

    /// Copy calldata into linear memory, expanding memory as required
    pub fn copy_input_to_memory(
        &mut self,
        memory_offset: usize,
        input_offset: usize,
        length: usize,
    ) -> Result<(), RuntimeError> {
        if length == 0 {
            return Ok(());
        }

        let data = self.read_input_slice(input_offset, length);
        self.write_memory(memory_offset, &data)
    }

    /// Size of the calldata buffer
    pub fn calldatasize(&self) -> usize {
        self.input_data.len()
    }

    /// Load a 32-byte word from calldata starting at `offset`
    pub fn calldataload_word(&self, offset: usize) -> [u8; 32] {
        let data = self.read_input_slice(offset, 32);
        let mut word = [0u8; 32];
        word.copy_from_slice(&data);
        word
    }

    /// Copy calldata into linear memory
    pub fn calldatacopy(
        &mut self,
        memory_offset: usize,
        data_offset: usize,
        length: usize,
    ) -> Result<(), RuntimeError> {
        if length == 0 {
            return Ok(());
        }
        let data = self.read_input_slice(data_offset, length);
        self.write_memory(memory_offset, &data)
    }
}

