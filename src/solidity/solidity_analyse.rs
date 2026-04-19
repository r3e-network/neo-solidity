pub fn analyse_source(source: &str) -> Result<ContractMetadata, SolidityError> {
    let mut contracts = analyse_all_sources(source)?;
    Ok(contracts.swap_remove(0))
}

pub fn analyse_all_sources(source: &str) -> Result<Vec<ContractMetadata>, SolidityError> {
    fn is_builtin_library_name(name: &str) -> bool {
        matches!(
            name,
            "Runtime" | "abi" | "Storage" | "Syscalls" | "Neo" | "NativeCalls"
        )
    }

    fn normalize_library_for_neo(mut contract: ContractIR) -> ContractIR {
        if !matches!(contract.kind, ContractKind::Library) {
            return contract;
        }

        // Neo N3 libraries are inlined into contracts; treat externally visible
        // library functions as internal helper functions to avoid exposing them
        // through the contract ABI.
        for function in &mut contract.functions {
            if !matches!(function.ty, FunctionTy::Function) {
                continue;
            }
            if matches!(
                function.visibility,
                VisibilityKind::External | VisibilityKind::Public
            ) {
                function.visibility = VisibilityKind::Internal;
            }
        }

        // Keep merged library state as internal implementation detail.
        // Public library constants would otherwise synthesize contract-level
        // getters and create ABI/name collisions in the consuming contract.
        for state in &mut contract.state_variables {
            state.visibility = Some("internal".to_string());
        }

        contract
    }

    fn collect_contract_types(
        contract_map: &std::collections::HashMap<String, ContractIR>,
    ) -> Vec<String> {
        let mut contract_types: Vec<String> = Vec::new();
        let mut seen_contract_types = std::collections::HashSet::new();

        for contract in contract_map.values() {
            let include_as_contract_type = match contract.kind {
                ContractKind::Contract | ContractKind::AbstractContract | ContractKind::Interface => {
                    true
                }
                ContractKind::Library => !is_builtin_library_name(contract.name.as_str()),
            };

            if include_as_contract_type
                && seen_contract_types.insert(contract.name.to_ascii_lowercase())
            {
                contract_types.push(contract.name.clone());
            }
        }

        contract_types
    }

    let mut primary = Vec::new();
    let mut fallback = Vec::new();

    let contracts = parse_source(source)?;
    for contract in contracts {
        if matches!(
            contract.kind,
            ContractKind::Contract | ContractKind::AbstractContract
        ) {
            primary.push(contract);
        } else {
            fallback.push(contract);
        }
    }

    let has_primary = !primary.is_empty();
    let pre_merge_contract_map: std::collections::HashMap<String, ContractIR> = primary
        .iter()
        .chain(fallback.iter())
        .map(|contract| (contract.name.clone(), contract.clone()))
        .collect();
    let contract_types = collect_contract_types(&pre_merge_contract_map);

    let raw_libraries: Vec<ContractIR> = if has_primary {
        fallback
            .iter()
            .filter(|contract| matches!(contract.kind, ContractKind::Library))
            // Built-in helper libraries (Runtime/Storage/Syscalls/Neo) are lowered directly during
            // IR generation. Avoid merging their Solidity bodies into user contracts since they
            // may contain EVM-only stubs or unsupported constructs, and they would bloat bytecode.
            .filter(|contract| !is_builtin_library_name(contract.name.as_str()))
            .cloned()
            .collect()
    } else {
        Vec::new()
    };

    // Validate user libraries before merging. Convert each library to metadata
    // and run the standard validation pipeline to catch library-specific errors
    // (state variables, constructors, external functions) early.
    for lib in &raw_libraries {
        let lib_metadata = convert_contract(
            lib.clone(),
            &[],
            &contract_types,
            std::sync::Arc::new(SelectorRegistry::default()),
        );
        let lib_diagnostics = validate_contract(&lib_metadata);
        let lib_errors: Vec<Diagnostic> = lib_diagnostics
            .into_iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Error))
            .collect();
        if !lib_errors.is_empty() {
            let messages: Vec<String> = lib_errors.iter().map(|d| {
                let mut msg = d.message.clone();
                if let Some(suggestion) = &d.suggestion {
                    msg.push_str(&format!("\n  suggestion: {suggestion}"));
                }
                msg
            }).collect();
            return Err(SolidityError::analysis(messages.join("\n")));
        }
    }

    let libraries: Vec<ContractIR> = raw_libraries
        .into_iter()
        .map(normalize_library_for_neo)
        .collect();

    // Merge library definitions into primary contracts so that library functions
    // (including `using for`-style member calls) can be lowered as internal calls.
    if has_primary && !libraries.is_empty() {
        for contract in primary.iter_mut() {
            for lib in &libraries {
                contract.functions.extend(lib.functions.clone());
                contract.state_variables.extend(lib.state_variables.clone());
                contract.structs.extend(lib.structs.clone());
                contract.enums.extend(lib.enums.clone());
            }
        }
    }

    // Task #83 — when a primary contract `A` runs `B b = new B(); b.foo();`
    // the compiler emits a 20-byte zero placeholder for `b` and lowers
    // `b.foo()` as `System.Contract.Call([0;20], "foo", flags, args)`. B's
    // compiled body is a separate artifact, so without help the call would
    // return `Null` and A's return value would silently go empty. Fix:
    // merge every sibling primary's public/external functions that A
    // references via `new X()` into A's own function table (name-preserving,
    // host-wins-on-collision); the runtime then routes the zero-hash call
    // through `self_method_offsets` — see the Task #83 branch in
    // `execution_impl_part2_contract_call.rs`.
    if has_primary {
        let sibling_fn_map: std::collections::HashMap<String, Vec<FunctionIR>> = primary
            .iter()
            .map(|c| {
                (
                    c.name.clone(),
                    c.functions
                        .iter()
                        .filter(|f| {
                            // Task #126 — include Fallback (and Receive) alongside
                            // ordinary external/public named functions so that a
                            // primary contract whose only entrypoint is
                            // `fallback()` still contributes its dispatcher to
                            // the caller's merged function table when the caller
                            // invokes a method the callee doesn't declare.
                            //
                            // Without this, `try Target(t).nonExistentMethod()`
                            // (where TargetImpl only defines `fallback()`)
                            // would never be able to route through the zero-
                            // placeholder self-offsets path: the fallback entry
                            // simply wouldn't be in the merge set, and the
                            // runtime's unknown-method path would silently
                            // return Null rather than propagating the fallback's
                            // revert back to the caller's catch clause.
                            let is_named_external = matches!(f.ty, FunctionTy::Function)
                                && matches!(
                                    f.visibility,
                                    VisibilityKind::External | VisibilityKind::Public
                                );
                            let is_fallback_like = matches!(
                                f.ty,
                                FunctionTy::Fallback | FunctionTy::Receive
                            );
                            is_named_external || is_fallback_like
                        })
                        .cloned()
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        let primary_names: std::collections::HashSet<String> =
            primary.iter().map(|c| c.name.clone()).collect();

        // Task #126 — a primary contract's `fallback()` acts as a universal
        // catch-all dispatcher: every unknown method name falls through to
        // it. For interface-cast routing `Target(t).someMethod()` where
        // `TargetImpl` has only `fallback()` (no named external methods),
        // we must still treat `TargetImpl` as a valid implementor of the
        // `Target` interface so the sibling-merge pass pulls its fallback
        // body into the caller's function table. This mirrors Solidity's
        // own runtime semantics: the ABI dispatcher routes unknown
        // selectors to `fallback()` when present.
        let primary_has_fallback: std::collections::HashSet<String> = primary
            .iter()
            .filter(|c| {
                c.functions
                    .iter()
                    .any(|f| matches!(f.ty, FunctionTy::Fallback))
            })
            .map(|c| c.name.clone())
            .collect();

        // Task #115 — collect interface kind names and their external method
        // sets. An expression like `I(t).getR()` in contract `C` (where `I`
        // is an interface declared in the same source unit) is a
        // cross-contract call routed through an `address`-typed receiver.
        // At runtime the 20-byte zero placeholder triggers self-offsets
        // dispatch (see `handle_contract_call` / Task #83 branch), so the
        // callee method must live in the caller's merged function table.
        // We match the interface to any sibling primary whose public/external
        // method set is a superset of the interface's method names, and
        // include those siblings in the sibling merge below.
        //
        // This mirrors the `new B()` / `B(addr)` / `B public b;` patterns
        // already handled — without this hook, interface-typed dispatch
        // silently returns `Null` (the `invoke_native_contract` fallback for
        // unknown-hash calls), which then blows up inside `r.a` /
        // `r.b` member accesses downstream.
        let interface_methods: std::collections::HashMap<
            String,
            std::collections::HashSet<String>,
        > = pre_merge_contract_map
            .values()
            .filter(|c| matches!(c.kind, ContractKind::Interface))
            .map(|c| {
                (
                    c.name.clone(),
                    c.functions
                        .iter()
                        .filter(|f| {
                            matches!(f.ty, FunctionTy::Function)
                                && matches!(
                                    f.visibility,
                                    VisibilityKind::External | VisibilityKind::Public
                                )
                        })
                        .map(|f| f.name.clone())
                        .collect(),
                )
            })
            .collect();

        // Reverse map: interface name → list of primary contracts whose
        // method set covers the interface's method set. We pre-compute this
        // once so we don't re-walk the primary function tables per function
        // body.
        let primary_method_names: std::collections::HashMap<
            String,
            std::collections::HashSet<String>,
        > = primary
            .iter()
            .map(|c| {
                (
                    c.name.clone(),
                    c.functions
                        .iter()
                        .filter(|f| {
                            matches!(f.ty, FunctionTy::Function)
                                && matches!(
                                    f.visibility,
                                    VisibilityKind::External | VisibilityKind::Public
                                )
                        })
                        .map(|f| f.name.clone())
                        .collect(),
                )
            })
            .collect();

        let interface_impls: std::collections::HashMap<String, Vec<String>> =
            interface_methods
                .iter()
                .map(|(iface_name, iface_method_set)| {
                    let mut impls: Vec<String> = primary_method_names
                        .iter()
                        .filter_map(|(prim_name, prim_set)| {
                            // Task #126 — a primary with a `fallback()` catches
                            // any interface method that isn't explicitly declared,
                            // so it's always a valid implementor for sibling-
                            // merge purposes (at runtime the call routes through
                            // the merged `fallback` entry, which may itself
                            // revert — that revert is what we propagate to the
                            // caller's try/catch).
                            if iface_method_set.is_subset(prim_set)
                                || primary_has_fallback.contains(prim_name)
                            {
                                Some(prim_name.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    // Deterministic order → reproducible bytecode offsets.
                    impls.sort();
                    (iface_name.clone(), impls)
                })
                .collect();

        let interface_names: std::collections::HashSet<String> =
            interface_methods.keys().cloned().collect();

        for contract in primary.iter_mut() {
            let mut referenced: std::collections::HashSet<String> =
                std::collections::HashSet::new();
            let mut iface_refs: std::collections::HashSet<String> =
                std::collections::HashSet::new();
            for function in &contract.functions {
                if let Some(body) = function.body.as_ref() {
                    collect_new_contract_refs(body, &primary_names, &mut referenced);
                    // Task #115 — interface casts `I(expr)` in statements.
                    collect_interface_casts_stmt(body, &interface_names, &mut iface_refs);
                }
                // Task K4 — function params/returns typed as a sibling contract
                // (e.g. `function bounce() external returns (B) {...}`, or
                // `function xfer(C to, ...)`) mean the function is wired to
                // call into the sibling. Merge so self-call routing can see
                // the target method at runtime.
                for p in function.parameters.iter().chain(function.returns.iter()) {
                    if primary_names.contains(&p.ty) {
                        referenced.insert(p.ty.clone());
                    }
                    // Task #115 — also scan for interface-typed parameters.
                    if interface_names.contains(&p.ty) {
                        iface_refs.insert(p.ty.clone());
                    }
                }
            }
            // Task K4 — also scan state-variable types and initializers.
            // `B public b;` means A is wired to call into B via the storage
            // slot without ever going through `new B()`. Without this hook,
            // K4 (cross-contract reentrancy) fails: `b.bounce()` routes
            // through `System.Contract.Call([0;20], "bounce", …)` which then
            // returns `Null` because B wasn't merged.
            for state in &contract.state_variables {
                if primary_names.contains(&state.ty) {
                    referenced.insert(state.ty.clone());
                }
                if interface_names.contains(&state.ty) {
                    iface_refs.insert(state.ty.clone());
                }
                if let Some(init) = state.initializer.as_ref() {
                    collect_new_refs_expr(init, &primary_names, &mut referenced);
                    collect_interface_casts_expr(init, &interface_names, &mut iface_refs);
                }
            }
            // Task #115 — expand interface references to the primary contracts
            // that implement them. Multiple primaries may satisfy the same
            // interface; merge all of them so dispatch sees any signature.
            for iface in &iface_refs {
                if let Some(impls) = interface_impls.get(iface) {
                    for prim in impls {
                        if prim != &contract.name {
                            referenced.insert(prim.clone());
                        }
                    }
                }
            }
            referenced.remove(&contract.name);
            if referenced.is_empty() {
                continue;
            }
            let mut existing_sigs: std::collections::HashSet<(String, usize)> = contract
                .functions
                .iter()
                .map(|f| (f.name.clone(), f.parameters.len()))
                .collect();
            // Deterministic order → reproducible bytecode offsets.
            let mut sibling_names: Vec<String> = referenced.into_iter().collect();
            sibling_names.sort();
            for sibling_name in &sibling_names {
                let Some(sibling_fns) = sibling_fn_map.get(sibling_name) else {
                    continue;
                };
                for sibling_fn in sibling_fns {
                    let sig = (sibling_fn.name.clone(), sibling_fn.parameters.len());
                    if existing_sigs.insert(sig) {
                        contract.functions.push(sibling_fn.clone());
                    }
                }
            }
        }
    }

    // Make non-inherited enum/struct namespaces visible across compilation
    // units so expressions like `Enum.Operation.DelegateCall` can resolve even
    // when the defining type lives in another top-level contract/library file.
    if has_primary {
        let shared_type_defs: Vec<(String, Vec<StructIR>, Vec<EnumIR>)> = pre_merge_contract_map
            .values()
            .filter(|contract| {
                !matches!(contract.kind, ContractKind::Library)
                    || !is_builtin_library_name(contract.name.as_str())
            })
            .map(|contract| {
                (
                    contract.name.clone(),
                    contract.structs.clone(),
                    contract.enums.clone(),
                )
            })
            .collect();

        for contract in primary.iter_mut() {
            let mut seen_structs: std::collections::HashSet<String> = contract
                .structs
                .iter()
                .map(|item| item.name.to_ascii_lowercase())
                .collect();
            let mut seen_enums: std::collections::HashSet<String> = contract
                .enums
                .iter()
                .map(|item| item.name.to_ascii_lowercase())
                .collect();

            for (owner_name, structs, enums) in &shared_type_defs {
                if owner_name == &contract.name {
                    continue;
                }
                for item in structs {
                    let key = item.name.to_ascii_lowercase();
                    if seen_structs.insert(key) {
                        contract.structs.push(item.clone());
                    }
                }
                for item in enums {
                    let key = item.name.to_ascii_lowercase();
                    if seen_enums.insert(key) {
                        contract.enums.push(item.clone());
                    }
                }
            }
        }
    }

    // Build a lookup map for inheritance flattening and modifier expansion.
    let contract_map: std::collections::HashMap<String, ContractIR> = primary
        .iter()
        .chain(fallback.iter())
        .map(|contract| (contract.name.clone(), contract.clone()))
        .collect();

    // Task #106 — gather struct fields across all contracts so canonical
    // signatures can expand struct params into their `(field1,field2,...)` tuple
    // form per the EVM ABI spec. Without this, the selector for
    // `f(P memory p)` where `struct P { uint256 a; bool b; }` is computed from
    // `f(P)` — which does not match the Solidity-spec selector for
    // `f((uint256,bool))`.
    let mut struct_fields_map: std::collections::HashMap<
        String,
        Vec<(String, String)>,
    > = std::collections::HashMap::new();
    for contract in contract_map.values() {
        for struct_def in &contract.structs {
            let entries: Vec<(String, String)> = struct_def
                .fields
                .iter()
                .map(|f| (f.name.clone(), f.ty.clone()))
                .collect();
            struct_fields_map
                .entry(struct_def.name.clone())
                .or_insert(entries);
        }
    }

    // Build a shared selector registry so `.selector` expressions can resolve against
    // any contract/interface visible to this compilation unit (including those defined
    // after the primary contract in the same file).
    let mut type_method_selectors: std::collections::HashMap<
        String,
        std::collections::HashMap<String, Vec<[u8; 4]>>,
    > = std::collections::HashMap::new();
    let mut interface_types: std::collections::HashSet<String> = std::collections::HashSet::new();
    for contract in contract_map.values() {
        if matches!(contract.kind, ContractKind::Interface) {
            interface_types.insert(contract.name.clone());
        }

        // When building selector lookups for `.selector` / `.interfaceId`, include inherited
        // interface methods as part of the derived interface. This matches Solidity behavior
        // and supports patterns like `type(IChild).interfaceId` where `IChild is IParent`.
        let selector_contract = match contract.kind {
            ContractKind::Contract | ContractKind::AbstractContract | ContractKind::Interface => {
                flatten_contract_inheritance(contract.clone(), &contract_map)
                    .map(|(ir, _warnings)| ir)
                    .unwrap_or_else(|_| contract.clone())
            }
            ContractKind::Library => contract.clone(),
        };

        let mut per_type: std::collections::HashMap<String, Vec<[u8; 4]>> =
            std::collections::HashMap::new();

        for function in &selector_contract.functions {
            if !matches!(function.ty, FunctionTy::Function) {
                continue;
            }

            if !matches!(
                function.visibility,
                VisibilityKind::External | VisibilityKind::Public
            ) {
                continue;
            }

            let param_signatures: Vec<String> = function
                .parameters
                .iter()
                .map(|param| {
                    crate::utils::canonical_param_type_with_structs(
                        &param.ty,
                        &struct_fields_map,
                    )
                })
                .collect();
            let selector = compute_function_selector(&function.name, &param_signatures);
            per_type
                .entry(function.name.clone())
                .or_default()
                .push(selector);
        }

        type_method_selectors.insert(contract.name.clone(), per_type);
    }
    let selector_registry = std::sync::Arc::new(SelectorRegistry {
        type_method_selectors,
        interface_types,
    });

    let mut selected = if has_primary { primary } else { fallback };

    if selected.is_empty() {
        return Ok(Vec::new());
    }

    let mut metadatas = Vec::new();
    for contract in selected.drain(..) {
        let (mut flattened, flatten_warnings) =
            flatten_contract_inheritance(contract, &contract_map)?;
        apply_modifiers_and_base_constructors(&mut flattened, &contract_map)?;
        let mut metadata = convert_contract(
            flattened,
            &[],
            &contract_types,
            selector_registry.clone(),
        );
        metadata.flatten_warnings = flatten_warnings;
        metadatas.push(metadata);
    }

    Ok(metadatas)
}

/// Task #83 — walk a statement tree collecting every `new X()` target name
/// that matches a known primary contract. Mirrors the ast_scan permissions
/// pass but accumulates matches instead of returning a boolean.
fn collect_new_contract_refs(
    stmt: &Statement,
    primary_names: &std::collections::HashSet<String>,
    sink: &mut std::collections::HashSet<String>,
) {
    match stmt {
        Statement::Block { statements, .. } => {
            for s in statements {
                collect_new_contract_refs(s, primary_names, sink);
            }
        }
        Statement::If(_, cond, t, e) => {
            collect_new_refs_expr(cond, primary_names, sink);
            collect_new_contract_refs(t, primary_names, sink);
            if let Some(s) = e {
                collect_new_contract_refs(s, primary_names, sink);
            }
        }
        Statement::While(_, cond, body) | Statement::DoWhile(_, body, cond) => {
            collect_new_refs_expr(cond, primary_names, sink);
            collect_new_contract_refs(body, primary_names, sink);
        }
        Statement::Expression(_, expr) => collect_new_refs_expr(expr, primary_names, sink),
        Statement::VariableDefinition(_, _, init) => {
            if let Some(expr) = init {
                collect_new_refs_expr(expr, primary_names, sink);
            }
        }
        Statement::For(_, i, c, n, b) => {
            if let Some(s) = i {
                collect_new_contract_refs(s, primary_names, sink);
            }
            if let Some(e) = c {
                collect_new_refs_expr(e, primary_names, sink);
            }
            if let Some(e) = n {
                collect_new_refs_expr(e, primary_names, sink);
            }
            if let Some(s) = b {
                collect_new_contract_refs(s, primary_names, sink);
            }
        }
        Statement::Return(_, Some(expr)) | Statement::Emit(_, expr) => {
            collect_new_refs_expr(expr, primary_names, sink);
        }
        Statement::Revert(_, _, args) => {
            for e in args {
                collect_new_refs_expr(e, primary_names, sink);
            }
        }
        Statement::Try(_, expr, returns, clauses) => {
            collect_new_refs_expr(expr, primary_names, sink);
            if let Some((_, b)) = returns {
                collect_new_contract_refs(b, primary_names, sink);
            }
            for c in clauses {
                match c {
                    CatchClause::Simple(_, _, b) | CatchClause::Named(_, _, _, b) => {
                        collect_new_contract_refs(b, primary_names, sink);
                    }
                }
            }
        }
        _ => {}
    }
}

/// Task #115 — statement-level walk that collects every `I(expr).method(...)`
/// interface-cast receiver where `I` is a known interface declared in the
/// same source unit. Mirrors `collect_new_contract_refs` but tracks a
/// different alphabet of names (interface kinds, not primary contracts).
fn collect_interface_casts_stmt(
    stmt: &Statement,
    interface_names: &std::collections::HashSet<String>,
    sink: &mut std::collections::HashSet<String>,
) {
    match stmt {
        Statement::Block { statements, .. } => {
            for s in statements {
                collect_interface_casts_stmt(s, interface_names, sink);
            }
        }
        Statement::If(_, cond, t, e) => {
            collect_interface_casts_expr(cond, interface_names, sink);
            collect_interface_casts_stmt(t, interface_names, sink);
            if let Some(s) = e {
                collect_interface_casts_stmt(s, interface_names, sink);
            }
        }
        Statement::While(_, cond, body) | Statement::DoWhile(_, body, cond) => {
            collect_interface_casts_expr(cond, interface_names, sink);
            collect_interface_casts_stmt(body, interface_names, sink);
        }
        Statement::Expression(_, expr) => {
            collect_interface_casts_expr(expr, interface_names, sink)
        }
        Statement::VariableDefinition(_, _, init) => {
            if let Some(expr) = init {
                collect_interface_casts_expr(expr, interface_names, sink);
            }
        }
        Statement::For(_, i, c, n, b) => {
            if let Some(s) = i {
                collect_interface_casts_stmt(s, interface_names, sink);
            }
            if let Some(e) = c {
                collect_interface_casts_expr(e, interface_names, sink);
            }
            if let Some(e) = n {
                collect_interface_casts_expr(e, interface_names, sink);
            }
            if let Some(s) = b {
                collect_interface_casts_stmt(s, interface_names, sink);
            }
        }
        Statement::Return(_, Some(expr)) | Statement::Emit(_, expr) => {
            collect_interface_casts_expr(expr, interface_names, sink);
        }
        Statement::Revert(_, _, args) => {
            for e in args {
                collect_interface_casts_expr(e, interface_names, sink);
            }
        }
        Statement::Try(_, expr, returns, clauses) => {
            collect_interface_casts_expr(expr, interface_names, sink);
            if let Some((_, b)) = returns {
                collect_interface_casts_stmt(b, interface_names, sink);
            }
            for c in clauses {
                match c {
                    CatchClause::Simple(_, _, b) | CatchClause::Named(_, _, _, b) => {
                        collect_interface_casts_stmt(b, interface_names, sink);
                    }
                }
            }
        }
        _ => {}
    }
}

/// Task #115 — expression-level half of `collect_interface_casts_stmt`.
/// Matches `FunctionCall(Variable(I), _)` where `I` is a known interface
/// name. The parser emits this shape for interface casts like `I(addr)`.
fn collect_interface_casts_expr(
    expr: &Expression,
    interface_names: &std::collections::HashSet<String>,
    sink: &mut std::collections::HashSet<String>,
) {
    if let Expression::FunctionCall(_, func, _) = expr {
        if let Expression::Variable(id) = func.as_ref() {
            if interface_names.contains(&id.name) {
                sink.insert(id.name.clone());
            }
        }
    }
    match expr {
        Expression::New(_, i)
        | Expression::Parenthesis(_, i)
        | Expression::MemberAccess(_, i, _)
        | Expression::Delete(_, i) => collect_interface_casts_expr(i, interface_names, sink),
        Expression::FunctionCall(_, func, args) => {
            collect_interface_casts_expr(func, interface_names, sink);
            for a in args {
                collect_interface_casts_expr(a, interface_names, sink);
            }
        }
        // Task #125 — symmetric fix with `collect_new_refs_expr`: the
        // `try-expr { success-block }` lowering parks the call inside a
        // FunctionCallBlock, so interface-cast chains such as
        // `try I(t).getR() returns (R r) { ... } catch ...` would also
        // silently skip the sibling-merge trigger without this arm.
        Expression::FunctionCallBlock(_, call, block) => {
            collect_interface_casts_expr(call, interface_names, sink);
            collect_interface_casts_stmt(block, interface_names, sink);
        }
        Expression::NamedFunctionCall(_, func, args) => {
            collect_interface_casts_expr(func, interface_names, sink);
            for a in args {
                collect_interface_casts_expr(&a.expr, interface_names, sink);
            }
        }
        Expression::ArraySubscript(_, a, b) => {
            collect_interface_casts_expr(a, interface_names, sink);
            if let Some(e) = b {
                collect_interface_casts_expr(e, interface_names, sink);
            }
        }
        Expression::ConditionalOperator(_, c, a, b) => {
            collect_interface_casts_expr(c, interface_names, sink);
            collect_interface_casts_expr(a, interface_names, sink);
            collect_interface_casts_expr(b, interface_names, sink);
        }
        Expression::Assign(_, a, b) => {
            collect_interface_casts_expr(a, interface_names, sink);
            collect_interface_casts_expr(b, interface_names, sink);
        }
        Expression::ArrayLiteral(_, values) => {
            for v in values {
                collect_interface_casts_expr(v, interface_names, sink);
            }
        }
        _ => {}
    }
}

/// Task #83 — expression-level half of `collect_new_contract_refs`. Matches
/// `Expression::New(FunctionCall(Variable(name), _))` and recurses through
/// the usual expression containers.
fn collect_new_refs_expr(
    expr: &Expression,
    primary_names: &std::collections::HashSet<String>,
    sink: &mut std::collections::HashSet<String>,
) {
    if let Expression::New(_, inner) = expr {
        if let Expression::FunctionCall(_, func, _) = inner.as_ref() {
            if let Expression::Variable(id) = func.as_ref() {
                if primary_names.contains(&id.name) {
                    sink.insert(id.name.clone());
                }
            }
        }
    }
    // Task K4 — `B(addr)` cast expressions mean A plans to call into B
    // through an address typed as B. The parser lowers these as
    // `FunctionCall(Variable("B"), [addr])`, identical in shape to a
    // `B.staticCall(addr)` helper, so we match on that before the generic
    // FunctionCall recursion below picks off the args.
    if let Expression::FunctionCall(_, func, _) = expr {
        if let Expression::Variable(id) = func.as_ref() {
            if primary_names.contains(&id.name) {
                sink.insert(id.name.clone());
            }
        }
    }
    match expr {
        Expression::New(_, i)
        | Expression::Parenthesis(_, i)
        | Expression::MemberAccess(_, i, _)
        | Expression::Delete(_, i) => collect_new_refs_expr(i, primary_names, sink),
        Expression::FunctionCall(_, func, args) => {
            collect_new_refs_expr(func, primary_names, sink);
            for a in args {
                collect_new_refs_expr(a, primary_names, sink);
            }
        }
        // Task #125 — `try X { ... } catch ...` parses the leading
        // `try-expr { success-block }` as `FunctionCallBlock(call, block)`
        // on the expression-tree side, so `try Target(t).willRevert() { ... }`
        // arrives here with `call = FunctionCall(MemberAccess(FunctionCall(
        // Variable("Target"), [t]), "willRevert"), [])` wrapped in a
        // FunctionCallBlock. Without this arm the walker's `_ => {}`
        // silently dropped the cast chain, so `Target` never made the
        // sibling-merge `referenced` set and `willRevert` never entered
        // C's `self_method_offsets` table — the runtime's
        // `handle_contract_call` then fell through to `invoke_native_contract`
        // which returned `Null` for the zero-placeholder hash, so the
        // target's `revert("bad")` was never dispatched and the outer
        // try-arm fired with its literal "ok" instead of the expected
        // `catch Error(string)` binding. The success block body is a
        // Statement, not an Expression, so we use the statement walker
        // for it — symmetric with the `Statement::Try` arm above.
        Expression::FunctionCallBlock(_, call, block) => {
            collect_new_refs_expr(call, primary_names, sink);
            collect_new_contract_refs(block, primary_names, sink);
        }
        Expression::NamedFunctionCall(_, func, args) => {
            collect_new_refs_expr(func, primary_names, sink);
            for a in args {
                collect_new_refs_expr(&a.expr, primary_names, sink);
            }
        }
        Expression::ArraySubscript(_, a, b) => {
            collect_new_refs_expr(a, primary_names, sink);
            if let Some(e) = b {
                collect_new_refs_expr(e, primary_names, sink);
            }
        }
        Expression::ConditionalOperator(_, c, a, b) => {
            collect_new_refs_expr(c, primary_names, sink);
            collect_new_refs_expr(a, primary_names, sink);
            collect_new_refs_expr(b, primary_names, sink);
        }
        Expression::Assign(_, a, b) => {
            collect_new_refs_expr(a, primary_names, sink);
            collect_new_refs_expr(b, primary_names, sink);
        }
        Expression::ArrayLiteral(_, values) => {
            for v in values {
                collect_new_refs_expr(v, primary_names, sink);
            }
        }
        _ => {}
    }
}
