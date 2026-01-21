#[test]
fn devpack_nep17_standard_compiles() {
    let source = [
        include_str!("../../../../devpack/contracts/Syscalls.sol"),
        include_str!("../../../../devpack/contracts/NativeCalls.sol"),
        include_str!("../../../../devpack/libraries/Storage.sol"),
        include_str!("../../../../devpack/libraries/Runtime.sol"),
        include_str!("../../../../devpack/libraries/Neo.sol"),
        include_str!("../../../../devpack/contracts/FrameworkBase.sol"),
        include_str!("../../../../devpack/standards/NEP17.sol"),
    ]
    .join("\n");

    let artifacts = compile_contracts(&source, false, 2).expect("devpack NEP17 compilation failed");
    let names: Vec<_> = artifacts.iter().map(|a| a.metadata.name.as_str()).collect();
    assert_eq!(artifacts.len(), 2, "expected FrameworkBase + NEP17 outputs");
    assert!(names.contains(&"FrameworkBase"));
    assert!(names.contains(&"NEP17"));

    let nep17 = artifacts
        .iter()
        .find(|a| a.metadata.name == "NEP17")
        .expect("NEP17 artifact");
    let permissions = nep17.manifest["permissions"]
        .as_array()
        .expect("permissions array");
    assert!(
        permissions.iter().all(|entry| {
            !(entry["contract"] == Value::String("*".into())
                && entry["methods"] == Value::String("*".into()))
        }),
        "NEP17 should not require full wildcard permissions"
    );
}

#[test]
fn devpack_nep11_standard_compiles() {
    let source = [
        include_str!("../../../../devpack/contracts/Syscalls.sol"),
        include_str!("../../../../devpack/contracts/NativeCalls.sol"),
        include_str!("../../../../devpack/libraries/Storage.sol"),
        include_str!("../../../../devpack/libraries/Runtime.sol"),
        include_str!("../../../../devpack/libraries/Neo.sol"),
        include_str!("../../../../devpack/contracts/FrameworkBase.sol"),
        include_str!("../../../../devpack/standards/NEP11.sol"),
    ]
    .join("\n");

    let artifacts = compile_contracts(&source, false, 2).expect("devpack NEP11 compilation failed");
    let names: Vec<_> = artifacts.iter().map(|a| a.metadata.name.as_str()).collect();
    assert_eq!(artifacts.len(), 2, "expected FrameworkBase + NEP11 outputs");
    assert!(names.contains(&"FrameworkBase"));
    assert!(names.contains(&"NEP11"));

    let nep11 = artifacts
        .iter()
        .find(|a| a.metadata.name == "NEP11")
        .expect("NEP11 artifact");
    let permissions = nep11.manifest["permissions"]
        .as_array()
        .expect("permissions array");
    assert!(
        permissions.iter().all(|entry| {
            !(entry["contract"] == Value::String("*".into())
                && entry["methods"] == Value::String("*".into()))
        }),
        "NEP11 should not require full wildcard permissions"
    );
}

#[test]
fn devpack_nep24_standard_compiles() {
    let source = [
        include_str!("../../../../devpack/contracts/Syscalls.sol"),
        include_str!("../../../../devpack/contracts/NativeCalls.sol"),
        include_str!("../../../../devpack/libraries/Storage.sol"),
        include_str!("../../../../devpack/libraries/Runtime.sol"),
        include_str!("../../../../devpack/libraries/Neo.sol"),
        include_str!("../../../../devpack/contracts/FrameworkBase.sol"),
        include_str!("../../../../devpack/standards/NEP24.sol"),
    ]
    .join("\n");

    let artifacts = compile_contracts(&source, false, 2).expect("devpack NEP24 compilation failed");
    let names: Vec<_> = artifacts.iter().map(|a| a.metadata.name.as_str()).collect();
    assert_eq!(artifacts.len(), 2, "expected FrameworkBase + NEP24 outputs");
    assert!(names.contains(&"FrameworkBase"));
    assert!(names.contains(&"NEP24Royalty"));

    let nep24 = artifacts
        .iter()
        .find(|a| a.metadata.name == "NEP24Royalty")
        .expect("NEP24Royalty artifact");
    let permissions = nep24.manifest["permissions"]
        .as_array()
        .expect("permissions array");
    assert!(
        permissions.iter().all(|entry| {
            !(entry["contract"] == Value::String("*".into())
                && entry["methods"] == Value::String("*".into()))
        }),
        "NEP24Royalty should not require full wildcard permissions"
    );
}
