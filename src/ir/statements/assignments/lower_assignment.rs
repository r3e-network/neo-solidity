fn lower_assignment(
    lhs: &Expression,
    rhs: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    if let Expression::Variable(identifier) = lhs {
        if ctx.storage_alias(&identifier.name).is_some() {
            if let Some(source_reference) = resolve_storage_reference(rhs, ctx) {
                ctx.set_storage_alias(identifier.name.clone(), source_reference);
                return;
            }
        }
    }

    // Task #117 regression guard: fix-117 widened `resolve_storage_reference`
    // so a bare `Expression::Variable` that names an Array- or Mapping-typed
    // state variable now returns a no-key/no-field StorageReference (needed so
    // `T[] storage a = arr;` alias declarations can anchor to `arr`). If we
    // routed `arr = m;` through that path here, the `emit_storage_store`
    // fallback would fire a single `StoreMappingElement` with empty
    // `key_types`, which writes the whole memory blob instead of the per-index
    // STORAGE_PUT loop that `arr[i]` reads expect. For direct state-var
    // assignments (no alias shadowing the name) we skip the reference path and
    // fall through to the Task #102 per-element copy below.
    let direct_state_var_array = if let Expression::Variable(identifier) = lhs {
        ctx.storage_alias(&identifier.name).is_none()
            && ctx
                .state_index_map
                .get(&identifier.name)
                .and_then(|idx| ctx.state_type(*idx))
                .map(|ty| matches!(ty, ValueType::Array(_) | ValueType::Mapping { .. }))
                .unwrap_or(false)
    } else {
        false
    };

    let resolved = if direct_state_var_array {
        None
    } else {
        resolve_storage_reference(lhs, ctx)
    };

    if let Some(reference) = resolved {
        if !ctx.ensure_state_writable(reference.state_index) {
            if lower_expression(rhs, ctx, instructions) {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
            return;
        }

        if let ValueType::Struct { name, fields } = &reference.value_type {
            // Support struct construction on the RHS for storage assignments, both
            // named and positional: `S({a:1,b:2})` or `S(1,2)`.
            let mut ctor_args_by_name: Option<&[solang_parser::pt::NamedArgument]> = None;
            let mut ctor_args_positional: Option<&[Expression]> = None;
            let mut ctor_name_matches = false;

            match rhs {
                Expression::NamedFunctionCall(_, func, args) => {
                    if let Expression::Variable(identifier) = func.as_ref() {
                        ctor_name_matches = identifier.name.eq_ignore_ascii_case(name);
                        if ctor_name_matches {
                            ctor_args_by_name = Some(args.as_slice());
                        }
                    }
                }
                Expression::FunctionCall(_, func, args) => {
                    if let Expression::Variable(identifier) = func.as_ref() {
                        ctor_name_matches = identifier.name.eq_ignore_ascii_case(name);
                        if ctor_name_matches {
                            ctor_args_positional = Some(args.as_slice());
                        }
                    }
                }
                _ => {}
            }

            if ctor_name_matches {
                for (index, field) in fields.iter().enumerate() {
                    let mut field_reference = reference.clone();
                    field_reference.field_path.push(StorageReferenceField {
                        key: field.key,
                        ty: field.ty.clone(),
                    });
                    field_reference.value_type = field.ty.clone();

                    let success = if let Some(named_args) = ctor_args_by_name {
                        if let Some(arg) = named_args.iter().find(|arg| arg.name.name == field.name)
                        {
                            lower_expression(&arg.expr, ctx, instructions)
                        } else {
                            push_default_for_storage_value_type(&field.ty, ctx, instructions)
                        }
                    } else if let Some(pos_args) = ctor_args_positional {
                        if let Some(arg) = pos_args.get(index) {
                            lower_expression(arg, ctx, instructions)
                        } else {
                            push_default_for_storage_value_type(&field.ty, ctx, instructions)
                        }
                    } else {
                        push_default_for_storage_value_type(&field.ty, ctx, instructions)
                    };

                    if success && !emit_storage_store(&field_reference, ctx, instructions) {
                        instructions.push(Instruction::Drop(ValueType::Any));
                    }
                }

                return;
            }
        }

        let success = lower_expression(rhs, ctx, instructions);
        if success {
            if !emit_storage_store(&reference, ctx, instructions) {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
        } else {
            instructions.push(Instruction::Drop(ValueType::Any));
        }
        return;
    }

    if let Expression::List(_, params) = lhs {
        // Tuple destructuring assignment: `(a, b) = expr;`
        // We represent tuples as arrays and assign by index.

        #[derive(Clone)]
        enum TupleTarget {
            Ignore,
            DeclaredLocal {
                local_index: usize,
                inferred_type: Option<ValueType>,
            },
            ExistingLocal(usize),
            // Task #156 — `(a, b) = (b, a)` where `a`/`b` are function parameters.
            // `resolve_local` only walks `local_index_map`; parameters live in
            // `param_index_map` and are written via `StoreParameter` / NeoVM
            // STARG. Prior to this target kind, the resolver returned `Invalid`
            // and the tuple element was silently dropped (visible effect: the
            // swap became a no-op on its own parameter slots).
            ExistingParameter(usize),
            ExistingState(usize),
            Storage(StorageReference),
            Nested(Vec<TupleTarget>),
            Invalid,
        }

        fn resolve_optional_tuple_target(
            parameter: &Option<solang_parser::pt::Parameter>,
            ctx: &mut LoweringContext,
        ) -> TupleTarget {
            let Some(parameter) = parameter else {
                return TupleTarget::Ignore;
            };

            if let Some(name) = parameter.name.as_ref() {
                // Declaration: `(bool ok, uint v) = ...`
                if ctx.is_local_in_current_scope(&name.name) {
                    ctx.record_error_with_suggestion(
                        format!("local variable '{}' redeclared", name.name),
                        "use a different variable name or assign to the existing variable instead of redeclaring",
                    );
                }

                let inferred_type = infer_type_from_expression(&parameter.ty, ctx);
                let local_index = ctx.allocate_local(name.name.clone(), inferred_type.clone());
                return TupleTarget::DeclaredLocal {
                    local_index,
                    inferred_type,
                };
            }

            if let Expression::List(_, nested_params) = &parameter.ty {
                let children = nested_params
                    .iter()
                    .map(|(_, param)| resolve_optional_tuple_target(param, ctx))
                    .collect();
                return TupleTarget::Nested(children);
            }

            // Assignment to an existing lvalue: `(ok, v) = ...`
            if let Some(reference) = resolve_storage_reference(&parameter.ty, ctx) {
                return TupleTarget::Storage(reference);
            }

            if let Expression::Variable(identifier) = &parameter.ty {
                if let Some(local_index) = ctx.resolve_local(&identifier.name) {
                    return TupleTarget::ExistingLocal(local_index);
                }

                // Task #156 — an identifier that doesn't match a local may still
                // be a function parameter. `lower_variable_expression` already
                // reads parameters via `LoadParameter`; we mirror that on the
                // write side with `StoreParameter` so a tuple-LVALUE `(a, b)`
                // on function params lands in the right slots. The RHS `(b, a)`
                // is fully stashed in `__tuple_assign` before any `StoreParameter`
                // fires, so the evaluate-RHS-first semantic holds.
                if let Some(param_index) = ctx.param_index_map.get(&identifier.name).copied() {
                    return TupleTarget::ExistingParameter(param_index);
                }

                if let Some(state_index) = ctx.state_index_map.get(&identifier.name).copied() {
                    return TupleTarget::ExistingState(state_index);
                }

                return TupleTarget::Invalid;
            }

            TupleTarget::Invalid
        }

        fn initialize_declared_tuple_targets(
            target: &TupleTarget,
            ctx: &mut LoweringContext,
            instructions: &mut Vec<Instruction>,
        ) {
            match target {
                TupleTarget::DeclaredLocal {
                    local_index,
                    inferred_type,
                } => {
                    if let Some(ty) = inferred_type.as_ref() {
                        push_default_for_value_type(ty, ctx, instructions);
                    } else {
                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                            BigInt::zero(),
                        )));
                    }
                    instructions.push(Instruction::StoreLocal(*local_index));
                }
                TupleTarget::Nested(children) => {
                    for child in children {
                        initialize_declared_tuple_targets(child, ctx, instructions);
                    }
                }
                _ => {}
            }
        }

        fn emit_tuple_element_load(
            tuple_local: usize,
            path: &[usize],
            instructions: &mut Vec<Instruction>,
        ) {
            instructions.push(Instruction::LoadLocal(tuple_local));
            for index in path {
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
                    *index as u64,
                ))));
                instructions.push(Instruction::ArrayGet);
            }
        }

        fn assign_tuple_target(
            tuple_local: usize,
            path: &mut Vec<usize>,
            target: &TupleTarget,
            ctx: &mut LoweringContext,
            instructions: &mut Vec<Instruction>,
        ) {
            match target {
                TupleTarget::Ignore => {}
                TupleTarget::Nested(children) => {
                    for (index, child) in children.iter().enumerate() {
                        path.push(index);
                        assign_tuple_target(tuple_local, path, child, ctx, instructions);
                        path.pop();
                    }
                }
                TupleTarget::DeclaredLocal { local_index, .. }
                | TupleTarget::ExistingLocal(local_index) => {
                    emit_tuple_element_load(tuple_local, path, instructions);
                    ctx.clear_call_data_local(*local_index);
                    instructions.push(Instruction::StoreLocal(*local_index));
                }
                TupleTarget::ExistingParameter(param_index) => {
                    // Task #156 — same shape as StoreLocal but writes to the NeoVM
                    // arg slot (STARG) so the caller-observed parameter is
                    // updated. The tuple RHS has already been materialised into
                    // `__tuple_assign`, so `(a, b) = (b, a)` reads pre-swap
                    // values before any STARG overwrites the originals.
                    emit_tuple_element_load(tuple_local, path, instructions);
                    instructions.push(Instruction::StoreParameter(*param_index));
                }
                TupleTarget::ExistingState(state_index) => {
                    emit_tuple_element_load(tuple_local, path, instructions);
                    if ctx.ensure_state_writable(*state_index) {
                        instructions.push(Instruction::StoreState(*state_index));
                    } else {
                        instructions.push(Instruction::Drop(ValueType::Any));
                    }
                }
                TupleTarget::Storage(reference) => {
                    emit_tuple_element_load(tuple_local, path, instructions);
                    if ctx.ensure_state_writable(reference.state_index) {
                        if !emit_storage_store(reference, ctx, instructions) {
                            instructions.push(Instruction::Drop(ValueType::Any));
                        }
                    } else {
                        instructions.push(Instruction::Drop(ValueType::Any));
                    }
                }
                TupleTarget::Invalid => {
                    emit_tuple_element_load(tuple_local, path, instructions);
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
            }
        }

        let targets: Vec<TupleTarget> = params
            .iter()
            .map(|(_, parameter)| resolve_optional_tuple_target(parameter, ctx))
            .collect();

        // Lower RHS into a temporary buffer so failures don't leave partial stack state.
        let mut rhs_instrs = Vec::new();
        if !lower_expression(rhs, ctx, &mut rhs_instrs) {
            // Ensure declared locals exist with default values to avoid cascading errors.
            for target in &targets {
                initialize_declared_tuple_targets(target, ctx, instructions);
            }
            return;
        }
        instructions.append(&mut rhs_instrs);

        // Detect `(a, b, ...) = this.<method>()` where `<method>` is an
        // externally-callable function in the current contract returning
        // multiple values. The external-call lowering routes `this.<method>`
        // through `System.Contract.Call` (which for self-dispatch runs the
        // callee's body, including its ABI-encoding epilogue) — so after
        // `lower_expression(rhs)` the stack carries an EVM-canonical
        // 32-byte-per-slot ABI-encoded ByteString, NOT a NeoVM `Array`.
        //
        // The standard destructure path below (STLOC → PICKITEM[i]) indexes
        // the top-of-stack item as an Array, which for a ByteString yields
        // individual bytes (each 0x00 for a well-aligned BE slot). Mirror
        // Solidity's `(a, b) = abi.decode(this.f(), (T1, T2, ...))` sugar by
        // ABI-decoding the buffer into an Array of native values, matching
        // the arity and static types of the LHS tuple targets. The existing
        // PICKITEM chain then operates on the Array correctly.
        //
        // Only static-slot types (uint*/int*/bool/address/bytesN) are
        // handled here; dynamic types (string/bytes/T[]) fall through to the
        // legacy path.
        if is_this_external_tuple_call(rhs, ctx) {
            // Walk the declared LHS parameter types (not the `TupleTarget`
            // enum, which is function-local) to collect static-slot types
            // suitable for the compile-time EVM ABI-decode path. Any miss
            // (Ignore/Invalid/nested/dynamic-type) falls back to the legacy
            // destructure.
            let target_static_types: Option<Vec<ValueType>> = params
                .iter()
                .map(|(_, param)| {
                    let parameter = param.as_ref()?;
                    let ty = infer_type_from_expression(&parameter.ty, ctx)?;
                    if abi_static_slot_count(&ty) == Some(1) {
                        Some(ty)
                    } else {
                        None
                    }
                })
                .collect();
            if let Some(static_types) = target_static_types {
                // Stack top: ABI-encoded ByteString of `static_types.len() * 32` bytes.
                let buffer_local = ctx.allocate_local(
                    "__this_tuple_abi_buf".to_string(),
                    Some(ValueType::ByteArray { fixed_len: None }),
                );
                instructions.push(Instruction::StoreLocal(buffer_local));

                // Materialise a fresh Array of the same arity as the LHS.
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(static_types.len() as u64),
                )));
                instructions.push(Instruction::NewArray {
                    element_type: ValueType::Any,
                });
                let array_local = ctx.allocate_local(
                    "__this_tuple_array".to_string(),
                    Some(ValueType::Array(Box::new(ValueType::Any))),
                );
                instructions.push(Instruction::StoreLocal(array_local));

                // For each slot, decode in-place from the buffer.
                for (index, value_type) in static_types.iter().enumerate() {
                    instructions.push(Instruction::LoadLocal(array_local));
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::from(index as u64),
                    )));
                    emit_abi_decode_static_slot(
                        buffer_local,
                        index,
                        value_type,
                        ctx,
                        instructions,
                    );
                    instructions.push(Instruction::ArraySet);
                }

                // Hand the populated Array to the legacy tuple-destructure below.
                instructions.push(Instruction::LoadLocal(array_local));
            }
        }

        let tuple_local = ctx.allocate_local("__tuple_assign".to_string(), None);
        instructions.push(Instruction::StoreLocal(tuple_local));

        for (index, target) in targets.iter().enumerate() {
            let mut path = vec![index];
            assign_tuple_target(tuple_local, &mut path, target, ctx, instructions);
        }

        return;
    }

    if matches!(lhs, Expression::ArraySubscript(_, _, Some(_))) {
        lower_array_store(lhs, rhs, ctx, instructions);
        return;
    }

    if let Expression::Variable(identifier) = lhs {
        if let Some(index) = ctx.resolve_local(&identifier.name) {
            match parse_low_level_call_data(rhs, ctx) {
                Ok(Some((method_name, encode_args))) => {
                    let mut lowered = true;
                    for arg in &encode_args {
                        if !lower_expression(arg, ctx, instructions) {
                            lowered = false;
                        }
                    }

                    if lowered {
                        if encode_args.is_empty() {
                            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                                BigInt::zero(),
                            )));
                            instructions.push(Instruction::NewArray {
                                element_type: ValueType::Any,
                            });
                            instructions.push(Instruction::CallBuiltin {
                                builtin: BuiltinCall::NativeCall {
                                    contract: NativeContract::StdLib,
                                    method: "serialize".to_string(),
                                },
                                arg_count: 1,
                            });
                        } else {
                            instructions.push(Instruction::CallBuiltin {
                                builtin: BuiltinCall::AbiEncode,
                                arg_count: encode_args.len(),
                            });
                        }
                        instructions.push(Instruction::StoreLocal(index));
                        ctx.set_call_data_local(index, method_name);
                    }
                }
                Ok(None) => {
                    // Wave-#28: same-contract dynamic-return path. See
                    // `try_lower_this_external_dynamic_assign` for details.
                    let dst_type = ctx.local_type(index).cloned();
                    let decoded = match dst_type.as_ref() {
                        Some(ty) => try_lower_this_external_dynamic_assign(
                            index, rhs, ty, ctx, instructions,
                        ),
                        None => false,
                    };
                    if !decoded && lower_expression(rhs, ctx, instructions) {
                        instructions.push(Instruction::StoreLocal(index));
                        ctx.clear_call_data_local(index);
                    }
                }
                Err(message) => {
                    ctx.record_error(message);
                    ctx.clear_call_data_local(index);
                }
            }
            return;
        }
        // Task #156 — plain `a = expr;` on a function parameter. Before this
        // branch, `ensure_local` further below would allocate a fresh local
        // (shadowing the name in `local_index_map` but NOT in
        // `param_index_map`), so subsequent reads — which check params first
        // in `lower_variable_expression` — would keep returning the original
        // parameter value. The write was silently lost. Emit `StoreParameter`
        // so the caller-visible slot is updated and later reads stay coherent.
        if let Some(index) = ctx.param_index_map.get(&identifier.name).copied() {
            if lower_expression(rhs, ctx, instructions) {
                instructions.push(Instruction::StoreParameter(index));
            }
            return;
        }
        if let Some(index) = ctx.state_index_map.get(&identifier.name).copied() {
            // Task #102: `storage_arr = memory_arr` must lower to element-wise
            // STORAGE_PUT plus a length-slot update (and delete any trailing
            // slots left over from a longer prior value). The default
            // StoreState path only writes the length slot — element slots
            // are keyed mapping-style via StoreMappingElement.
            if matches!(ctx.state_type(index), Some(ValueType::Array(_))) {
                lower_storage_array_assign_from_memory(index, rhs, ctx, instructions);
                return;
            }
            if lower_expression(rhs, ctx, instructions) {
                if ctx.ensure_state_writable(index) {
                    instructions.push(Instruction::StoreState(index));
                } else {
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
            }
            return;
        }

        let index = ctx.ensure_local(&identifier.name);
        match parse_low_level_call_data(rhs, ctx) {
            Ok(Some((method_name, encode_args))) => {
                let mut lowered = true;
                for arg in &encode_args {
                    if !lower_expression(arg, ctx, instructions) {
                        lowered = false;
                    }
                }

                if lowered {
                    if encode_args.is_empty() {
                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                            BigInt::zero(),
                        )));
                        instructions.push(Instruction::NewArray {
                            element_type: ValueType::Any,
                        });
                        instructions.push(Instruction::CallBuiltin {
                            builtin: BuiltinCall::NativeCall {
                                contract: NativeContract::StdLib,
                                method: "serialize".to_string(),
                            },
                            arg_count: 1,
                        });
                    } else {
                        instructions.push(Instruction::CallBuiltin {
                            builtin: BuiltinCall::AbiEncode,
                            arg_count: encode_args.len(),
                        });
                    }
                    instructions.push(Instruction::StoreLocal(index));
                    ctx.set_call_data_local(index, method_name);
                }
            }
            Ok(None) => {
                if lower_expression(rhs, ctx, instructions) {
                    instructions.push(Instruction::StoreLocal(index));
                    ctx.clear_call_data_local(index);
                }
            }
            Err(message) => {
                ctx.record_error(message);
                ctx.clear_call_data_local(index);
            }
        }
        return;
    }

    // Task #191 — memory struct field assignment: `s.field = rhs` where `s` is a
    // local or function parameter whose type is a struct represented as an
    // array of fields. The compound-assignment path (`s.field += v`,
    // `s.field++`) already handles the local-base case via ArrayGet/ArraySet,
    // but plain assignment had no MemberAccess-LHS branch and fell through to
    // the silent-drop fallback below. This made every `s.field = v` a no-op,
    // which in turn made free-function-attach chains like
    // `c.inc().inc().inc()` (where `inc(Counter memory c)` writes `c.value`)
    // appear to flatten to identity — the returned copy was always the
    // untouched initial struct.
    //
    // NeoVM Array slots hold an Rc<RefCell<Vec<StackItem>>>, so loading the
    // parameter/local and then ArraySet-ing a field mutates the underlying
    // storage in place. No write-back to the parameter/local slot is needed
    // because the slot still holds the same reference.
    if let Expression::MemberAccess(_, inner, member) = lhs {
        if let Expression::Variable(base) = inner.as_ref() {
            if let Some(ValueType::Struct { fields, .. }) = infer_type_from_expression(inner, ctx) {
                if let Some((field_index, _field)) = fields
                    .iter()
                    .enumerate()
                    .find(|(_, field)| field.name == member.name)
                {
                    let load_base = ctx
                        .resolve_local(&base.name)
                        .map(Instruction::LoadLocal)
                        .or_else(|| {
                            ctx.param_index_map
                                .get(&base.name)
                                .copied()
                                .map(Instruction::LoadParameter)
                        });

                    if let Some(load_base) = load_base {
                        // Emit: load base → push field_index → load rhs → ArraySet.
                        // ArraySet pops (array, index, value) and mutates the
                        // array in place.
                        instructions.push(load_base);
                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                            BigInt::from(field_index as u64),
                        )));
                        if !lower_expression(rhs, ctx, instructions) {
                            // Leave the half-built frame balanced: we've pushed
                            // base + index; drop them to keep the stack clean.
                            instructions.push(Instruction::Drop(ValueType::Any));
                            instructions.push(Instruction::Drop(ValueType::Any));
                            return;
                        }
                        instructions.push(Instruction::ArraySet);
                        return;
                    }
                }
            }
        }
    }

    // Fallback: evaluate RHS (if possible) and drop to allow compilation to continue.
    if lower_expression(rhs, ctx, instructions) {
        instructions.push(Instruction::Drop(ValueType::Any));
    }
}

// Task #102 — `storage_arr = memory_arr` bulk copy.
//
// Storage dynamic arrays in this IR lower their length to a scalar state slot
// (accessed via `LoadState`/`StoreState`) and their elements to mapping-style
// slots keyed by uint256 index (accessed via `LoadMappingElement`/
// `StoreMappingElement` with `key_types = [uint256]`). A plain `arr = m`
// therefore needs three things: (1) update the length slot, (2) copy each
// `m[i]` into the per-index slot, and (3) clear any trailing slots that were
// populated by a longer prior value so the state matches Solidity semantics.
//
// Slot-key scheme: identical to the existing `.push()` / `.pop()` / subscript
// paths — `state_index` plus a single uint256 key. Re-using those paths keeps
// the in-storage representation compatible with subsequent reads via
// `arr[i]` and with length-aware helpers like `arr.length`.
fn lower_storage_array_assign_from_memory(
    state_index: usize,
    rhs: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let writable = ctx.ensure_state_writable(state_index);

    // Lower RHS into a dedicated local so we can iterate over it twice (length
    // probe + per-index read) without re-evaluating side-effectful expressions.
    let src_local = ctx.allocate_local("__arr_assign_src".to_string(), None);
    if !lower_expression(rhs, ctx, instructions) {
        // RHS failed to lower; nothing useful we can do. `lower_expression`
        // already recorded an error, so just bail out quietly.
        return;
    }
    instructions.push(Instruction::StoreLocal(src_local));

    // If the target state is not writable (e.g. immutable after construction),
    // we've still evaluated the RHS for side effects; skip the mutation half.
    if !writable {
        return;
    }

    let uint256 = ValueType::Integer {
        signed: false,
        bits: 256,
    };

    // new_len = m.length (GetSize on the NeoVM array/buffer).
    let new_len_local = ctx.allocate_local("__arr_assign_new_len".to_string(), Some(uint256.clone()));
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(new_len_local));

    // old_len = current length slot (the state var itself holds the length).
    let old_len_local = ctx.allocate_local("__arr_assign_old_len".to_string(), Some(uint256.clone()));
    instructions.push(Instruction::LoadState(state_index));
    instructions.push(Instruction::StoreLocal(old_len_local));

    // Length slot := new_len. Writing the length before the loops matches the
    // existing `.pop()` helper's ordering and keeps `arr.length` consistent
    // if the copy loop ever panics partway through.
    instructions.push(Instruction::LoadLocal(new_len_local));
    instructions.push(Instruction::StoreState(state_index));

    let element_type = match ctx.state_type(state_index).cloned() {
        Some(ValueType::Array(elem)) => (*elem).clone(),
        _ => ValueType::Any,
    };

    // Copy loop: for (i = 0; i < new_len; i++) { slot[i] = src[i] }
    let copy_cond_label = ctx.next_label();
    let copy_end_label = ctx.next_label();
    let idx_local = ctx.allocate_local("__arr_assign_idx".to_string(), Some(uint256.clone()));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::StoreLocal(idx_local));

    instructions.push(Instruction::Label(copy_cond_label));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::LoadLocal(new_len_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    // IR JumpIf branches when the condition is false, so this exits the loop
    // once idx >= new_len.
    instructions.push(Instruction::JumpIf { target: copy_end_label });

    // slot[idx] := src[idx]
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::ArrayGet);
    instructions.push(Instruction::LoadLocal(idx_local));
    if matches!(element_type, ValueType::Array(_)) {
        instructions.push(Instruction::StoreArrayDeepCopy {
            state_index,
            key_types: vec![uint256.clone()],
        });
    } else {
        instructions.push(Instruction::StoreMappingElement {
            state_index,
            key_types: vec![uint256.clone()],
        });
    }

    // idx += 1
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(idx_local));
    instructions.push(Instruction::Jump { target: copy_cond_label });
    instructions.push(Instruction::Label(copy_end_label));

    // Deletion loop: for (i = new_len; i < old_len; i++) { slot[i] = default }
    // Matches the `.pop()` convention of overwriting removed slots with the
    // element type's default value so subsequent reads of the shrunk tail
    // return 0 / "" / etc. instead of stale data.
    let delete_cond_label = ctx.next_label();
    let delete_end_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(new_len_local));
    instructions.push(Instruction::StoreLocal(idx_local));

    instructions.push(Instruction::Label(delete_cond_label));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::LoadLocal(old_len_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: delete_end_label });

    push_default_for_value_type(&element_type, ctx, instructions);
    instructions.push(Instruction::LoadLocal(idx_local));
    if matches!(element_type, ValueType::Array(_)) {
        instructions.push(Instruction::StoreArrayDeepCopy {
            state_index,
            key_types: vec![uint256.clone()],
        });
    } else {
        instructions.push(Instruction::StoreMappingElement {
            state_index,
            key_types: vec![uint256.clone()],
        });
    }

    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(idx_local));
    instructions.push(Instruction::Jump { target: delete_cond_label });
    instructions.push(Instruction::Label(delete_end_label));
}

/// True if `rhs` is an external-target member call that, at the lowered
/// bytecode level, goes through `System.Contract.Call` and therefore
/// leaves an EVM-canonical ABI-encoded ByteString on the stack for
/// multi-return methods. Covers three shapes:
///
///   1. `this.<method>(...)`            — Variable("this") inner
///   2. `IContract(addr).<method>(...)` — FunctionCall(ContractType, [addr]) inner
///   3. `<addr-typed var>.<method>(...)` — inner infers to `ValueType::Address`
///
/// Without this broader match, (2) and (3) skipped the compile-time
/// ABI-decode and the destructure PICKITEM chain indexed the raw
/// encoded bytes — producing zero-valued per-byte reads.
fn is_this_external_tuple_call(rhs: &Expression, ctx: &mut LoweringContext) -> bool {
    let Expression::FunctionCall(_, func, _) = rhs else {
        return false;
    };
    let Expression::MemberAccess(_, inner, _member) = func.as_ref() else {
        return false;
    };

    // Shape 1 — `this.method()`.
    if matches!(inner.as_ref(), Expression::Variable(id) if id.name == "this") {
        return true;
    }

    // Shape 2 — `IContract(addr).method()` / `IContract(other).method()`.
    // The inner is a single-arg FunctionCall whose callee resolves to a
    // contract / interface type name.
    if let Expression::FunctionCall(_, cast_func, cast_args) = inner.as_ref() {
        if cast_args.len() == 1 {
            let is_contract_type = match cast_func.as_ref() {
                Expression::Variable(id) => ctx.is_contract_type_name(&id.name),
                Expression::MemberAccess(_, _, id) => ctx.is_contract_type_name(&id.name),
                _ => false,
            };
            if is_contract_type {
                return true;
            }
        }
    }

    // Shape 3 — any other inner that type-infers to `Address`. This catches
    // stored state vars / parameters of contract/interface types that flow
    // through type inference as `Address`.
    matches!(
        infer_type_from_expression(inner.as_ref(), ctx),
        Some(ValueType::Address)
    )
}

/// Wave-#28 fix — same-contract `T[] memory got = this.method();` (and the
/// interface-cast / address-typed call shapes covered by
/// `is_this_external_tuple_call`) where `method` returns a single
/// dynamic-ABI-shaped value (`T[]` / `bytes` / `string`).
///
/// Background. Bug #25 wired up `abi.decode(buf, (T[]))` correctly in the
/// builtins helpers (`emit_abi_decode_dynamic_top_level`). The sibling
/// fix (above, ~line 347) extends the same compile-time decode to the
/// tuple-destructure RHS — but only for **static-1-slot** members. A
/// single-LHS dynamic return (`uint256[] memory got = this.makeArray()`)
/// dropped through to the generic `Variable` branch which `StoreLocal`'d
/// the raw ABI ByteString. Subsequent `got.length` then read the byte
/// length of the encoded payload (e.g. 224 = 32 head + 32 length-prefix +
/// 5*32 elements for a five-element `uint256[]`) instead of the element
/// count (5).
///
/// This helper, called from both the plain-assignment branch and the
/// `lower_variable_definition_statement` initializer path, detects the
/// pattern, runs the dynamic-decode chain on the buffer, and stores the
/// resulting NeoVM Array (or ByteString for string/bytes) into the
/// destination slot. Callers must have already verified the LHS slot
/// type — passed via `dst_type` — and resolved the slot index. Returns
/// `true` if the decode-and-store path fired (the caller must skip its
/// own lowering of `rhs`); `false` otherwise (caller falls back).
///
/// Static-slot single-LHS returns (`uint256 x = this.foo()`) are NOT
/// routed through here — they are handled by the existing single-result
/// dispatch lowering elsewhere in the call chain. Dynamic types are the
/// novel case: their wire shape (head-offset + length + payload) needs
/// the dedicated walker.
fn try_lower_this_external_dynamic_assign(
    slot: usize,
    rhs: &Expression,
    dst_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if !abi_dynamic_decode_value_type_is_supported(dst_type) {
        return false;
    }
    if !is_this_external_tuple_call(rhs, ctx) {
        return false;
    }

    // Lower the call into a temp buffer; on failure restore the caller's
    // instruction list to avoid half-built stack frames (matches the
    // tuple-destructure branch's `pre_len`/`truncate` discipline).
    let pre_len = instructions.len();
    if !lower_expression(rhs, ctx, instructions) {
        instructions.truncate(pre_len);
        return false;
    }

    let buffer_local = ctx.allocate_local(
        "__this_dyn_abi_buf".to_string(),
        Some(ValueType::ByteArray { fixed_len: None }),
    );
    instructions.push(Instruction::StoreLocal(buffer_local));

    // Decode the top-level dynamic value (`T[]` => Array of decoded
    // elements; `string`/`bytes` => ByteString). Stack on exit:
    // `[decoded_value]`.
    emit_abi_decode_dynamic_top_level(buffer_local, dst_type, ctx, instructions);

    instructions.push(Instruction::StoreLocal(slot));
    ctx.clear_call_data_local(slot);
    true
}
