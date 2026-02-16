//! Storage Layout Optimization
//!
//! Optimizes storage variable layout for gas efficiency.

/// Storage slot info
#[derive(Debug, Clone)]
pub struct StorageSlot {
    pub slot: u64,
    pub offset: u8,
    pub size: u8,
}

/// Pack variables into storage slots
pub fn pack_variables(sizes: &[u8]) -> Vec<StorageSlot> {
    let mut result = Vec::new();
    let mut slot = 0u64;
    let mut offset = 0u8;

    for &size in sizes {
        if offset + size > 32 {
            slot += 1;
            offset = 0;
        }
        result.push(StorageSlot { slot, offset, size });
        offset += size;
    }
    result
}
