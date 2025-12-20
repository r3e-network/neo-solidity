fn build_defined_struct_types(
    structs: &[crate::solidity::StructMetadata],
    enums: &[EnumMetadata],
    contract_types: &[String],
) -> Vec<ValueType> {
    use crate::type_system::{
        EnumTypeMetadata, NeoType, StructFieldMetadata as TypeStructFieldMetadata,
        StructFieldType, StructTypeMetadata,
    };

    let struct_type_info: Vec<StructTypeMetadata> = structs
        .iter()
        .map(|struct_meta| StructTypeMetadata {
            name: struct_meta.name.clone(),
            fields: struct_meta
                .fields
                .iter()
                .map(|field| TypeStructFieldMetadata {
                    name: field.name.clone(),
                    ty: field.ty.clone(),
                })
                .collect(),
        })
        .collect();

    let enum_type_info: Vec<EnumTypeMetadata> = enums
        .iter()
        .map(|enum_meta| EnumTypeMetadata {
            name: enum_meta.name.clone(),
            variants: enum_meta.values.len(),
        })
        .collect();

    structs
        .iter()
        .map(|struct_meta| {
            let mut fields = Vec::new();
            for field in &struct_meta.fields {
                let field_type = NeoType::from_solidity(
                    &field.ty,
                    &struct_type_info,
                    &enum_type_info,
                    contract_types,
                )
                .unwrap_or(NeoType::Any);

                fields.push(StructFieldType {
                    name: field.name.clone(),
                    ty: Box::new(field_type),
                });
            }

            ValueType::from_neotype(&NeoType::Struct {
                name: struct_meta.name.clone(),
                fields,
            })
        })
        .collect()
}
