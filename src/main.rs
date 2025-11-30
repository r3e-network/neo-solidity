//! Neo Solidity Compiler (neo-solc)
//!
//! Command-line compiler for Neo N3 smart contracts written in Solidity.
//!
//! # Usage
//!
//! ```bash
//! # Compile a single file
//! neo-solc input.sol -o output/
//!
//! # Use standard JSON interface
//! neo-solc --standard-json < input.json > output.json
//!
//! # Set optimization level (0-3)
//! neo-solc input.sol -o output/ -O 3
//! ```
//!
//! # Output Files
//!
//! - `*.nef` - Neo Executable Format bytecode
//! - `*.manifest.json` - Contract manifest for deployment
//! - `*.abi.json` - Application Binary Interface

mod cli;

fn main() {
    cli::run();
}
