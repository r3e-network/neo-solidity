use super::*;
use solang_parser::pt::Identifier;

#[test]
fn lower_emit_pushes_event_name_before_args() {
    let state_index_map = HashMap::new();
    let state_variables: Vec<StateVariableMetadata> = Vec::new();
    let event_index_map = HashMap::new();
    let event_signature_map: HashMap<String, Vec<ManifestType>> = HashMap::new();
    let enum_variant_map: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let contract_types = HashSet::new();
    let selector_registry = SelectorRegistry::default();
    let function_names = HashSet::new();
    let function_overloads: HashMap<(String, usize), String> = HashMap::new();
    let void_functions = HashSet::new();
    let state_types: Vec<ValueType> = Vec::new();
    let defined_struct_types: Vec<ValueType> = Vec::new();

    let mut ctx = LoweringContext::new(
        "test_emit",
        false,
        HashMap::new(),
        &[],
        &state_variables,
        &state_index_map,
        &state_types,
        &defined_struct_types,
        &event_index_map,
        &event_signature_map,
        &enum_variant_map,
        &contract_types,
        &selector_registry,
        &function_names,
        &function_overloads,
        &void_functions,
    );

    let expr = Expression::FunctionCall(
        Default::default(),
        Box::new(Expression::Variable(Identifier {
            loc: Default::default(),
            name: "MyEvent".to_string(),
        })),
        vec![
            Expression::NumberLiteral(Default::default(), "1".to_string(), "".to_string(), None),
            Expression::NumberLiteral(Default::default(), "2".to_string(), "".to_string(), None),
        ],
    );

    let mut instructions = Vec::new();
    lower_emit(&expr, &mut ctx, &mut instructions);

    assert!(
        ctx.errors.is_empty(),
        "lowering produced errors: {:?}",
        ctx.errors
    );
    assert!(
        matches!(
            instructions.first(),
            Some(Instruction::PushLiteral(LiteralValue::String(bytes))) if bytes == b"MyEvent"
        ),
        "expected event name literal to be pushed first, got {:?}",
        instructions.first()
    );
    assert!(
        instructions.iter().any(|instr| matches!(
            instr,
            Instruction::EmitEventByName { name, arg_count }
                if name == "MyEvent" && *arg_count == 2
        )),
        "expected EmitEventByName at the end of lowering"
    );
}
