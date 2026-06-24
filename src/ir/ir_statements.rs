use super::*;

#[path = "statements/dispatch.rs"]
mod dispatch;
pub(crate) use dispatch::*;

#[path = "statements/assignments.rs"]
mod assignments;
pub(crate) use assignments::*;
#[path = "statements/events.rs"]
mod events;
pub(crate) use events::*;
#[path = "statements/assembly.rs"]
mod assembly;
pub(crate) use assembly::*;
#[path = "statements/logical.rs"]
mod logical;
pub(crate) use logical::*;
