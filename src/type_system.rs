//! Basic Neo type representations derived from Solidity type strings.

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeoType {
    Integer { signed: bool, bits: u16 },
    Boolean,
    String,
    Address,
    ByteArray { fixed_len: Option<u16> },
    Array(Box<NeoType>),
    Mapping {
        key: Box<NeoType>,
        value: Box<NeoType>,
    },
    Any,
}

#[derive(Debug, Error)]
pub enum TypeParseError {
    #[error("unsupported Solidity type '{0}'")]
    Unsupported(String),
}

impl NeoType {
    pub fn from_solidity(ty: &str) -> Result<Self, TypeParseError> {
        let ty = ty.trim();
        let lower = ty.to_ascii_lowercase();

        if lower.starts_with("uint") {
            let bits = lower[4..].parse::<u16>().unwrap_or(256);
            return Ok(NeoType::Integer {
                signed: false,
                bits,
            });
        }

        if lower.starts_with("int") {
            let bits = lower[3..].parse::<u16>().unwrap_or(256);
            return Ok(NeoType::Integer { signed: true, bits });
        }

        if lower == "bool" {
            return Ok(NeoType::Boolean);
        }

        if lower == "string" {
            return Ok(NeoType::String);
        }

        if lower == "address" || lower == "hash160" || lower == "bytes20" {
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

        if lower.ends_with("[]") {
            let inner = &ty[..ty.len() - 2];
            let element = NeoType::from_solidity(inner)?;
            return Ok(NeoType::Array(Box::new(element)));
        }

        if lower.starts_with("mapping") {
            return parse_mapping_type(ty);
        }

        Err(TypeParseError::Unsupported(ty.to_string()))
    }
}

fn parse_mapping_type(ty: &str) -> Result<NeoType, TypeParseError> {
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

    let key = NeoType::from_solidity(key_str)?;
    let value = NeoType::from_solidity(value_str)?;

    Ok(NeoType::Mapping {
        key: Box::new(key),
        value: Box::new(value),
    })
}
