//! Register Allocation
//!
//! Allocates local variables to NeoVM slots.

/// Slot allocation result
#[derive(Debug, Clone)]
pub struct SlotAllocation {
    pub var_name: String,
    pub slot_index: u8,
}

/// Simple slot allocator
#[derive(Default)]
pub struct SlotAllocator {
    next_slot: u8,
}

impl SlotAllocator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate(&mut self, name: &str) -> SlotAllocation {
        let slot = self.next_slot;
        self.next_slot += 1;
        SlotAllocation {
            var_name: name.to_string(),
            slot_index: slot,
        }
    }
}
