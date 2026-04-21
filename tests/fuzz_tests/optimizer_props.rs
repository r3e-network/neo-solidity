//! Optimizer-level property tests.
//!
//! Verifies that optimization levels preserve semantics, NEF/Manifest correctness,
//! and compilation determinism.

#![allow(unused_imports)]
#![allow(clippy::uninlined_format_args)]

use super::common::*;
use neo_solidity::cli::compile_contracts;
use neo_solidity::neo::{build_nef_with_tokens, parse_nef};
use neo_solidity::runtime::{NeoRuntime, RuntimeConfig};
use neo_solidity::runtime::types::StackItem;
use proptest::prelude::*;
use sha2::{Digest, Sha256};

/// Compile `source` at the given optimization level and call `method` with `args`.
fn compile_and_call(
    source: &str,
    opt_level: u8,
    method: &str,
    args: &[StackItem],
) -> Result<neo_solidity::runtime::ExecutionResult, String> {
    let artifacts = compile_contracts(source, false, opt_level)
        .map_err(|e| format!("compile failed at O{opt_level}: {e:?}"))?;
    if artifacts.is_empty() {
        return Err("no artifacts".to_string());
    }
    let art = &artifacts[0];
    let mut rt = NeoRuntime::new(RuntimeConfig::default())
        .map_err(|e| format!("runtime failed: {e:?}"))?;
    rt.call_method(&art.bytecode, &art.tokens, &art.manifest, method, args)
        .map_err(|e| format!("call_method failed: {e:?}"))
}

/// Assert that two execution results are semantically equivalent (success,
/// return data, and exception shape).
fn assert_results_equivalent(
    a: &neo_solidity::runtime::ExecutionResult,
    b: &neo_solidity::runtime::ExecutionResult,
) {
    assert_eq!(a.success, b.success, "success mismatch between O0 and O3");
    assert_eq!(a.return_data, b.return_data, "return_data mismatch between O0 and O3");
    match (&a.exception, &b.exception) {
        (None, None) => {}
        (Some(ae), Some(be)) => {
            assert_eq!(
                ae.exception_type, be.exception_type,
                "exception type mismatch between O0 and O3"
            );
            assert_eq!(
                ae.message, be.message,
                "exception message mismatch between O0 and O3"
            );
        }
        _ => panic!(
            "exception presence mismatch: a={:?}, b={:?}",
            a.exception, b.exception
        ),
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(30))]

    // ------------------------------------------------------------------
    // 1. Optimizer semantic equivalence — pure arithmetic with branching
    // ------------------------------------------------------------------
    #[test]
    fn optimizer_semantic_equivalence_pure_arith(
        func_name in identifier_strategy(),
        x in any::<u32>(),
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function {func_name}(uint256 a) public pure returns (uint256) {{
        uint256 b = a * 3 + 7;
        uint256 c = b / 2;
        if (c > 1000) {{
            c = c - 1000;
        }}
        return c;
    }}
}}"#,
            func_name = func_name
        );

        let r0 = compile_and_call(&source, 0, &func_name, &[StackItem::Integer(x as i64)]);
        let r3 = compile_and_call(&source, 3, &func_name, &[StackItem::Integer(x as i64)]);

        prop_assert!(r0.is_ok(), "O0 execution failed: {:?}", r0.err());
        prop_assert!(r3.is_ok(), "O3 execution failed: {:?}", r3.err());
        assert_results_equivalent(&r0.unwrap(), &r3.unwrap());
    }

    // ------------------------------------------------------------------
    // 2. Optimizer semantic equivalence — bounded loops + conditionals
    // ------------------------------------------------------------------
    #[test]
    fn optimizer_semantic_equivalence_loop_conditional(
        func_name in identifier_strategy(),
        n in 0u32..50u32,
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function {func_name}(uint256 n) public pure returns (uint256) {{
        uint256 sum = 0;
        for (uint256 i = 0; i < n && i < 50; i++) {{
            if (i % 2 == 0) {{
                sum += i;
            }} else {{
                sum += i * 2;
            }}
        }}
        return sum;
    }}
}}"#,
            func_name = func_name
        );

        let r0 = compile_and_call(&source, 0, &func_name, &[StackItem::Integer(n as i64)]);
        let r3 = compile_and_call(&source, 3, &func_name, &[StackItem::Integer(n as i64)]);

        prop_assert!(r0.is_ok(), "O0 execution failed: {:?}", r0.err());
        prop_assert!(r3.is_ok(), "O3 execution failed: {:?}", r3.err());
        assert_results_equivalent(&r0.unwrap(), &r3.unwrap());
    }

    // ------------------------------------------------------------------
    // 3. Optimizer semantic equivalence — recursion (factorial)
    // ------------------------------------------------------------------
    #[test]
    fn optimizer_semantic_equivalence_recursion(
        func_name in identifier_strategy(),
        n in 0u32..15u32,
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function {func_name}(uint256 n) public pure returns (uint256) {{
        if (n <= 1) return 1;
        return n * {func_name}(n - 1);
    }}
}}"#,
            func_name = func_name
        );

        let r0 = compile_and_call(&source, 0, &func_name, &[StackItem::Integer(n as i64)]);
        let r3 = compile_and_call(&source, 3, &func_name, &[StackItem::Integer(n as i64)]);

        prop_assert!(r0.is_ok(), "O0 execution failed: {:?}", r0.err());
        prop_assert!(r3.is_ok(), "O3 execution failed: {:?}", r3.err());
        assert_results_equivalent(&r0.unwrap(), &r3.unwrap());
    }

    // ------------------------------------------------------------------
    // 4. Optimizer semantic equivalence — internal function calls
    // ------------------------------------------------------------------
    #[test]
    fn optimizer_semantic_equivalence_internal_calls(
        func_name in identifier_strategy(),
        x in any::<u32>(),
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    function helper(uint256 a) internal pure returns (uint256) {{
        return a * 3 + 1;
    }}
    function {func_name}(uint256 x) public pure returns (uint256) {{
        return helper(x) + helper(x + 1);
    }}
}}"#,
            func_name = func_name
        );

        let r0 = compile_and_call(&source, 0, &func_name, &[StackItem::Integer(x as i64)]);
        let r3 = compile_and_call(&source, 3, &func_name, &[StackItem::Integer(x as i64)]);

        prop_assert!(r0.is_ok(), "O0 execution failed: {:?}", r0.err());
        prop_assert!(r3.is_ok(), "O3 execution failed: {:?}", r3.err());
        assert_results_equivalent(&r0.unwrap(), &r3.unwrap());
    }

    // ------------------------------------------------------------------
    // 5. Optimizer semantic equivalence — modifier with require
    // ------------------------------------------------------------------
    #[test]
    fn optimizer_semantic_equivalence_modifier(
        func_name in identifier_strategy(),
        mod_name in identifier_strategy(),
        x in 1u32..100u32,
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    modifier {mod_name}(uint256 min) {{
        require(min >= 1, "min too small");
        _;
    }}
    function {func_name}(uint256 x) public {mod_name}(x) pure returns (uint256) {{
        return x + 1;
    }}
}}"#,
            func_name = func_name,
            mod_name = mod_name
        );

        let r0 = compile_and_call(&source, 0, &func_name, &[StackItem::Integer(x as i64)]);
        let r3 = compile_and_call(&source, 3, &func_name, &[StackItem::Integer(x as i64)]);

        prop_assert!(r0.is_ok(), "O0 execution failed: {:?}", r0.err());
        prop_assert!(r3.is_ok(), "O3 execution failed: {:?}", r3.err());
        assert_results_equivalent(&r0.unwrap(), &r3.unwrap());
    }

    // ------------------------------------------------------------------
    // 6. Optimizer semantic equivalence — storage variables + events
    // ------------------------------------------------------------------
    #[test]
    fn optimizer_semantic_equivalence_storage_and_events(
        func_name in identifier_strategy(),
        event_name in identifier_strategy(),
        init_value in uint_value_strategy(),
        add_value in any::<u32>(),
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    uint256 public value = {init};
    event {event_name}(uint256 oldValue, uint256 newValue);

    function {func_name}(uint256 x) public returns (uint256) {{
        uint256 oldValue = value;
        value = oldValue + x;
        emit {event_name}(oldValue, value);
        return value;
    }}
}}"#,
            func_name = func_name,
            event_name = event_name,
            init = init_value
        );

        let artifacts0 = compile_contracts(&source, false, 0).expect("O0 compile");
        let artifacts3 = compile_contracts(&source, false, 3).expect("O3 compile");
        prop_assert!(!artifacts0.is_empty());
        prop_assert!(!artifacts3.is_empty());

        let mut rt0 = NeoRuntime::new(RuntimeConfig::default()).expect("rt0");
        let mut rt3 = NeoRuntime::new(RuntimeConfig::default()).expect("rt3");

        let res0 = rt0.call_method(
            &artifacts0[0].bytecode, &artifacts0[0].tokens,
            &artifacts0[0].manifest, &func_name,
            &[StackItem::Integer(add_value as i64)],
        );
        let res3 = rt3.call_method(
            &artifacts3[0].bytecode, &artifacts3[0].tokens,
            &artifacts3[0].manifest, &func_name,
            &[StackItem::Integer(add_value as i64)],
        );

        prop_assert!(res0.is_ok(), "O0 call failed: {:?}", res0.err());
        prop_assert!(res3.is_ok(), "O3 call failed: {:?}", res3.err());
        assert_results_equivalent(&res0.unwrap(), &res3.unwrap());
    }

    // ------------------------------------------------------------------
    // 7. NEF correctness — magic header, checksum, tokens, round-trip
    // ------------------------------------------------------------------
    #[test]
    fn nef_format_validity(
        var_name in identifier_strategy(),
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    uint256 public {} = 42;
    function add(uint256 a, uint256 b) public pure returns (uint256) {{
        return a + b;
    }}
}}"#,
            var_name
        );

        let artifacts = compile_contracts(&source, false, 3).expect("compile");
        prop_assert!(!artifacts.is_empty());
        let art = &artifacts[0];

        let nef = build_nef_with_tokens(&art.bytecode, "neo-solidity-fuzz", "", &art.tokens)
            .expect("NEF build");

        // Magic header
        prop_assert!(nef.starts_with(b"NEF3"), "NEF must start with NEF3 magic");
        prop_assert!(nef.len() > 4, "NEF must be larger than checksum");

        // Checksum = sha256(sha256(prefix))[..4]
        let prefix = &nef[..nef.len() - 4];
        let stored_trailer = &nef[nef.len() - 4..];
        let first = Sha256::digest(prefix);
        let second = Sha256::digest(first);
        prop_assert_eq!(stored_trailer, &second[..4], "NEF checksum invalid");

        // Parse round-trip
        let parsed = parse_nef(&nef).expect("parse_nef must succeed");
        prop_assert_eq!(parsed.script, art.bytecode.clone(), "script mismatch after NEF parse");
        prop_assert_eq!(parsed.tokens.len(), art.tokens.len(), "token count mismatch");

        for (orig, out) in art.tokens.iter().zip(parsed.tokens.iter()) {
            prop_assert_eq!(orig.hash, out.hash, "token hash mismatch");
            prop_assert_eq!(&orig.method, &out.method, "token method mismatch");
            prop_assert_eq!(
                orig.parameters_count, out.parameters_count,
                "token parameters_count mismatch"
            );
            prop_assert_eq!(
                orig.has_return_value, out.has_return_value,
                "token has_return_value mismatch"
            );
            prop_assert_eq!(orig.call_flags, out.call_flags, "token call_flags mismatch");
        }
    }

    // ------------------------------------------------------------------
    // 8. Manifest correctness — required fields + ABI consistency
    // ------------------------------------------------------------------
    #[test]
    fn manifest_correctness(
        contract_name in identifier_strategy(),
        func_name in identifier_strategy(),
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract {name} {{
    uint256 public stored = 1;
    event ValueChanged(uint256 newValue);

    function {func}(uint256 x) public pure returns (uint256) {{
        return x * 2;
    }}
}}"#,
            name = contract_name,
            func = func_name
        );

        let artifacts = compile_contracts(&source, false, 2).expect("compile");
        prop_assert!(!artifacts.is_empty());
        let manifest = &artifacts[0].manifest;

        // Required top-level keys
        for key in ["name", "abi", "permissions", "supportedstandards", "groups"] {
            prop_assert!(
                manifest.get(key).is_some(),
                "manifest missing required top-level key: {key}"
            );
        }

        // ABI structure
        let abi = manifest.get("abi").expect("abi object");
        let methods = abi.get("methods")
            .and_then(|m| m.as_array())
            .expect("abi.methods must be an array");
        let events = abi.get("events")
            .and_then(|e| e.as_array())
            .expect("abi.events must be an array");

        prop_assert!(!methods.is_empty(), "manifest abi.methods must not be empty");

        // Our function should be declared
        let has_func = methods.iter().any(|m| {
            m.get("name").and_then(serde_json::Value::as_str) == Some(func_name.as_str())
        });
        prop_assert!(has_func, "function '{}' missing from manifest methods", func_name);

        // Event should be declared
        let has_event = events.iter().any(|e| {
            e.get("name").and_then(serde_json::Value::as_str) == Some("ValueChanged")
        });
        prop_assert!(has_event, "event 'ValueChanged' missing from manifest events");

        // Each method has consistent shape
        for method in methods {
            prop_assert!(method.get("name").is_some(), "method missing name");
            prop_assert!(method.get("parameters").is_some(), "method missing parameters");
            prop_assert!(method.get("returntype").is_some(), "method missing returntype");
            if let Some(offset) = method.get("offset") {
                prop_assert!(offset.is_number(), "method offset must be a number");
            }
        }

        // Each event has consistent shape
        for event in events {
            prop_assert!(event.get("name").is_some(), "event missing name");
            prop_assert!(event.get("parameters").is_some(), "event missing parameters");
        }

        // JSON round-trip
        let as_string = serde_json::to_string(manifest).expect("manifest must serialize");
        let reparsed: serde_json::Value = serde_json::from_str(&as_string)
            .expect("manifest must reparse");
        prop_assert_eq!(&reparsed, manifest, "manifest JSON round-trip was not lossless");
    }

    // ------------------------------------------------------------------
    // 9. Determinism — same source compiled twice produces identical output
    // ------------------------------------------------------------------
    #[test]
    fn optimizer_determinism(
        var_name in identifier_strategy(),
        opt_level in 0u8..=3u8,
    ) {
        let source = format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract TestContract {{
    uint256 public {} = 123;
    function compute(uint256 a, uint256 b) public pure returns (uint256) {{
        uint256 c = a + b;
        if (c > 100) {{
            c = c * 2;
        }}
        return c;
    }}
}}"#,
            var_name
        );

        let result1 = compile_contracts(&source, false, opt_level);
        let result2 = compile_contracts(&source, false, opt_level);

        prop_assert!(result1.is_ok(), "first compile failed: {:?}", result1.err());
        prop_assert!(result2.is_ok(), "second compile failed: {:?}", result2.err());

        let arts1 = result1.unwrap();
        let arts2 = result2.unwrap();
        prop_assert_eq!(arts1.len(), arts2.len(), "artifact count mismatch");

        for (a1, a2) in arts1.iter().zip(arts2.iter()) {
            prop_assert_eq!(
                &a1.bytecode, &a2.bytecode,
                "bytecode differed for opt_level={}", opt_level
            );
            prop_assert_eq!(
                &a1.manifest, &a2.manifest,
                "manifest differed for opt_level={}", opt_level
            );
            prop_assert_eq!(
                a1.tokens.len(), a2.tokens.len(),
                "token count differed for opt_level={}", opt_level
            );
            for (t1, t2) in a1.tokens.iter().zip(a2.tokens.iter()) {
                prop_assert_eq!(t1.hash, t2.hash, "token hash mismatch");
                prop_assert_eq!(&t1.method, &t2.method, "token method mismatch");
                prop_assert_eq!(
                    t1.parameters_count, t2.parameters_count,
                    "token parameters_count mismatch"
                );
                prop_assert_eq!(
                    t1.has_return_value, t2.has_return_value,
                    "token has_return_value mismatch"
                );
                prop_assert_eq!(t1.call_flags, t2.call_flags, "token call_flags mismatch");
            }
        }
    }
}
