//! IR (Intermediate Representation) unit tests.
//!
//! Tests lowering from Solidity/Yul to IR and IR analysis.

use neo_solidity::ir::Instruction;
use neo_solidity::ir::Module;
use neo_solidity::solidity::analyse_source;

#[cfg(test)]
mod ir_tests {
    #[test]
    fn lowers_single_slot_extsload_assembly() {
        let source = r#"
        contract Extsload {
            function extsload(bytes32 slot) external view returns (bytes32) {
                assembly {
                    mstore(0, sload(slot))
                    return(0, 0x20)
                }
            }
        }
        "#;

        let metadata = analyse_source(source).expect("analysis failed");
        let module = Module::from_contract(&metadata).expect("IR lowering failed");
        let function = module
            .functions
            .iter()
            .find(|f| f.name == "extsload")
            .expect("function not found");
        let mut has_dynamic_sload = false;
        let mut has_return = false;
        for bb in &function.basic_blocks {
            for instr in &bb.instructions {
                match instr {
                    Instruction::LoadStorageDynamic => has_dynamic_sload = true,
                    Instruction::Return => has_return = true,
                    _ => {}
                }
            }
        }
        assert!(has_dynamic_sload, "expected LoadStorageDynamic instruction");
        assert!(has_return, "expected Return instruction");
    }

    #[test]
    fn lowers_range_extsload_assembly() {
        let source = r#"
        contract ExtsloadRange {
            function extsload(bytes32 startSlot, uint256 nSlots) external view returns (bytes32[] memory) {
                assembly {
                    let memptr := mload(0x40)
                    let start := memptr
                    let length := shl(5, nSlots)
                    mstore(memptr, 0x20)
                    mstore(add(memptr, 0x20), nSlots)
                    memptr := add(memptr, 0x40)
                    let end := add(memptr, length)
                    for {} 1 {} {
                        mstore(memptr, sload(startSlot))
                        memptr := add(memptr, 0x20)
                        startSlot := add(startSlot, 1)
                        if iszero(lt(memptr, end)) { break }
                    }
                    return(start, sub(end, start))
                }
            }
        }
        "#;

        let metadata = analyse_source(source).expect("analysis failed");
        let module = Module::from_contract(&metadata).expect("IR lowering failed");
        let function = module
            .functions
            .iter()
            .find(|f| f.name == "extsload")
            .expect("function not found");

        let mut has_array_init = false;
        let mut has_loop = false;

        for bb in &function.basic_blocks {
            for instr in &bb.instructions {
                match instr {
                    Instruction::NewArray { .. } => has_array_init = true,
                    Instruction::Label(_) => has_loop = true,
                    _ => {}
                }
            }
        }

        assert!(has_array_init, "expected NewArray instruction");
        assert!(has_loop, "expected loop labels in instructions");
    }

    #[test]
    fn lowers_array_literal_and_slice() {
        let source = r#"
        contract ArrayOps {
            function literal() external pure returns (uint256[] memory out) {
                out = [1, 2, 3];
            }

            function slice(uint256[] memory input) external pure returns (uint256[] memory) {
                return input[1:3];
            }
        }
        "#;

        let metadata = analyse_source(source).expect("analysis failed");
        let module = Module::from_contract(&metadata).expect("IR lowering failed");

        let literal_fn = module
            .functions
            .iter()
            .find(|f| f.name == "literal")
            .unwrap();
        assert!(
            literal_fn
                .basic_blocks
                .iter()
                .flat_map(|bb| &bb.instructions)
                .any(|i| matches!(i, Instruction::NewArray { .. })),
            "literal should allocate an array"
        );

        let slice_fn = module.functions.iter().find(|f| f.name == "slice").unwrap();
        let mut has_slice_copy = false;
        for instr in slice_fn.basic_blocks.iter().flat_map(|bb| &bb.instructions) {
            if matches!(instr, Instruction::ArrayGet) {
                has_slice_copy = true;
                break;
            }
        }
        assert!(has_slice_copy, "slice lowering should copy elements");
    }

    #[test]
    fn extload_overloads_have_unique_function_entries() {
        let source = r#"
        contract ExtsloadMulti {
            function extsload(bytes32 slot) external view returns (bytes32) {
                assembly {
                    mstore(0, sload(slot))
                    return(0, 0x20)
                }
            }

            function extsload(bytes32 startSlot, uint256 nSlots) external view returns (bytes32[] memory) {
                assembly {
                    let memptr := mload(0x40)
                    let start := memptr
                    let length := shl(5, nSlots)
                    mstore(memptr, 0x20)
                    mstore(add(memptr, 0x20), nSlots)
                    memptr := add(memptr, 0x40)
                    let end := add(memptr, length)
                    for {} 1 {} {
                        mstore(memptr, sload(startSlot))
                        memptr := add(memptr, 0x20)
                        startSlot := add(startSlot, 1)
                        if iszero(lt(memptr, end)) { break }
                    }
                    return(start, sub(end, start))
                }
            }
        }
        "#;

        let metadata = analyse_source(source).expect("analysis failed");
        let module = Module::from_contract(&metadata).expect("IR lowering failed");
        let names: Vec<_> = module.functions.iter().map(|f| f.name.clone()).collect();
        assert!(
            names.iter().any(|name| name == "extsload(bytes32)"),
            "expected overload named 'extsload(bytes32)'"
        );
        assert!(
            names.iter().any(|name| name == "extsload(bytes32,uint256)"),
            "expected overload named 'extsload(bytes32,uint256)'"
        );
    }

    #[test]
    fn extload_multi_parameter_names() {
        let source = r#"
        contract ExtsloadMulti {
            function extsload(bytes32 slot) external view returns (bytes32) {
                assembly {
                    mstore(0, sload(slot))
                    return(0, 0x20)
                }
            }

            function extsload(bytes32 startSlot, uint256 nSlots) external view returns (bytes32[] memory) {
                assembly {
                    let memptr := mload(0x40)
                    let start := memptr
                    let length := shl(5, nSlots)
                    mstore(memptr, 0x20)
                    mstore(add(memptr, 0x20), nSlots)
                    memptr := add(memptr, 0x40)
                    let end := add(memptr, length)
                    for {} 1 {} {
                        mstore(memptr, sload(startSlot))
                        memptr := add(memptr, 0x20)
                        startSlot := add(startSlot, 1)
                        if iszero(lt(memptr, end)) { break }
                    }
                    return(start, sub(end, start))
                }
            }
        }
        "#;

        let metadata = analyse_source(source).expect("analysis failed");
        let range_method = metadata
            .methods
            .iter()
            .find(|m| m.name == "extsload" && m.parameters.len() == 2)
            .expect("range overload not found");

        let names: Vec<_> = range_method
            .parameters
            .iter()
            .map(|p| p.name.clone())
            .collect();
        assert_eq!(names[0].as_deref(), Some("startSlot"));
        assert_eq!(names[1].as_deref(), Some("nSlots"));
    }
}
