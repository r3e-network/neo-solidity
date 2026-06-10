/// Task #91 — inlinable metadata for a library function whose first
/// parameter is a storage-pointer struct. `param_names[0]` is bound as a
/// storage alias to the receiver's `StorageReference`; `param_names[1..]`
/// are bound as locals populated from the call-site args. See
/// `member_calls.rs::inline_library_storage_call`.
#[derive(Debug, Clone)]
struct LibraryStorageBody {
    param_names: Vec<String>,
    value_param_types: Vec<Option<ValueType>>,
    body: Statement,
    return_type: Option<ValueType>,
}

/// Structured diagnostic emitted during IR lowering.
///
/// Captures the originating function name, a human-readable message, and an
/// optional actionable suggestion so that CLI consumers can render richer
/// error output than a bare string.
#[derive(Debug, Clone, serde::Serialize)]
pub struct IrDiagnostic {
    pub function_name: String,
    pub message: String,
    pub suggestion: Option<String>,
    pub code: Option<String>,
}

impl IrDiagnostic {
    /// Format as a human-readable error string.
    pub fn display(&self) -> String {
        let mut out = format!("function '{}': {}", self.function_name, self.message);
        if let Some(ref suggestion) = self.suggestion {
            out.push_str(&format!("\n  help: {suggestion}"));
        }
        out
    }
}

struct LoweringContext<'a> {
    function_name: String,
    /// Name of the contract that owns this lowering context.
    ///
    /// Used by member-access selector resolution to look up `this.method.selector`
    /// expressions against the current contract's method table so they can be
    /// lowered to their Ethereum keccak-4 selectors (matching how the AST shape
    /// `MemberAccess(MemberAccess(Variable("this"), method), selector)` is
    /// expected to behave in Solidity).
    current_contract_name: String,
    current_function_selector: [u8; 4],
    is_safe: bool,
    /// Task #64 — whether this function is directly callable from the host
    /// (External or Public visibility). When true, multi-value returns are
    /// lowered through `abiEncode` so the main-frame RET emits EVM-canonical
    /// BE-packed bytes rather than a `StackItem::Array` that would leak as
    /// serde_json at `stack_item_to_bytes`. Internal/Private functions keep
    /// the Array shape so callers can destructure via `ArrayGet`.
    is_externally_callable: bool,
    param_index_map: HashMap<String, usize>,
    param_types: &'a [ValueType],
    return_slots: Vec<Option<usize>>,
    return_types: Vec<ValueType>,
    /// Task #185 — original Solidity type string for each declared return
    /// parameter (e.g. `uint[3][2]`). Used by `lower_return_statement` to
    /// detect nested fixed-size arrays (`T[N1][N2]...[Nk]`) so the return
    /// path can emit flat EVM-canonical static encoding instead of the
    /// dynamic-array offset+length wrapper. ValueType alone cannot carry
    /// fixed-size info today (adding `fixed_size: Option<u64>` to
    /// `ValueType::Array` would cascade through 55+ call sites), so the
    /// fixed-shape hint rides alongside as a string.
    return_type_strings: Vec<String>,
    state_variables: &'a [StateVariableMetadata],
    state_index_map: &'a HashMap<String, usize>,
    state_types: &'a [ValueType],
    /// Canonical struct type definitions available to the compilation unit.
    ///
    /// This enables resolving user-defined structs even when they are only used
    /// in local variables (i.e., not present in state/param/return types).
    defined_struct_types: &'a [ValueType],
    /// Compile-time bound `N` for every fixed-size array struct field
    /// (`struct S { uint256[3] arr; }`), keyed by `(struct_name, field_name)`.
    ///
    /// `ValueType::Array` collapses `T[N]` and `T[]` into one variant, but the
    /// storage layout differs: fixed-size fields never maintain a length slot,
    /// so the struct-field array bounds guard (storage-soundness fix, see
    /// `lower_array_subscript_expression`) must use the declared `N` instead
    /// of loading a length that would always read 0.
    struct_fixed_array_bounds: &'a HashMap<(String, String), u64>,
    event_index_map: &'a HashMap<String, usize>,
    event_signature_map: &'a HashMap<String, Vec<ManifestType>>,
    event_params_map: &'a HashMap<String, EventSignature>,
    /// Declared custom `error` signatures keyed by error name. Used by the
    /// revert/require lowering to compute EVM custom-error selectors from
    /// the DECLARED parameter types and to reorder named arguments into
    /// declaration order.
    error_signature_map: &'a HashMap<String, ErrorAbiSignature>,
    enum_variant_map: &'a HashMap<String, HashMap<String, u64>>,
    contract_types: &'a HashSet<String>,
    selector_registry: &'a SelectorRegistry,
    function_names: &'a HashSet<String>,
    function_overloads: &'a HashMap<(String, usize), String>,
    /// Every observed first-parameter type for each overload key
    /// (name, arg_count).
    ///
    /// Used to enforce Solidity-style receiver compatibility for `using for`
    /// member calls (`x.f(...)` lowers to `f(x, ...)`). Solidity allows
    /// overloading by parameter type — `toInt128(int256)` and
    /// `toInt128(uint256)` share the same `(name, arity)` key — so the
    /// bucket stores ALL observed first-parameter types and the receiver
    /// check accepts a match against ANY of them.
    function_first_param_types: &'a HashMap<(String, usize), Vec<ValueType>>,
    /// Task #191 — first return-parameter type for each overload key
    /// (name, arg_count). Used by `infer_type_from_expression` to resolve
    /// member access on a FunctionCall result (e.g. `makeCounter().value`
    /// or `c.inc().value`), so the struct-field-access lowering can pick
    /// the right field index instead of falling through to the drop-and-
    /// push-zero compatibility branch. Only the FIRST return type is
    /// recorded because member access on a multi-return function call is
    /// not valid Solidity — `(a, b) = f()` uses tuple destructuring.
    function_return_types: &'a HashMap<(String, usize), ValueType>,
    /// Normalized target types from parsed `using` directives.
    ///
    /// `None` means wildcard target (`for *`).
    using_target_types: &'a [Option<String>],
    /// Function-list constraints from `using {f, g} for T`.
    ///
    /// Map key is normalized function name (lowercase) and values are allowed
    /// target types (`None` means wildcard).
    using_function_list_targets: &'a HashMap<String, Vec<Option<String>>>,
    /// Target scopes that have at least one `using { ... } for T` directive.
    using_function_list_scope_targets: &'a [Option<String>],
    /// Ordered parameter names for each function overload, keyed by (name, arg_count).
    /// Used to reorder named function call arguments into positional order.
    function_param_names: &'a HashMap<(String, usize), Vec<String>>,
    /// Functions that return void (empty return_parameters). Used to avoid
    /// emitting DROP after calling a void internal function as a statement.
    void_functions: &'a HashSet<String>,
    /// Mapping from original method name to renamed super-method name.
    /// Used to resolve `super.method()` calls during IR lowering.
    super_method_map: &'a HashMap<String, String>,
    /// Task #91 — library functions whose first parameter is `T storage`,
    /// keyed by (name, arg_count). `member_calls.rs` inlines the body at
    /// the call site rather than emitting `CallFunction`, so storage writes
    /// (`d.x = v`) hit the caller's slot instead of a materialised copy.
    library_storage_bodies: &'a HashMap<(String, usize), LibraryStorageBody>,
    /// Task #196 — zero-arg internal functions that trivially return a
    /// storage pointer to a state variable (body is `return <state_var>;`
    /// and the return parameter is declared `T storage`). When a call site
    /// references `foo()` whose result feeds into a storage operation
    /// (`foo().push(v)`, `foo().length`, `foo()[i] = v`), the resolver
    /// unwraps the call into the backing `Variable(state_var)` so the
    /// downstream storage-reference machinery can alias the actual slot
    /// instead of the raw `LoadState` value (which for an array state
    /// variable is the LENGTH, not the backing Array — see
    /// `emit_coerce_storage_value` for `ValueType::Array`). Extends Task
    /// #117's local-binding alias fix (`uint[] storage a = arr;`) across
    /// the function-return boundary.
    storage_pointer_returning_fns: &'a HashMap<String, String>,
    /// Task #91 — stack of (inline-return slot, end-label). When set,
    /// `lower_return_statement` redirects `return expr;` to store into
    /// `slot` and jump to `end_label` instead of emitting a raw `Return`
    /// that would exit the caller.
    inline_return_stack: Vec<(Option<usize>, usize)>,
    /// Task #114 — modifier-epilogue return redirect. When set by
    /// `function.rs` for a function whose body was wrapped by at least one
    /// modifier with an epilogue (statements after `_;`), every
    /// `Statement::Return(expr)` in the body must store into `slots` (one
    /// per declared return parameter, already allocated at function
    /// prologue) and jump to the INNERMOST modifier-wrap break label (see
    /// `modifier_break_stack`) — or to `end_label` as a fallback when the
    /// wrap hasn't been entered yet. The modifier-wrap break label lands
    /// inside the Solidity expansion BETWEEN the inlined body and the
    /// modifier epilogue, so tail statements (`locked = 0;` and friends)
    /// still run before the actual RET. Distinct from the library-inline
    /// single-slot mechanism (`inline_return_stack`): modifier returns must
    /// carry multi-value tuples when the function declares `returns (T, U)`.
    modifier_return_redirect: Option<(Vec<Option<usize>>, usize)>,
    /// Task #114 — stack of break labels belonging to the synthetic
    /// `do { body } while(false)` wrappers emitted by
    /// `apply_modifier_calls_to_body_with_epilogue`. Unlike the normal
    /// `loop_stack`, this tracks ONLY modifier-wrap scopes so a `return`
    /// inside a user loop jumps past the user loop and into the OUTERMOST
    /// modifier wrap's epilogue chain. Innermost wrap sits at the top.
    modifier_break_stack: Vec<usize>,
    local_index_map: HashMap<String, Vec<usize>>,
    local_types: HashMap<usize, ValueType>,
    scope_stack: Vec<Vec<String>>,
    storage_aliases: HashMap<String, StorageReference>,
    call_data_locals: HashMap<usize, String>,
    local_count: u16,
    label_counter: usize,
    loop_stack: Vec<LoopLabels>,
    /// State variable indices currently being inlined (constant resolution).
    /// Used to break infinite recursion when a constant's initializer
    /// transitively references itself through a cross-contract alias.
    resolving_constants: Vec<usize>,
    errors: Vec<IrDiagnostic>,
    warnings: Vec<crate::solidity::Diagnostic>,
    /// Task #30: nested `unchecked { }` depth. When > 0, binary-arithmetic
    /// lowerings skip the Solidity-0.8.x checked overflow guard emission.
    /// Counter semantics (vs. a bool) correctly handle nested blocks:
    /// ```text
    /// unchecked { unchecked { a + b; } }  // depth 2 inside; guard still skipped
    /// ```
    unchecked_depth: usize,
    /// Task #186 — function-pointer parameter/local bindings keyed by the
    /// binding name. Populated by `Function::from_metadata_with_warnings` for
    /// parameters whose declared Solidity type starts with `function` (the
    /// only source of function-pointer values currently supported). Consumed
    /// by `try_lower_variable_call` to emit a `CallIndirect` through `CALLA`
    /// instead of the legacy "drop args, push 0" compatibility fallback.
    function_pointer_bindings: HashMap<String, FunctionPointerBinding>,
}

/// Task #186 — per-binding metadata for an internal function-pointer local or
/// parameter. Knowing `arg_count` lets the bytecode emitter pick the correct
/// REVERSEN window size; `has_return` controls whether the `CallIndirect`
/// result is treated as a value or a statement-expression.
#[derive(Debug, Clone, Copy)]
struct FunctionPointerBinding {
    arg_count: usize,
    has_return: bool,
}

impl<'a> LoweringContext<'a> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        function_name: &str,
        current_contract_name: &str,
        current_function_selector: [u8; 4],
        is_safe: bool,
        is_externally_callable: bool,
        param_index_map: HashMap<String, usize>,
        param_types: &'a [ValueType],
        state_variables: &'a [StateVariableMetadata],
        state_index_map: &'a HashMap<String, usize>,
        state_types: &'a [ValueType],
        defined_struct_types: &'a [ValueType],
        struct_fixed_array_bounds: &'a HashMap<(String, String), u64>,
        event_index_map: &'a HashMap<String, usize>,
        event_signature_map: &'a HashMap<String, Vec<ManifestType>>,
        event_params_map: &'a HashMap<String, EventSignature>,
        error_signature_map: &'a HashMap<String, ErrorAbiSignature>,
        enum_variant_map: &'a HashMap<String, HashMap<String, u64>>,
        contract_types: &'a HashSet<String>,
        selector_registry: &'a SelectorRegistry,
        function_names: &'a HashSet<String>,
        function_overloads: &'a HashMap<(String, usize), String>,
        function_first_param_types: &'a HashMap<(String, usize), Vec<ValueType>>,
        function_return_types: &'a HashMap<(String, usize), ValueType>,
        using_target_types: &'a [Option<String>],
        using_function_list_targets: &'a HashMap<String, Vec<Option<String>>>,
        using_function_list_scope_targets: &'a [Option<String>],
        function_param_names: &'a HashMap<(String, usize), Vec<String>>,
        void_functions: &'a HashSet<String>,
        super_method_map: &'a HashMap<String, String>,
        library_storage_bodies: &'a HashMap<(String, usize), LibraryStorageBody>,
        storage_pointer_returning_fns: &'a HashMap<String, String>,
    ) -> Self {
        Self {
            function_name: function_name.to_string(),
            current_contract_name: current_contract_name.to_string(),
            current_function_selector,
            is_safe,
            is_externally_callable,
            param_index_map,
            param_types,
            return_slots: Vec::new(),
            return_types: Vec::new(),
            return_type_strings: Vec::new(),
            state_variables,
            state_index_map,
            state_types,
            defined_struct_types,
            struct_fixed_array_bounds,
            event_index_map,
            event_signature_map,
            event_params_map,
            error_signature_map,
            enum_variant_map,
            contract_types,
            selector_registry,
            function_names,
            function_overloads,
            function_first_param_types,
            function_return_types,
            using_target_types,
            using_function_list_targets,
            using_function_list_scope_targets,
            function_param_names,
            void_functions,
            super_method_map,
            library_storage_bodies,
            storage_pointer_returning_fns,
            inline_return_stack: Vec::new(),
            modifier_return_redirect: None,
            modifier_break_stack: Vec::new(),
            local_index_map: HashMap::new(),
            local_types: HashMap::new(),
            scope_stack: vec![Vec::new()],
            storage_aliases: HashMap::new(),
            call_data_locals: HashMap::new(),
            local_count: 0,
            label_counter: 0,
            loop_stack: Vec::new(),
            resolving_constants: Vec::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            unchecked_depth: 0,
            function_pointer_bindings: HashMap::new(),
        }
    }

    /// Task #186 — register a function-pointer binding for a parameter or
    /// local. `name` is the Solidity source name; `arg_count` and `has_return`
    /// are derived from the declared `function(...)` type.
    fn register_function_pointer_binding(
        &mut self,
        name: &str,
        arg_count: usize,
        has_return: bool,
    ) {
        self.function_pointer_bindings.insert(
            name.to_string(),
            FunctionPointerBinding {
                arg_count,
                has_return,
            },
        );
    }

    /// Task #186 — look up a function-pointer binding for an identifier.
    fn function_pointer_binding(&self, name: &str) -> Option<&FunctionPointerBinding> {
        self.function_pointer_bindings.get(name)
    }

    /// Returns `true` if currently lowering inside an `unchecked { ... }` block.
    /// Used by `lower_binary_expr` to suppress the Solidity 0.8.x checked
    /// overflow guard emission for Add/Sub/Mul.
    fn in_unchecked_block(&self) -> bool {
        self.unchecked_depth > 0
    }

    /// Increment unchecked depth when entering `unchecked { ... }`.
    fn enter_unchecked_block(&mut self) {
        self.unchecked_depth = self.unchecked_depth.saturating_add(1);
    }

    /// Decrement unchecked depth when leaving an `unchecked { ... }` block.
    fn exit_unchecked_block(&mut self) {
        self.unchecked_depth = self.unchecked_depth.saturating_sub(1);
    }

    fn current_function_selector(&self) -> [u8; 4] {
        self.current_function_selector
    }

    /// Returns the name of the contract that owns this lowering context.
    ///
    /// Used to resolve `this.method.selector` expressions against the current
    /// contract's method registry when the inner expression is
    /// `Expression::Variable("this")`.
    fn current_contract_name(&self) -> &str {
        &self.current_contract_name
    }

    fn set_return_info(&mut self, slots: Vec<Option<usize>>, types: Vec<ValueType>) {
        self.return_slots = slots;
        self.return_types = types;
    }

    /// Task #185 — record the original Solidity type strings for each
    /// declared return parameter. Called once per function during
    /// `Function::from_metadata_with_warnings`. The strings retain static
    /// fixed-size-array dimensions (e.g. `uint[3][2]`) that `ValueType`
    /// doesn't preserve, enabling `lower_return_statement` to pick the
    /// flat EVM-canonical encoding for nested fixed-size array returns.
    fn set_return_type_strings(&mut self, type_strings: Vec<String>) {
        self.return_type_strings = type_strings;
    }

    fn return_type_strings(&self) -> &[String] {
        &self.return_type_strings
    }

    /// Task #91 — temporarily hide caller parameters that collide with an
    /// inlined library parameter name (`setX(uint256 v) { d.store(v); }` would
    /// otherwise resolve `v` inside the body to the caller's
    /// `LoadParameter(0)`). Returns the previous entry for later restoration.
    fn hide_param_binding(&mut self, name: &str) -> Option<usize> {
        self.param_index_map.remove(name)
    }

    fn restore_param_binding(&mut self, name: String, index: Option<usize>) {
        if let Some(idx) = index {
            self.param_index_map.insert(name, idx);
        }
    }

    /// Task #91 — push/pop an inline-return redirect; see
    /// `inline_return_stack` docs and `lower_return_statement`.
    fn push_inline_return(&mut self, slot: Option<usize>, end_label: usize) {
        self.inline_return_stack.push((slot, end_label));
    }

    fn pop_inline_return(&mut self) {
        self.inline_return_stack.pop();
    }

    /// Return the current inline-return target, if any.
    fn inline_return_target(&self) -> Option<(Option<usize>, usize)> {
        self.inline_return_stack.last().copied()
    }

    /// Task #114 — activate the modifier-epilogue return redirect. `slots`
    /// is a per-return-parameter `LoadLocal` index (one per declared return),
    /// and `end_label` is emitted after the expanded body. Inside
    /// `lower_return_statement`, a Return expression stores into the slots
    /// (in declaration order for multi-return) and jumps to `end_label`,
    /// bypassing the raw RET that would otherwise skip the modifier
    /// epilogue (e.g. `locked = 0;` after `_;`).
    fn set_modifier_return_redirect(&mut self, slots: Vec<Option<usize>>, end_label: usize) {
        self.modifier_return_redirect = Some((slots, end_label));
    }

    fn clear_modifier_return_redirect(&mut self) {
        self.modifier_return_redirect = None;
    }

    /// Return the current modifier-return redirect (slots, end_label) if set.
    /// The slots Vec mirrors `return_slots` (one entry per declared return).
    fn modifier_return_target(&self) -> Option<(Vec<Option<usize>>, usize)> {
        self.modifier_return_redirect.clone()
    }

    /// Task #114 — push a modifier-wrap break label (from the synthetic
    /// `do { body } while(false)` emitted by the Solidity expander). Used by
    /// `lower_do_while_statement` when `had_modifier_epilogue` is active AND
    /// the condition is the constant `false` we inject — see
    /// `src/solidity/analyse/modifiers/expand.rs`.
    fn push_modifier_break_label(&mut self, label: usize) {
        self.modifier_break_stack.push(label);
    }

    fn pop_modifier_break_label(&mut self) {
        self.modifier_break_stack.pop();
    }

    /// Return the innermost modifier-wrap break label, or `None` if we are
    /// not currently inside a modifier-epilogue scope. Used by
    /// `lower_return_statement` to pick the correct jump target when
    /// redirecting `return expr;` past user loops.
    fn innermost_modifier_break_label(&self) -> Option<usize> {
        self.modifier_break_stack.last().copied()
    }

    /// Returns `true` iff this function was flagged with
    /// `had_modifier_epilogue` at the Solidity analyse layer. Mirrors the
    /// `modifier_return_redirect` being Some — that redirect is only set by
    /// `function.rs` when the flag was true.
    fn in_modifier_epilogue_scope(&self) -> bool {
        self.modifier_return_redirect.is_some()
    }

    fn return_slots(&self) -> &[Option<usize>] {
        &self.return_slots
    }

    fn return_types(&self) -> &[ValueType] {
        &self.return_types
    }

    fn is_externally_callable(&self) -> bool {
        self.is_externally_callable
    }

    fn next_label(&mut self) -> usize {
        let label = self.label_counter;
        self.label_counter += 1;
        label
    }

    fn push_loop(&mut self, continue_label: usize, break_label: usize) {
        self.loop_stack.push(LoopLabels {
            continue_label,
            break_label,
        });
    }

    fn pop_loop(&mut self) {
        self.loop_stack.pop();
    }

    fn break_target(&self) -> Option<usize> {
        self.loop_stack.last().map(|labels| labels.break_label)
    }

    fn continue_target(&self) -> Option<usize> {
        self.loop_stack.last().map(|labels| labels.continue_label)
    }

    fn record_error(&mut self, message: impl Into<String>) {
        self.errors.push(IrDiagnostic {
            function_name: self.function_name.clone(),
            message: message.into(),
            suggestion: None,
            code: None,
        });
    }

    fn record_error_with_suggestion(
        &mut self,
        message: impl Into<String>,
        suggestion: impl Into<String>,
    ) {
        self.errors.push(IrDiagnostic {
            function_name: self.function_name.clone(),
            message: message.into(),
            suggestion: Some(suggestion.into()),
            code: None,
        });
    }

    fn record_warning(&mut self, message: impl Into<String>) {
        self.warnings
            .push(crate::solidity::Diagnostic::warning(message));
    }

    fn record_warning_with_suggestion(
        &mut self,
        message: impl Into<String>,
        suggestion: impl Into<String>,
    ) {
        self.warnings
            .push(crate::solidity::Diagnostic::warning(message).with_suggestion(suggestion));
    }

    fn set_call_data_local(&mut self, local_index: usize, method: String) {
        self.call_data_locals.insert(local_index, method);
    }

    fn clear_call_data_local(&mut self, local_index: usize) {
        self.call_data_locals.remove(&local_index);
    }

    fn call_data_method_for_local(&self, local_index: usize) -> Option<&str> {
        self.call_data_locals
            .get(&local_index)
            .map(|method| method.as_str())
    }

    fn is_contract_type_name(&self, name: &str) -> bool {
        self.contract_types.contains(name)
    }

    /// Returns `true` if the given state variable index is currently being
    /// resolved (constant inlining). Used to break infinite recursion when a
    /// constant's initializer transitively references itself.
    fn is_resolving_constant(&self, index: usize) -> bool {
        self.resolving_constants.contains(&index)
    }

    fn push_resolving_constant(&mut self, index: usize) {
        self.resolving_constants.push(index);
    }

    fn pop_resolving_constant(&mut self) {
        self.resolving_constants.pop();
    }

    fn type_method_selectors(&self, type_name: &str, method_name: &str) -> Option<&Vec<[u8; 4]>> {
        self.selector_registry
            .type_method_selectors
            .get(type_name)
            .and_then(|methods| methods.get(method_name))
    }

    fn is_interface_type_name(&self, name: &str) -> bool {
        self.selector_registry.interface_types.contains(name)
    }

    fn interface_id_for_type(&self, type_name: &str) -> Option<[u8; 4]> {
        let methods = self
            .selector_registry
            .type_method_selectors
            .get(type_name)?;
        let mut selectors: HashSet<[u8; 4]> = HashSet::new();
        for overloads in methods.values() {
            for selector in overloads {
                selectors.insert(*selector);
            }
        }

        let mut interface_id = [0u8; 4];
        for selector in selectors {
            for (idx, byte) in selector.iter().enumerate() {
                interface_id[idx] ^= byte;
            }
        }
        Some(interface_id)
    }

    fn state_type(&self, index: usize) -> Option<&ValueType> {
        self.state_types.get(index)
    }

    fn state_metadata(&self, index: usize) -> Option<&StateVariableMetadata> {
        self.state_variables.get(index)
    }

    /// Compile-time bound `N` when `struct_name.field_name` is declared as a
    /// fixed-size array (`T[N]`), `None` for dynamic (`T[]`) or non-array
    /// fields. See `struct_fixed_array_bounds`.
    fn struct_fixed_array_bound(&self, struct_name: &str, field_name: &str) -> Option<u64> {
        self.struct_fixed_array_bounds
            .get(&(struct_name.to_string(), field_name.to_string()))
            .copied()
    }

    fn can_write_state(&self, state_index: usize) -> bool {
        let Some(meta) = self.state_metadata(state_index) else {
            return true;
        };

        if !meta.is_immutable {
            return true;
        }

        // Sibling-merge (Task #198 in solidity_analyse.rs) renames sibling
        // constructors to `__ctor__<SiblingName>` and re-types them as
        // `FunctionTy::Function` so the host's `_deploy` prologue doesn't
        // accidentally re-run them. The body still semantically runs as
        // construction code (it's invoked from the host's `new Sibling(...)`
        // lowering), so it must remain allowed to assign to immutable
        // state vars — same as a regular constructor.
        //
        // Inheritance flattening additionally preserves a base constructor
        // body under `__super_<original>` / `__super2_<original>` etc. when
        // a derived contract overrides the constructor's name. Those super-
        // bodies also run as construction code from the derived constructor,
        // so they need the same exemption. Concrete repro: Chainlink
        // AutomationRegistry2_3 inherits from AutomationForwarder whose
        // constructor writes to `immutable i_target`; the flattener stores
        // the base body as `__super___ctor__AutomationForwarder` and the
        // immutable-write check rejected it without this clause.
        self.function_name == "constructor"
            || self.function_name == "_deploy"
            || self.function_name.starts_with("__ctor__")
            || self.function_name.contains("__ctor__")
    }

    fn ensure_state_writable(&mut self, state_index: usize) -> bool {
        if self.can_write_state(state_index) {
            return true;
        }

        let variable_name = self
            .state_metadata(state_index)
            .and_then(|meta| meta.name.as_deref())
            .unwrap_or("<unnamed>")
            .to_string();

        self.record_error_with_suggestion(
            format!(
                "cannot assign to immutable state variable '{variable_name}' outside constructor/deploy initialization"
            ),
            "initialize immutable values in the declaration or constructor only",
        );
        false
    }

    fn parameter_type(&self, name: &str) -> Option<&ValueType> {
        self.param_index_map
            .get(name)
            .and_then(|idx| self.param_types.get(*idx))
    }

    fn local_type(&self, index: usize) -> Option<&ValueType> {
        self.local_types.get(&index)
    }

    fn variable_type(&self, name: &str) -> Option<ValueType> {
        if let Some(reference) = self.storage_alias(name) {
            return Some(reference.value_type.clone());
        }
        if let Some(index) = self.state_index_map.get(name) {
            if let Some(ty) = self.state_type(*index) {
                return Some(ty.clone());
            }
        }
        if let Some(ty) = self.parameter_type(name) {
            return Some(ty.clone());
        }
        if let Some(local_index) = self.resolve_local(name) {
            if let Some(ty) = self.local_type(local_index) {
                return Some(ty.clone());
            }
        }
        None
    }

    fn neo_function_name(&self, name: &str, arg_count: usize) -> Option<String> {
        self.function_overloads
            .get(&(name.to_string(), arg_count))
            .cloned()
    }

    /// Task #91 — fetch the inlinable body for a library function whose first
    /// parameter is a storage-pointer struct. Returns `None` when the call
    /// should go through the normal `CallFunction` path.
    fn library_storage_body(&self, name: &str, arg_count: usize) -> Option<&LibraryStorageBody> {
        self.library_storage_bodies
            .get(&(name.to_string(), arg_count))
    }

    /// Task #196 — look up the state variable name aliased by a zero-arg
    /// internal function returning `T storage`. Returns `None` when the
    /// function isn't a simple storage-pointer alias. Used by the
    /// storage-reference resolver and the `.length` fast path to unwrap
    /// `fn()` into the backing state-var Expression so the caller can
    /// write to the actual storage slot instead of a materialised copy.
    fn storage_pointer_returning_fn(&self, name: &str) -> Option<&str> {
        self.storage_pointer_returning_fns
            .get(name)
            .map(|s| s.as_str())
    }

    fn has_using_directives(&self) -> bool {
        !self.using_target_types.is_empty()
    }

    fn using_target_allows_receiver(&self, receiver_type: &ValueType) -> bool {
        if self.using_target_types.is_empty() {
            return false;
        }

        let receiver_sig =
            normalize_solidity_like_type_signature(&value_type_signature(receiver_type));
        self.using_target_types.iter().any(|target| match target {
            None => true,
            Some(target_type) => using_target_matches_signature(target_type, &receiver_sig),
        })
    }

    fn using_function_list_allows_receiver(
        &self,
        function_name: &str,
        receiver_type: Option<&ValueType>,
    ) -> bool {
        let Some(receiver_type) = receiver_type else {
            // Unknown receiver type: preserve compatibility, defer to runtime behavior.
            return true;
        };

        let receiver_sig =
            normalize_solidity_like_type_signature(&value_type_signature(receiver_type));
        let list_scope_applies = self.using_function_list_scope_targets.iter().any(|target| {
            target
                .as_ref()
                .is_none_or(|expected| using_target_matches_signature(expected, &receiver_sig))
        });

        // No function-list directives apply to this receiver type.
        if !list_scope_applies {
            return true;
        }

        let key = function_name.to_ascii_lowercase();
        let Some(targets) = self.using_function_list_targets.get(&key) else {
            // Function-list directives apply, but this function name was not listed.
            return false;
        };

        targets.iter().any(|target| match target {
            None => true,
            Some(target_type) => using_target_matches_signature(target_type, &receiver_sig),
        })
    }

    fn receiver_matches_function_overload(
        &self,
        function_name: &str,
        arg_count: usize,
        receiver_type: &ValueType,
    ) -> bool {
        let key = (function_name.to_string(), arg_count);
        let Some(expected_types) = self.function_first_param_types.get(&key) else {
            // Without type metadata, don't over-constrain existing behavior.
            return true;
        };
        // A library `using` directive resolves the receiver against ANY
        // overload of the named function — Solidity's overload resolution
        // picks the first one whose parameter types are implicitly
        // convertible from the actuals. We mirror that here: accept the call
        // if any registered overload's first parameter accepts the receiver.
        // This fixes Uniswap V4 `int256.toInt128()` after we taught the IR
        // builder to keep BOTH `toInt128(int256)` and `toInt128(uint256)`
        // entries in the same bucket.
        expected_types
            .iter()
            .any(|expected| is_implicitly_convertible(receiver_type, expected))
    }

    /// Returns the ordered parameter names for a function overload, if known.
    fn get_function_param_names(&self, name: &str, arg_count: usize) -> Option<&[String]> {
        self.function_param_names
            .get(&(name.to_string(), arg_count))
            .map(|v| v.as_slice())
    }

    /// Task #191 — look up the first return type for a function overload.
    /// `None` means no metadata (the function may be external, have no
    /// return value, or the build pipeline didn't register it). Callers
    /// should degrade gracefully (e.g., `infer_type_from_expression` falls
    /// through to the existing type-inference logic).
    fn get_function_return_type(&self, name: &str, arg_count: usize) -> Option<&ValueType> {
        self.function_return_types
            .get(&(name.to_string(), arg_count))
    }

    /// Returns true if the named function returns void (no return values).
    fn is_void_function(&self, name: &str) -> bool {
        self.void_functions.contains(name)
    }

    /// Returns the renamed super-method name for `super.method()` resolution.
    ///
    /// Task #85 — look up the caller-qualified key first
    /// (`"{current_fn}::{method_name}"`) so that inside a preserved base body
    /// `__super_foo` the nested `super.foo()` resolves to the NEXT-older
    /// `__super2_foo`, not back to itself. Falls back to the unqualified key
    /// for the top-of-chain derived frame and for any legacy call sites.
    fn super_method_name(&self, method_name: &str) -> Option<&str> {
        let qualified = format!("{}::{}", self.function_name, method_name);
        if let Some(target) = self.super_method_map.get(&qualified) {
            return Some(target.as_str());
        }
        self.super_method_map.get(method_name).map(|s| s.as_str())
    }

    fn event_signature(&self, event_name: &str) -> Option<&[ManifestType]> {
        self.event_signature_map
            .get(event_name)
            .map(|sig| sig.as_slice())
    }

    fn event_evm_signature(&self, event_name: &str) -> Option<&EventSignature> {
        self.event_params_map.get(event_name)
    }

    /// Declared signature for a custom `error`, or `None` when the name was
    /// never declared in (or inherited by / file-level-merged into) the
    /// current contract — callers then fall back to inferring canonical
    /// types from the revert-site argument expressions.
    fn error_signature(&self, error_name: &str) -> Option<&ErrorAbiSignature> {
        self.error_signature_map.get(error_name)
    }

    fn allocate_local(&mut self, name: String, value_type: Option<ValueType>) -> usize {
        let index = self.local_count as usize;
        self.local_count = self.local_count.checked_add(1).unwrap_or(self.local_count);
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.push(name.clone());
        }
        self.local_index_map.entry(name).or_default().push(index);
        if let Some(ty) = value_type {
            self.local_types.insert(index, ty);
        }
        index
    }

    fn resolve_local(&self, name: &str) -> Option<usize> {
        self.local_index_map
            .get(name)
            .and_then(|stack| stack.last().copied())
    }

    fn ensure_local(&mut self, name: &str) -> usize {
        if let Some(index) = self.resolve_local(name) {
            index
        } else {
            self.allocate_local(name.to_string(), None)
        }
    }

    fn enter_scope(&mut self) {
        self.scope_stack.push(Vec::new());
    }

    fn exit_scope(&mut self) {
        if let Some(names) = self.scope_stack.pop() {
            for name in names {
                if let Some(stack) = self.local_index_map.get_mut(&name) {
                    if let Some(index) = stack.pop() {
                        self.local_types.remove(&index);
                    }
                    if stack.is_empty() {
                        self.local_index_map.remove(&name);
                    }
                }
                self.storage_aliases.remove(&name);
            }
        }
    }

    fn is_local_in_current_scope(&self, name: &str) -> bool {
        self.scope_stack
            .last()
            .is_some_and(|scope| scope.iter().any(|existing| existing == name))
    }

    fn set_storage_alias(&mut self, name: String, alias: StorageReference) {
        self.storage_aliases.insert(name, alias);
    }

    fn storage_alias(&self, name: &str) -> Option<&StorageReference> {
        self.storage_aliases.get(name)
    }
}

fn normalize_solidity_like_type_signature(raw: &str) -> String {
    let compact = raw
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>()
        .replace("payable", "");
    let lowered = compact.to_ascii_lowercase();
    match lowered.as_str() {
        "uint" => "uint256".to_string(),
        "int" => "int256".to_string(),
        "byte" => "bytes1".to_string(),
        other => other.to_string(),
    }
}

fn value_type_signature(value_type: &ValueType) -> String {
    match value_type {
        ValueType::Integer { signed: true, bits } => format!("int{bits}"),
        ValueType::Integer {
            signed: false,
            bits,
        } => format!("uint{bits}"),
        ValueType::Boolean => "bool".to_string(),
        ValueType::String => "string".to_string(),
        ValueType::Address => "address".to_string(),
        ValueType::ByteArray {
            fixed_len: Some(len),
        } => format!("bytes{len}"),
        ValueType::ByteArray { fixed_len: None } => "bytes".to_string(),
        ValueType::Array(inner) => format!("{}[]", value_type_signature(inner)),
        ValueType::Mapping { key, value } => format!(
            "mapping({}=>{})",
            value_type_signature(key),
            value_type_signature(value)
        ),
        ValueType::Struct { name, .. } => name.to_ascii_lowercase(),
        ValueType::Any => "any".to_string(),
    }
}

/// Task #91 — match a `using X for T` directive target against a receiver
/// signature. The frontend renders `using L for L.Data;` as target
/// `"l.data"`, but `ValueType::Struct { name: "Data" }` normalises to
/// `"data"` (`lookup_struct` strips the qualifier). Fall back to matching
/// by the last `.`-separated segment so storage-pointer struct receivers
/// dispatch correctly.
fn using_target_matches_signature(target: &str, receiver_sig: &str) -> bool {
    target == receiver_sig
        || target
            .rsplit_once('.')
            .is_some_and(|(_, last)| last == receiver_sig)
}

fn is_implicitly_convertible(actual: &ValueType, expected: &ValueType) -> bool {
    match (actual, expected) {
        (_, ValueType::Any) | (ValueType::Any, _) => true,
        (
            ValueType::Integer {
                signed: actual_signed,
                bits: actual_bits,
            },
            ValueType::Integer {
                signed: expected_signed,
                bits: expected_bits,
            },
        ) => actual_signed == expected_signed && actual_bits <= expected_bits,
        (ValueType::Boolean, ValueType::Boolean)
        | (ValueType::String, ValueType::String)
        | (ValueType::Address, ValueType::Address) => true,
        (
            ValueType::ByteArray {
                fixed_len: actual_len,
            },
            ValueType::ByteArray {
                fixed_len: expected_len,
            },
        ) => match (actual_len, expected_len) {
            (_, None) => true,
            (Some(actual), Some(expected)) => actual == expected,
            (None, Some(_)) => false,
        },
        (ValueType::Array(actual_inner), ValueType::Array(expected_inner)) => {
            is_implicitly_convertible(actual_inner, expected_inner)
        }
        (
            ValueType::Mapping {
                key: actual_key,
                value: actual_value,
            },
            ValueType::Mapping {
                key: expected_key,
                value: expected_value,
            },
        ) => {
            is_implicitly_convertible(actual_key, expected_key)
                && is_implicitly_convertible(actual_value, expected_value)
        }
        (
            ValueType::Struct {
                name: actual_name, ..
            },
            ValueType::Struct {
                name: expected_name,
                ..
            },
        ) => actual_name.eq_ignore_ascii_case(expected_name),
        _ => false,
    }
}
