//! Basic Neo type representations derived from Solidity type strings.

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructFieldMetadata {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructTypeMetadata {
    pub name: String,
    pub fields: Vec<StructFieldMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeoType {
    Integer {
        signed: bool,
        bits: u16,
    },
    Boolean,
    String,
    Address,
    ByteArray {
        fixed_len: Option<u16>,
    },
    Array(Box<NeoType>),
    Mapping {
        key: Box<NeoType>,
        value: Box<NeoType>,
    },
    Struct {
        name: String,
        fields: Vec<StructFieldType>,
    },
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructFieldType {
    pub name: String,
    pub ty: Box<NeoType>,
}

#[derive(Debug, Error)]
pub enum TypeParseError {
    #[error("unsupported Solidity type '{0}'")]
    Unsupported(String),
}

impl NeoType {
    pub fn from_solidity(ty: &str, structs: &[StructTypeMetadata]) -> Result<Self, TypeParseError> {
        let ty = strip_data_location(ty);
        let lower = ty.to_ascii_lowercase();

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
            let element = NeoType::from_solidity(inner, structs)?;
            return Ok(NeoType::Array(Box::new(element)));
        }

        if lower.starts_with("mapping") {
            return parse_mapping_type(ty, structs);
        }

        if let Some(struct_meta) = lookup_struct(ty, structs) {
            let mut fields = Vec::new();
            for field in &struct_meta.fields {
                let field_type = NeoType::from_solidity(&field.ty, structs)?;
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
    structs.iter().find(|s| {
        s.name
            .eq_ignore_ascii_case(ty.trim_start_matches("struct ").trim())
    })
}

fn parse_mapping_type(ty: &str, structs: &[StructTypeMetadata]) -> Result<NeoType, TypeParseError> {
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

    let key = NeoType::from_solidity(key_str, structs)?;
    let value = NeoType::from_solidity(value_str, structs)?;

    Ok(NeoType::Mapping {
        key: Box::new(key),
        value: Box::new(value),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn struct_meta(name: &str, fields: &[(&str, &str)]) -> StructTypeMetadata {
        StructTypeMetadata {
            name: name.to_string(),
            fields: fields
                .iter()
                .map(|(fname, fty)| StructFieldMetadata {
                    name: fname.to_string(),
                    ty: fty.to_string(),
                })
                .collect(),
        }
    }

    #[test]
    fn parses_simple_struct_type() {
        let structs = vec![struct_meta("Point", &[("x", "uint256"), ("y", "uint256")])];

        let ty = NeoType::from_solidity("Point", &structs).expect("struct type");
        match ty {
            NeoType::Struct { name, fields } => {
                assert_eq!(name, "Point");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].name, "x");
                assert!(matches!(
                    *fields[0].ty,
                    NeoType::Integer {
                        signed: false,
                        bits: 256
                    }
                ));
            }
            other => panic!("expected struct type, got {:?}", other),
        }
    }

    #[test]
    fn strips_data_location_suffixes() {
        let structs = vec![struct_meta("Point", &[("x", "uint256")])];
        let ty = NeoType::from_solidity("Point storage", &structs).expect("struct storage type");
        assert!(matches!(ty, NeoType::Struct { .. }));

        let mapping = NeoType::from_solidity("mapping(address => Point memory)", &structs)
            .expect("mapping with struct value");
        match mapping {
            NeoType::Mapping { value, .. } => {
                assert!(matches!(*value, NeoType::Struct { .. }));
            }
            other => panic!("expected mapping type, got {:?}", other),
        }
    }
}
