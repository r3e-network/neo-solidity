//! Neo N3 Manifest Generation Module
//!
//! Generates the contract manifest required for Neo N3 deployment. The manifest
//! describes the contract's ABI, permissions, and supported standards.
//!
//! # Manifest Contents
//!
//! - Contract name and supported standards (NEP-17, NEP-11, etc.)
//! - Method signatures with parameter types and return types
//! - Event definitions
//! - Required permissions for cross-contract calls
//! - Storage and payable feature flags

use super::{standard_json, COMPILER_EMAIL, COMPILER_ID, VERSION};
use neo_solidity::solidity::{ContractMetadata, FunctionKind, FunctionMetadata, StateMutability};
use serde_json::json;
use std::collections::HashSet;

pub(crate) fn build_manifest(metadata: &ContractMetadata) -> serde_json::Value {
    let payable = metadata
        .methods
        .iter()
        .any(|method| matches!(method.state_mutability, StateMutability::Payable));

    let mut features = serde_json::Map::new();
    features.insert("storage".to_string(), json!(metadata.uses_storage));
    features.insert("payable".to_string(), json!(payable));

    let methods_json: Vec<_> = metadata
        .methods
        .iter()
        .filter(|method| !matches!(method.kind, FunctionKind::Constructor))
        .map(|method| {
            let params_json: Vec<_> = method
                .parameters
                .iter()
                .enumerate()
                .map(|(param_index, param)| {
                    json!({
                        "name": param
                            .name
                            .clone()
                            .unwrap_or_else(|| format!("arg{}", param_index)),
                        "type": standard_json::solidity_to_manifest_type(&param.ty),
                    })
                })
                .collect();

            json!({
                "name": method.name,
                "offset": method.offset,
                "parameters": params_json,
                "returntype": method
                    .return_parameters
                    .first()
                    .map(|param| standard_json::solidity_to_manifest_type(&param.ty))
                    .unwrap_or("Void"),
                "safe": method.state_mutability.is_safe(),
            })
        })
        .collect();

    let events_json: Vec<_> = metadata
        .events
        .iter()
        .map(|event| {
            let params: Vec<_> = event
                .parameters
                .iter()
                .enumerate()
                .map(|(idx, param)| {
                    json!({
                        "name": param
                            .name
                            .clone()
                            .unwrap_or_else(|| format!("param{}", idx)),
                        "type": standard_json::solidity_to_manifest_type(&param.ty),
                        "indexed": param.indexed,
                    })
                })
                .collect();

            json!({
                "name": event.name,
                "parameters": params,
            })
        })
        .collect();

    let supported_standards = detect_supported_standards(&metadata.methods);
    let permissions = infer_permissions(metadata);

    json!({
        "name": metadata.name,
        "groups": [],
        "features": features,
        "supportedstandards": supported_standards,
        "abi": {
            "methods": methods_json,
            "events": events_json,
        },
        "permissions": permissions,
        "trusts": [],
        "extra": {
            "Author": COMPILER_EMAIL,
            "Description": format!("Solidity contract '{}' compiled to NeoVM", metadata.name),
            "Version": format!("{}.{}.{}.{}", VERSION.0, VERSION.1, VERSION.2, VERSION.3),
            "Compiler": COMPILER_ID,
        }
    })
}

pub(crate) fn detect_supported_standards(methods: &[FunctionMetadata]) -> Vec<String> {
    let names: HashSet<String> = methods
        .iter()
        .filter(|m| !matches!(m.kind, FunctionKind::Constructor))
        .map(|m| m.name.to_ascii_lowercase())
        .collect();
    let mut standards = Vec::new();

    // NEP-17: Fungible Token Standard (equivalent to ERC-20)
    // Required: symbol, decimals, totalSupply, balanceOf, transfer
    let nep17_required = ["symbol", "decimals", "totalsupply", "balanceof", "transfer"];
    if nep17_required.iter().all(|m| names.contains(*m)) {
        standards.push("NEP-17".to_string());
    }

    // NEP-11: Non-Fungible Token Standard (equivalent to ERC-721)
    // Core: balanceOf, ownerOf + at least one of: transfer, transferFrom, tokensOf
    let nep11_core = ["balanceof", "ownerof"];
    let nep11_transfer = ["transfer", "transferfrom", "tokensof"];
    if nep11_core.iter().all(|m| names.contains(*m))
        && nep11_transfer.iter().any(|m| names.contains(*m))
    {
        standards.push("NEP-11".to_string());
    }

    // NEP-24: Token Discovery Standard
    // For contracts that implement royalty info, token URIs, etc.
    if names.contains("tokenuri") || names.contains("royaltyinfo") {
        standards.push("NEP-24".to_string());
    }

    // NEP-26: Contract Upgrade Standard
    if names.contains("update") && names.contains("destroy") {
        standards.push("NEP-26".to_string());
    }

    // NEP-27: Invoke File Standard (for dynamic invocation)
    if names.contains("onpayment") || names.contains("onnep17payment") {
        standards.push("NEP-27".to_string());
    }

    standards
}

/// Infer contract permissions based on method signatures and behavior
pub(crate) fn infer_permissions(metadata: &ContractMetadata) -> Vec<serde_json::Value> {
    use std::collections::BTreeSet;

    let mut required_methods: BTreeSet<String> = BTreeSet::new();
    let mut requires_external = false;
    let mut targets: BTreeSet<String> = BTreeSet::new();
    let mut syscall_targets: BTreeSet<String> = BTreeSet::new();

    // Known native contract hashes (Policy, NEO) as hex strings to narrow permissions.
    const POLICY_HASH: &str = "0xfffdc93764dbaddd97c48f252a53ea4643faa3fd";
    const NEO_HASH: &str = "0xef405b1609b5f2935ec4efcd1991d9f79b74c317";
    const STORAGE_HASH: &str = "0x290bdded77b84db5b8b9f66e0d2791bf0e8d3b42";
    const CRYPTO_HASH: &str = "0x3c05b488bf4cf699d44e6d89a6ff7c5b2d1576c9";
    const RUNTIME_HASH: &str = "0xd2a4cff31913016155e38e474a2c06d08be276cf";

    for method in &metadata.methods {
        let method_lower = method.name.to_ascii_lowercase();

        // Token operations typically need to call other contracts
        if method_lower.contains("transfer") || method_lower.contains("approve") {
            required_methods.insert("transfer".to_string());
            required_methods.insert("balanceOf".to_string());
            requires_external = true;
            targets.insert("*".to_string());
        }

        // If contract has onPayment handlers, it receives tokens
        if method_lower.contains("onpayment") || method_lower.contains("onnep") {
            required_methods.insert("*".to_string());
            requires_external = true;
            targets.insert("*".to_string());
        }

        // Emergency recovery functions need broad permissions
        if method_lower.contains("emergency") || method_lower.contains("recovery") {
            required_methods.insert("*".to_string());
            requires_external = true;
            targets.insert("*".to_string());
        }

        // Best-effort detection of common native contracts by name hints
        if method_lower.contains("gas") || method_lower.contains("policy") {
            targets.insert(POLICY_HASH.to_string());
            requires_external = true;
            required_methods.insert("*".to_string());
        }
        if method_lower.contains("neo") || method_lower.contains("neo_") {
            targets.insert(NEO_HASH.to_string());
            requires_external = true;
            required_methods.insert("*".to_string());
        }

        // Syscall-like method names imply broad runtime/crypto/storage access
        if method_lower.contains("storage") {
            syscall_targets.insert(STORAGE_HASH.to_string());
        }
        if method_lower.contains("sha") || method_lower.contains("crypto") {
            syscall_targets.insert(CRYPTO_HASH.to_string());
        }
        if method_lower.contains("runtime") || method_lower.contains("notify") {
            syscall_targets.insert(RUNTIME_HASH.to_string());
        }
    }

    if required_methods.is_empty() {
        return Vec::new();
    }

    let methods_value = if required_methods.contains("*") {
        json!("*")
    } else {
        json!(required_methods.into_iter().collect::<Vec<_>>())
    };

    let contract_value = if requires_external {
        if targets.is_empty() {
            json!("*")
        } else if targets.len() == 1 && targets.contains("*") {
            json!("*")
        } else {
            json!(targets.into_iter().collect::<Vec<_>>())
        }
    } else {
        json!("self")
    };

    vec![json!({
        "contract": contract_value,
        "methods": methods_value
    })]
    .into_iter()
    .chain(if syscall_targets.is_empty() {
        Vec::new()
    } else {
        vec![json!({
            "contract": syscall_targets.into_iter().collect::<Vec<_>>(),
            "methods": "*"
        })]
    })
    .collect()
}

pub(crate) fn contract_output_prefix(
    base: &str,
    contract_name: &str,
    index: usize,
    total: usize,
) -> String {
    if total <= 1 {
        return base.to_string();
    }

    let sanitized = standard_json::sanitize_contract_name(contract_name).unwrap_or_else(|| {
        if total <= 1 {
            "contract".to_string()
        } else {
            format!("contract{index}")
        }
    });

    let (stem, ext) = split_extension(base);
    if ext.is_empty() {
        format!("{stem}-{sanitized}")
    } else {
        format!("{stem}-{sanitized}{ext}")
    }
}

fn split_extension(path: &str) -> (String, String) {
    match path.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => (stem.to_string(), format!(".{ext}")),
        _ => (path.to_string(), String::new()),
    }
}
