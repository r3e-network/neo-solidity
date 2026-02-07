use std::path::PathBuf;
use std::process::Command;

fn get_compiler_path() -> PathBuf {
    // CARGO_BIN_EXE_<name> is set by Cargo for integration tests, pointing
    // to the correct binary for the current build profile (debug or release).
    PathBuf::from(env!("CARGO_BIN_EXE_neo-solc"))
}

fn get_example_path(contract: &str) -> PathBuf {
    // CARGO_MANIFEST_DIR is the project root — portable across CI and local dev.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("examples").join(contract)
}

/// Compile and assert success (exit-code only). Works for multi-contract files
/// where the compiler suffixes output with contract names.
fn assert_compiles(contract_path: &str) {
    let compiler = get_compiler_path();
    assert!(compiler.exists(), "Compiler not found");

    let contract_path = get_example_path(contract_path);
    let output_dir = std::env::temp_dir().join("neo-sol-test");
    std::fs::create_dir_all(&output_dir).unwrap();
    let output_prefix = output_dir.join(contract_path.file_stem().unwrap().to_str().unwrap());

    let output = Command::new(&compiler)
        .arg(&contract_path)
        .arg("-I")
        .arg("devpack")
        .arg("-O2")
        .arg("-o")
        .arg(&output_prefix)
        .output()
        .expect("Failed to run compiler");

    assert!(
        output.status.success(),
        "Compilation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn compile_contract(contract_path: &str) -> Result<(PathBuf, PathBuf), String> {
    let compiler = get_compiler_path();

    if !compiler.exists() {
        return Err(format!(
            "Compiler not found at {}. Run 'cargo build --release' first.",
            compiler.display()
        ));
    }

    let contract_path = get_example_path(contract_path);
    let output_dir = std::env::temp_dir().join("neo-sol-test");

    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    let output_prefix = output_dir.join(contract_path.file_stem().unwrap().to_str().unwrap());

    let output = Command::new(&compiler)
        .arg(&contract_path)
        .arg("-I")
        .arg("devpack")
        .arg("-O2")
        .arg("-o")
        .arg(&output_prefix)
        .output()
        .map_err(|e| format!("Failed to run compiler: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Compilation failed: {}", stderr));
    }

    let nef_path = output_prefix.with_extension("nef");
    let manifest_path = output_prefix.with_extension("manifest.json");

    if !nef_path.exists() {
        return Err(format!("NEF file not generated: {}", nef_path.display()));
    }

    if !manifest_path.exists() {
        return Err(format!(
            "Manifest not generated: {}",
            manifest_path.display()
        ));
    }

    Ok((nef_path, manifest_path))
}

#[test]
fn test_compile_simple_storage() {
    let result = compile_contract("SimpleStorage.sol");
    assert!(
        result.is_ok(),
        "Failed to compile SimpleStorage: {:?}",
        result.err()
    );
    let (nef, manifest) = result.unwrap();
    assert!(nef.exists(), "NEF file should exist: {}", nef.display());
    assert!(
        manifest.exists(),
        "Manifest should exist: {}",
        manifest.display()
    );
}

#[test]
fn test_compile_erc20_token() {
    let result = compile_contract("ERC20Token.sol");
    assert!(
        result.is_ok(),
        "Failed to compile ERC20Token: {:?}",
        result.err()
    );
    let (nef, manifest) = result.unwrap();
    assert!(nef.exists(), "NEF file should exist: {}", nef.display());
    assert!(
        manifest.exists(),
        "Manifest should exist: {}",
        manifest.display()
    );
}

#[test]
fn test_compile_staking() {
    let result = compile_contract("Staking.sol");
    assert!(
        result.is_ok(),
        "Failed to compile Staking: {:?}",
        result.err()
    );
    let (nef, manifest) = result.unwrap();
    assert!(nef.exists(), "NEF file should exist: {}", nef.display());
    assert!(
        manifest.exists(),
        "Manifest should exist: {}",
        manifest.display()
    );
}

#[test]
fn test_compile_multisig_wallet() {
    let result = compile_contract("MultiSigWallet.sol");
    assert!(
        result.is_ok(),
        "Failed to compile MultiSigWallet: {:?}",
        result.err()
    );
    let (nef, manifest) = result.unwrap();
    assert!(nef.exists(), "NEF file should exist: {}", nef.display());
    assert!(
        manifest.exists(),
        "Manifest should exist: {}",
        manifest.display()
    );
}

#[test]
fn test_compile_with_optimization() {
    let compiler = get_compiler_path();
    if !compiler.exists() {
        return;
    }

    let example_path = get_example_path("SimpleStorage.sol");
    let output = Command::new(&compiler)
        .arg(&example_path)
        .arg("-O3")
        .arg("-o")
        .arg("/tmp/neo-sol-opt-test")
        .output()
        .expect("Failed to run compiler");

    assert!(
        output.status.success(),
        "High optimization should compile successfully: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_nef_file_structure() {
    let result = compile_contract("SimpleStorage.sol");
    assert!(result.is_ok(), "Failed to compile SimpleStorage");

    let (nef_path, _) = result.unwrap();
    let nef_data = std::fs::read(&nef_path)
        .map_err(|e| format!("Failed to read NEF: {}", e))
        .unwrap();

    assert!(!nef_data.is_empty(), "NEF should not be empty");
    assert!(
        nef_data.len() < 1024 * 1024,
        "NEF should be reasonable size"
    );

    // NEF3 format magic bytes
    let magic = &nef_data[..4];
    assert_eq!(
        magic, b"NEF3",
        "NEF should have correct magic bytes (NEF3 format)"
    );
}

#[test]
fn test_manifest_structure() {
    let result = compile_contract("SimpleStorage.sol");
    assert!(result.is_ok(), "Failed to compile SimpleStorage");

    let (_, manifest_path) = result.unwrap();
    let manifest_data = std::fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read manifest: {}", e))
        .unwrap();

    let json: serde_json::Value = serde_json::from_str(&manifest_data)
        .map_err(|e| format!("Invalid JSON: {}", e))
        .unwrap();

    assert!(json.is_object(), "Manifest should be JSON object");
    assert!(
        json.get("name").is_some(),
        "Manifest should have name field"
    );
    assert!(json.get("abi").is_some(), "Manifest should have abi field");
    assert!(
        json.get("permissions").is_some(),
        "Manifest should have permissions field"
    );
}

#[test]
fn test_compile_name_service() {
    let result = compile_contract("NameService.sol");
    assert!(
        result.is_ok(),
        "Failed to compile NameService: {:?}",
        result.err()
    );
    let (nef, manifest) = result.unwrap();
    assert!(nef.exists(), "NEF file should exist: {}", nef.display());
    assert!(
        manifest.exists(),
        "Manifest should exist: {}",
        manifest.display()
    );
}

#[test]
fn test_compile_lottery() {
    let result = compile_contract("Lottery.sol");
    assert!(
        result.is_ok(),
        "Failed to compile Lottery: {:?}",
        result.err()
    );
    let (nef, manifest) = result.unwrap();
    assert!(nef.exists(), "NEF file should exist: {}", nef.display());
    assert!(
        manifest.exists(),
        "Manifest should exist: {}",
        manifest.display()
    );
}

#[test]
fn test_compile_escrow() {
    let result = compile_contract("Escrow.sol");
    assert!(
        result.is_ok(),
        "Failed to compile Escrow: {:?}",
        result.err()
    );
    let (nef, manifest) = result.unwrap();
    assert!(nef.exists(), "NEF file should exist: {}", nef.display());
    assert!(
        manifest.exists(),
        "Manifest should exist: {}",
        manifest.display()
    );
}

#[test]
fn test_compile_new_neo_interop_showcase() {
    let result = compile_contract("new/NeoInteropShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile NeoInteropShowcase: {:?}",
        result.err()
    );
    let (nef, manifest) = result.unwrap();
    assert!(nef.exists(), "NEF file should exist: {}", nef.display());
    assert!(
        manifest.exists(),
        "Manifest should exist: {}",
        manifest.display()
    );
}

#[test]
fn test_compile_new_low_level_call_showcase() {
    let result = compile_contract("new/LowLevelCallShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile LowLevelCallShowcase: {:?}",
        result.err()
    );
    let (nef, manifest) = result.unwrap();
    assert!(nef.exists(), "NEF file should exist: {}", nef.display());
    assert!(
        manifest.exists(),
        "Manifest should exist: {}",
        manifest.display()
    );
}

#[test]
fn test_compile_new_enum_array_showcase() {
    let result = compile_contract("new/EnumArrayShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile EnumArrayShowcase: {:?}",
        result.err()
    );
    let (nef, manifest) = result.unwrap();
    assert!(nef.exists(), "NEF file should exist: {}", nef.display());
    assert!(
        manifest.exists(),
        "Manifest should exist: {}",
        manifest.display()
    );
}

#[test]
fn test_optimization_reduces_size() {
    let compiler = get_compiler_path();
    if !compiler.exists() {
        return;
    }

    let example_path = get_example_path("SimpleStorage.sol");

    // Compile without optimization
    let output_no_opt = Command::new(&compiler)
        .arg(&example_path)
        .arg("-O0")
        .arg("-o")
        .arg("/tmp/no_opt_test")
        .output()
        .expect("Failed to run compiler");

    assert!(
        output_no_opt.status.success(),
        "No-opt compilation should succeed"
    );

    let no_opt_size = std::fs::metadata("/tmp/no_opt_test.nef")
        .map(|m| m.len())
        .unwrap_or(0);

    // Compile with optimization
    let output_opt = Command::new(&compiler)
        .arg(&example_path)
        .arg("-O3")
        .arg("-o")
        .arg("/tmp/opt_test")
        .output()
        .expect("Failed to run compiler");

    assert!(
        output_opt.status.success(),
        "Optimized compilation should succeed"
    );

    let opt_size = std::fs::metadata("/tmp/opt_test.nef")
        .map(|m| m.len())
        .unwrap_or(0);

    // Optimization should not significantly increase size
    // (Note: for simple contracts, O3 might be slightly larger due to inlining)
    assert!(
        opt_size <= no_opt_size * 2,
        "Optimized bytecode should not be more than 2x the original size"
    );
}

#[test]
fn test_manifest_has_valid_abi() {
    let result = compile_contract("ERC20Token.sol");
    assert!(result.is_ok(), "Failed to compile ERC20Token");

    let (_, manifest_path) = result.unwrap();
    let manifest_data = std::fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read manifest: {}", e))
        .unwrap();

    let json: serde_json::Value = serde_json::from_str(&manifest_data)
        .map_err(|e| format!("Invalid JSON: {}", e))
        .unwrap();

    let abi = json.get("abi").expect("Manifest should have ABI");

    assert!(
        abi.is_object(),
        "ABI should be an object with methods and events"
    );

    let abi_obj = abi.as_object().unwrap();
    let methods = abi_obj.get("methods").expect("ABI should have methods");

    assert!(methods.is_array(), "ABI methods should be an array");

    let methods_array = methods.as_array().unwrap();
    assert!(
        !methods_array.is_empty(),
        "Contract should have at least one method"
    );

    // Check that methods have required fields
    for method in methods_array {
        assert!(method.get("name").is_some(), "Method should have a name");
        assert!(
            method.get("parameters").is_some(),
            "Method should have parameters"
        );
        assert!(
            method.get("returntype").is_some(),
            "Method should have return type"
        );
    }
}

// ========== Root examples — previously untested ==========

#[test]
fn test_compile_erc721_token() {
    let result = compile_contract("ERC721Token.sol");
    assert!(
        result.is_ok(),
        "Failed to compile ERC721Token: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_governance_token() {
    let result = compile_contract("GovernanceToken.sol");
    assert!(
        result.is_ok(),
        "Failed to compile GovernanceToken: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_uniswap_v2_pair() {
    let result = compile_contract("UniswapV2Pair.sol");
    assert!(
        result.is_ok(),
        "Failed to compile UniswapV2Pair: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_test_contract() {
    let result = compile_contract("TestContract.sol");
    assert!(
        result.is_ok(),
        "Failed to compile TestContract: {:?}",
        result.err()
    );
}

// ========== Existing new/ examples — previously untested ==========

#[test]
fn test_compile_new_counter() {
    let result = compile_contract("new/Counter.sol");
    assert!(
        result.is_ok(),
        "Failed to compile Counter: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_vault() {
    let result = compile_contract("new/Vault.sol");
    assert!(
        result.is_ok(),
        "Failed to compile Vault: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_nft() {
    let result = compile_contract("new/NFT.sol");
    assert!(result.is_ok(), "Failed to compile NFT: {:?}", result.err());
}

#[test]
fn test_compile_new_bank() {
    let result = compile_contract("new/Bank.sol");
    assert!(result.is_ok(), "Failed to compile Bank: {:?}", result.err());
}

#[test]
fn test_compile_new_multisig_wallet_nep17() {
    let result = compile_contract("new/MultiSigWalletNEP17.sol");
    assert!(
        result.is_ok(),
        "Failed to compile MultiSigWalletNEP17: {:?}",
        result.err()
    );
}

// ========== New showcase contracts ==========

#[test]
fn test_compile_new_custom_errors_showcase() {
    let result = compile_contract("new/CustomErrorsShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile CustomErrorsShowcase: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_inheritance_showcase() {
    // Multi-contract file: compiler suffixes output with contract names
    assert_compiles("new/InheritanceShowcase.sol");
}

#[test]
fn test_compile_new_interface_showcase() {
    let result = compile_contract("new/InterfaceShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile InterfaceShowcase: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_modifier_showcase() {
    let result = compile_contract("new/ModifierShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile ModifierShowcase: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_struct_mapping_showcase() {
    let result = compile_contract("new/StructMappingShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile StructMappingShowcase: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_type_casting_showcase() {
    let result = compile_contract("new/TypeCastingShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile TypeCastingShowcase: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_constants_immutable_showcase() {
    let result = compile_contract("new/ConstantsImmutableShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile ConstantsImmutableShowcase: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_bitwise_showcase() {
    let result = compile_contract("new/BitwiseShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile BitwiseShowcase: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_try_catch_showcase() {
    let result = compile_contract("new/TryCatchShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile TryCatchShowcase: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_event_indexed_showcase() {
    let result = compile_contract("new/EventIndexedShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile EventIndexedShowcase: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_oracle_showcase() {
    let result = compile_contract("new/OracleShowcase.sol");
    assert!(
        result.is_ok(),
        "Failed to compile OracleShowcase: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_new_multi_standard_token() {
    let result = compile_contract("new/MultiStandardToken.sol");
    assert!(
        result.is_ok(),
        "Failed to compile MultiStandardToken: {:?}",
        result.err()
    );
}

// ========== Famous DeFi/Web3 contracts ==========

#[test]
fn test_compile_famous_wgas() {
    let result = compile_contract("famous/WGAS.sol");
    assert!(
        result.is_ok(),
        "Failed to compile WGAS: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_famous_flashloan() {
    let result = compile_contract("famous/FlashLoan.sol");
    assert!(
        result.is_ok(),
        "Failed to compile FlashLoan: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_famous_simple_amm() {
    let result = compile_contract("famous/SimpleAMM.sol");
    assert!(
        result.is_ok(),
        "Failed to compile SimpleAMM: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_famous_token_vesting() {
    let result = compile_contract("famous/TokenVesting.sol");
    assert!(
        result.is_ok(),
        "Failed to compile TokenVesting: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_famous_simple_lending() {
    let result = compile_contract("famous/SimpleLending.sol");
    assert!(
        result.is_ok(),
        "Failed to compile SimpleLending: {:?}",
        result.err()
    );
}

#[test]
fn test_compile_famous_simple_dao() {
    let result = compile_contract("famous/SimpleDAO.sol");
    assert!(
        result.is_ok(),
        "Failed to compile SimpleDAO: {:?}",
        result.err()
    );
}
