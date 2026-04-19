//! Compiler-level property tests.
//!
//! Extracted from the top-level "Compiler Fuzz Tests" banner in
//! `tests/fuzz_tests.rs`. Contents unchanged from the pre-split file.

#![allow(unused_imports)]

use super::common::*;
use neo_solidity::cli::compile_contracts;
use neo_solidity::runtime::{NeoRuntime, RuntimeConfig};
use proptest::prelude::*;

// ==================== Compiler Fuzz Tests ====================

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
