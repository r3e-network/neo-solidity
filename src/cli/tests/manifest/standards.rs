#[test]
fn manifest_supported_standards_match_detection() {
    let source = r#"
    pragma solidity ^0.8.19;

    contract ERC20Like {
        mapping(address => uint256) public balances;

        function symbol() public pure returns (string memory) { return "TKN"; }
        function decimals() public pure returns (uint8) { return 8; }
        function totalSupply() public view returns (uint256) { return 0; }
        function balanceOf(address account) public view returns (uint256) { return balances[account]; }

        function transfer(address from, address to, uint256 amount) public returns (bool) {
            balances[from] -= amount;
            balances[to] += amount;
            return true;
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    let manifest = &artifacts[0].manifest;

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
    let source = r#"
    pragma solidity ^0.8.19;

    contract ERC721Like {
        mapping(uint256 => address) private owners;
        mapping(address => uint256) private balances;

        function balanceOf(address owner) public view returns (uint256) { return balances[owner]; }
        function ownerOf(uint256 tokenId) public view returns (address) { return owners[tokenId]; }

        function transferFrom(address from, address to, uint256 tokenId) public {
            owners[tokenId] = to;
            balances[from] -= 1;
            balances[to] += 1;
        }
    }
    "#;

    let artifacts = compile_contracts(source, false, 2).expect("compilation failed");
    let manifest = &artifacts[0].manifest;

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
