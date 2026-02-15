fn try_lower_type_constructor_call(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    fn lower_contract_like_cast(
        args: &[Expression],
        ctx: &mut LoweringContext,
        instructions: &mut Vec<Instruction>,
    ) -> bool {
        if let Some(arg) = args.first() {
            if matches!(
                infer_type_from_expression(arg, ctx),
                Some(ValueType::Address)
            ) {
                return lower_expression(arg, ctx, instructions);
            }

            if let Some(bytes) = address_bytes_le_from_expression(arg) {
                instructions.push(Instruction::PushLiteral(LiteralValue::Address(bytes)));
                return true;
            }

            if lower_expression(arg, ctx, instructions) {
                instructions.push(Instruction::Convert {
                    target: ConvertTarget::ByteArray,
                });
                coerce_to_fixed_bytes(20, false, ctx, instructions);
            } else {
                instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                    vec![0u8; 20],
                )));
            }
        } else {
            instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                vec![0u8; 20],
            )));
        }

        true
    }

    // Contract/interface casts represented as plain identifiers:
    // `IFoo(target)` should behave like `address(target)` at runtime.
    if let Expression::Variable(type_id) = func {
        if ctx.is_contract_type_name(&type_id.name) && args.len() == 1 {
            return Some(lower_contract_like_cast(args, ctx, instructions));
        }
    }

    // `import * as NS` namespace-qualified contract/interface casts:
    // `NS.IFoo(target)` should behave like `IFoo(target)`.
    if let Expression::MemberAccess(_, namespace_expr, type_id) = func {
        let is_namespace = matches!(
            namespace_expr.as_ref(),
            Expression::Variable(namespace_id)
                if !ctx.param_index_map.contains_key(&namespace_id.name)
                    && ctx.resolve_local(&namespace_id.name).is_none()
                    && !ctx.state_index_map.contains_key(&namespace_id.name)
                    && !ctx.is_contract_type_name(&namespace_id.name)
        );

        if is_namespace && ctx.is_contract_type_name(&type_id.name) && args.len() == 1 {
            return Some(lower_contract_like_cast(args, ctx, instructions));
        }
    }

    if let Expression::Type(_, ty) = func {
        match ty {
            PtType::Address | PtType::AddressPayable => {
                if let Some(arg) = args.first() {
                    // Preserve `this` as the executing script hash so manifest permission
                    // inference can recognize self-calls without requiring wildcard entries.
                    if matches!(arg, Expression::Variable(id) if id.name == "this") {
                        if !lower_expression(arg, ctx, instructions) {
                            instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                                vec![0u8; 20],
                            )));
                        }
                        return Some(true);
                    }

                    // Prefer a fixed-width UInt160 literal for common address casts like
                    // `address(0x...)`. If we lowered the numeric literal as an Integer it
                    // may become 21 bytes due to sign-extension rules, which Neo N3 rejects
                    // for UInt160 inputs.
                    if let Some(bytes) = address_bytes_le_from_expression(arg) {
                        instructions.push(Instruction::PushLiteral(LiteralValue::Address(bytes)));
                        return Some(true);
                    }

                    if lower_expression(arg, ctx, instructions) {
                        // Coerce to a fixed-width UInt160 (little-endian) so comparisons with
                        // `address(0)` and storage defaults behave consistently.
                        instructions.push(Instruction::Convert {
                            target: ConvertTarget::ByteArray,
                        });
                        coerce_to_fixed_bytes(20, false, ctx, instructions);
                    } else {
                        instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                            vec![0u8; 20],
                        )));
                    }
                } else {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                        vec![0u8; 20],
                    )));
                }
                return Some(true);
            }
            PtType::Uint(bits) => {
                let bits = *bits as usize;
                let arg = args.first();
                if arg.is_none_or(|arg| !lower_expression(arg, ctx, instructions)) {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                } else if let Some(arg) = arg {
                    // Solidity allows casting `bytesN` and `bool` into integers; NeoVM represents
                    // these as ByteString/Boolean stack items, so coerce them to Integer first.
                    if matches!(
                        infer_type_from_expression(arg, ctx),
                        Some(ValueType::ByteArray { .. } | ValueType::Address | ValueType::Boolean)
                    ) {
                        instructions.push(Instruction::Convert {
                            target: ConvertTarget::Integer,
                        });
                    }
                }

                // Solidity integer casts truncate modulo 2^bits.
                // For unsigned, this is equivalent to `value & ((1<<bits)-1)`.
                //
                // Avoid emitting a redundant `& (2^256-1)` mask for common no-op casts like
                // `uint256(1)` so bytecode patterns (and tests) remain stable.
                let needs_mask = if bits != 256 {
                    true
                } else if let Some(arg) = args.first() {
                    if matches!(arg, Expression::Negate(..)) {
                        true
                    } else if let Some(LiteralValue::Integer(value)) = literal_from_expression(arg)
                    {
                        value < BigInt::zero() || value >= (BigInt::one() << 256)
                    } else {
                        matches!(
                            infer_type_from_expression(arg, ctx),
                            Some(ValueType::Integer { signed: true, .. })
                        )
                    }
                } else {
                    false
                };

                if needs_mask {
                    let mask = (BigInt::one() << bits) - BigInt::one();
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(mask)));
                    instructions.push(Instruction::BinaryOp(BinaryOperator::BitAnd));
                }
                return Some(true);
            }
            PtType::Int(bits) => {
                let bits = *bits as usize;
                let arg = args.first();
                if arg.is_none_or(|arg| !lower_expression(arg, ctx, instructions)) {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                } else if let Some(arg) = arg {
                    if matches!(
                        infer_type_from_expression(arg, ctx),
                        Some(ValueType::ByteArray { .. } | ValueType::Address | ValueType::Boolean)
                    ) {
                        instructions.push(Instruction::Convert {
                            target: ConvertTarget::Integer,
                        });
                    }
                }

                // Solidity signed integer casts:
                // 1) truncate to `bits` low bits (mod 2^bits)
                // 2) interpret as signed using two's complement (sign-extend).
                let mask = (BigInt::one() << bits) - BigInt::one();
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(mask)));
                instructions.push(Instruction::BinaryOp(BinaryOperator::BitAnd));

                let tmp_id = ctx.next_label();
                let value_local = ctx.allocate_local(format!("__int_cast_{tmp_id}"), None);
                instructions.push(Instruction::StoreLocal(value_local));

                let sign_bit = BigInt::one() << (bits.saturating_sub(1));
                let modulus = BigInt::one() << bits;
                let positive_label = ctx.next_label();
                let end_label = ctx.next_label();

                // if value < sign_bit -> already positive
                instructions.push(Instruction::LoadLocal(value_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(sign_bit)));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Ge));
                instructions.push(Instruction::JumpIf {
                    target: positive_label,
                });

                // negative: value - 2^bits
                instructions.push(Instruction::LoadLocal(value_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(modulus)));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Sub));
                instructions.push(Instruction::Jump { target: end_label });

                // positive: value
                instructions.push(Instruction::Label(positive_label));
                instructions.push(Instruction::LoadLocal(value_local));

                instructions.push(Instruction::Label(end_label));
                return Some(true);
            }
            PtType::Bytes(len) => {
                if let Some(arg) = args.first() {
                    let reverse = matches!(
                        infer_type_from_expression(arg, ctx),
                        Some(ValueType::Integer { .. } | ValueType::Boolean)
                    );

                    if lower_expression(arg, ctx, instructions) {
                        instructions.push(Instruction::Convert {
                            target: ConvertTarget::ByteArray,
                        });
                        coerce_to_fixed_bytes(*len as usize, reverse, ctx, instructions);
                    } else {
                        instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                            vec![0u8; *len as usize],
                        )));
                    }
                } else {
                    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                        vec![0u8; *len as usize],
                    )));
                }
                return Some(true);
            }
            PtType::DynamicBytes => {
                if args
                    .first()
                    .is_none_or(|arg| !lower_expression(arg, ctx, instructions))
                {
                    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                        Vec::new(),
                    )));
                }
                return Some(true);
            }
            PtType::Rational => {
                ctx.record_error_with_suggestion(
                    "fixed-point types (fixed/ufixed) are not supported on NeoVM",
                    "use scaled integer arithmetic instead (e.g., multiply by 10^18 for 18 decimal places)",
                );
                return Some(false);
            }
            _ => {}
        }
    }

    None
}

fn coerce_to_fixed_bytes(
    len: usize,
    reverse: bool,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let tmp_id = ctx.next_label();
    let src_local = ctx.allocate_local(format!("__bytes_cast_src_{tmp_id}"), None);
    let dst_local = ctx.allocate_local(format!("__bytes_cast_dst_{tmp_id}"), None);
    let size_local = ctx.allocate_local(format!("__bytes_cast_size_{tmp_id}"), None);
    let count_local = ctx.allocate_local(format!("__bytes_cast_count_{tmp_id}"), None);

    // Store the source bytes so we can compute its size.
    instructions.push(Instruction::StoreLocal(src_local));

    // Allocate zero-initialized destination buffer of fixed length.
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        len as u64,
    ))));
    instructions.push(Instruction::NewBuffer);
    instructions.push(Instruction::StoreLocal(dst_local));

    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(size_local));

    // count = min(size, len)
    let ge_label = ctx.next_label();
    let end_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        len as u64,
    ))));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: ge_label });

    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Jump { target: end_label });

    instructions.push(Instruction::Label(ge_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        len as u64,
    ))));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Label(end_label));

    // NeoVM MEMCPY stack order: [dst, dst_offset, src, src_offset, count]
    instructions.push(Instruction::LoadLocal(dst_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::MemCpy);
    // Note: MEMCPY modifies the buffer in-place and does NOT push anything
    // back onto the stack. dst_local already holds the buffer reference.

    if reverse {
        // Reverse in place, keeping one reference on the stack.
        instructions.push(Instruction::LoadLocal(dst_local));
        instructions.push(Instruction::LoadLocal(dst_local));
        instructions.push(Instruction::ReverseItems);
    } else {
        instructions.push(Instruction::LoadLocal(dst_local));
    }

    // Canonicalize fixed-byte casts to ByteString so equality checks against
    // storage loads (also ByteString) are value-based and deterministic.
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
}
