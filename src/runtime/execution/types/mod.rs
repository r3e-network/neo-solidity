//! Type definitions for execution context.
//!
//! This module organizes all the types used by the execution context into
//! logical submodules:
//!
//! - **stack**: StackItem enum and related serialization implementations
//! - **frame**: CallFrame and TryFrame for function calls and exception handling
//! - **state**: IteratorState, ContractState, and storage-related types
//! - **debug**: StepResult and MemoryChange for debugging support
//! - **gas**: GasTracker for gas consumption tracking
//! - **context**: ExecutionContext, the main execution context type

pub mod stack;
pub mod frame;
pub mod state;
pub mod debug;
pub mod gas;
pub mod context;

// Re-export commonly used types
pub use stack::StackItem;
pub use frame::{CallFrame, TryFrame, TryFrameState};
pub use state::{
    IteratorState, ContractState, OverlayEntry, IsolatedStorageKey, StorageOverlayEntries,
    WhitelistedFeeContract, OracleRequest, LedgerBlock, LedgerTransaction,
    TransactionSigner, NotaryDeposit,
};
pub use debug::{StepResult, MemoryChange};
pub use gas::GasTracker;
pub use context::ExecutionContext;
