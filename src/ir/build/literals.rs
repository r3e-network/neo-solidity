fn literal_from_expression(expr: &Expression) -> Option<LiteralValue> {
    match expr {
        Expression::BoolLiteral(_, value) => Some(LiteralValue::Boolean(*value)),
        Expression::NumberLiteral(_, integer, exp, unit) => {
            let mut value = parse_decimal_bigint(integer)?;
            let exponent = parse_signed_decimal_i32(exp)?;

            if exponent >= 0 {
                value *= pow10(exponent as u32);
            } else {
                let divisor = pow10((-exponent) as u32);
                if (&value % &divisor).is_zero() {
                    value /= divisor;
                } else {
                    return None;
                }
            }

            if let Some(unit) = unit.as_ref() {
                value *= unit_multiplier(unit)?;
            }

            Some(LiteralValue::Integer(value))
        }
        Expression::HexNumberLiteral(_, value, unit) => {
            let mut number = parse_hex_bigint(value)?;
            if let Some(unit) = unit.as_ref() {
                number *= unit_multiplier(unit)?;
            }
            Some(LiteralValue::Integer(number))
        }
        Expression::RationalNumberLiteral(_, integer, fraction, exp, unit) => {
            let int_part = parse_decimal_bigint(integer)?;
            let fraction_digits = sanitize_numeric_token(fraction);
            let frac_len = fraction_digits.len() as u32;
            let frac_part = if fraction_digits.trim().is_empty() {
                BigInt::zero()
            } else {
                BigInt::parse_bytes(fraction_digits.as_bytes(), 10)?
            };

            let mut numerator = int_part * pow10(frac_len) + frac_part;
            let mut denominator = pow10(frac_len);

            let exponent = parse_signed_decimal_i32(exp)?;
            if exponent >= 0 {
                numerator *= pow10(exponent as u32);
            } else {
                denominator *= pow10((-exponent) as u32);
            }

            if let Some(unit) = unit.as_ref() {
                numerator *= unit_multiplier(unit)?;
            }

            if denominator.is_zero() {
                return None;
            }

            if (&numerator % &denominator).is_zero() {
                Some(LiteralValue::Integer(numerator / denominator))
            } else {
                eprintln!(
                    "warning: non-integer rational literal {}.{} cannot be represented \
                     as an integer; fractional values are not supported on NeoVM",
                    integer, fraction
                );
                None
            }
        }
        Expression::StringLiteral(parts) => Some(LiteralValue::String(string_literal_bytes(parts))),
        Expression::HexLiteral(parts) => decode_hex_segments(parts).map(LiteralValue::ByteArray),
        Expression::AddressLiteral(_, value) => decode_hex_bytes(value).and_then(|mut bytes| {
            // Neo addresses are UInt160 (20 bytes). Reject malformed literals early.
            if bytes.len() != 20 {
                eprintln!(
                    "warning: address literal has {} bytes, expected 20 (UInt160)",
                    bytes.len()
                );
                return None;
            }
            // Neo smart contracts treat UInt160 values (script hashes) in little-endian byte
            // order on the VM stack. Solidity address literals are written in the canonical
            // big-endian hex form, so we reverse here to match Neo N3 conventions.
            bytes.reverse();
            Some(LiteralValue::Address(bytes))
        }),
        Expression::Parenthesis(_, inner) => literal_from_expression(inner),
        _ => None,
    }
}

fn address_bytes_le_from_expression(expr: &Expression) -> Option<Vec<u8>> {
    match expr {
        Expression::Parenthesis(_, inner) => address_bytes_le_from_expression(inner),
        Expression::AddressLiteral(_, value) => decode_hex_bytes(value).map(|mut bytes| {
            if bytes.len() > 20 {
                bytes.truncate(20);
            } else if bytes.len() < 20 {
                let mut padded = vec![0u8; 20 - bytes.len()];
                padded.extend_from_slice(&bytes);
                bytes = padded;
            }
            bytes.reverse();
            bytes
        }),
        Expression::HexNumberLiteral(_, value, unit) if unit.is_none() => {
            let raw = value.trim().trim_start_matches("0x");
            let mut hex: String = raw
                .chars()
                .filter(|c| !c.is_whitespace() && *c != '_')
                .collect();
            if hex.is_empty() {
                return None;
            }

            if hex.len() > 40 {
                return None;
            }

            if hex.len() % 2 == 1 {
                hex.insert(0, '0');
            }

            if hex.len() < 40 {
                hex = format!("{:0>40}", hex);
            }

            let mut bytes = hex_decode(&hex).ok()?;
            if bytes.len() != 20 {
                return None;
            }
            bytes.reverse();
            Some(bytes)
        }
        _ => None,
    }
}

fn decode_hex_segments(parts: &[PtHexLiteral]) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    for part in parts {
        let segment = part.hex.trim();
        let inner = segment
            .strip_prefix("hex")
            .and_then(|s| s.trim().strip_prefix('\"'))
            .and_then(|s| s.strip_suffix('\"'))
            .unwrap_or(segment);
        let cleaned: String = inner.chars().filter(|c| !c.is_whitespace()).collect();
        bytes.extend(hex_decode(&cleaned).ok()?);
    }
    Some(bytes)
}

fn decode_hex_bytes(value: &str) -> Option<Vec<u8>> {
    let cleaned = value.trim();
    if let Some(inner) = cleaned.strip_prefix("0x") {
        hex_decode(inner).ok()
    } else {
        hex_decode(cleaned).ok()
    }
}

fn parse_decimal_bigint(value: &str) -> Option<BigInt> {
    let sanitized: String = value.chars().filter(|c| *c != '_').collect();
    BigInt::parse_bytes(sanitized.as_bytes(), 10)
}

fn sanitize_numeric_token(value: &str) -> String {
    value.chars().filter(|c| *c != '_').collect()
}

fn parse_signed_decimal_i32(value: &str) -> Option<i32> {
    let sanitized = sanitize_numeric_token(value);
    if sanitized.trim().is_empty() {
        Some(0)
    } else {
        sanitized.parse::<i32>().ok()
    }
}

fn unit_multiplier(unit: &Identifier) -> Option<BigInt> {
    match unit.name.as_str() {
        // Solidity ether units
        "wei" => Some(BigInt::one()),
        "gwei" => Some(pow10(9)),
        "szabo" => Some(pow10(12)),
        "finney" => Some(pow10(15)),
        "ether" => Some(pow10(18)),

        // Solidity time units (Neo Runtime.GetTime is milliseconds; we normalize block.timestamp
        // elsewhere to seconds to preserve Solidity semantics.)
        "second" | "seconds" => Some(BigInt::one()),
        "minute" | "minutes" => Some(BigInt::from(60u64)),
        "hour" | "hours" => Some(BigInt::from(60u64 * 60)),
        "day" | "days" => Some(BigInt::from(60u64 * 60 * 24)),
        "week" | "weeks" => Some(BigInt::from(60u64 * 60 * 24 * 7)),
        "year" | "years" => Some(BigInt::from(60u64 * 60 * 24 * 365)),

        _ => None,
    }
}

fn is_ether_unit(name: &str) -> bool {
    matches!(name, "wei" | "gwei" | "szabo" | "finney" | "ether")
}

fn has_ether_unit(expr: &Expression) -> bool {
    match expr {
        Expression::NumberLiteral(_, _, _, Some(unit)) => is_ether_unit(&unit.name),
        Expression::HexNumberLiteral(_, _, Some(unit)) => is_ether_unit(&unit.name),
        Expression::RationalNumberLiteral(_, _, _, _, Some(unit)) => is_ether_unit(&unit.name),
        Expression::Parenthesis(_, inner) => has_ether_unit(inner),
        _ => false,
    }
}

fn pow10(exp: u32) -> BigInt {
    let ten = BigInt::from(10u8);
    ten.pow(exp)
}

fn parse_hex_bigint(value: &str) -> Option<BigInt> {
    let sanitized = value.trim_start_matches("0x");
    BigInt::parse_bytes(sanitized.as_bytes(), 16)
}
