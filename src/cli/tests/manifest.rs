use crate::cli::tests::standard_json::parse_manifest;
use serde_json::Value;

#[test]
fn manifest_methods_have_offsets_and_safe_flag() {
    let temp = tempfile::tempdir().expect("tempdir");
    let output_prefix = temp.path().join("TestContract");

    std::process::Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--",
            "examples/TestContract.sol",
            "-o",
            output_prefix.to_str().unwrap(),
        ])
        .status()
        .expect("cargo run should succeed");

    let manifest_path = output_prefix.with_extension("manifest.json");
    assert!(manifest_path.exists(), "manifest must be generated");
    let manifest = parse_manifest(manifest_path.to_str().unwrap());

    let methods = manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    assert!(
        methods
            .iter()
            .all(|m| m.get("offset").and_then(Value::as_u64).is_some()),
        "all methods should have numeric offsets"
    );
    for method in methods {
        let name = method
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let safe = method.get("safe").and_then(Value::as_bool).unwrap_or(false);
        if name == "getValue" {
            assert!(safe, "view getter should be marked safe");
        }
    }
}

#[test]
fn manifest_supported_standards_match_detection() {
    let temp = tempfile::tempdir().expect("tempdir");
    let output_prefix = temp.path().join("ERC20Token");

    std::process::Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--",
            "examples/ERC20Token.sol",
            "-o",
            output_prefix.to_str().unwrap(),
        ])
        .status()
        .expect("cargo run should succeed");

    let manifest_path = output_prefix.with_extension("manifest.json");
    assert!(manifest_path.exists(), "manifest must be generated");
    let manifest = parse_manifest(manifest_path.to_str().unwrap());

    let standards = manifest["supportedstandards"]
        .as_array()
        .expect("supportedstandards array");
    assert!(
        standards.iter().any(|s| s.as_str() == Some("NEP-17")),
        "ERC20-like token should advertise NEP-17"
    );
}

#[test]
fn erc721_manifest_advertises_nep11() {
    let temp = tempfile::tempdir().expect("tempdir");
    let output_prefix = temp.path().join("ERC721Token");

    std::process::Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--",
            "examples/ERC721Token.sol",
            "-o",
            output_prefix.to_str().unwrap(),
        ])
        .status()
        .expect("cargo run should succeed");

    let manifest_path = output_prefix.with_extension("manifest.json");
    assert!(manifest_path.exists(), "manifest must be generated");
    let manifest = parse_manifest(manifest_path.to_str().unwrap());

    let standards = manifest["supportedstandards"]
        .as_array()
        .expect("supportedstandards array");
    assert!(
        standards.iter().any(|s| s.as_str() == Some("NEP-11")),
        "ERC721-like token should advertise NEP-11"
    );

    let methods = manifest["abi"]["methods"]
        .as_array()
        .expect("methods array");
    let requires = ["ownerOf", "transferFrom"];
    for required in requires {
        assert!(
            methods
                .iter()
                .any(|m| m.get("name").and_then(Value::as_str) == Some(required)),
            "expected method '{}' in manifest",
            required
        );
    }
}
