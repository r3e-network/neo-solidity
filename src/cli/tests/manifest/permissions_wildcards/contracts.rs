#[test]
fn self_contract_calls_do_not_require_wildcard_permissions() {
    let source = r#"
    pragma solidity ^0.8.20;

    contract SelfCallHarness {
        function dispatch(string calldata methodName) public {
            bytes memory params = abi.encode(uint256(1));
            // Self-calls are allowed on Neo N3 and should not force wildcard permissions.
            Syscalls.contractCall(address(this), methodName, params);
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
        permissions
            .iter()
            .all(|entry| entry["contract"] != Value::String("*".into())),
        "self contract calls should not require wildcard permissions"
    );
}

#[test]
fn syscalls_contract_call_dynamic_method_restricts_to_contract() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract CallsNativeViaContractCall {
        function callGas(address /*ignored*/, string memory method) public returns (bytes memory) {
            return Syscalls.contractCall(
                NativeCalls.GAS_CONTRACT,
                method,
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

    assert!(
        permissions
            .iter()
            .any(|entry| entry["contract"] == gas_contract && entry["methods"] == "*"),
        "expected GAS wildcard methods (methods='*') when the method name is dynamic"
    );

    assert!(
        permissions
            .iter()
            .all(|entry| entry["contract"] != Value::String("*".into())),
        "dynamic method name alone should not force wildcard contract permissions"
    );
}

#[test]
fn external_contract_call_dynamic_target_restricts_to_method() {
    let source = r#"
    pragma solidity ^0.8.19;

    interface IFace {
        function ping(uint256 value) external returns (uint256);
    }

    contract ExternalCallPermissionHarness {
        function callPing(address target, uint256 value) public returns (uint256) {
            return IFace(target).ping(value);
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
        permissions.iter().any(|entry| {
            entry["contract"] == Value::String("*".into())
                && entry["methods"]
                    .as_array()
                    .is_some_and(|methods| methods.iter().any(|m| m == "ping"))
        }),
        "expected wildcard contract permission restricted to ping() method"
    );

    assert!(
        permissions
            .iter()
            .all(|entry| !(entry["contract"] == Value::String("*".into())
                && entry["methods"] == "*")),
        "expected no full wildcard permissions for external call with static method name"
    );
}

#[test]
fn deny_wildcard_contracts_rejects_dynamic_target_permissions() {
    let source = r#"
    pragma solidity ^0.8.19;

    interface IFace {
        function ping(uint256 value) external returns (uint256);
    }

    contract ExternalCallPermissionHarness {
        function callPing(address target, uint256 value) public returns (uint256) {
            return IFace(target).ping(value);
        }
    }
    "#;

    let err = compile_contracts_with_options(
        source,
        false,
        CompileOptions {
            optimizer_level: 2,
            use_callt: false,
            deny_wildcard_permissions: false,
            deny_wildcard_contracts: true,
            deny_wildcard_methods: false,
            manifest_permissions: None,
        },
    )
    .expect_err("expected manifest wildcard contract error");

    match err {
        CompileError::Manifest(message) => {
            assert!(
                message
                    .to_ascii_lowercase()
                    .contains("wildcard contract manifest permissions"),
                "unexpected error message: {message}"
            );
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn manifest_permissions_replace_wildcards_allows_deny_wildcard_contracts() {
    let source = r#"
    pragma solidity ^0.8.19;

    interface IFace {
        function ping(uint256 value) external returns (uint256);
    }

    contract ExternalCallPermissionHarness {
        function callPing(address target, uint256 value) public returns (uint256) {
            return IFace(target).ping(value);
        }
    }
    "#;

    let temp = tempdir().expect("tempdir");
    let permissions_path = temp.path().join("permissions.json");
    std::fs::write(
        &permissions_path,
        r#"[{"contract":"0x0102030405060708090a0b0c0d0e0f1011121314","methods":["ping"]}]"#,
    )
    .expect("write permissions");

    let override_permissions = load_manifest_permissions_override(
        permissions_path.to_str().expect("permissions path"),
        "replace-wildcards",
    )
    .expect("permissions override parse");

    let artifacts = compile_contracts_with_options(
        source,
        false,
        CompileOptions {
            optimizer_level: 2,
            use_callt: false,
            deny_wildcard_permissions: false,
            deny_wildcard_contracts: true,
            deny_wildcard_methods: false,
            manifest_permissions: Some(override_permissions),
        },
    )
    .expect("expected compilation to succeed with wildcard replacement");
    assert_eq!(artifacts.len(), 1);

    let permissions = artifacts[0].manifest["permissions"]
        .as_array()
        .expect("permissions array");

    assert!(
        permissions
            .iter()
            .all(|entry| entry["contract"] != Value::String("*".into())),
        "expected wildcard contract permissions to be replaced"
    );

    assert!(
        permissions.iter().any(|entry| {
            entry["contract"] == Value::String("0x0102030405060708090a0b0c0d0e0f1011121314".into())
                && entry["methods"]
                    .as_array()
                    .is_some_and(|methods| methods.iter().any(|m| m == "ping"))
        }),
        "expected explicit allowlist entry for the target contract"
    );
}

#[test]
fn address_constants_in_contract_calls_infer_explicit_contract_permissions() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract ConstantTargetCallHarness {
        address constant TARGET = 0xd2a4cff31913016155e38e474a2c06d08be276cf;

        function callSupply() public view returns (bytes memory) {
            return Syscalls.contractCall(TARGET, "totalSupply", abi.encode());
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    assert_eq!(artifacts.len(), 1);

    let permissions = artifacts[0].manifest["permissions"]
        .as_array()
        .expect("permissions array");

    assert!(
        permissions.iter().any(|entry| {
            entry["contract"] == Value::String("0xd2a4cff31913016155e38e474a2c06d08be276cf".into())
                && entry["methods"]
                    .as_array()
                    .is_some_and(|methods| methods.iter().any(|m| m == "totalSupply"))
        }),
        "expected explicit GAS contract permission for totalSupply"
    );

    assert!(
        permissions
            .iter()
            .all(|entry| entry["contract"] != Value::String("*".into())),
        "address constants should not degrade into wildcard contract permissions"
    );
}
