fn infer_literal_array_element_type(elements: &[Expression]) -> ValueType {
    if elements.is_empty() {
        // Empty array literals (`[]`) have no elements to infer from.
        // Callers should use assignment-target context when available.
        return ValueType::Any;
    }

    let mut inferred: Option<ValueType> = None;
    for expr in elements {
        let candidate = match literal_from_expression(expr) {
            Some(LiteralValue::Boolean(_)) => Some(ValueType::Boolean),
            Some(LiteralValue::Integer(_)) => Some(ValueType::Integer {
                signed: false,
                bits: 256,
            }),
            Some(LiteralValue::String(_)) => Some(ValueType::String),
            Some(LiteralValue::ByteArray(_)) => Some(ValueType::ByteArray { fixed_len: None }),
            Some(LiteralValue::Address(_)) => Some(ValueType::Address),
            Some(LiteralValue::Null) | None => None,
        };

        if let Some(ty) = candidate {
            if let Some(ref prev) = inferred {
                if *prev != ty {
                    // Mixed element types — fall back to Any.
                    return ValueType::Any;
                }
            } else {
                inferred = Some(ty);
            }
        }
        // Skip unparseable elements; keep searching for a concrete type.
    }

    inferred.unwrap_or(ValueType::Any)
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
        ("NativeCalls", "WhitelistedContract") => Some(mk_struct(
            "NativeCalls.WhitelistedContract",
            vec![
                ("contractHash", ValueType::Address),
                ("method", ValueType::String),
                (
                    "argCount",
                    ValueType::Integer {
                        signed: false,
                        bits: 256,
                    },
                ),
                (
                    "fixedFee",
                    ValueType::Integer {
                        signed: true,
                        bits: 256,
                    },
                ),
            ],
        )),
        ("NativeCalls", "NetworkConfig") => Some(mk_struct(
            "NativeCalls.NetworkConfig",
            vec![
                ("feePerByte", u256.clone()),
                (
                    "execFeeFactor",
                    ValueType::Integer {
                        signed: false,
                        bits: 32,
                    },
                ),
                ("storagePrice", u256.clone()),
                ("gasPerBlock", u256.clone()),
                ("oraclePrice", u256.clone()),
                ("minimumDeploymentFee", u256),
            ],
        )),
        _ => None,
    }
}

/// Returns true when the expression is structurally a Solidity type expression
/// rather than a value expression. Used to distinguish fixed-size array types
/// (`T[N]`) from value-subscripts (`arr[i]`) when both parse as
/// `Expression::ArraySubscript(inner, Some(_))`. A type expression is either a
/// primitive `Type` node or nested `ArraySubscript` whose base is itself a
/// type expression (covers nested fixed-size arrays like `uint[3][2]`).
fn is_type_expression(expr: &Expression) -> bool {
    match expr {
        Expression::Type(_, _) => true,
        Expression::ArraySubscript(_, inner, _) => is_type_expression(inner),
        Expression::Parenthesis(_, inner) => is_type_expression(inner),
        _ => false,
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
        Expression::FunctionCall(_, func, args) if args.len() == 1 => {
            if let Expression::MemberAccess(_, inner, member) = func.as_ref() {
                if let Expression::Variable(base) = inner.as_ref() {
                    match (base.name.as_str(), member.name.as_str()) {
                        ("Syscalls", "scriptHashToAddress") => return Some(ValueType::Address),
                        ("Syscalls", "addressToScriptHash") => {
                            return Some(ValueType::ByteArray {
                                fixed_len: Some(20),
                            })
                        }
                        ("Syscalls", "isValidAddress") => return Some(ValueType::Boolean),
                        _ => {}
                    }
                }
            }

            // Contract/interface type-casts like `IPool(addr)` and namespace-qualified
            // imports like `NS.IPool(addr)` evaluate to an address-like value.
            if let Expression::Variable(type_id) = func.as_ref() {
                if ctx.is_contract_type_name(&type_id.name) {
                    return Some(ValueType::Address);
                }
            }

            if let Expression::MemberAccess(_, namespace_expr, type_id) = func.as_ref() {
                if matches!(
                    namespace_expr.as_ref(),
                    Expression::Variable(namespace_id)
                        if !ctx.param_index_map.contains_key(&namespace_id.name)
                            && ctx.resolve_local(&namespace_id.name).is_none()
                            && !ctx.state_index_map.contains_key(&namespace_id.name)
                            && !ctx.is_contract_type_name(&namespace_id.name)
                ) && ctx.is_contract_type_name(&type_id.name)
                {
                    return Some(ValueType::Address);
                }
            }

            None
        }
        // Task #191 — infer a user-defined function's return type from the
        // registered `function_return_types` map so chained calls like
        // `c.inc().value` can resolve the struct-field index. The specific
        // cases already handled above (`T(x)` casts, `Interface(addr)`,
        // `Syscalls.*`, etc.) stay on their fast paths; this branch covers
        // bare `f(...)` where `f` is a contract method or injected free
        // function (e.g. `using { inc } for T` with inc at file scope).
        //
        // Two call shapes produce a user-defined function name here:
        //   - `Expression::Variable(id)` → `f(args)` (free function / same-
        //     contract method after Task #187 injection).
        //   - `Expression::MemberAccess(_, Variable(id), method)` → method
        //     call on `this` / library-qualified call. We key the lookup on
        //     the method name with the literal argument count, matching how
        //     `function_return_types` is populated at module-build time.
        Expression::FunctionCall(_, func, args) => {
            if let Expression::Variable(identifier) = func.as_ref() {
                if let Some(ty) = ctx.get_function_return_type(&identifier.name, args.len()) {
                    return Some(ty.clone());
                }
            }
            if let Expression::MemberAccess(_, inner, method) = func.as_ref() {
                // `x.f(args)` attached via `using { f } for T;` lowers to
                // `f(x, args)` — the registered return-type key therefore uses
                // `args.len() + 1` for the library-attach form. Try both
                // shapes before giving up.
                if ctx.has_using_directives() {
                    if let Some(ty) =
                        ctx.get_function_return_type(&method.name, args.len() + 1)
                    {
                        // Only honour the receiver-attached form when the
                        // inner expression isn't a namespace-style
                        // `Library.f(...)` call (which keeps its literal
                        // arg count). Distinguishing the two without full
                        // type info: treat a bare `Variable` inner that
                        // doesn't name a library/contract as a receiver.
                        if let Expression::Variable(base) = inner.as_ref() {
                            let is_namespace = ctx.is_contract_type_name(&base.name);
                            if !is_namespace {
                                return Some(ty.clone());
                            }
                        } else {
                            // Chained-call receiver (e.g. `c.inc().inc()`):
                            // the inner isn't a namespace, so the outer is
                            // genuinely receiver-attached.
                            return Some(ty.clone());
                        }
                    }
                }
                if let Some(ty) = ctx.get_function_return_type(&method.name, args.len()) {
                    return Some(ty.clone());
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
            } else if is_type_expression(array) {
                // Task #185: Fixed-size array type expression `T[N]` (e.g. the outer
                // element type of a multi-dim declaration `uint[3][2] memory a;`).
                // solang-parser represents this as `ArraySubscript(T, Some(N))` in type
                // contexts — structurally identical to a value subscript `arr[i]`, but the
                // base resolves to a scalar/struct type rather than an Array. When the
                // base is structurally a type expression (`Type(_)` or nested
                // `ArraySubscript(type_expr, _)`), wrap the inferred base type in
                // `Array(..)` so inner-dimension `new T[N]` allocations receive the
                // correct element type.
                infer_type_from_expression(array, ctx)
                    .map(|inner| ValueType::Array(Box::new(inner)))
            } else {
                None
            }
        }
        Expression::Variable(identifier) => {
            if identifier.name == "this" || ctx.is_contract_type_name(&identifier.name) {
                Some(ValueType::Address)
            } else if ctx.enum_variant_map.contains_key(&identifier.name) {
                // Solidity enums lower to unsigned integers. We model enum-typed values as
                // uint8 for IR inference to support constructs like `new MyEnum[](n)`.
                Some(ValueType::Integer {
                    signed: false,
                    bits: 8,
                })
            } else {
                ctx.variable_type(&identifier.name).or_else(|| {
                    // In type contexts solang-parser represents user-defined type names (e.g.,
                    // structs) as `Expression::Variable`. Resolve by scanning known value types
                    // with scope-priority ordering: local → param → return → state → defined
                    // structs. This prevents cross-scope type collisions when the same name
                    // appears at multiple levels.
                    ctx.local_types
                        .values()
                        .chain(ctx.param_types.iter())
                        .chain(ctx.return_types.iter())
                        .chain(ctx.state_types.iter())
                        .chain(ctx.defined_struct_types.iter())
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

            // Treat known NativeCalls/NativeContracts native contract hash constants as
            // addresses so downstream lowering can recognize static native targets.
            if matches!(
                inner.as_ref(),
                Expression::Variable(id)
                    if id.name == "NativeCalls" || id.name == "NativeContracts"
            )
                && matches!(
                    member.name.as_str(),
                    "NEO_CONTRACT"
                        | "GAS_CONTRACT"
                        | "CONTRACT_MANAGEMENT"
                        | "POLICY_CONTRACT"
                        | "ORACLE_CONTRACT"
                        | "ROLE_MANAGEMENT"
                        | "NOTARY_CONTRACT"
                        | "TREASURY_CONTRACT"
                        | "LEDGER_CONTRACT"
                        | "CRYPTO_LIB"
                        | "STD_LIB"
                )
            {
                return Some(ValueType::Address);
            }

            // Treat known Syscalls.* native contract hash constants as addresses so
            // member access is typed consistently with NativeCalls constants.
            if matches!(inner.as_ref(), Expression::Variable(id) if id.name == "Syscalls")
                && matches!(
                    member.name.as_str(),
                    "CONTRACT_MANAGEMENT"
                        | "POLICY_CONTRACT"
                        | "ORACLE_CONTRACT"
                        | "ROLE_MANAGEMENT"
                        | "LEDGER_CONTRACT"
                        | "CRYPTO_LIB"
                        | "STD_LIB"
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
                    ("block", "timestamp" | "number" | "chainid") => {
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
        PtType::Address | PtType::AddressPayable | PtType::Payable => Some(ValueType::Address),
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
        // Solidity `fixed`/`ufixed` rational literals resolve to integers at compile time.
        PtType::Rational => Some(ValueType::Integer {
            signed: false,
            bits: 256,
        }),
        // Mapping type: extract key/value from inner type expressions when possible.
        PtType::Mapping { key, value, .. } => {
            let key_ty = if let Expression::Type(_, inner) = key.as_ref() {
                value_type_from_ptype(inner)?
            } else {
                ValueType::Any
            };
            let val_ty = if let Expression::Type(_, inner) = value.as_ref() {
                value_type_from_ptype(inner)?
            } else {
                ValueType::Any
            };
            Some(ValueType::Mapping {
                key: Box::new(key_ty),
                value: Box::new(val_ty),
            })
        }
        // Function types are not representable on NeoVM.
        PtType::Function { .. } => None,
    }
}

fn infer_array_element_type(expr: &Expression, ctx: &LoweringContext) -> Option<ValueType> {
    match infer_type_from_expression(expr, ctx) {
        Some(ValueType::Array(inner)) => Some(*inner.clone()),
        _ => None,
    }
}
