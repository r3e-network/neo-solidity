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
    is_safe: bool,
    param_index_map: HashMap<String, usize>,
    param_types: &'a [ValueType],
    return_slots: Vec<Option<usize>>,
    return_types: Vec<ValueType>,
    state_variables: &'a [StateVariableMetadata],
    state_index_map: &'a HashMap<String, usize>,
    state_types: &'a [ValueType],
    /// Canonical struct type definitions available to the compilation unit.
    ///
    /// This enables resolving user-defined structs even when they are only used
    /// in local variables (i.e., not present in state/param/return types).
    defined_struct_types: &'a [ValueType],
    event_index_map: &'a HashMap<String, usize>,
    event_signature_map: &'a HashMap<String, Vec<ManifestType>>,
    enum_variant_map: &'a HashMap<String, HashMap<String, u64>>,
    contract_types: &'a HashSet<String>,
    selector_registry: &'a SelectorRegistry,
    function_names: &'a HashSet<String>,
    function_overloads: &'a HashMap<(String, usize), String>,
    /// First parameter type for each overload key (name, arg_count).
    ///
    /// Used to enforce Solidity-style receiver compatibility for `using for`
    /// member calls (`x.f(...)` lowers to `f(x, ...)`).
    function_first_param_types: &'a HashMap<(String, usize), ValueType>,
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
    local_index_map: HashMap<String, Vec<usize>>,
    local_types: HashMap<usize, ValueType>,
    scope_stack: Vec<Vec<String>>,
    storage_aliases: HashMap<String, StorageReference>,
    call_data_locals: HashMap<usize, String>,
    local_count: u16,
    label_counter: usize,
    loop_stack: Vec<LoopLabels>,
    errors: Vec<IrDiagnostic>,
}

impl<'a> LoweringContext<'a> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        function_name: &str,
        is_safe: bool,
        param_index_map: HashMap<String, usize>,
        param_types: &'a [ValueType],
        state_variables: &'a [StateVariableMetadata],
        state_index_map: &'a HashMap<String, usize>,
        state_types: &'a [ValueType],
        defined_struct_types: &'a [ValueType],
        event_index_map: &'a HashMap<String, usize>,
        event_signature_map: &'a HashMap<String, Vec<ManifestType>>,
        enum_variant_map: &'a HashMap<String, HashMap<String, u64>>,
        contract_types: &'a HashSet<String>,
        selector_registry: &'a SelectorRegistry,
        function_names: &'a HashSet<String>,
        function_overloads: &'a HashMap<(String, usize), String>,
        function_first_param_types: &'a HashMap<(String, usize), ValueType>,
        using_target_types: &'a [Option<String>],
        using_function_list_targets: &'a HashMap<String, Vec<Option<String>>>,
        using_function_list_scope_targets: &'a [Option<String>],
        function_param_names: &'a HashMap<(String, usize), Vec<String>>,
        void_functions: &'a HashSet<String>,
        super_method_map: &'a HashMap<String, String>,
    ) -> Self {
        Self {
            function_name: function_name.to_string(),
            is_safe,
            param_index_map,
            param_types,
            return_slots: Vec::new(),
            return_types: Vec::new(),
            state_variables,
            state_index_map,
            state_types,
            defined_struct_types,
            event_index_map,
            event_signature_map,
            enum_variant_map,
            contract_types,
            selector_registry,
            function_names,
            function_overloads,
            function_first_param_types,
            using_target_types,
            using_function_list_targets,
            using_function_list_scope_targets,
            function_param_names,
            void_functions,
            super_method_map,
            local_index_map: HashMap::new(),
            local_types: HashMap::new(),
            scope_stack: vec![Vec::new()],
            storage_aliases: HashMap::new(),
            call_data_locals: HashMap::new(),
            local_count: 0,
            label_counter: 0,
            loop_stack: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn set_return_info(&mut self, slots: Vec<Option<usize>>, types: Vec<ValueType>) {
        self.return_slots = slots;
        self.return_types = types;
    }

    fn return_slots(&self) -> &[Option<usize>] {
        &self.return_slots
    }

    fn return_types(&self) -> &[ValueType] {
        &self.return_types
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
        let methods = self.selector_registry.type_method_selectors.get(type_name)?;
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

    fn can_write_state(&self, state_index: usize) -> bool {
        let Some(meta) = self.state_metadata(state_index) else {
            return true;
        };

        if !meta.is_immutable {
            return true;
        }

        self.function_name == "constructor" || self.function_name == "_deploy"
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

    fn has_using_directives(&self) -> bool {
        !self.using_target_types.is_empty()
    }

    fn using_target_allows_receiver(&self, receiver_type: &ValueType) -> bool {
        if self.using_target_types.is_empty() {
            return false;
        }

        let receiver_sig = normalize_solidity_like_type_signature(&value_type_signature(receiver_type));
        self.using_target_types.iter().any(|target| match target {
            None => true,
            Some(target_type) => target_type == &receiver_sig,
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

        let receiver_sig = normalize_solidity_like_type_signature(&value_type_signature(receiver_type));
        let list_scope_applies = self
            .using_function_list_scope_targets
            .iter()
            .any(|target| target.as_ref().is_none_or(|expected| expected == &receiver_sig));

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
            Some(target_type) => target_type == &receiver_sig,
        })
    }

    fn receiver_matches_function_overload(
        &self,
        function_name: &str,
        arg_count: usize,
        receiver_type: &ValueType,
    ) -> bool {
        let key = (function_name.to_string(), arg_count);
        let Some(expected_receiver_type) = self.function_first_param_types.get(&key) else {
            // Without type metadata, don't over-constrain existing behavior.
            return true;
        };

        is_implicitly_convertible(receiver_type, expected_receiver_type)
    }

    /// Returns the ordered parameter names for a function overload, if known.
    fn get_function_param_names(&self, name: &str, arg_count: usize) -> Option<&[String]> {
        self.function_param_names
            .get(&(name.to_string(), arg_count))
            .map(|v| v.as_slice())
    }

    /// Returns true if the named function returns void (no return values).
    fn is_void_function(&self, name: &str) -> bool {
        self.void_functions.contains(name)
    }

    /// Returns the renamed super-method name for `super.method()` resolution.
    fn super_method_name(&self, method_name: &str) -> Option<&str> {
        self.super_method_map.get(method_name).map(|s| s.as_str())
    }

    fn event_signature(&self, event_name: &str) -> Option<&[ManifestType]> {
        self.event_signature_map
            .get(event_name)
            .map(|sig| sig.as_slice())
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
        ValueType::Integer {
            signed: true,
            bits,
        } => format!("int{bits}"),
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
        ValueType::Mapping {
            key,
            value,
        } => format!(
            "mapping({}=>{})",
            value_type_signature(key),
            value_type_signature(value)
        ),
        ValueType::Struct { name, .. } => name.to_ascii_lowercase(),
        ValueType::Any => "any".to_string(),
    }
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
                name: actual_name,
                ..
            },
            ValueType::Struct {
                name: expected_name,
                ..
            },
        ) => actual_name.eq_ignore_ascii_case(expected_name),
        _ => false,
    }
}
