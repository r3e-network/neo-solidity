use super::*;

#[test]
fn non_builtin_libraries_are_merged_for_internal_calls() {
    let source = r#"
    pragma solidity ^0.8.19;

    library MathLib {
        function double(uint256 x) internal pure returns (uint256) {
            return x * 2;
        }
    }

    contract UsesLib {
        function f(uint256 x) public pure returns (uint256) {
            return MathLib.double(x);
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);
}

#[test]
fn builtin_libraries_are_not_merged_into_contract_metadata() {
    let source = r#"
    pragma solidity ^0.8.19;

    library Syscalls {
        // Even if a Solidity source provides an implementation, `Syscalls.*` is treated as a
        // compiler intrinsic and should not be merged into the user contract.
        function getTime() internal view returns (uint256) {
            return 123;
        }
    }

    contract UsesSyscalls {
        function nowTs() public view returns (uint256) {
            return Syscalls.getTime();
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let method_names: Vec<_> = artifacts[0]
        .metadata
        .methods
        .iter()
        .map(|m| m.name.as_str())
        .collect();

    assert!(
        method_names.contains(&"nowTs"),
        "expected user method to remain present"
    );
    assert!(
        !method_names.contains(&"getTime"),
        "expected Syscalls.getTime to not be merged into contract metadata"
    );
}
