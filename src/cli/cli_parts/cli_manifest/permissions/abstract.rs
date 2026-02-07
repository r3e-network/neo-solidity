fn pop_value(stack: &mut Vec<AbstractValue>) -> Result<AbstractValue, ()> {
    stack.pop().ok_or(())
}

fn pop_n(stack: &mut Vec<AbstractValue>, n: usize) -> Result<(), ()> {
    if stack.len() < n {
        return Err(());
    }
    for _ in 0..n {
        stack.pop();
    }
    Ok(())
}

fn apply_instruction(state: &mut AbstractState, instr: &ir::Instruction) -> Result<(), ()> {
    use ir::Instruction::*;

    match instr {
        Drop(_) => {
            pop_value(&mut state.stack)?;
        }
        LoadParameter(_) => state.stack.push(AbstractValue::Unknown),
        PushLiteral(lit) => state.stack.push(AbstractValue::Literal(lit.clone())),
        Return | ReturnVoid | ReturnDefault(_) | Abort => {}
        AbortMsg | Throw => {
            pop_value(&mut state.stack)?;
        }
        BinaryOp(_) => {
            pop_n(&mut state.stack, 2)?;
            state.stack.push(AbstractValue::Unknown);
        }
        LoadState(_) => state.stack.push(AbstractValue::Unknown),
        StoreState(_) => {
            pop_value(&mut state.stack)?;
        }
        LoadStorageDynamic => {
            pop_value(&mut state.stack)?;
            state.stack.push(AbstractValue::Unknown);
        }
        LoadLocal(index) => {
            let value = state
                .locals
                .get(*index)
                .cloned()
                .unwrap_or(AbstractValue::Unknown);
            state.stack.push(value);
        }
        StoreLocal(index) => {
            let value = pop_value(&mut state.stack)?;
            if let Some(slot) = state.locals.get_mut(*index) {
                *slot = value;
            }
        }
        LoadMappingElement { key_types, .. } => {
            pop_n(&mut state.stack, key_types.len())?;
            state.stack.push(AbstractValue::Unknown);
        }
        StoreMappingElement { key_types, .. } => {
            pop_n(&mut state.stack, key_types.len() + 1)?;
        }
        LoadStructField { key_types, .. } => {
            pop_n(&mut state.stack, key_types.len())?;
            state.stack.push(AbstractValue::Unknown);
        }
        StoreStructField { key_types, .. } => {
            pop_n(&mut state.stack, key_types.len() + 1)?;
        }
        LoadStructArrayElement { key_types, .. } => {
            pop_n(&mut state.stack, key_types.len() + 1)?;
            state.stack.push(AbstractValue::Unknown);
        }
        StoreStructArrayElement { key_types, .. } => {
            pop_n(&mut state.stack, key_types.len() + 2)?;
        }
        LoadRuntimeValue(_) => state.stack.push(AbstractValue::Unknown),
        GetSize => {
            pop_value(&mut state.stack)?;
            state.stack.push(AbstractValue::Unknown);
        }
        CallFunction { arg_count, .. } => {
            pop_n(&mut state.stack, *arg_count)?;
            // For manifest inference we conservatively treat internal calls as producing a
            // single stack item (e.g., `null`, a value, or an array-wrapped tuple).
            state.stack.push(AbstractValue::Unknown);
        }
        CallBuiltin { builtin, arg_count } => {
            pop_n(&mut state.stack, *arg_count)?;
            match builtin {
                // Track `this` / `address(this)` values so we can avoid emitting
                // impossible manifest permissions for self-calls.
                ir::BuiltinCall::Syscall(name)
                    if name == "System.Runtime.GetExecutingScriptHash" =>
                {
                    state.stack.push(AbstractValue::ExecutingScriptHash);
                }
                _ => state.stack.push(AbstractValue::Unknown),
            }
        }
        EmitEvent { arg_count, .. } | EmitEventByName { arg_count, .. } => {
            pop_n(&mut state.stack, arg_count + 1)?;
        }
        Convert { .. } => {
            pop_value(&mut state.stack)?;
            state.stack.push(AbstractValue::Unknown);
        }
        NewBuffer => {
            pop_value(&mut state.stack)?;
            state.stack.push(AbstractValue::Unknown);
        }
        NewArray { .. } => {
            pop_value(&mut state.stack)?;
            state.stack.push(AbstractValue::Unknown);
        }
        ArrayGet => {
            pop_n(&mut state.stack, 2)?;
            state.stack.push(AbstractValue::Unknown);
        }
        ArraySet => {
            pop_n(&mut state.stack, 3)?;
        }
        MemCpy => {
            // Stack order: [dst, dst_offset, src, src_offset, count]
            pop_n(&mut state.stack, 5)?;
            state.stack.push(AbstractValue::Unknown);
        }
        ReverseItems => {
            pop_value(&mut state.stack)?;
        }
        BitwiseNot | LogicalNot => {
            pop_value(&mut state.stack)?;
            state.stack.push(AbstractValue::Unknown);
        }
        Try { .. } | EndTry { .. } | Jump { .. } | Label(_) => {}
        JumpIf { .. } => {
            pop_value(&mut state.stack)?;
        }
        Dup => {
            let value = state.stack.last().cloned().unwrap_or(AbstractValue::Unknown);
            state.stack.push(value);
        }
        Swap => {
            let len = state.stack.len();
            if len >= 2 {
                state.stack.swap(len - 1, len - 2);
            }
        }
    }

    Ok(())
}
