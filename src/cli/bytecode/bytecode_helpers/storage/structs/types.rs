#[derive(Clone, Copy)]
struct StructFieldAccess<'a> {
    field_keys: &'a [[u8; 32]],
    ty: &'a ValueType,
}

#[derive(Clone, Copy)]
struct StructArrayElementAccess<'a> {
    field_keys: &'a [[u8; 32]],
    element_type: &'a ValueType,
}
