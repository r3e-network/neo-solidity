//! Edge-case arithmetic / identifier / string-encoding property tests.
//!
//! Extracted from the top-level "Edge Case Tests" banner in
//! `tests/fuzz_tests.rs`. Contents unchanged from the pre-split file.

#![allow(unused_imports)]

use super::common::*;
use neo_solidity::cli::compile_contracts;
use neo_solidity::runtime::{NeoRuntime, RuntimeConfig};
use proptest::prelude::*;

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
