// Type definitions for execution context.
//
// This file re-exports all types from the types submodule for backward compatibility.
//
// The actual type definitions are now organized in:
// - types/stack.rs: StackItem enum and serialization
// - types/frame.rs: CallFrame and TryFrame for call/exception handling
// - types/state.rs: IteratorState, ContractState, and storage types
// - types/debug.rs: StepResult and MemoryChange for debugging
// - types/gas.rs: GasTracker for gas consumption tracking
// - types/context.rs: ExecutionContext, the main execution context

pub mod types;

// Re-export all types for backward compatibility
pub use types::*;
