fn find_named_struct_type(value_type: &ValueType, name: &str) -> Option<ValueType> {
    match value_type {
        ValueType::Struct {
            name: struct_name,
            fields,
        } => {
            if struct_name.eq_ignore_ascii_case(name) {
                return Some(value_type.clone());
            }

            fields
                .iter()
                .find_map(|field| find_named_struct_type(&field.ty, name))
        }
        ValueType::Array(inner) => find_named_struct_type(inner, name),
        ValueType::Mapping { key, value } => find_named_struct_type(key, name)
            .or_else(|| find_named_struct_type(value, name)),
        _ => None,
    }
}
