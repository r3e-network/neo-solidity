use super::*;

#[path = "inheritance/helpers.rs"]
mod helpers;
pub(crate) use helpers::*;
#[path = "inheritance/linearization.rs"]
mod linearization;
pub(crate) use linearization::*;
#[path = "inheritance/interface_collection.rs"]
mod interface_collection;
pub(crate) use interface_collection::*;
#[path = "inheritance/flatten.rs"]
mod flatten;
pub(crate) use flatten::*;
