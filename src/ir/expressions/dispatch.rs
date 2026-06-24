use super::*;

#[path = "dispatch/entry.rs"]
mod entry;
pub(crate) use entry::*;
#[path = "dispatch/binary_ops.rs"]
mod binary_ops;
pub(crate) use binary_ops::*;
#[path = "dispatch/assignments.rs"]
mod assignments;
pub(crate) use assignments::*;
#[path = "dispatch/unary.rs"]
mod unary;
pub(crate) use unary::*;
#[path = "dispatch/comparisons.rs"]
mod comparisons;
pub(crate) use comparisons::*;
#[path = "dispatch/primary.rs"]
mod primary;
pub(crate) use primary::*;
#[path = "dispatch/calls.rs"]
mod calls;
pub(crate) use calls::*;
#[path = "dispatch/tuple.rs"]
mod tuple;
pub(crate) use tuple::*;
#[path = "dispatch/conditional.rs"]
mod conditional;
pub(crate) use conditional::*;
#[path = "dispatch/binary_predicates.rs"]
mod binary_predicates;
pub(crate) use binary_predicates::*;
#[path = "dispatch/binary_u256_softarith.rs"]
mod binary_u256_softarith;
pub(crate) use binary_u256_softarith::*;
#[path = "dispatch/binary_overflow_guards.rs"]
mod binary_overflow_guards;
pub(crate) use binary_overflow_guards::*;
#[path = "dispatch/binary.rs"]
mod binary;
pub(crate) use binary::*;
