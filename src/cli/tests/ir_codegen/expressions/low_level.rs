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
    let serialize_calls: Vec<_> = instrs
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
