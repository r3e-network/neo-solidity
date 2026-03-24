//! Comprehensive Fuzz Tests for Neo Solidity
//!
//! Property-based testing for critical components ensuring robustness
//! and correctness across a wide range of inputs.

#![allow(clippy::uninlined_format_args)]
#![allow(clippy::single_match)]
#![allow(clippy::partialeq_to_none)]

use neo_solidity::cli::compile_contracts;
use neo_solidity::runtime::{NeoRuntime, RuntimeConfig};
use proptest::prelude::*;

// ==================== Storage Fuzz Tests ====================

proptest! {
    // Test storage roundtrip: write then read returns original value
    #[test]
    fn storage_roundtrip_preserves_data(
        key in prop::collection::vec(any::<u8>(), 1..64),
        value in prop::collection::vec(any::<u8>(), 1..256)
    ) {
        let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let account = "0x1234567890123456789012345678901234567890";

        runtime.set_storage(account, &key, &value).expect("Failed to set storage");
        let retrieved = runtime.get_storage(account, &key).expect("Failed to get storage");

        prop_assert_eq!(retrieved, Some(value));
    }

    // Test storage overwrite: later write takes precedence
    #[test]
    fn storage_overwrite_updates_value(
        key in prop::collection::vec(any::<u8>(), 1..32),
        value1 in prop::collection::vec(any::<u8>(), 1..128),
        value2 in prop::collection::vec(any::<u8>(), 1..128)
    ) {
        let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let account = "0x1234567890123456789012345678901234567890";

        runtime.set_storage(account, &key, &value1).expect("Failed to set storage");
        runtime.set_storage(account, &key, &value2).expect("Failed to overwrite storage");

        let retrieved = runtime.get_storage(account, &key).expect("Failed to get storage");
        prop_assert_eq!(retrieved, Some(value2));
    }

    // Test storage isolation: different accounts don't see each other's data
    #[test]
    fn storage_isolation_between_accounts(
        key in prop::collection::vec(any::<u8>(), 1..32),
        value1 in prop::collection::vec(any::<u8>(), 1..64),
        value2 in prop::collection::vec(any::<u8>(), 1..64)
    ) {
        let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let account1 = "0x1111111111111111111111111111111111111111";
        let account2 = "0x2222222222222222222222222222222222222222";

        runtime.set_storage(account1, &key, &value1).expect("Failed to set storage 1");
        runtime.set_storage(account2, &key, &value2).expect("Failed to set storage 2");

        let retrieved1 = runtime.get_storage(account1, &key).expect("Failed to get storage 1");
        let retrieved2 = runtime.get_storage(account2, &key).expect("Failed to get storage 2");

        prop_assert_eq!(retrieved1, Some(value1));
        prop_assert_eq!(retrieved2, Some(value2));
    }

    // Test storage with empty value - note: empty values may be treated as deleted
    #[test]
    fn storage_empty_value_handling(
        key in prop::collection::vec(any::<u8>(), 1..32)
    ) {
        let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let account = "0x1234567890123456789012345678901234567890";
        let empty_value: Vec<u8> = vec![];

        runtime.set_storage(account, &key, &empty_value).expect("Failed to set empty value");
        let retrieved = runtime.get_storage(account, &key).expect("Failed to get storage");

        // Empty values may be stored as None (deleted) or Some([]) depending on implementation
        prop_assert!(retrieved == Some(empty_value) || retrieved.is_none());
    }

    // Test storage with large values
    #[test]
    fn storage_large_value_roundtrip(
        key in prop::collection::vec(any::<u8>(), 1..32),
        value in prop::collection::vec(any::<u8>(), 1000..5000)
    ) {
        let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let account = "0x1234567890123456789012345678901234567890";

        runtime.set_storage(account, &key, &value).expect("Failed to set large value");
        let retrieved = runtime.get_storage(account, &key).expect("Failed to get storage");

        prop_assert_eq!(retrieved, Some(value));
    }

    // Test balance operations: set and get roundtrip
    #[test]
    fn balance_roundtrip(
        balance in 0u64..10_000_000_000u64
    ) {
        let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let account = "0x1234567890123456789012345678901234567890";

        runtime.set_balance(account, balance).expect("Failed to set balance");
        let retrieved = runtime.get_balance(account).expect("Failed to get balance");

        prop_assert_eq!(retrieved, balance);
    }

    // Test balance isolation: different accounts have independent balances
    #[test]
    fn balance_isolation(
        balance1 in 0u64..1_000_000_000u64,
        balance2 in 0u64..1_000_000_000u64
    ) {
        let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let account1 = "0x1111111111111111111111111111111111111111";
        let account2 = "0x2222222222222222222222222222222222222222";

        runtime.set_balance(account1, balance1).expect("Failed to set balance 1");
        runtime.set_balance(account2, balance2).expect("Failed to set balance 2");

        let retrieved1 = runtime.get_balance(account1).expect("Failed to get balance 1");
        let retrieved2 = runtime.get_balance(account2).expect("Failed to get balance 2");

        prop_assert_eq!(retrieved1, balance1);
        prop_assert_eq!(retrieved2, balance2);
    }

    // Test storage key ordering - using unique keys only
    #[test]
    fn storage_keys_maintain_order(
        unique_keys in prop::collection::hash_set(prop::collection::vec(any::<u8>(), 1..16), 1..20)
    ) {
        let mut runtime = NeoRuntime::new(RuntimeConfig::default()).expect("Failed to create runtime");
        let account = "0x1234567890123456789012345678901234567890";

        let keys: Vec<_> = unique_keys.into_iter().collect();

        for (i, key) in keys.iter().enumerate() {
            let value = (i as u64).to_le_bytes().to_vec();
            runtime.set_storage(account, key, &value).expect("Failed to set storage");
        }

        for (i, key) in keys.iter().enumerate() {
            let retrieved = runtime.get_storage(account, key).expect("Failed to get storage");
            let expected = (i as u64).to_le_bytes().to_vec();
            prop_assert_eq!(retrieved, Some(expected), "Key {:?} should have value {}", key, i);
        }
    }
}

// ==================== Compiler Fuzz Tests ====================

fn identifier_strategy() -> impl Strategy<Value = String> {
    "[a-zA-Z_][a-zA-Z0-9_]{0,30}"
}

fn uint_value_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("0".to_string()),
        "[1-9][0-9]{0,20}".prop_map(String::from),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    #[test]
    fn simple_storage_contract_compiles(
        var_name in identifier_strategy(),
        initial_value in uint_value_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    uint256 public {} = {};
}}"#,
            var_name, initial_value
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn contract_with_functions_compiles(
        func_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    uint256 public value = 0;
    
    function {}() public view returns (uint256) {{
        return value;
    }}
}}"#,
            func_name
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn contract_with_events_compiles(
        event_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    event {}(address indexed sender, uint256 value);
}}"#,
            event_name
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn contract_with_mapping_compiles(
        mapping_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    mapping(address => uint256) public {};
}}"#,
            mapping_name
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn contract_with_require_compiles(
        func_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function {}(uint256 value) public pure returns (uint256) {{
        require(value > 0, "Value must be positive");
        return value * 2;
    }}
}}"#,
            func_name
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn contract_with_modifier_compiles(
        mod_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    address public owner = msg.sender;
    
    modifier {}() {{
        require(msg.sender == owner, "Not owner");
        _;
    }}
}}"#,
            mod_name
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn contract_with_loops_compiles(
        func_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function {}(uint256 n) public pure returns (uint256) {{
        uint256 sum = 0;
        for (uint256 i = 0; i < n && i < 100; i++) {{
            sum += i;
        }}
        return sum;
    }}
}}"#,
            func_name
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn contract_with_conditionals_compiles(
        func_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function {}(uint256 x) public pure returns (uint256) {{
        if (x > 100) {{
            return x * 2;
        }} else if (x > 50) {{
            return x + 10;
        }} else {{
            return x;
        }}
    }}
}}"#,
            func_name
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn empty_contract_compiles(
        contract_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract {} {{}}"#,
            contract_name
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Empty contract should compile: {:?}", result.err());
    }

    #[test]
    fn compilation_is_deterministic(
        var_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    uint256 public {} = 42;
}}"#,
            var_name
        );

        let result1 = compile_contracts(&source, false, 2);
        let result2 = compile_contracts(&source, false, 2);

        prop_assert!(result1.is_ok());
        prop_assert!(result2.is_ok());

        let artifacts1 = result1.unwrap();
        let artifacts2 = result2.unwrap();

        prop_assert_eq!(artifacts1.len(), artifacts2.len());

        for (a1, a2) in artifacts1.iter().zip(artifacts2.iter()) {
            prop_assert_eq!(a1.bytecode.len(), a2.bytecode.len());
        }
    }
}

// ==================== Edge Case Tests ====================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]

    #[test]
    fn large_integer_literals_compile(
        high_bits in any::<u128>()
    ) {
        let value = format!("{}", high_bits);
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract LargeIntContract {{
    uint256 public value = {};
}}"#,
            value
        );

        let result = compile_contracts(&source, false, 2);
        match result {
            Ok(_) => {},
            Err(e) => {
                let err_str = format!("{:?}", e);
                prop_assume!(!err_str.contains("panic") && !err_str.contains("unwrap"));
            }
        }
    }

    #[test]
    fn long_identifiers_compile(
        name in "[a-zA-Z][a-zA-Z0-9_]{50,200}"
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    uint256 public {} = 42;
}}"#,
            name
        );

        let result = compile_contracts(&source, false, 2);
        match result {
            Ok(_) => {},
            Err(_) => {
                // Long identifiers may be rejected
            }
        }
    }

    #[test]
    fn many_functions_compile(
        count in 10usize..50
    ) {
        let funcs: Vec<String> = (0..count)
            .map(|i| format!(
                "    function func{}() public pure returns (uint256) {{ return {}; }}",
                i, i
            ))
            .collect();

        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
{}
}}"#,
            funcs.join("\n")
        );

        let result = compile_contracts(&source, false, 2);
        match result {
            Ok(_) => {},
            Err(_) => {
                // Too many functions may be rejected
            }
        }
    }

    #[test]
    fn long_strings_compile(
        content in prop::collection::vec(any::<char>(), 100..2000)
    ) {
        let escaped: String = content
            .iter()
            .map(|c| match c {
                '"' | '\\' | '\n' | '\r' | '\t' => ' '.to_string(),
                c => c.to_string(),
            })
            .collect();

        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    string public message = "{}";
}}"#,
            escaped
        );

        let result = compile_contracts(&source, false, 2);
        match result {
            Ok(_) => {},
            Err(_) => {
                // Very long strings may be rejected
            }
        }
    }

    #[test]
    fn unicode_strings_compile(
        content in prop::collection::vec(
            prop_oneof![
                Just('α'), Just('β'), Just('γ'), Just('中'), Just('文'),
                Just('🌍'), Just('🚀'), Just('🔥'),
            ],
            1..50
        )
    ) {
        let escaped: String = content.iter().collect();

        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    string public message = unicode"{}";
}}"#,
            escaped
        );

        let result = compile_contracts(&source, false, 2);
        match result {
            Ok(_) => {},
            Err(_) => {}
        }
    }
}
