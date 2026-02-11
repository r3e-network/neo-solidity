#[test]
fn try_catch_lowers_to_try_and_endtry_in_ir() {
    let source = r#"
    pragma solidity ^0.8.19;

    interface IFoo {
        function foo() external;
    }

    contract TryHarness {
        function run(address target) public {
            try IFoo(target).foo() {
                // success
            } catch {
                // catch
            }
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

    let tries: Vec<_> = instrs
        .iter()
        .filter_map(|instr| match instr {
            ir::Instruction::Try { catch_target } => Some(*catch_target),
            _ => None,
        })
        .collect();
    assert_eq!(tries.len(), 1, "expected exactly one TRY in lowered IR");

    let endtrys: Vec<_> = instrs
        .iter()
        .filter_map(|instr| match instr {
            ir::Instruction::EndTry { target } => Some(*target),
            _ => None,
        })
        .collect();
    assert_eq!(
        endtrys.len(),
        2,
        "expected success+catch ENDTRY in lowered IR"
    );

    let catch_label = tries[0];
    let catch_index = instrs
        .iter()
        .position(|instr| matches!(instr, ir::Instruction::Label(id) if *id == catch_label))
        .expect("catch label should exist");

    assert!(
        matches!(instrs.get(catch_index + 1), Some(ir::Instruction::Drop(_))),
        "expected simple catch handler to drop NeoVM exception value"
    );
}

#[test]
fn try_catch_multiple_clauses_emit_runtime_type_guards() {
    let source = r#"
    pragma solidity ^0.8.19;

    interface IFoo {
        function foo() external;
    }

    contract TryHarness {
        function run(address target) public {
            try IFoo(target).foo() {
                // success
            } catch Panic(uint256 code) {
                code;
            } catch (bytes memory lowLevelData) {
                lowLevelData;
            } catch {
                // fallback
            }
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

    assert!(
        instrs.iter().any(|instr| {
            matches!(
                instr,
                ir::Instruction::IsType {
                    target: ir::ConvertTarget::Integer
                }
            )
        }),
        "expected catch Panic(uint256) to emit an integer runtime type guard"
    );

    assert!(
        instrs.iter().any(|instr| {
            matches!(
                instr,
                ir::Instruction::IsType {
                    target: ir::ConvertTarget::ByteArray
                }
            )
        }),
        "expected catch(bytes) to emit a byte-array runtime type guard"
    );
}
