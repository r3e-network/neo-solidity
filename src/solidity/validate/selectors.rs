fn canonical_param_type(ty: &str) -> String {
    let mut parts = ty.split_whitespace();
    match parts.next() {
        Some("struct" | "enum") => parts.next().unwrap_or_default().to_string(),
        Some(first) => first.to_string(),
        None => String::new(),
    }
}

fn compute_function_selector(name: &str, param_types: &[String]) -> [u8; 4] {
    if name.eq_ignore_ascii_case("constructor") {
        return [0u8; 4];
    }

    let signature = if param_types.is_empty() {
        format!("{name}()")
    } else {
        format!("{name}({})", param_types.join(","))
    };

    let mut hasher = Keccak256::new();
    hasher.update(signature.as_bytes());
    let digest = hasher.finalize();
    let mut selector = [0u8; 4];
    selector.copy_from_slice(&digest[..4]);
    selector
}

