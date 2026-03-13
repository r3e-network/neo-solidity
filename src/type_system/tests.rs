use super::*;

fn struct_meta(name: &str, fields: &[(&str, &str)]) -> StructTypeMetadata {
    StructTypeMetadata {
        name: name.to_string(),
        fields: fields
            .iter()
            .map(|&(fname, fty)| StructFieldMetadata {
                name: fname.to_string(),
                ty: fty.to_string(),
            })
            .collect(),
    }
}

#[test]
fn parses_simple_struct_type() {
    let structs = vec![struct_meta("Point", &[("x", "uint256"), ("y", "uint256")])];
    let contract_types: Vec<String> = Vec::new();

    let ty = NeoType::from_solidity("Point", &structs, &[], &contract_types).expect("struct type");
    match ty {
        NeoType::Struct { name, fields } => {
            assert_eq!(name, "Point");
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name, "x");
            assert!(matches!(
                *fields[0].ty,
                NeoType::Integer {
                    signed: false,
                    bits: 256
                }
            ));
        }
        other => panic!("expected struct type, got {other:?}"),
    }
}

#[test]
fn strips_data_location_suffixes() {
    let structs = vec![struct_meta("Point", &[("x", "uint256")])];
    let contract_types: Vec<String> = Vec::new();
    let ty = NeoType::from_solidity("Point storage", &structs, &[], &contract_types)
        .expect("struct storage type");
    assert!(matches!(ty, NeoType::Struct { .. }));

    let mapping = NeoType::from_solidity(
        "mapping(address => Point memory)",
        &structs,
        &[],
        &contract_types,
    )
    .expect("mapping with struct value");
    match mapping {
        NeoType::Mapping { value, .. } => {
            assert!(matches!(*value, NeoType::Struct { .. }));
        }
        other => panic!("expected mapping type, got {other:?}"),
    }
}

#[test]
fn parses_arrays_before_scalar_prefixes() {
    let structs = Vec::new();
    let contract_types: Vec<String> = Vec::new();

    let dyn_array = NeoType::from_solidity("uint256[]", &structs, &[], &contract_types)
        .expect("dynamic array type");
    match dyn_array {
        NeoType::Array(element) => {
            assert!(matches!(
                *element,
                NeoType::Integer {
                    signed: false,
                    bits: 256
                }
            ));
        }
        other => panic!("expected array type, got {other:?}"),
    }

    let fixed_array = NeoType::from_solidity("bool[3]", &structs, &[], &contract_types)
        .expect("fixed array type");
    match fixed_array {
        NeoType::Array(element) => assert!(matches!(*element, NeoType::Boolean)),
        other => panic!("expected array type, got {other:?}"),
    }
}
