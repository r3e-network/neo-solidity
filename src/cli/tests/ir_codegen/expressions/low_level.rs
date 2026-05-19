#[test]
fn low_level_call_serializes_exception_into_return_data() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract LowLevelCallHarness {
        function run(address target) public view returns (bool ok, bytes memory data) {
            (bool success, bytes memory ret) = target.staticcall(abi.encodeWithSignature("foo()"));
            return (success, ret);
        }
    }
    "#;

    let metadata = analyse_source(source).expect("analysis failed");
    let module = ir::Module::from_contract(&metadata).expect("IR lowering failed");
    let run_fn = module
        .functions
        .iter()
        .find(|function| function.name == "run")
        .expect("expected run function");

    let instrs = &run_fn.basic_blocks[0].instructions;
    let catch_label = instrs
        .iter()
        .find_map(|instr| match instr {
            ir::Instruction::Try { catch_target } => Some(*catch_target),
            _ => None,
        })
        .expect("expected low-level call lowering to emit TRY");
    let catch_index = instrs
        .iter()
        .position(|instr| matches!(instr, ir::Instruction::Label(id) if *id == catch_label))
        .expect("expected low-level call catch label");
    let catch_end = instrs[catch_index + 1..]
        .iter()
        .position(|instr| matches!(instr, ir::Instruction::EndTry { .. }))
        .map(|offset| catch_index + 1 + offset)
        .expect("expected low-level call catch block to end with ENDTRY");

    let serialize_calls: Vec<_> = instrs[catch_index + 1..catch_end]
        .iter()
        .filter(|instr| {
            matches!(
                instr,
                ir::Instruction::CallBuiltin {
                    builtin: ir::BuiltinCall::NativeCall { contract, method },
                    arg_count: 1,
                } if *contract == ir::NativeContract::StdLib && method == "serialize"
            )
        })
        .collect();

    assert_eq!(
        serialize_calls.len(),
        1,
        "expected low-level call catch handler to serialize exception into return data"
    );
}

#[test]
fn opaque_dynamic_low_level_call_is_rejected_before_ir_codegen() {
    // v0.19.0 changed the contract: opaque `address.call(<bytes>)` no
    // longer aborts compilation. The compiler emits a compile-time warning
    // explaining the constraint and lowers the call site to a runtime
    // `ABORTMSG` trap, so the well-formed parts of the contract still
    // deploy and only the specific opaque-call path fails if reached.
    let source = r#"
    pragma solidity ^0.8.19;

    contract OpaqueCallHarness {
        function run(address target, bytes memory data) public returns (bool ok, bytes memory ret) {
            return target.call(data);
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2)
        .expect("opaque low-level call should compile with warning + runtime trap (v0.19.0)");
    let warnings: Vec<String> = artifacts
        .iter()
        .flat_map(|a| a.warnings.iter().map(|w| w.message.clone()))
        .collect();
    let combined = warnings.join("\n").to_lowercase();
    assert!(
        combined.contains("opaque") && combined.contains("runtime trap"),
        "expected opaque-call warning surface; got warnings: {warnings:?}"
    );
    assert!(
        artifacts.iter().any(|a| a.bytecode.contains(&0xE0)),
        "expected ABORTMSG (0xE0) at the opaque-call site"
    );
}
