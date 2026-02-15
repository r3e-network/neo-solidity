fn string_literal_bytes(parts: &[PtStringLiteral]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for part in parts {
        bytes.extend_from_slice(part.string.as_bytes());
    }
    bytes
}

fn extract_signature_string(expr: &Expression) -> Option<String> {
    match expr {
        Expression::StringLiteral(parts) => {
            Some(String::from_utf8_lossy(&string_literal_bytes(parts)).to_string())
        }
        Expression::FunctionCall(_, func, args) => {
            if args.len() == 1 {
                match func.as_ref() {
                    Expression::Type(_, _) => extract_signature_string(&args[0]),
                    Expression::Variable(id) if id.name == "bytes" || id.name == "string" => {
                        extract_signature_string(&args[0])
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn resolve_selector_method_name(expr: &Expression, ctx: &LoweringContext) -> Option<String> {
    match expr {
        Expression::Variable(identifier) => {
            if let Some(state_index) = ctx.state_index_map.get(&identifier.name).copied() {
                if let Some(meta) = ctx.state_metadata(state_index) {
                    if meta.is_constant {
                        if let Some(initializer) = meta.initializer.as_ref() {
                            return resolve_selector_method_name(initializer, ctx);
                        }
                    }
                }
            }
            None
        }
        Expression::MemberAccess(_, inner, member) => {
            if member.name == "selector" {
                match inner.as_ref() {
                    Expression::MemberAccess(_, _, function_name) => {
                        if !function_name.name.trim().is_empty() {
                            return Some(function_name.name.clone());
                        }
                    }
                    // Custom-error pattern: `ErrorName.selector`
                    Expression::Variable(function_name) => {
                        if !function_name.name.trim().is_empty() {
                            return Some(function_name.name.clone());
                        }
                    }
                    _ => {}
                }
            }
            None
        }
        Expression::FunctionCall(_, func, args) => {
            // Ignore simple casts like bytes4(...) around selector expressions.
            if matches!(func.as_ref(), Expression::Type(_, _)) && args.len() == 1 {
                return resolve_selector_method_name(&args[0], ctx);
            }

            if let Expression::Variable(id) = func.as_ref() {
                if id.name == "keccak256" && args.len() == 1 {
                    if let Some(signature) = extract_signature_string(&args[0]) {
                        let name = signature
                            .split('(')
                            .next()
                            .unwrap_or(signature.as_str())
                            .trim()
                            .to_string();
                        if !name.is_empty() {
                            return Some(name);
                        }
                    }
                }
            }

            None
        }
        _ => None,
    }
}
