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

    #[test]
    fn identifier_keyword_case_variants_compile(
        name in prop_oneof![
            Just("Contract".to_string()),
            Just("FUNCTION".to_string()),
            Just("pUblic".to_string()),
            Just("PrAgMa".to_string()),
            Just("uInT256".to_string()),
            Just("SoLidity".to_string()),
            Just("RETURN".to_string()),
            Just("Memory".to_string()),
            Just("STORAGE".to_string()),
            Just("Calldata".to_string()),
            Just("EVEnt".to_string()),
            Just("ModiFIER".to_string()),
        ]
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
        prop_assert!(result.is_ok(), "Compilation failed for case-variant '{}': {:?}", name, result.err());
    }

    #[test]
    fn nested_ternary_compile(
        depth in 1usize..8
    ) {
        let mut expr = "0".to_string();
        for i in 0..depth {
            expr = format!("x > {} ? {} : {}", i, i + 1, expr);
        }
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function test(uint256 x) public pure returns (uint256) {{
        return {};
    }}
}}"#,
            expr
        );
        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn nested_function_calls_compile(
        depth in 1usize..8
    ) {
        let mut calls = "x".to_string();
        for i in 0..depth {
            calls = format!("add({}, {})", calls, i);
        }
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function add(uint256 a, uint256 b) public pure returns (uint256) {{
        return a + b;
    }}
    function test(uint256 x) public pure returns (uint256) {{
        return {};
    }}
}}"#,
            calls
        );
        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn hex_literals_compile(
        hex in "[0-9a-fA-F]{1,64}"
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    uint256 public value = 0x{};
}}"#,
            hex
        );
        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn scientific_notation_literals_compile(
        mantissa in "[1-9][0-9]{0,5}",
        exp in 0u32..40
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    uint256 public value = {}e{};
}}"#,
            mantissa, exp
        );
        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn leading_zeros_compile(
        digits in "[0-9]{1,20}"
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    uint256 public value = 0{};
}}"#,
            digits
        );
        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn comments_in_unusual_places_compile(
        name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma/*comment*/solidity ^0.8.0;
contract TestContract {{
    uint256/*a*/public {} = 42;
    function/*b*/test()/*c*/public/*d*/pure/*e*/returns/*f*/(uint256) {{
        return/*g*/42/*h*/;
    }}
}}"#,
            name
        );
        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn empty_string_compile(
        _dummy in any::<bool>()
    ) {
        let source = "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\ncontract TestContract {\n    string public message = \"\";\n}";
        let result = compile_contracts(source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }

    #[test]
    fn string_escape_sequences_compile(
        esc in prop_oneof![
            Just("\\n".to_string()),
            Just("\\t".to_string()),
            Just("\\r".to_string()),
            Just("\\\\".to_string()),
            Just("\\x41".to_string()),
            Just("\\x00".to_string()),
        ]
    ) {
        let source = format!(
            "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\ncontract TestContract {{\n    string public message = \"{}\";\n}}",
            esc
        );
        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Compilation failed: {:?}", result.err());
    }
}

// ==================== Coverage-gap targeted fuzz tests ====================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(20))]

    // Invariant: function overloading with SAME arity but DIFFERENT parameter types
    // compiles and each overload is declared in the manifest with a distinct name.
    #[test]
    fn overloaded_functions_same_arity_different_types(
        fn_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function {name}(uint256 a) public pure returns (uint256) {{ return a; }}
    function {name}(string memory a) public pure returns (string memory) {{ return a; }}
    function {name}(address a) public pure returns (address) {{ return a; }}
}}"#,
            name = fn_name
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Overload compile failed: {:?}", result.err());

        let artifacts = result.unwrap();
        prop_assert!(!artifacts.is_empty(), "expected at least one artifact");

        let methods = artifacts[0].manifest["abi"]["methods"]
            .as_array()
            .expect("abi.methods array");

        let entries: Vec<_> = methods.iter().filter(|m| {
            m.get("name").and_then(serde_json::Value::as_str)
                .map(|n| n == fn_name.as_str() || n.starts_with(&format!("{}(", fn_name)))
                .unwrap_or(false)
        }).collect();

        prop_assert!(
            entries.len() >= 2,
            "Expected at least 2 overload entries for '{}', got {}. methods={:?}",
            fn_name,
            entries.len(),
            methods.iter().map(|m| m.get("name").cloned()).collect::<Vec<_>>()
        );
    }

    // Invariant: immutable variables of various types compile.
    #[test]
    fn immutable_various_types_compile(
        ty in prop_oneof![Just("uint256"), Just("address"), Just("bytes32"), Just("bool")],
        var_name in identifier_strategy()
    ) {
        let init = match ty {
            "uint256" => "42",
            "address" => "address(0x1234567890123456789012345678901234567890)",
            "bytes32" => "bytes32(uint256(0xabcd))",
            "bool" => "true",
            _ => "0",
        };

        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    {ty} public immutable {name} = {init};
}}"#,
            ty = ty,
            name = var_name,
            init = init
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Immutable {ty} compile failed: {:?}", result.err());
    }

    // Invariant: try/catch with varying clause combinations compiles.
    #[test]
    fn try_catch_clause_combinations_compile(
        has_error_clause in any::<bool>(),
        has_custom_clause in any::<bool>(),
        has_bare_clause in any::<bool>(),
    ) {
        prop_assume!(has_error_clause || has_custom_clause || has_bare_clause);

        let mut clauses = String::new();
        if has_error_clause {
            clauses.push_str("        } catch Error(string memory reason) {\n            reason;\n");
        }
        if has_custom_clause {
            clauses.push_str("        } catch Panic(uint256 code) {\n            code;\n");
        }
        if has_bare_clause {
            clauses.push_str("        } catch (bytes memory lowLevelData) {\n            lowLevelData;\n");
        }

        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract TryCatchFuzz {{
    function foo() external pure {{ }}
    function bar() external {{
        try this.foo() {{
{clauses}        }}
    }}
}}"#,
            clauses = clauses
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Try/catch compile failed: {:?}", result.err());
    }

    // Invariant: payable functions compile and appear in the manifest.
    #[test]
    fn payable_function_compiles_and_manifest_correct(
        fn_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function {name}() external payable {{ }}
}}"#,
            name = fn_name
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Payable compile failed: {:?}", result.err());

        let artifacts = result.unwrap();
        prop_assert!(!artifacts.is_empty());

        let methods = artifacts[0].manifest["abi"]["methods"]
            .as_array()
            .expect("abi.methods array");

        let method = methods.iter().find(|m| {
            m.get("name").and_then(serde_json::Value::as_str) == Some(fn_name.as_str())
        });
        prop_assert!(method.is_some(), "Payable method '{}' missing from manifest", fn_name);
    }

    // Invariant: fixed-size arrays of various types and sizes compile.
    #[test]
    fn fixed_array_various_types_compile(
        size in 1usize..10,
        ty in prop_oneof![Just("uint256"), Just("address"), Just("bytes32")],
        var_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    {ty}[{size}] public {name};
}}"#,
            ty = ty,
            size = size,
            name = var_name
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Fixed array {ty}[{size}] compile failed: {:?}", result.err());
    }

    // Invariant: abi.encode/decode round-trip with varying static-type pairs compiles
    // and both wrapper methods are declared in the manifest.
    #[test]
    fn abi_encode_decode_various_types_compile(
        ty1 in prop_oneof![Just("uint256"), Just("address"), Just("bool")],
        ty2 in prop_oneof![Just("uint256"), Just("address"), Just("bool")],
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;
contract AbiRoundTrip {{
    function pack({ty1} a, {ty2} b) external pure returns (bytes memory) {{
        return abi.encode(a, b);
    }}
    function unpack(bytes calldata data) external pure returns ({ty1}, {ty2}) {{
        return abi.decode(data, ({ty1}, {ty2}));
    }}
}}"#,
            ty1 = ty1,
            ty2 = ty2
        );

        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "abi.encode/decode compile failed: {:?}", result.err());

        let artifacts = result.unwrap();
        prop_assert!(!artifacts.is_empty());

        let methods = artifacts[0].manifest["abi"]["methods"]
            .as_array()
            .expect("abi.methods array");

        prop_assert!(
            methods.iter().any(|m| m.get("name").and_then(serde_json::Value::as_str) == Some("pack")),
            "pack missing from manifest"
        );
        prop_assert!(
            methods.iter().any(|m| m.get("name").and_then(serde_json::Value::as_str) == Some("unpack")),
            "unpack missing from manifest"
        );
    }

    // Invariant: manifest JSON for a basic contract satisfies the Neo N3 required schema.
    #[test]
    fn manifest_schema_validation(
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

        let artifacts = compile_contracts(&source, false, 2).expect("compile");
        prop_assert!(!artifacts.is_empty());
        let manifest = &artifacts[0].manifest;

        // Top-level required keys.
        for key in ["name", "abi", "permissions", "supportedstandards"] {
            prop_assert!(manifest.get(key).is_some(), "manifest missing key: {}", key);
        }

        // abi.methods must be an array.
        let methods = manifest["abi"]["methods"].as_array();
        prop_assert!(methods.is_some(), "abi.methods must be an array");

        // Each method must have required keys.
        for m in methods.unwrap() {
            for key in ["name", "parameters", "returntype", "offset", "safe"] {
                prop_assert!(m.get(key).is_some(), "method missing key: {}", key);
            }
        }

        // permissions must be an array.
        prop_assert!(
            manifest["permissions"].is_array(),
            "permissions must be an array"
        );

        // supportedstandards must be an array.
        prop_assert!(
            manifest["supportedstandards"].is_array(),
            "supportedstandards must be an array"
        );
    }

    // Invariant: internal function pointers compile and can be passed/returned.
    #[test]
    fn internal_function_pointer_compiles(
        func_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function {name}(uint256 a) internal pure returns (uint256) {{ return a + 1; }}
    function apply(uint256 x) public pure returns (uint256) {{
        function (uint256) internal pure returns (uint256) f = {name};
        return f(x);
    }}
}}"#,
            name = func_name
        );
        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Internal function pointer compile failed: {:?}", result.err());
    }

    // Invariant: type(X).interfaceId compiles for interfaces.
    #[test]
    fn type_interface_id_compiles(
        iface_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
interface {name} {{
    function foo() external;
}}
contract TestContract {{
    function getId() public pure returns (bytes4) {{
        return type({name}).interfaceId;
    }}
}}"#,
            name = iface_name
        );
        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "type(X).interfaceId compile failed: {:?}", result.err());
    }

    // Invariant: global using-for directive compiles.
    #[test]
    fn global_using_for_compiles(
        lib_name in identifier_strategy(),
        func_name in identifier_strategy()
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
library {lib} {{
    function {fn}(uint256 a) internal pure returns (uint256) {{ return a * 2; }}
}}
using {lib} for uint256;
contract TestContract {{
    function compute(uint256 x) public pure returns (uint256) {{
        return x.{fn}();
    }}
}}"#,
            lib = lib_name,
            fn = func_name
        );
        let result = compile_contracts(&source, false, 2);
        prop_assert!(result.is_ok(), "Global using-for compile failed: {:?}", result.err());
    }
}
