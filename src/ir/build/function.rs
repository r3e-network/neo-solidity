impl Function {
    #[allow(clippy::too_many_arguments)]
    fn from_metadata(
        metadata: &FunctionMetadata,
        state_variables: &[StateVariableMetadata],
        state_index_map: &HashMap<String, usize>,
        state_types: &[ValueType],
        defined_struct_types: &[ValueType],
        event_index_map: &HashMap<String, usize>,
        event_signature_map: &HashMap<String, Vec<ManifestType>>,
        enum_variant_map: &HashMap<String, HashMap<String, u64>>,
        contract_types: &HashSet<String>,
        selector_registry: &SelectorRegistry,
        function_names: &HashSet<String>,
        function_overloads: &HashMap<(String, usize), String>,
        function_first_param_types: &HashMap<(String, usize), ValueType>,
        using_target_types: &[Option<String>],
        using_function_list_targets: &HashMap<String, Vec<Option<String>>>,
        using_function_list_scope_targets: &[Option<String>],
        function_param_names: &HashMap<(String, usize), Vec<String>>,
        void_functions: &HashSet<String>,
        super_method_map: &HashMap<String, String>,
    ) -> Result<Self, Vec<IrDiagnostic>> {
        let parameters: Vec<ValueType> = metadata
            .parameters
            .iter()
            .map(ValueType::from_parameter)
            .collect();

        let returns: Vec<ValueType> = metadata
            .return_parameters
            .iter()
            .map(ValueType::from_parameter)
            .collect();

        let param_index_map = build_parameter_index_map(metadata);
        let mut ctx = LoweringContext::new(
            &metadata.name,
            metadata.state_mutability.is_safe(),
            param_index_map,
            &parameters,
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
        );

        let mut instructions: Vec<Instruction> = Vec::new();
        let mut return_slots: Vec<Option<usize>> = Vec::new();
        for (ret_param, value_type) in metadata.return_parameters.iter().zip(returns.iter()) {
            if let Some(name) = &ret_param.name {
                let slot = ctx.allocate_local(name.clone(), Some(value_type.clone()));
                if push_default_for_value_type(value_type, &mut ctx, &mut instructions) {
                    instructions.push(Instruction::StoreLocal(slot));
                }
                return_slots.push(Some(slot));
            } else {
                return_slots.push(None);
            }
        }
        ctx.set_return_info(return_slots.clone(), returns.clone());
        let mut returned = false;

        if let Some(body) = &metadata.body {
            returned = lower_statement(body, &mut ctx, &mut instructions);
        }

        if !returned {
            match metadata.kind {
                MetadataFunctionKind::Constructor => instructions.push(Instruction::ReturnVoid),
                _ if returns.is_empty() => instructions.push(Instruction::ReturnVoid),
                _ => {
                    if returns.len() == 1 {
                        if let Some(index) = return_slots.first().and_then(|slot| *slot) {
                            instructions.push(Instruction::LoadLocal(index));
                            instructions.push(Instruction::Return);
                        } else if let Some(ret_ty) = returns.first() {
                            push_default_for_value_type(ret_ty, &mut ctx, &mut instructions);
                            instructions.push(Instruction::Return);
                        } else {
                            instructions.push(Instruction::ReturnVoid);
                        }
                    } else {
                        let tmp_id = ctx.next_label();
                        let array_local = ctx.allocate_local(
                            format!("__return_tuple_{tmp_id}"),
                            Some(ValueType::Array(Box::new(ValueType::Any))),
                        );

                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                            BigInt::from(returns.len() as u64),
                        )));
                        instructions.push(Instruction::NewArray {
                            element_type: ValueType::Any,
                        });
                        instructions.push(Instruction::StoreLocal(array_local));

                        for (index, (slot, value_type)) in
                            return_slots.iter().zip(returns.iter()).enumerate()
                        {
                            instructions.push(Instruction::LoadLocal(array_local));
                            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                                BigInt::from(index as u64),
                            )));

                            if let Some(local_index) = slot {
                                instructions.push(Instruction::LoadLocal(*local_index));
                            } else {
                                push_default_for_value_type(value_type, &mut ctx, &mut instructions);
                            }

                            instructions.push(Instruction::ArraySet);
                        }

                        instructions.push(Instruction::LoadLocal(array_local));
                        instructions.push(Instruction::Return);
                    }
                }
            };
        }

        if !ctx.errors.is_empty() {
            return Err(ctx.errors);
        }

        let local_count = ctx.local_count;

        Ok(Self {
            name: metadata.neo_name.clone(),
            kind: match metadata.kind {
                MetadataFunctionKind::Constructor => FunctionKind::Constructor,
                MetadataFunctionKind::Regular => FunctionKind::Regular,
            },
            parameters,
            returns,
            basic_blocks: vec![BasicBlock { instructions }],
            local_count,
        })
    }
}
