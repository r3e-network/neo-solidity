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
    local_index_map: HashMap<String, Vec<usize>>,
    local_types: HashMap<usize, ValueType>,
    scope_stack: Vec<Vec<String>>,
    storage_aliases: HashMap<String, StorageReference>,
    call_data_locals: HashMap<usize, String>,
    local_count: u16,
    label_counter: usize,
    loop_stack: Vec<LoopLabels>,
    errors: Vec<String>,
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
        let msg = message.into();
        self.errors
            .push(format!("function '{}': {}", self.function_name, msg));
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
