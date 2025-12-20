impl NeoType {
    pub fn from_solidity(
        ty: &str,
        structs: &[StructTypeMetadata],
        enums: &[EnumTypeMetadata],
        contract_types: &[String],
    ) -> Result<Self, TypeParseError> {
        let ty = strip_data_location(ty);
        let lower = ty.to_ascii_lowercase();

        // Arrays must be detected before scalar prefixes like `uint`/`int`.
        // Support dynamic arrays (`T[]`) and fixed-size arrays (`T[n]`).
        if lower.ends_with("[]") {
            let inner = &ty[..ty.len() - 2];
            let element = NeoType::from_solidity(inner, structs, enums, contract_types)?;
            return Ok(NeoType::Array(Box::new(element)));
        }

        if lower.ends_with(']') {
            if let Some(stripped) = lower.strip_suffix(']') {
                if let Some((_inner_ty, size_str)) = stripped.rsplit_once('[') {
                    if !size_str.is_empty() && size_str.chars().all(|c| c.is_ascii_digit()) {
                        let inner_original = &ty[..ty.len() - size_str.len() - 2];
                        let element =
                            NeoType::from_solidity(inner_original, structs, enums, contract_types)?;
                        return Ok(NeoType::Array(Box::new(element)));
                    }
                }
            }
        }

        if let Some(rest) = lower.strip_prefix("uint") {
            let bits = rest.parse::<u16>().unwrap_or(256);
            return Ok(NeoType::Integer {
                signed: false,
                bits,
            });
        }

        if let Some(rest) = lower.strip_prefix("int") {
            let bits = rest.parse::<u16>().unwrap_or(256);
            return Ok(NeoType::Integer { signed: true, bits });
        }

        if lower == "bool" {
            return Ok(NeoType::Boolean);
        }

        if lower == "string" {
            return Ok(NeoType::String);
        }

        if lower == "address"
            || lower == "address payable"
            || lower == "hash160"
            || lower == "bytes20"
        {
            return Ok(NeoType::Address);
        }

        if lower == "bytes" {
            return Ok(NeoType::ByteArray { fixed_len: None });
        }

        if let Some(hex_suffix) = lower.strip_prefix("bytes") {
            if let Ok(len) = hex_suffix.parse::<u16>() {
                return Ok(NeoType::ByteArray {
                    fixed_len: Some(len),
                });
            }
        }

        if lower.starts_with("mapping") {
            return parse_mapping_type(ty, structs, enums, contract_types);
        }

        if let Some(struct_meta) = lookup_struct(ty, structs) {
            let mut fields = Vec::new();
            for field in &struct_meta.fields {
                let field_type = NeoType::from_solidity(&field.ty, structs, enums, contract_types)?;
                fields.push(StructFieldType {
                    name: field.name.clone(),
                    ty: Box::new(field_type),
                });
            }
            return Ok(NeoType::Struct {
                name: struct_meta.name.clone(),
                fields,
            });
        }

        if lookup_enum(ty, enums).is_some() {
            // Solidity encodes enums as unsigned integers (typically uint8).
            return Ok(NeoType::Integer {
                signed: false,
                bits: 8,
            });
        }

        if lower == "any" {
            return Ok(NeoType::Any);
        }

        // Contract/interface-typed values are represented as Neo UInt160 addresses.
        let mut candidate = ty.trim();
        // Strip common Solidity prefixes (`contract Foo`, `interface Foo`) if present.
        let candidate_lower = candidate.to_ascii_lowercase();
        for prefix in ["contract ", "interface ", "library "] {
            if candidate_lower.starts_with(prefix) {
                candidate = candidate[prefix.len()..].trim_start();
                break;
            }
        }
        if let Some((_, last)) = candidate.rsplit_once('.') {
            candidate = last.trim();
        }
        if contract_types
            .iter()
            .any(|name| name.eq_ignore_ascii_case(candidate))
        {
            return Ok(NeoType::Address);
        }

        Ok(NeoType::Any)
    }
}

fn strip_data_location(ty: &str) -> &str {
    let mut trimmed = ty.trim();
    for suffix in [" storage", " memory", " calldata"] {
        if let Some(stripped) = trimmed.strip_suffix(suffix) {
            trimmed = stripped.trim_end();
        }
    }
    trimmed
}

fn lookup_struct<'a>(
    ty: &str,
    structs: &'a [StructTypeMetadata],
) -> Option<&'a StructTypeMetadata> {
    fn normalize(raw: &str) -> &str {
        let mut candidate = raw.trim_start_matches("struct ").trim();
        if let Some((_, last)) = candidate.rsplit_once('.') {
            candidate = last.trim();
        }
        candidate
    }

    let name = normalize(ty);
    structs.iter().find(|s| s.name.eq_ignore_ascii_case(name))
}

fn lookup_enum<'a>(ty: &str, enums: &'a [EnumTypeMetadata]) -> Option<&'a EnumTypeMetadata> {
    fn normalize(raw: &str) -> &str {
        let mut candidate = raw.trim_start_matches("enum ").trim();
        if let Some((_, last)) = candidate.rsplit_once('.') {
            candidate = last.trim();
        }
        candidate
    }

    let name = normalize(ty);
    enums.iter().find(|e| e.name.eq_ignore_ascii_case(name))
}

fn parse_mapping_type(
    ty: &str,
    structs: &[StructTypeMetadata],
    enums: &[EnumTypeMetadata],
    contract_types: &[String],
) -> Result<NeoType, TypeParseError> {
    // Expect "mapping(<key> => <value>)"
    let after_keyword = ty
        .trim_start()
        .strip_prefix("mapping")
        .ok_or_else(|| TypeParseError::Unsupported(ty.to_string()))?;

    let mut rest = after_keyword.trim_start();
    if !rest.starts_with('(') {
        return Err(TypeParseError::Unsupported(ty.to_string()));
    }
    rest = &rest[1..]; // skip '('

    // Find matching ')'
    let mut depth = 1isize;
    let mut idx = 0usize;
    let bytes = rest.as_bytes();
    while idx < bytes.len() && depth > 0 {
        match bytes[idx] as char {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            break;
        }
        idx += 1;
    }

    if depth != 0 {
        return Err(TypeParseError::Unsupported(ty.to_string()));
    }

    let inner = &rest[..idx];
    let remaining = rest[idx + 1..].trim();
    if !remaining.is_empty() {
        return Err(TypeParseError::Unsupported(ty.to_string()));
    }

    // Split inner on top-level "=>"
    let mut split_index = None;
    let mut paren_depth = 0isize;
    let mut chars = inner.char_indices();
    while let Some((i, ch)) = chars.next() {
        match ch {
            '(' | '[' => paren_depth += 1,
            ')' | ']' => paren_depth -= 1,
            '=' if paren_depth == 0 => {
                if let Some((_, next_char)) = chars.clone().next() {
                    if next_char == '>' {
                        split_index = Some(i);
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    let split_idx = split_index.ok_or_else(|| TypeParseError::Unsupported(ty.to_string()))?;
    let key_str = inner[..split_idx].trim();
    let value_str = inner[split_idx + 2..].trim();

    if key_str.is_empty() || value_str.is_empty() {
        return Err(TypeParseError::Unsupported(ty.to_string()));
    }

    let key = NeoType::from_solidity(key_str, structs, enums, contract_types)?;
    let value = NeoType::from_solidity(value_str, structs, enums, contract_types)?;

    Ok(NeoType::Mapping {
        key: Box::new(key),
        value: Box::new(value),
    })
}
