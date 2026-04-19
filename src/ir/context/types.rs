struct LoopLabels {
    continue_label: usize,
    break_label: usize,
}

struct MappingAccess<'a> {
    state_index: usize,
    key_expressions: Vec<&'a Expression>,
    key_types: Vec<ValueType>,
    value_type: ValueType,
}

#[derive(Clone)]
struct StorageReference {
    state_index: usize,
    key_expressions: Vec<Expression>,
    key_types: Vec<ValueType>,
    value_type: ValueType,
    field_path: Vec<StorageReferenceField>,
    // Task #82: inner-mapping key chain applied AFTER `field_path`.
    // Populated when the reference walks through a struct-field mapping such as
    // `slots[k].balances[a]`.
    trailing_key_expressions: Vec<Expression>,
    trailing_key_types: Vec<ValueType>,
}

#[derive(Clone)]
struct StorageReferenceField {
    key: [u8; 32],
    ty: ValueType,
}

impl MappingAccess<'_> {
    fn to_storage_reference(&self) -> StorageReference {
        StorageReference {
            state_index: self.state_index,
            key_expressions: self
                .key_expressions
                .iter()
                .map(|expr| (*expr).clone())
                .collect(),
            key_types: self.key_types.clone(),
            value_type: self.value_type.clone(),
            field_path: Vec::new(),
            trailing_key_expressions: Vec::new(),
            trailing_key_types: Vec::new(),
        }
    }
}
