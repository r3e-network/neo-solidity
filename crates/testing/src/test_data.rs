//! Test data generation and fixtures for Neo Solidity
//!
//! Provides sample contracts, bytecode, and test vectors

use serde::{Deserialize, Serialize};

/// Sample Solidity contract for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleContract {
    pub name: String,
    pub source: String,
    pub expected_bytecode: Option<Vec<u8>>,
}

/// Test vector for bytecode execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTestVector {
    pub name: String,
    pub bytecode: Vec<u8>,
    pub input: Vec<u8>,
    pub expected_output: Vec<u8>,
    pub expected_gas: u64,
}

/// Get sample ERC20-like contract
pub fn sample_erc20_contract() -> SampleContract {
    SampleContract {
        name: "SimpleToken".to_string(),
        source: r#"
            contract SimpleToken {
                mapping(address => uint256) public balances;

                function transfer(address to, uint256 amount) public returns (bool) {
                    require(balances[msg.sender] >= amount, "Insufficient balance");
                    balances[msg.sender] -= amount;
                    balances[to] += amount;
                    return true;
                }
            }
        "#.to_string(),
        expected_bytecode: None,
    }
}

/// Get sample storage contract
pub fn sample_storage_contract() -> SampleContract {
    SampleContract {
        name: "SimpleStorage".to_string(),
        source: r#"
            contract SimpleStorage {
                uint256 public value;

                function set(uint256 newValue) public {
                    value = newValue;
                }

                function get() public view returns (uint256) {
                    return value;
                }
            }
        "#.to_string(),
        expected_bytecode: None,
    }
}

/// Get basic execution test vectors
pub fn basic_execution_vectors() -> Vec<ExecutionTestVector> {
    vec![
        ExecutionTestVector {
            name: "push_and_return".to_string(),
            bytecode: vec![0x11, 0x40], // PUSH1, RET
            input: vec![],
            expected_output: vec![1],
            expected_gas: 10,
        },
        ExecutionTestVector {
            name: "add_two_numbers".to_string(),
            bytecode: vec![0x11, 0x12, 0x9E, 0x40], // PUSH1, PUSH2, ADD, RET
            input: vec![],
            expected_output: vec![3],
            expected_gas: 20,
        },
    ]
}
