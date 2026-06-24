use super::*;

#[path = "builtins/syscalls.rs"]
mod syscalls;
pub(crate) use syscalls::*;
#[path = "builtins/native_calls.rs"]
mod native_calls;
pub(crate) use native_calls::*;
#[path = "builtins/helpers.rs"]
mod helpers;
pub(crate) use helpers::*;
#[path = "builtins/resolve.rs"]
mod resolve;
pub use resolve::builtin_intrinsic_surface;
pub(crate) use resolve::*;
