//! IR Optimization Module
//!
//! Performs optimization passes on the intermediate representation (IR) before
//! bytecode generation. These optimizations reduce code size and improve runtime
//! performance.
//!
//! # Optimization Levels
//!
//! - **Level 0**: No IR optimization
//! - **Level 1**: Basic dead code elimination
//! - **Level 2**: Constant folding and propagation
//! - **Level 3**: NeoVM-specific optimizations (identity ops, boolean simplification)

use neo_devpack_solidity::ir;
use num_traits::{ToPrimitive, Zero};

include!("ir_optimize/optimize.rs");
include!("ir_optimize/constant_folding.rs");
include!("ir_optimize/labels.rs");
include!("ir_optimize/neovm.rs");
