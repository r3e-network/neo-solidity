#[test]
fn syscalls_script_hash_to_address_keeps_precise_contract_permissions() {
    let source = r#"
    pragma solidity ^0.8.20;

    interface ICallee {
        function foo(uint256 x) external view returns (uint256);
    }

    contract UsesScriptHashConversions {
        bytes20 constant CALLEE_HASH_LE = hex"14131211100f0e0d0c0b0a090807060504030201";

        function run(uint256 x) public view returns (uint256) {
            return ICallee(Syscalls.scriptHashToAddress(CALLEE_HASH_LE)).foo(x);
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let manifest = &artifacts[0].manifest;
    let permissions = manifest["permissions"]
        .as_array()
        .expect("permissions array");

    assert!(
        !permissions.is_empty(),
        "expected permissions in manifest: {permissions:?}"
    );
}

#[test]
fn syscalls_contract_call_static_target_emits_exact_permissions() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract CallsNativeViaContractCall {
        function callGasSupply() public returns (bytes memory) {
            return Syscalls.contractCall(
                NativeCalls.GAS_CONTRACT,
                "totalSupply",
                abi.encode()
            );
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let manifest = &artifacts[0].manifest;
    let permissions = manifest["permissions"]
        .as_array()
        .expect("permissions array");

    let gas_hash_le = super::bytecode::native_contract_hash(neo_solidity::ir::NativeContract::Gas);
    let gas_hash_be: Vec<u8> = gas_hash_le.iter().rev().copied().collect();
    let gas_contract = format!("0x{}", hex::encode(gas_hash_be));

    let stdlib_hash_le =
        super::bytecode::native_contract_hash(neo_solidity::ir::NativeContract::StdLib);
    let stdlib_hash_be: Vec<u8> = stdlib_hash_le.iter().rev().copied().collect();
    let stdlib_contract = format!("0x{}", hex::encode(stdlib_hash_be));

    assert!(
        permissions.iter().any(|entry| {
            entry["contract"] == gas_contract
                && entry["methods"]
                    .as_array()
                    .is_some_and(|methods| methods.iter().any(|m| m == "totalSupply"))
        }),
        "expected GAS.totalSupply permission in manifest"
    );

    assert!(
        permissions.iter().any(|entry| {
            entry["contract"] == stdlib_contract
                && entry["methods"].as_array().is_some_and(|methods| {
                    methods.iter().any(|m| m == "serialize")
                        && methods.iter().any(|m| m == "deserialize")
                })
        }),
        "expected StdLib.serialize + StdLib.deserialize permissions for Syscalls.contractCall"
    );

    assert!(
        permissions
            .iter()
            .all(|entry| entry["contract"] != Value::String("*".into())),
        "static Syscalls.contractCall should not require wildcard permissions"
    );
}

#[test]
fn external_contract_call_static_target_emits_exact_permissions() {
    let source = r#"
    pragma solidity ^0.8.19;

    interface IFace {
        function ping(uint256 value) external returns (uint256);
    }

    contract ExternalCallPermissionHarness {
        function callPing(uint256 value) public returns (uint256) {
            return IFace(0x0102030405060708090a0b0c0d0e0f1011121314).ping(value);
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let manifest = &artifacts[0].manifest;
    let permissions = manifest["permissions"]
        .as_array()
        .expect("permissions array");

    let contract = "0x0102030405060708090a0b0c0d0e0f1011121314";

    assert!(
        permissions.iter().any(|entry| {
            entry["contract"] == Value::String(contract.into())
                && entry["methods"]
                    .as_array()
                    .is_some_and(|methods| methods.iter().any(|m| m == "ping"))
        }),
        "expected exact contract permission for static target address"
    );

    assert!(
        permissions
            .iter()
            .all(|entry| entry["contract"] != Value::String("*".into())),
        "static target should not require wildcard contract permissions"
    );
}

#[test]
fn native_contracts_member_call_emits_exact_native_permission() {
    let source = [
        include_str!("../../../../devpack/contracts/NativeContracts.sol"),
        r#"
        pragma solidity ^0.8.19;

        contract NativeContractsPermissionHarness {
            function callGasSupply() public view returns (uint256) {
                return NativeContracts.GAS_CONTRACT.totalSupply();
            }
        }
        "#,
    ]
    .join("\n");

    let artifacts = compile_contracts(&source, false, 2).expect("compilation failed");
    let harness = artifacts
        .iter()
        .find(|artifact| artifact.metadata.name == "NativeContractsPermissionHarness")
        .expect("expected NativeContractsPermissionHarness artifact");

    let permissions = harness.manifest["permissions"]
        .as_array()
        .expect("permissions array");

    let gas_hash_le = super::bytecode::native_contract_hash(neo_solidity::ir::NativeContract::Gas);
    let gas_hash_be: Vec<u8> = gas_hash_le.iter().rev().copied().collect();
    let gas_contract = format!("0x{}", hex::encode(gas_hash_be));

    assert!(
        permissions.iter().any(|entry| {
            entry["contract"] == gas_contract
                && entry["methods"]
                    .as_array()
                    .is_some_and(|methods| methods.iter().any(|m| m == "totalSupply"))
        }),
        "expected GAS.totalSupply permission in manifest for NativeContracts member call"
    );

    assert!(
        permissions
            .iter()
            .all(|entry| entry["contract"] != Value::String("*".into())),
        "NativeContracts member call should not require wildcard contract permissions"
    );
}
