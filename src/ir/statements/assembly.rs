use super::*;

pub(crate) fn lower_special_assembly(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    match ctx.function_name.as_str() {
        "extsload" | "exttload" => {
            lower_extsload_single(ctx, instructions)
                || lower_extsload_range(ctx, instructions)
                || lower_extsload_slots(ctx, instructions)
        }
        _ => false,
    }
}

pub(crate) fn lower_extsload_single(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let slot_index = match ctx.param_index_map.get("slot").copied() {
        Some(index) if ctx.param_index_map.len() == 1 => index,
        _ => return false,
    };

    instructions.push(Instruction::LoadParameter(slot_index));
    instructions.push(Instruction::LoadStorageDynamic);
    instructions.push(Instruction::Return);
    true
}

pub(crate) fn lower_extsload_range(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let start_index = match ctx.param_index_map.get("startSlot").copied() {
        Some(index) => index,
        None => return false,
    };
    let count_index = match ctx.param_index_map.get("nSlots").copied() {
        Some(index) => index,
        None => return false,
    };

    if ctx.param_index_map.len() != 2 {
        return false;
    }

    let start_local = ctx.allocate_local("__extsload_start".to_string(), None);
    instructions.push(Instruction::LoadParameter(start_index));
    instructions.push(Instruction::StoreLocal(start_local));

    let count_local = ctx.allocate_local("__extsload_count".to_string(), None);
    instructions.push(Instruction::LoadParameter(count_index));
    instructions.push(Instruction::StoreLocal(count_local));

    let array_element_type = ValueType::ByteArray {
        fixed_len: Some(32),
    };
    let array_value_type = ValueType::Array(Box::new(array_element_type.clone()));
    let array_local = ctx.allocate_local(
        "__extsload_array".to_string(),
        Some(array_value_type.clone()),
    );
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::NewArray {
        element_type: array_element_type,
    });
    instructions.push(Instruction::StoreLocal(array_local));

    let index_local = ctx.allocate_local("__extsload_index".to_string(), None);
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::StoreLocal(index_local));

    let value_local = ctx.allocate_local("__extsload_value".to_string(), None);

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();

    instructions.push(Instruction::Label(loop_label));
    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: end_label });

    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::LoadStorageDynamic);
    instructions.push(Instruction::StoreLocal(value_local));

    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::LoadLocal(value_local));
    instructions.push(Instruction::ArraySet);

    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(1u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(index_local));

    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(1u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(start_local));

    instructions.push(Instruction::Jump { target: loop_label });
    instructions.push(Instruction::Label(end_label));
    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::Return);
    true
}

pub(crate) fn lower_extsload_slots(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let slots_index = match ctx.param_index_map.get("slots").copied() {
        Some(index) if ctx.param_index_map.len() == 1 => index,
        _ => return false,
    };

    let slots_local = ctx.allocate_local("__extsload_slots".to_string(), None);
    instructions.push(Instruction::LoadParameter(slots_index));
    instructions.push(Instruction::StoreLocal(slots_local));

    let count_local = ctx.allocate_local("__extsload_count".to_string(), None);
    instructions.push(Instruction::LoadLocal(slots_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(count_local));

    let slots_array_element = ValueType::ByteArray {
        fixed_len: Some(32),
    };
    let slots_array_type = ValueType::Array(Box::new(slots_array_element.clone()));
    let array_local = ctx.allocate_local(
        "__extsload_array".to_string(),
        Some(slots_array_type.clone()),
    );
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::NewArray {
        element_type: slots_array_element,
    });
    instructions.push(Instruction::StoreLocal(array_local));

    let index_local = ctx.allocate_local("__extsload_index".to_string(), None);
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::StoreLocal(index_local));

    let value_local = ctx.allocate_local("__extsload_value".to_string(), None);

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();

    instructions.push(Instruction::Label(loop_label));
    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: end_label });

    instructions.push(Instruction::LoadLocal(slots_local));
    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::ArrayGet);
    instructions.push(Instruction::LoadStorageDynamic);
    instructions.push(Instruction::StoreLocal(value_local));

    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::LoadLocal(value_local));
    instructions.push(Instruction::ArraySet);

    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(1u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(index_local));

    instructions.push(Instruction::Jump { target: loop_label });
    instructions.push(Instruction::Label(end_label));
    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::Return);
    true
}

// ===========================================================================
// Task #99 — yul (inline assembly) lowering.
//
// Scope (per the task plan):
//   * mstore/mload/return  — the three memory opcodes exercised by the
//     `batch39_n3_yul_mstore_mload_return_ignored_until_task_99` harness.
//   * `let x := expr` / `x := expr` — yul local variable declarations and
//     assignments (the harness uses `let v := mload(0x40)`).
//   * A handful of pure-arithmetic opcodes: add/sub/mul/div + iszero/eq/lt/gt
//     which show up alongside mstore/mload in real yul idioms. Everything
//     else (for/switch/sload/sstore/call/...) falls back to the legacy no-op
//     warning path so contracts using more exotic yul still compile.
//
// Memory model:
//   Yul exposes a flat byte-addressed memory. We materialise it as a NeoVM
//   Buffer allocated on first mstore/mload inside the block. Because we
//   can't know the maximum offset ahead of time without a second static
//   pass, we pre-size conservatively to 256 bytes (enough for the
//   `mstore(0x40, ...)` free-memory-pointer slot plus a 32-byte return slot
//   at offset 0). Accesses above that bound trap at runtime with
//   `MEMCPY: range out of bounds` — an acceptable signal for "this yul block
//   is too ambitious for the stub."
//
// Return handling:
//   yul `return(offset, length)` terminates the function with the memory
//   slice as its return payload. For a Solidity function declared
//   `returns (uintN)` / `returns (intN)` / `returns (bool)` (the common
//   case, and the one exercised by the batch39_n3 harness), we re-interpret
//   the slice as a big-endian integer so the main-frame RET emits the
//   expected 32-byte BE-packed value. Otherwise the raw buffer is returned.

/// Lower a yul block, returning `true` iff every statement in the block was
/// successfully lowered. A `false` result causes the enclosing assembly
/// statement to fall back to the legacy no-op warning path, preserving
/// compilation for contracts using unsupported yul constructs.
pub(crate) fn lower_yul_block(
    block: &solang_parser::pt::YulBlock,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    // Snapshot the instruction stream length so we can roll back on failure.
    let snapshot = instructions.len();

    let mut state = YulLoweringState::new();

    for stmt in &block.statements {
        if !lower_yul_statement(stmt, &mut state, ctx, instructions) {
            // Roll back any partial IR so the enclosing caller can emit the
            // legacy no-op warning without leaving orphan instructions.
            instructions.truncate(snapshot);
            return false;
        }
    }

    // Prepend memory-buffer AND transient-map initialisation at the snapshot
    // boundary. The memory buffer is always per-block (yul memory is scratch,
    // cleared each call to `assembly`). The transient map is initialised
    // only in the block that first introduces `__yul_transient` — later
    // blocks in the same function resolve the slot by name and reuse the
    // already-live map (EIP-1153 per-tx persistence across yul blocks).
    // Task #184 mirrors the transient-map pattern for `__yul_returndata`:
    // allocated function-wide, initialised once in the first block that
    // references it.
    let needs_memory_init = state.memory_local.is_some();
    let needs_transient_init = state.transient_allocated_here;
    let needs_returndata_init = state.returndata_allocated_here;

    if needs_memory_init || needs_transient_init || needs_returndata_init {
        let mut init: Vec<Instruction> = Vec::new();
        if let Some(mem_local) = state.memory_local {
            init.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(YUL_MEMORY_BYTES),
            )));
            init.push(Instruction::NewBuffer);
            init.push(Instruction::StoreLocal(mem_local));
        }
        if needs_transient_init {
            // `transient_allocated_here ⇒ transient_local` is a compiler
            // invariant (`ensure_transient` sets both together). Under
            // `panic = "abort"` a future refactor that broke the pairing
            // would abort the process instantly; degrade to a diagnostic
            // and roll back this block instead.
            let Some(transient_local) = state.transient_local else {
                ctx.record_error(
                    "internal: transient_allocated_here set but transient_local was not populated",
                );
                instructions.truncate(snapshot);
                return false;
            };
            init.push(Instruction::NewMap);
            init.push(Instruction::StoreLocal(transient_local));
        }
        if needs_returndata_init {
            let Some(rd_local) = state.returndata_local else {
                ctx.record_error(
                    "internal: returndata_allocated_here set but returndata_local was not populated",
                );
                instructions.truncate(snapshot);
                return false;
            };
            // Task #184 — seed with an empty ByteString so `GetSize` returns 0
            // and `returndatacopy(_, _, 0)` is a no-op. `NewBuffer(0)` would
            // emit a zero-length Buffer which also reports Size 0, but the
            // empty-ByteString literal matches the shape of a real callee's
            // ByteString return value so the follow-up "stash after call"
            // plumbing drops into the same slot without a type change.
            init.push(Instruction::PushLiteral(
                LiteralValue::ByteArray(Vec::new()),
            ));
            init.push(Instruction::StoreLocal(rd_local));
        }
        let tail = instructions.split_off(snapshot);
        instructions.extend(init);
        instructions.extend(tail);
    }

    true
}

/// Size in bytes of the Yul memory buffer. Chosen to cover the standard
/// `0x40` free-memory-pointer slot (64–95) plus a 32-byte return slot at
/// 0–31 plus some scratch space. Yul blocks that mstore above this bound
/// will trap at runtime with `MEMCPY: range out of bounds`.
pub(crate) const YUL_MEMORY_BYTES: u64 = 256;

/// Per-block lowering state. Tracks the NeoVM local slot holding the yul
/// memory buffer (lazily allocated on first mstore/mload) and a map of yul
/// variable names to NeoVM local slots.
pub(crate) struct YulLoweringState {
    memory_local: Option<usize>,
    /// Task #100 — the transient-storage map local. Unlike `memory_local`,
    /// this is allocated at the FUNCTION level (via the shared
    /// `__yul_transient` name) so it persists across multiple `assembly { }`
    /// blocks in the same function — which is what EIP-1153 requires
    /// (tstore in block A; tload in block B must return the stored value).
    transient_local: Option<usize>,
    /// True iff this block is the one that first introduced the transient
    /// map. The block prelude will emit `NEWMAP + StoreLocal` so the map is
    /// initialised before any tstore/tload. Subsequent yul blocks in the
    /// same function find the slot via `ctx.resolve_local("__yul_transient")`
    /// and skip the init.
    transient_allocated_here: bool,
    /// Task #184 — the yul-visible returndata buffer. EVM exposes the result
    /// of the most-recent external call through `returndatasize` / `returndatacopy`.
    /// NeoVM has no native returndata concept, so we model it as a per-function
    /// ByteArray local `__yul_returndata` that is initialised to the empty
    /// buffer (returndatasize = 0). Because Task #184 covers the degenerate
    /// "no preceding external call" surface only, the buffer starts empty and
    /// `returndatacopy` panics (Panic 0x32 / "returndata out of bounds") on
    /// any non-zero-length read. A follow-up task can extend this to stash
    /// the callee's return bytes after `Target(t).f()` / CALLT / DYNCALL so
    /// `returndatacopy` recovers the real payload.
    returndata_local: Option<usize>,
    /// True iff this block is the one that first introduced the returndata
    /// buffer. The block prelude will emit `PUSH0 ; NEWBUFFER ; StoreLocal`
    /// so the buffer is initialised (to zero length) before any
    /// returndatacopy. Subsequent yul blocks in the same function find the
    /// slot via `ctx.resolve_local("__yul_returndata")` and skip the init.
    returndata_allocated_here: bool,
    yul_locals: std::collections::HashMap<String, usize>,
}

impl YulLoweringState {
    pub(crate) fn new() -> Self {
        Self {
            memory_local: None,
            transient_local: None,
            transient_allocated_here: false,
            returndata_local: None,
            returndata_allocated_here: false,
            yul_locals: std::collections::HashMap::new(),
        }
    }

    /// Lazily allocate the yul memory buffer local. The caller is responsible
    /// for emitting the NEWBUFFER + StoreLocal prelude — we do this centrally
    /// in `lower_yul_block` after the whole body has been lowered so the
    /// allocation always sits before the first use.
    pub(crate) fn ensure_memory(&mut self, ctx: &mut LoweringContext) -> usize {
        if let Some(slot) = self.memory_local {
            return slot;
        }
        let slot = ctx.allocate_local("__yul_memory".to_string(), None);
        self.memory_local = Some(slot);
        slot
    }

    /// Lazily allocate the yul transient-storage map local. Unlike
    /// `ensure_memory`, this is keyed on a FUNCTION-wide name so later yul
    /// blocks in the same function pick up the same slot (and the same
    /// already-initialised map). The block that first introduces the local
    /// sets `transient_allocated_here` so `lower_yul_block` emits the
    /// NEWMAP prelude before the body.
    pub(crate) fn ensure_transient(&mut self, ctx: &mut LoweringContext) -> usize {
        if let Some(slot) = self.transient_local {
            return slot;
        }
        // If an earlier yul block in the SAME function already allocated
        // `__yul_transient`, reuse that slot — the map is already live and
        // any stored values must persist (EIP-1153 per-tx semantics).
        if let Some(existing) = ctx.resolve_local("__yul_transient") {
            self.transient_local = Some(existing);
            return existing;
        }
        // First use anywhere in the function: allocate the slot and mark
        // this block as the one responsible for emitting the NEWMAP
        // init prelude.
        let slot = ctx.allocate_local("__yul_transient".to_string(), None);
        self.transient_local = Some(slot);
        self.transient_allocated_here = true;
        slot
    }

    /// Task #184 — lazily allocate the yul returndata buffer local. Mirrors
    /// `ensure_transient` in that the slot is function-scoped (so a hypothetical
    /// future "stash return value after Target(t).f()" shim can write to the
    /// same buffer that `returndatacopy` reads from across yul blocks). The
    /// block that first introduces the slot sets `returndata_allocated_here`
    /// so `lower_yul_block` emits a `PUSH 0 ; NEWBUFFER ; StoreLocal` prelude
    /// — an empty buffer models `returndatasize() == 0`, which is what the
    /// minimal Task #184 surface needs (no preceding external call).
    pub(crate) fn ensure_returndata(&mut self, ctx: &mut LoweringContext) -> usize {
        if let Some(slot) = self.returndata_local {
            return slot;
        }
        if let Some(existing) = ctx.resolve_local("__yul_returndata") {
            self.returndata_local = Some(existing);
            return existing;
        }
        let slot = ctx.allocate_local("__yul_returndata".to_string(), None);
        self.returndata_local = Some(slot);
        self.returndata_allocated_here = true;
        slot
    }
}

/// Lower a slice of yul statements with the standard "stop on first
/// failure" semantics. Used by the `Block`, `If`, `For` (init / body /
/// post), and `Switch` (case body, default body) arms of
/// `lower_yul_statement` — six sites that previously inlined the same
/// `for s in stmts { if !lower_yul_statement(s, …) { return false; } }`
/// loop. Returns `true` iff every statement lowered successfully.
fn lower_yul_block_stmts(
    stmts: &[solang_parser::pt::YulStatement],
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    for s in stmts {
        if !lower_yul_statement(s, state, ctx, instructions) {
            return false;
        }
    }
    true
}

/// Lower a yul 2-arg arithmetic / bitwise call (`add`/`sub`/`mul` and
/// `and`/`or`/`xor`) and emit a single `BinaryOp`. The arity check,
/// recursive argument lowering, and operator lookup were duplicated
/// across the two match arms. `op_for` returns the `BinaryOperator` for
/// the matched name; the caller is responsible for restricting `name`
/// to the set the closure handles.
fn lower_two_arg_yul_call(
    call: &solang_parser::pt::YulFunctionCall,
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    op_for: impl Fn(&str) -> BinaryOperator,
) -> bool {
    if call.arguments.len() != 2 {
        return false;
    }
    if !lower_yul_expression(&call.arguments[0], state, ctx, instructions) {
        return false;
    }
    if !lower_yul_expression(&call.arguments[1], state, ctx, instructions) {
        return false;
    }
    instructions.push(Instruction::BinaryOp(op_for(&call.id.name)));
    true
}

pub(crate) fn lower_yul_statement(
    stmt: &solang_parser::pt::YulStatement,
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    use solang_parser::pt::YulStatement;
    match stmt {
        YulStatement::VariableDeclaration(_, idents, init) => {
            if idents.len() != 1 {
                // Multi-return yul locals (`let a, b := f()`) are out of
                // scope for Task #99.
                return false;
            }
            let ident = &idents[0];
            let name = ident.id.name.clone();
            let slot = ctx.allocate_local(format!("__yul_var_{name}"), None);
            state.yul_locals.insert(name, slot);
            if let Some(expr) = init {
                if !lower_yul_expression(expr, state, ctx, instructions) {
                    return false;
                }
                instructions.push(Instruction::StoreLocal(slot));
            } else {
                // yul default-inits decls to 0.
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::zero(),
                )));
                instructions.push(Instruction::StoreLocal(slot));
            }
            true
        }
        YulStatement::Assign(_, targets, value) => {
            if targets.len() != 1 {
                return false;
            }
            let solang_parser::pt::YulExpression::Variable(ident) = &targets[0] else {
                return false;
            };
            // Task #100 — resolution order matches yul's reference semantics:
            // (1) yul-local `let x` bindings in this block; (2) outer Solidity
            // locals the assembly block has visibility on (e.g. `uint v; assembly { v := tload(0) }`).
            // Task #183 — also (3) outer Solidity parameters (`function f(uint v) { assembly { v := tload(0) }}`).
            // Parameters live in `param_index_map` and are written via
            // `StoreParameter` / NeoVM STARG. Mirror Task #156's tuple-assign
            // fix (TupleTarget::ExistingParameter) on the yul write side.
            enum YulAssignTarget {
                Local(usize),
                Parameter(usize),
            }
            let target = if let Some(&slot) = state.yul_locals.get(&ident.name) {
                YulAssignTarget::Local(slot)
            } else if let Some(&param_index) = ctx.param_index_map.get(&ident.name) {
                YulAssignTarget::Parameter(param_index)
            } else if let Some(slot) = ctx.resolve_local(&ident.name) {
                YulAssignTarget::Local(slot)
            } else {
                // Assigning to an un-declared yul identifier: out of scope.
                return false;
            };
            if !lower_yul_expression(value, state, ctx, instructions) {
                return false;
            }
            match target {
                YulAssignTarget::Local(slot) => {
                    instructions.push(Instruction::StoreLocal(slot));
                }
                YulAssignTarget::Parameter(param_index) => {
                    instructions.push(Instruction::StoreParameter(param_index));
                }
            }
            true
        }
        YulStatement::FunctionCall(call) => {
            lower_yul_function_call_as_statement(call, state, ctx, instructions)
        }
        YulStatement::Block(inner) => {
            lower_yul_block_stmts(&inner.statements, state, ctx, instructions)
        }
        // Task #200 — yul `if <cond> <body>`. The cond expression evaluates
        // to a yul uint256 (0 ⇒ false, non-zero ⇒ true). The NeoVM IR
        // `JumpIf` jumps when the top-of-stack is FALSY (see
        // `src/cli/bytecode/bytecode_emit_ir.rs:319` — "IR JumpIf branches
        // when the condition is false."), so the lowering is:
        //     <eval cond>
        //     JumpIf end_label      ; skip body when cond == 0
        //     <eval body>
        //     Label(end_label)
        YulStatement::If(_, cond, body) => {
            let end_label = ctx.next_label();
            if !lower_yul_expression(cond, state, ctx, instructions) {
                return false;
            }
            instructions.push(Instruction::JumpIf { target: end_label });
            if !lower_yul_block_stmts(&body.statements, state, ctx, instructions) {
                return false;
            }
            instructions.push(Instruction::Label(end_label));
            true
        }
        // Task #200 — yul `for { init } cond { post } { body }`. Classic
        // condition-top loop. `init` statements run once before entering;
        // `cond` is re-evaluated at the top of every iteration and a FALSY
        // value exits the loop; `post` runs after each body iteration and
        // is the continue-target (so `continue` re-enters at post, then
        // falls through to the condition check). Mirrors the canonical
        // Solidity `for` lowering in
        // src/ir/statements/dispatch/control_flow.rs::lower_for_statement.
        YulStatement::For(for_stmt) => {
            // Init statements are lowered in the enclosing scope so any
            // yul-locals they declare (via `let i := 0`) remain visible
            // to the condition / post / body — which matches yul semantics
            // (`for { let i := 0 } lt(i, n) { i := add(i, 1) } { ... }`).
            if !lower_yul_block_stmts(
                &for_stmt.init_block.statements,
                state,
                ctx,
                instructions,
            ) {
                return false;
            }

            let loop_start = ctx.next_label();
            let post_label = ctx.next_label();
            let loop_end = ctx.next_label();

            instructions.push(Instruction::Label(loop_start));
            if !lower_yul_expression(&for_stmt.condition, state, ctx, instructions) {
                return false;
            }
            instructions.push(Instruction::JumpIf { target: loop_end });

            // Register break/continue targets so yul `break` / `continue`
            // (if/when we lower them) and — for symmetry with the Solidity
            // control-flow lowering — land on the right labels. `continue`
            // jumps to `post_label` (run post, then re-check cond).
            ctx.push_loop(post_label, loop_end);
            let body_ok = lower_yul_block_stmts(
                &for_stmt.execution_block.statements,
                state,
                ctx,
                instructions,
            );
            ctx.pop_loop();
            if !body_ok {
                return false;
            }

            instructions.push(Instruction::Label(post_label));
            if !lower_yul_block_stmts(
                &for_stmt.post_block.statements,
                state,
                ctx,
                instructions,
            ) {
                return false;
            }
            instructions.push(Instruction::Jump { target: loop_start });
            instructions.push(Instruction::Label(loop_end));
            true
        }
        // Task #200 — yul `switch <expr> case v1 { ... } ... default { ... }`.
        // Evaluate the discriminant once into a fresh local, then emit a
        // linear case chain: for each case, compare the local against the
        // case literal and `JumpIf` over the body when unequal. After the
        // body, `Jump` to the shared end label. The default block (if any)
        // sits just before the end label so an unmatched dispatch falls
        // through to it naturally. Yul guarantees (per foundry-solang-parser
        // at solang-parser-0.3.5/src/pt.rs:1593) that `cases` contains only
        // `YulSwitchOptions::Case` and `default` is exactly `Default`.
        YulStatement::Switch(switch_stmt) => {
            if !lower_yul_expression(&switch_stmt.condition, state, ctx, instructions) {
                return false;
            }
            let disc_label = ctx.next_label();
            let disc_local = ctx.allocate_local(format!("__yul_switch_disc_{disc_label}"), None);
            instructions.push(Instruction::StoreLocal(disc_local));

            let end_label = ctx.next_label();

            for case_opt in &switch_stmt.cases {
                let solang_parser::pt::YulSwitchOptions::Case(_, value_expr, body) = case_opt
                else {
                    // Parser guarantees only Case here; defensive bail if
                    // that invariant ever slips.
                    return false;
                };
                let next_case_label = ctx.next_label();
                // Compare disc against the case literal; `JumpIf` skips the
                // body when they differ (Eq ⇒ 1 truthy, stays; Ne ⇒ 0 falsy,
                // JumpIf fires).
                instructions.push(Instruction::LoadLocal(disc_local));
                if !lower_yul_expression(value_expr, state, ctx, instructions) {
                    return false;
                }
                instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
                instructions.push(Instruction::Convert {
                    target: ConvertTarget::Integer,
                });
                instructions.push(Instruction::JumpIf {
                    target: next_case_label,
                });
                if !lower_yul_block_stmts(&body.statements, state, ctx, instructions) {
                    return false;
                }
                instructions.push(Instruction::Jump { target: end_label });
                instructions.push(Instruction::Label(next_case_label));
            }

            if let Some(default_opt) = &switch_stmt.default {
                let solang_parser::pt::YulSwitchOptions::Default(_, default_body) = default_opt
                else {
                    return false;
                };
                if !lower_yul_block_stmts(&default_body.statements, state, ctx, instructions) {
                    return false;
                }
            }
            instructions.push(Instruction::Label(end_label));
            true
        }
        // Task #200 — yul `break` / `continue` jump to the innermost loop's
        // break / continue labels (both pushed by the `For` arm above). If
        // these appear outside a loop, solang's parser already rejects the
        // source, but defensively we also return false so the enclosing
        // assembly bails to the no-op warning path rather than emit a Jump
        // with no matching Label.
        YulStatement::Break(_) => {
            if let Some(label) = ctx.break_target() {
                instructions.push(Instruction::Jump { target: label });
                true
            } else {
                false
            }
        }
        YulStatement::Continue(_) => {
            if let Some(label) = ctx.continue_target() {
                instructions.push(Instruction::Jump { target: label });
                true
            } else {
                false
            }
        }
        // Out of scope: leave/FunctionDefinition.
        _ => false,
    }
}

/// Lower a yul function-call used as a top-level statement. Handles the
/// side-effect opcodes (mstore, return) whose yul signatures have no return
/// values. `mload`, `add`, etc. are only valid as expressions.
pub(crate) fn lower_yul_function_call_as_statement(
    call: &solang_parser::pt::YulFunctionCall,
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let name = call.id.name.as_str();
    match name {
        "mstore" => {
            if call.arguments.len() != 2 {
                return false;
            }
            lower_yul_mstore(
                &call.arguments[0],
                &call.arguments[1],
                state,
                ctx,
                instructions,
            )
        }
        "mstore8" => {
            // mstore8 would require a single-byte write path distinct from
            // the 32-byte MEMCPY we use for mstore.
            false
        }
        "tstore" => {
            // Task #100 — EIP-1153 transient store. `tstore(slot, value)`
            // writes `value` into the per-invocation `__yul_transient` map
            // under key `slot`. No persistence beyond the current function
            // frame (which matches EIP-1153's per-tx semantics because each
            // runtime call is one tx in this host).
            if call.arguments.len() != 2 {
                return false;
            }
            lower_yul_tstore(
                &call.arguments[0],
                &call.arguments[1],
                state,
                ctx,
                instructions,
            )
        }
        "return" => {
            if call.arguments.len() != 2 {
                return false;
            }
            lower_yul_return(
                &call.arguments[0],
                &call.arguments[1],
                state,
                ctx,
                instructions,
            )
        }
        "returndatacopy" => {
            // Task #184 — `returndatacopy(dst, src, len)` copies `len` bytes
            // from the last-call returndata buffer into yul memory at `dst`,
            // reading from returndata offset `src`. Currently modeled against
            // a lazily-initialised empty `__yul_returndata` buffer (see
            // `ensure_returndata` comments): any non-zero-length read panics
            // with `"returndata: read past returndatasize"` because no prior
            // external call has populated the buffer. A follow-up task can
            // extend `Target(t).f()` / CALLT / DYNCALL sites to stash the
            // callee's return bytes into the same slot so this opcode
            // recovers the real payload.
            if call.arguments.len() != 3 {
                return false;
            }
            lower_yul_returndatacopy(
                &call.arguments[0],
                &call.arguments[1],
                &call.arguments[2],
                state,
                ctx,
                instructions,
            )
        }
        _ => false,
    }
}

/// Lower a yul expression, leaving its integer value on the NeoVM stack.
pub(crate) fn lower_yul_expression(
    expr: &solang_parser::pt::YulExpression,
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    use solang_parser::pt::YulExpression;
    match expr {
        YulExpression::NumberLiteral(_, integer, _exp, _) => match integer.parse::<BigInt>() {
            Ok(value) => {
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(value)));
                true
            }
            Err(_) => false,
        },
        YulExpression::HexNumberLiteral(_, raw, _) => {
            let digits = raw.trim_start_matches("0x").trim_start_matches("0X");
            match BigInt::parse_bytes(digits.as_bytes(), 16) {
                Some(value) => {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(value)));
                    true
                }
                None => false,
            }
        }
        YulExpression::BoolLiteral(_, value, _) => {
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(u8::from(*value)),
            )));
            true
        }
        YulExpression::Variable(ident) => {
            if let Some(&slot) = state.yul_locals.get(&ident.name) {
                instructions.push(Instruction::LoadLocal(slot));
                return true;
            }
            // Task #183 — a yul identifier may also resolve to a function
            // parameter (e.g. `function f(bytes32 x) { assembly { mstore(0, x) }}`).
            // Parameters live in `param_index_map` and are read via
            // `LoadParameter` / NeoVM LDARG. Task #99 covered yul-locals and
            // Solidity-locals (both in `local_index_map`) but missed params.
            // Mirror `lower_variable_expression`: check params first, then
            // Solidity-locals.
            if let Some(&param_index) = ctx.param_index_map.get(&ident.name) {
                instructions.push(Instruction::LoadParameter(param_index));
                return true;
            }
            if let Some(slot) = ctx.resolve_local(&ident.name) {
                instructions.push(Instruction::LoadLocal(slot));
                return true;
            }
            false
        }
        YulExpression::FunctionCall(call) => {
            lower_yul_function_call_as_expression(call, state, ctx, instructions)
        }
        _ => false,
    }
}

pub(crate) fn lower_yul_function_call_as_expression(
    call: &solang_parser::pt::YulFunctionCall,
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let name = call.id.name.as_str();
    match name {
        "mload" => {
            if call.arguments.len() != 1 {
                return false;
            }
            lower_yul_mload(&call.arguments[0], state, ctx, instructions)
        }
        "tload" => {
            // Task #100 — EIP-1153 transient load. Returns the value stored
            // under `slot` in the `__yul_transient` map, or 0 if the slot
            // has never been tstore'd in this invocation.
            if call.arguments.len() != 1 {
                return false;
            }
            lower_yul_tload(&call.arguments[0], state, ctx, instructions)
        }
        "returndatasize" => {
            // Task #184 — `returndatasize()` returns the byte length of the
            // last-call returndata buffer. Because the Task #184 minimal
            // surface leaves `__yul_returndata` at its empty-byte seed, this
            // evaluates to 0 on any top-level call. Added here so yul bodies
            // that guard `returndatacopy` with a `returndatasize()` check
            // (idiomatic EVM pattern: `if lt(returndatasize(), len) { revert }`)
            // compile without dropping to the legacy no-op warning path.
            if !call.arguments.is_empty() {
                return false;
            }
            let rd_local = state.ensure_returndata(ctx);
            instructions.push(Instruction::LoadLocal(rd_local));
            instructions.push(Instruction::GetSize);
            true
        }
        "add" | "sub" | "mul" => lower_two_arg_yul_call(
            call,
            state,
            ctx,
            instructions,
            |n| match n {
                "add" => BinaryOperator::Add,
                "sub" => BinaryOperator::Sub,
                _ => BinaryOperator::Mul, // "mul" — the only remaining arm
            },
        ),
        "div" | "mod" => {
            // Yul (EVM) semantics: division/modulo by zero yields 0, NOT a fault.
            // (Unlike high-level Solidity, which Panics 0x12 — that guard lives in
            // the binary-expression path, not here.)
            if call.arguments.len() != 2 {
                return false;
            }
            if !lower_yul_expression(&call.arguments[0], state, ctx, instructions) {
                return false;
            }
            if !lower_yul_expression(&call.arguments[1], state, ctx, instructions) {
                return false;
            }
            let tmp = ctx.next_label();
            let b_local = ctx.allocate_local(format!("__yul_div_b_{tmp}"), None);
            let a_local = ctx.allocate_local(format!("__yul_div_a_{tmp}"), None);
            instructions.push(Instruction::StoreLocal(b_local)); // divisor (top)
            instructions.push(Instruction::StoreLocal(a_local)); // dividend
            let nonzero = ctx.next_label();
            let done = ctx.next_label();
            // `JumpIf` jumps when the condition is FALSE: divisor != 0 -> divide.
            instructions.push(Instruction::LoadLocal(b_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf { target: nonzero });
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::Jump { target: done });
            instructions.push(Instruction::Label(nonzero));
            instructions.push(Instruction::LoadLocal(a_local));
            instructions.push(Instruction::LoadLocal(b_local));
            // EVM div/mod are UNSIGNED; native NeoVM DIV/MOD are signed and
            // wrong for operands >= 2^255. Route through the software unsigned
            // divmod (the divisor is provably non-zero here).
            emit_u256_divmod_ir(ctx, instructions, name == "mod");
            instructions.push(Instruction::Label(done));
            true
        }
        "and" | "or" | "xor" => lower_two_arg_yul_call(
            call,
            state,
            ctx,
            instructions,
            |n| match n {
                "and" => BinaryOperator::BitAnd,
                "or" => BinaryOperator::BitOr,
                _ => BinaryOperator::BitXor, // "xor" — the only remaining arm
            },
        ),
        "shl" | "shr" => {
            // Yul shift args are (shift_amount, value); NeoVM's BinaryOp
            // Shl/Shr take (value, shift_amount) bottom-up.
            if call.arguments.len() != 2 {
                return false;
            }
            if !lower_yul_expression(&call.arguments[1], state, ctx, instructions) {
                return false;
            }
            if !lower_yul_expression(&call.arguments[0], state, ctx, instructions) {
                return false;
            }
            // Stack: [value, shift]. EVM `shr` is a LOGICAL shift; native NeoVM
            // SHR is arithmetic and sign-extends a high-bit-set 256-bit word.
            if name == "shr" {
                emit_u256_logical_shr_ir(ctx, instructions);
            } else {
                instructions.push(Instruction::BinaryOp(BinaryOperator::Shl));
            }
            true
        }
        "lt" | "gt" | "eq" => {
            if call.arguments.len() != 2 {
                return false;
            }
            if !lower_yul_expression(&call.arguments[0], state, ctx, instructions) {
                return false;
            }
            if !lower_yul_expression(&call.arguments[1], state, ctx, instructions) {
                return false;
            }
            // EVM `lt`/`gt` are UNSIGNED comparisons; native NeoVM LT/GT are
            // signed and wrong for 256-bit words >= 2^255 (e.g. after a `sub`
            // underflow produces a negative-looking value). Route lt/gt through
            // the unsigned-256 compare; `eq` is sign-agnostic.
            match name {
                "lt" => emit_u256_unsigned_compare(instructions, BinaryOperator::Lt),
                "gt" => emit_u256_unsigned_compare(instructions, BinaryOperator::Gt),
                _ => instructions.push(Instruction::BinaryOp(BinaryOperator::Eq)),
            }
            // Yul returns 0/1; convert the NeoVM Boolean to an Integer so
            // the value can chain into arithmetic (e.g. `add(lt(...), 1)`).
            instructions.push(Instruction::Convert {
                target: ConvertTarget::Integer,
            });
            true
        }
        "iszero" => {
            if call.arguments.len() != 1 {
                return false;
            }
            if !lower_yul_expression(&call.arguments[0], state, ctx, instructions) {
                return false;
            }
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::Convert {
                target: ConvertTarget::Integer,
            });
            true
        }
        "not" => {
            if call.arguments.len() != 1 {
                return false;
            }
            if !lower_yul_expression(&call.arguments[0], state, ctx, instructions) {
                return false;
            }
            // EVM `not(x)` is the 256-bit complement `2^256-1-x`, NOT NeoVM
            // INVERT's arbitrary-precision `-x-1`. XOR with the 256-bit all-ones
            // literal routes through the runtime's wide BitXor, which masks the
            // result to 256 bits (so `not(0)` == 2^256-1, and `and(x, not(mask))`
            // is correct).
            let max_u256: BigInt = (BigInt::one() << 256usize) - BigInt::one();
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(max_u256)));
            instructions.push(Instruction::BinaryOp(BinaryOperator::BitXor));
            true
        }
        _ => false,
    }
}

/// Emit `mstore(offset, value)` — copy the 32 big-endian bytes of `value`
/// into `__yul_memory[offset .. offset+32]`.
pub(crate) fn lower_yul_mstore(
    offset_expr: &solang_parser::pt::YulExpression,
    value_expr: &solang_parser::pt::YulExpression,
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let mem_local = state.ensure_memory(ctx);

    // Evaluate value first and stash it in a local so BE-encoding can work
    // without juggling offset on the stack at the same time.
    if !lower_yul_expression(value_expr, state, ctx, instructions) {
        return false;
    }
    let tmp_id = ctx.next_label();
    let value_local = ctx.allocate_local(format!("__yul_mstore_value_{tmp_id}"), None);
    instructions.push(Instruction::StoreLocal(value_local));

    // Evaluate offset into its own local (kept as a NeoVM integer).
    if !lower_yul_expression(offset_expr, state, ctx, instructions) {
        return false;
    }
    let offset_local = ctx.allocate_local(format!("__yul_mstore_offset_{tmp_id}"), None);
    instructions.push(Instruction::StoreLocal(offset_local));

    // Build the 32-byte BE encoding of `value` in a scratch buffer. Mirror
    // the well-worn `coerce_to_fixed_bytes(32, reverse=true)` recipe used by
    // bytesN(..) casts: LE bytes from Convert→ByteArray, MEMCPY into a
    // fresh 32-byte zero buffer at offset 0, then REVERSEITEMS in place.
    instructions.push(Instruction::LoadLocal(value_local));
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
    let src_local = ctx.allocate_local(format!("__yul_mstore_src_{tmp_id}"), None);
    instructions.push(Instruction::StoreLocal(src_local));

    let scratch_local = ctx.allocate_local(format!("__yul_mstore_scratch_{tmp_id}"), None);
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u64),
    )));
    instructions.push(Instruction::NewBuffer);
    instructions.push(Instruction::StoreLocal(scratch_local));

    // count = min(src.len(), 32)
    let size_local = ctx.allocate_local(format!("__yul_mstore_size_{tmp_id}"), None);
    let count_local = ctx.allocate_local(format!("__yul_mstore_count_{tmp_id}"), None);
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(size_local));

    let ge_label = ctx.next_label();
    let end_label = ctx.next_label();
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u64),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: ge_label });
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Jump { target: end_label });
    instructions.push(Instruction::Label(ge_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u64),
    )));
    instructions.push(Instruction::StoreLocal(count_local));
    instructions.push(Instruction::Label(end_label));

    // Copy LE bytes into scratch[0 .. count].
    instructions.push(Instruction::LoadLocal(scratch_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::MemCpy);
    // Real NeoVM MEMCPY: Pop 5, Push 0. Load scratch explicitly.
    instructions.push(Instruction::LoadLocal(scratch_local));
    instructions.push(Instruction::ReverseItems);

    // Copy scratch → __yul_memory[offset .. offset+32].
    instructions.push(Instruction::LoadLocal(mem_local));
    instructions.push(Instruction::LoadLocal(offset_local));
    instructions.push(Instruction::LoadLocal(scratch_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u64),
    )));
    instructions.push(Instruction::MemCpy);
    // Real NeoVM MEMCPY: Pop 5, Push 0. Nothing to discard.
    true
}

/// Emit `mload(offset)` — read 32 bytes from `__yul_memory[offset..offset+32]`,
/// decoded as a big-endian uint256.
pub(crate) fn lower_yul_mload(
    offset_expr: &solang_parser::pt::YulExpression,
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let mem_local = state.ensure_memory(ctx);

    // SUBSTR wants [bytes, index, count] — push in that order.
    instructions.push(Instruction::LoadLocal(mem_local));
    if !lower_yul_expression(offset_expr, state, ctx, instructions) {
        return false;
    }
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u64),
    )));
    instructions.push(Instruction::Substr);

    // SUBSTR returns a ByteString. Reverse into LE and CONVERT→Integer to
    // recover the big-endian magnitude.
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
    instructions.push(Instruction::Dup);
    instructions.push(Instruction::ReverseItems);
    instructions.push(Instruction::Convert {
        target: ConvertTarget::Integer,
    });
    true
}

/// Emit `return(offset, length)` — terminate the function with the memory
/// slice as its return value. When the enclosing function's declared single
/// return type is an integer, re-interpret the slice as BE-packed uint so
/// the main-frame RET emits the expected 32-byte value; otherwise hand the
/// raw buffer to RET.
pub(crate) fn lower_yul_return(
    offset_expr: &solang_parser::pt::YulExpression,
    length_expr: &solang_parser::pt::YulExpression,
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let mem_local = state.ensure_memory(ctx);

    instructions.push(Instruction::LoadLocal(mem_local));
    if !lower_yul_expression(offset_expr, state, ctx, instructions) {
        return false;
    }
    if !lower_yul_expression(length_expr, state, ctx, instructions) {
        return false;
    }
    instructions.push(Instruction::Substr);

    // Decide the canonical return shape. If the function is declared
    // `returns (uintN)` / `returns (intN)` / `returns (bool)` (single
    // scalar), re-interpret the BE-packed slice as an integer so the
    // main-frame RET emits the spec-matching 32-byte value. Otherwise
    // leave the raw buffer.
    let return_types = ctx.return_types();
    let want_integer = return_types.len() == 1
        && matches!(
            return_types[0],
            ValueType::Integer { .. } | ValueType::Boolean
        );

    if want_integer {
        // Reverse the BE slice into LE and CONVERT→Integer. Mirror the
        // `uint256(bytes32)` reinterpret recipe from type_constructors.rs
        // so short slices (< 32 bytes) still decode as zero-extended uint.
        instructions.push(Instruction::Convert {
            target: ConvertTarget::ByteArray,
        });
        instructions.push(Instruction::Dup);
        instructions.push(Instruction::ReverseItems);
        instructions.push(Instruction::Convert {
            target: ConvertTarget::Integer,
        });
    }

    instructions.push(Instruction::Return);
    true
}

/// Emit `tstore(slot, value)` — store `value` in the transient map under
/// key `slot`. The `__yul_transient` map is lazily allocated in the first
/// yul block that uses transient storage and reused (by name lookup) by
/// every subsequent yul block in the same function, which gives EIP-1153's
/// per-transaction persistence for the lifetime of this invocation.
///
/// Stack shape for NeoVM SETITEM (0xD0): bottom → top = [collection, key, value].
/// After the op, the map has been mutated in place and the stack is empty.
pub(crate) fn lower_yul_tstore(
    slot_expr: &solang_parser::pt::YulExpression,
    value_expr: &solang_parser::pt::YulExpression,
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let transient_local = state.ensure_transient(ctx);

    // Push [map, slot, value] in the order SETITEM expects.
    instructions.push(Instruction::LoadLocal(transient_local));
    if !lower_yul_expression(slot_expr, state, ctx, instructions) {
        return false;
    }
    if !lower_yul_expression(value_expr, state, ctx, instructions) {
        return false;
    }
    instructions.push(Instruction::ArraySet); // SETITEM 0xD0 — also works on Maps.
    true
}

/// Emit `tload(slot)` — read the value stored under `slot` in the transient
/// map, or `0` if the slot was never tstore'd in this invocation. Uses
/// HASKEY to branch so that missing keys fall through to PUSH0 instead of
/// raising `PICKITEM: key not found`.
///
/// Generated shape (pseudocode):
///     if HAS_KEY(map, slot) { push(map_slot); } else { push(0); }
pub(crate) fn lower_yul_tload(
    slot_expr: &solang_parser::pt::YulExpression,
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let transient_local = state.ensure_transient(ctx);

    // Stash the slot in a local so we can use it twice (once for HASKEY,
    // once for PICKITEM) without re-evaluating the expression.
    let tmp_id = ctx.next_label();
    let slot_local = ctx.allocate_local(format!("__yul_tload_slot_{tmp_id}"), None);
    if !lower_yul_expression(slot_expr, state, ctx, instructions) {
        return false;
    }
    instructions.push(Instruction::StoreLocal(slot_local));

    // HASKEY pops [collection, key] and pushes Boolean. NeoVM JMPIFNOT_L
    // branches on falsy, so we jump to the "push 0" arm when the key is
    // absent. IR's JumpIf has the same "jump when false" semantic
    // (see lower_yul_mstore above for the pattern).
    let missing_label = ctx.next_label();
    let end_label = ctx.next_label();

    instructions.push(Instruction::LoadLocal(transient_local));
    instructions.push(Instruction::LoadLocal(slot_local));
    instructions.push(Instruction::HasKey);
    instructions.push(Instruction::JumpIf {
        target: missing_label,
    });

    // key present → push map[slot]
    instructions.push(Instruction::LoadLocal(transient_local));
    instructions.push(Instruction::LoadLocal(slot_local));
    instructions.push(Instruction::ArrayGet); // PICKITEM 0xCE — works on Maps.
    instructions.push(Instruction::Jump { target: end_label });

    // key absent → push 0 (EIP-1153 default for unset transient slot)
    instructions.push(Instruction::Label(missing_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));

    instructions.push(Instruction::Label(end_label));
    true
}

/// Task #184 — emit `returndatacopy(dst, src, len)`.
///
/// EVM semantics: copy `len` bytes from the last-call returndata buffer
/// (starting at offset `src`) into yul memory at offset `dst`. If
/// `src + len > returndatasize()`, the contract MUST revert with
/// `Panic(0x32)` (read past returndatasize). When `len == 0`, the opcode
/// is a strict no-op regardless of `src`/`dst`.
///
/// Task #184 surface: the minimal harness (`batch76_zz4`) exercises the
/// "no preceding external call" case where `returndatasize() == 0`, so the
/// bounds check always fires on any non-zero-length read. We still
/// implement the full `MemCpy` path so a follow-up task can populate
/// `__yul_returndata` after external calls without revisiting this lower.
///
/// Generated shape (pseudocode):
/// ```text
///   // 1. Evaluate & stash args.
///   let dst = <dst_expr>;
///   let src = <src_expr>;
///   let len = <len_expr>;
///
///   // 2. Zero-length fast path (EVM spec: no-op even if src >= rdsize).
///   if len != 0 {
///     // 3. Bounds check: src + len > returndatasize() → revert.
///     if src + len > __yul_returndata.size() {
///       throw "returndata: read past returndatasize";
///     }
///
///     // 4. Copy.
///     MEMCPY(__yul_memory, dst, __yul_returndata, src, len);
///   }
/// ```
///
/// Diagnostic shape on the bounds-check fault: the exception message is
/// `THROW: returndata: read past returndatasize`, which contains the
/// `"returndata"` substring the `batch76_zz4` harness accepts as a clean
/// underflow marker (see `tests/fuzz_tests/batches_66_80.rs`).
pub(crate) fn lower_yul_returndatacopy(
    dst_expr: &solang_parser::pt::YulExpression,
    src_expr: &solang_parser::pt::YulExpression,
    len_expr: &solang_parser::pt::YulExpression,
    state: &mut YulLoweringState,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let mem_local = state.ensure_memory(ctx);
    let rd_local = state.ensure_returndata(ctx);

    // Evaluate dst, src, len in that order (yul's left-to-right semantic) and
    // stash each in its own local so the bounds check and MEMCPY can reference
    // them multiple times without re-evaluating (which would double any side
    // effects in the source expressions).
    let tmp_id = ctx.next_label();

    if !lower_yul_expression(dst_expr, state, ctx, instructions) {
        return false;
    }
    let dst_local = ctx.allocate_local(format!("__yul_rdc_dst_{tmp_id}"), None);
    instructions.push(Instruction::StoreLocal(dst_local));

    if !lower_yul_expression(src_expr, state, ctx, instructions) {
        return false;
    }
    let src_local = ctx.allocate_local(format!("__yul_rdc_src_{tmp_id}"), None);
    instructions.push(Instruction::StoreLocal(src_local));

    if !lower_yul_expression(len_expr, state, ctx, instructions) {
        return false;
    }
    let len_local = ctx.allocate_local(format!("__yul_rdc_len_{tmp_id}"), None);
    instructions.push(Instruction::StoreLocal(len_local));

    let skip_label = ctx.next_label(); // target for len == 0 fast path
    let ok_label = ctx.next_label(); // target for (src+len) <= rdsize branch

    // Fast path: if `len == 0`, skip the whole copy (including the bounds
    // check — EVM spec treats zero-length returndatacopy as a no-op even
    // when returndatasize() is 0).
    //
    //   LoadLocal(len); Push(0); BinaryOp(Ne) → Boolean(len != 0)
    //   JumpIf(skip_label)   — IR JumpIf = JMPIFNOT: branches when falsy,
    //                          i.e. when (len != 0) is false, i.e. len == 0.
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ne));
    instructions.push(Instruction::JumpIf { target: skip_label });

    // Bounds check: compute (src + len) and compare with returndatasize.
    //
    //   Push (src + len)
    //   Push rdsize
    //   Compute (src + len) <= rdsize  ⇒  push Boolean
    //   JumpIf(ok_label) — branches when falsy, so we fall through to the
    //   "throw" arm when (src + len) > rdsize.
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));

    instructions.push(Instruction::LoadLocal(rd_local));
    instructions.push(Instruction::GetSize);

    instructions.push(Instruction::BinaryOp(BinaryOperator::Le));
    instructions.push(Instruction::JumpIf { target: ok_label });

    // Fault arm: push a descriptive error message and THROW. The runtime's
    // `execute_flow_exceptions` UTF-8-lossies the payload into the exception
    // message with a `THROW: ` prefix, producing `THROW: returndata: read
    // past returndatasize`. The `"returndata"` substring matches the
    // `batch76_zz4` harness's `clean_underflow` acceptance set.
    instructions.push(Instruction::PushLiteral(LiteralValue::String(
        b"returndata: read past returndatasize".to_vec(),
    )));
    instructions.push(Instruction::Throw);

    // OK arm: execute the copy.
    //
    //   MemCpy stack order (bottom → top): [dst, dst_offset, src, src_offset, count]
    //   → copy src[src_offset .. src_offset+count] into dst[dst_offset .. dst_offset+count]
    //   → leaves dst on the stack; we `Drop` it because the buffer was
    //     already stored via the `__yul_memory` local.
    instructions.push(Instruction::Label(ok_label));
    instructions.push(Instruction::LoadLocal(mem_local));
    instructions.push(Instruction::LoadLocal(dst_local));
    instructions.push(Instruction::LoadLocal(rd_local));
    instructions.push(Instruction::LoadLocal(src_local));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::MemCpy);
    // Real NeoVM MEMCPY: Pop 5, Push 0. Nothing to discard.

    // Skip target: zero-length fast path and end of the non-fault path meet
    // here. The `ok_label` arm falls through to `skip_label`; both paths
    // continue with the next yul statement.
    instructions.push(Instruction::Label(skip_label));
    true
}
