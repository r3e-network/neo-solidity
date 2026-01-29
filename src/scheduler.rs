//! Instruction Scheduler
//!
//! Reorders instructions for better performance.

/// Scheduling priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
}

/// Scheduled instruction
#[derive(Debug)]
pub struct ScheduledInstr {
    pub index: usize,
    pub priority: Priority,
}
