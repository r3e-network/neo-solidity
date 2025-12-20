fn infer_literal_array_element_type(elements: &[Expression]) -> ValueType {
    if elements.is_empty() {
        return ValueType::Any;
    }

    let mut ty = ValueType::Any;
    for expr in elements {
        match literal_from_expression(expr) {
            Some(LiteralValue::Boolean(_)) => ty = ValueType::Boolean,
            Some(LiteralValue::Integer(_)) => {
                ty = ValueType::Integer {
                    signed: false,
                    bits: 256,
                }
            }
            Some(LiteralValue::String(_)) => ty = ValueType::String,
            Some(LiteralValue::ByteArray(_)) => ty = ValueType::ByteArray { fixed_len: None },
            Some(LiteralValue::Address(_)) => ty = ValueType::Address,
            Some(LiteralValue::Null) => {
                ty = ValueType::Any;
                break;
            }
            None => {
                ty = ValueType::Any;
                break;
            }
        }
    }
    ty
}

fn builtin_struct_type(base: &str, member: &str) -> Option<ValueType> {
    use crate::storage_key::compute_state_slot;

    fn mk_struct(name: &str, fields: Vec<(&str, ValueType)>) -> ValueType {
        ValueType::Struct {
            name: name.to_string(),
            fields: fields
                .into_iter()
                .map(|(field_name, ty)| StructField {
                    name: field_name.to_string(),
                    ty,
                    key: compute_state_slot(&format!("{name}::{field_name}")),
                })
                .collect(),
        }
    }

    let u256 = ValueType::Integer {
        signed: false,
        bits: 256,
    };

    match (base, member) {
        ("NativeCalls", "NeoCandidate") => Some(mk_struct(
            "NativeCalls.NeoCandidate",
            vec![
                ("publicKey", ValueType::ByteArray { fixed_len: None }),
                ("votes", u256.clone()),
            ],
        )),
        ("NativeCalls", "AccountState") => Some(mk_struct(
            "NativeCalls.AccountState",
            vec![
                ("balance", u256.clone()),
                ("balanceHeight", u256.clone()),
                ("voteTo", ValueType::ByteArray { fixed_len: None }),
                ("lastGasPerVote", u256),
            ],
        )),
        ("NativeCalls", "ContractState") => Some(mk_struct(
            "NativeCalls.ContractState",
            vec![
                ("hash", ValueType::Address),
                ("nef", ValueType::ByteArray { fixed_len: None }),
                ("manifest", ValueType::ByteArray { fixed_len: None }),
                ("updateCounter", u256),
            ],
        )),
        _ => None,
    }
}

fn infer_type_from_expression(expr: &Expression, ctx: &LoweringContext) -> Option<ValueType> {
    match expr {
        Expression::Parenthesis(_, inner) => infer_type_from_expression(inner, ctx),
        Expression::BoolLiteral(_, _) => Some(ValueType::Boolean),
        Expression::NumberLiteral(_, _, _, _)
        | Expression::HexNumberLiteral(_, _, _)
        | Expression::RationalNumberLiteral(_, _, _, _, _) => Some(ValueType::Integer {
            signed: false,
            bits: 256,
        }),
        Expression::StringLiteral(_) => Some(ValueType::String),
        Expression::HexLiteral(_) => Some(ValueType::ByteArray { fixed_len: None }),
        Expression::AddressLiteral(_, _) => Some(ValueType::Address),
        Expression::Type(_, ty) => value_type_from_ptype(ty),
        Expression::FunctionCall(_, func, args)
            if args.len() == 1 && matches!(func.as_ref(), Expression::Type(_, _)) =>
        {
            if let Expression::Type(_, ty) = func.as_ref() {
                value_type_from_ptype(ty)
            } else {
                None
            }
        }
        Expression::FunctionCall(_, func, _) => {
            if let Expression::MemberAccess(_, inner, member) = func.as_ref() {
                if let Expression::Variable(base) = inner.as_ref() {
                    match (base.name.as_str(), member.name.as_str()) {
                        ("Syscalls", "scriptHashToAddress") => return Some(ValueType::Address),
                        ("Syscalls", "addressToScriptHash") => {
                            return Some(ValueType::ByteArray { fixed_len: Some(20) })
                        }
                        ("Syscalls", "isValidAddress") => return Some(ValueType::Boolean),
                        _ => {}
                    }
                }
            }
            None
        }
        Expression::ArrayLiteral(_, elements) => Some(ValueType::Array(Box::new(
            infer_literal_array_element_type(elements),
        ))),
        Expression::ArraySubscript(_, array, index) => {
            // Type expression: `T[]` (dynamic array) is represented by solang-parser as an
            // ArraySubscript with a missing index expression. Use this to infer array element
            // types for locals/parameters so struct member access can be lowered correctly.
            if index.is_none() {
                infer_type_from_expression(array, ctx)
                    .map(|inner| ValueType::Array(Box::new(inner)))
            } else if let Some(ValueType::Array(inner)) = infer_type_from_expression(array, ctx) {
                // Value expression: `arr[i]`
                Some(*inner.clone())
            } else {
                None
            }
        }
        Expression::Variable(identifier) => {
            if identifier.name == "this" {
                Some(ValueType::Address)
            } else {
                ctx.variable_type(&identifier.name).or_else(|| {
                    // In type contexts solang-parser represents user-defined type names (e.g.,
                    // structs) as `Expression::Variable`. Resolve these by scanning the known
                    // state/param/return/local value types for a matching struct.
                    ctx.defined_struct_types
                        .iter()
                        .chain(ctx.state_types.iter())
                        .chain(ctx.param_types.iter())
                        .chain(ctx.return_types.iter())
                        .chain(ctx.local_types.values())
                        .find_map(|ty| find_named_struct_type(ty, &identifier.name))
                })
            }
        }
        Expression::MemberAccess(_, inner, member) => {
            if member.name == "selector" {
                return Some(ValueType::ByteArray { fixed_len: Some(4) });
            }

            if member.name == "interfaceId" {
                if let Expression::FunctionCall(_, func, args) = inner.as_ref() {
                    if args.len() == 1
                        && matches!(func.as_ref(), Expression::Variable(id) if id.name == "type")
                    {
                        return Some(ValueType::ByteArray { fixed_len: Some(4) });
                    }
                }
            }

            if matches!(member.name.as_str(), "max" | "min") {
                if let Expression::FunctionCall(_, func, args) = inner.as_ref() {
                    if args.len() == 1
                        && matches!(func.as_ref(), Expression::Variable(id) if id.name == "type")
                    {
                        if let Expression::Type(_, ty) = &args[0] {
                            if let Some(value_type) = value_type_from_ptype(ty) {
                                return Some(value_type);
                            }
                        }
                    }
                }
            }

            // Treat known NativeCalls.* native contract hash constants as addresses so
            // downstream lowering can recognize `NativeCalls.GAS_CONTRACT.balanceOf(...)`
            // as an external call target.
            if matches!(inner.as_ref(), Expression::Variable(id) if id.name == "NativeCalls")
                && matches!(
                    member.name.as_str(),
                    "NEO_CONTRACT"
                        | "GAS_CONTRACT"
                        | "CONTRACT_MANAGEMENT"
                        | "POLICY_CONTRACT"
                        | "ORACLE_CONTRACT"
                        | "ROLE_MANAGEMENT"
                )
            {
                return Some(ValueType::Address);
            }

            // Type-qualified user-defined structs from interfaces/contracts: `Interface.StructName`.
            // solang-parser represents these as `MemberAccess(Variable("Interface"), "StructName")`.
            if let Expression::Variable(base) = inner.as_ref() {
                if ctx.is_contract_type_name(&base.name) {
                    if let Some(struct_ty) = ctx
                        .defined_struct_types
                        .iter()
                        .chain(ctx.state_types.iter())
                        .chain(ctx.param_types.iter())
                        .chain(ctx.return_types.iter())
                        .chain(ctx.local_types.values())
                        .find_map(|ty| find_named_struct_type(ty, &member.name))
                    {
                        return Some(struct_ty);
                    }
                }
            }

            if let Expression::Variable(base) = inner.as_ref() {
                match (base.name.as_str(), member.name.as_str()) {
                    ("msg", "sender") => return Some(ValueType::Address),
                    ("msg", "value") => {
                        return Some(ValueType::Integer {
                            signed: false,
                            bits: 256,
                        })
                    }
                    ("msg", "data") => return Some(ValueType::ByteArray { fixed_len: None }),
                    ("tx", "origin") => return Some(ValueType::Address),
                    ("block", "timestamp") | ("block", "number") | ("block", "chainid") => {
                        return Some(ValueType::Integer {
                            signed: false,
                            bits: 256,
                        })
                    }
                    _ => {}
                }
            }

            if let Expression::Variable(base) = inner.as_ref() {
                if let Some(struct_ty) = builtin_struct_type(&base.name, &member.name) {
                    return Some(struct_ty);
                }
            }

            // Best-effort struct member typing (e.g., `tmp.field`). This is required for
            // patterns like `IERC20(req.token).transfer(...)` where `req.token` must be
            // recognized as address-like for external call lowering.
            let inner_ty = infer_type_from_expression(inner, ctx)?;
            if let ValueType::Struct { fields, .. } = &inner_ty {
                if let Some(field) = fields.iter().find(|field| field.name == member.name) {
                    return Some(field.ty.clone());
                }
            }

            Some(inner_ty)
        }
        _ => None,
    }
}

fn value_type_from_ptype(ty: &PtType) -> Option<ValueType> {
    match ty {
        PtType::Bool => Some(ValueType::Boolean),
        PtType::Address | PtType::AddressPayable => Some(ValueType::Address),
        PtType::Uint(bits) => Some(ValueType::Integer {
            signed: false,
            bits: *bits,
        }),
        PtType::Int(bits) => Some(ValueType::Integer {
            signed: true,
            bits: *bits,
        }),
        PtType::String => Some(ValueType::String),
        PtType::Bytes(len) => Some(ValueType::ByteArray {
            fixed_len: Some(*len as u16),
        }),
        PtType::DynamicBytes => Some(ValueType::ByteArray { fixed_len: None }),
        _ => None,
    }
}

fn infer_array_element_type(expr: &Expression, ctx: &LoweringContext) -> Option<ValueType> {
    match infer_type_from_expression(expr, ctx) {
        Some(ValueType::Array(inner)) => Some(*inner.clone()),
        _ => None,
    }
}
