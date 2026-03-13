fn resolve_signature_string(expr: &Expression, ctx: &LoweringContext) -> Option<String> {
	match expr {
		Expression::Parenthesis(_, inner) => resolve_signature_string(inner, ctx),
		Expression::StringLiteral(parts) => {
			Some(String::from_utf8_lossy(&string_literal_bytes(parts)).to_string())
		}
		Expression::Variable(identifier) => {
			let state_index = ctx.state_index_map.get(&identifier.name).copied()?;
			let meta = ctx.state_metadata(state_index)?;
			if !meta.is_constant {
				return None;
			}
			let initializer = meta.initializer.as_ref()?;
			resolve_signature_string(initializer, ctx)
		}
		Expression::FunctionCall(_, func, args) => {
			if args.len() == 1 {
				match func.as_ref() {
					Expression::Type(_, _) => resolve_signature_string(&args[0], ctx),
					Expression::Variable(id) if id.name == "bytes" || id.name == "string" => {
						resolve_signature_string(&args[0], ctx)
					}
					_ => None,
				}
			} else {
				None
			}
		}
		_ => None,
	}
}

fn is_single_argument_bytes_or_type_wrapper(func: &Expression, args: &[Expression]) -> bool {
	if args.len() != 1 {
		return false;
	}

	match func {
		Expression::Type(_, _) => true,
		Expression::Variable(id) => id.name == "bytes" || id.name == "string",
		_ => false,
	}
}

fn is_empty_low_level_payload(expr: &Expression) -> bool {
	match expr {
		Expression::Parenthesis(_, inner) => is_empty_low_level_payload(inner),
		Expression::FunctionCall(_, func, args)
			if is_single_argument_bytes_or_type_wrapper(func.as_ref(), args.as_slice()) =>
		{
			is_empty_low_level_payload(&args[0])
		}
		Expression::StringLiteral(parts) => string_literal_bytes(parts).is_empty(),
		_ => match literal_from_expression(expr) {
			Some(LiteralValue::ByteArray(bytes)) => bytes.is_empty(),
			Some(LiteralValue::String(bytes)) => bytes.is_empty(),
			_ => false,
		},
	}
}

fn is_contract_type_reference(expr: &Expression, ctx: &LoweringContext) -> bool {
	match expr {
		Expression::Variable(type_id) => ctx.is_contract_type_name(&type_id.name),
		Expression::MemberAccess(_, namespace_expr, type_id) => {
			matches!(
				namespace_expr.as_ref(),
				Expression::Variable(namespace_id)
					if !ctx.param_index_map.contains_key(&namespace_id.name)
						&& ctx.resolve_local(&namespace_id.name).is_none()
						&& !ctx.state_index_map.contains_key(&namespace_id.name)
						&& !ctx.is_contract_type_name(&namespace_id.name)
			) && ctx.is_contract_type_name(&type_id.name)
		}
		_ => false,
	}
}

fn resolve_encode_call_method_name(expr: &Expression, ctx: &LoweringContext) -> Option<String> {
	if let Some(name) = resolve_selector_method_name(expr, ctx) {
		if !name.trim().is_empty() {
			return Some(name);
		}
	}

	match expr {
		Expression::Parenthesis(_, inner) => resolve_encode_call_method_name(inner, ctx),
		Expression::FunctionCall(_, func, args)
			if is_single_argument_bytes_or_type_wrapper(func.as_ref(), args.as_slice()) =>
		{
			resolve_encode_call_method_name(&args[0], ctx)
		}
		Expression::MemberAccess(_, inner, member) => {
			if member.name == "selector" {
				if let Expression::MemberAccess(_, type_expr, function_member) = inner.as_ref() {
					if is_contract_type_reference(type_expr.as_ref(), ctx) {
						let function_name = function_member.name.trim();
						if !function_name.is_empty() {
							return Some(function_name.to_string());
						}
					}
				}
				return None;
			}

			let name = member.name.trim();
			if name.is_empty() {
				return None;
			}

			if is_contract_type_reference(inner.as_ref(), ctx) {
				return Some(name.to_string());
			}

			// Compatibility fallback for instance-style references like
			// `token.transfer` in `abi.encodeCall(token.transfer, (...))`.
			if matches!(
				inner.as_ref(),
				Expression::Variable(_)
					| Expression::MemberAccess(_, _, _)
					| Expression::FunctionCall(_, _, _)
			) {
				return Some(name.to_string());
			}

			None
		}
		_ => None,
	}
}

fn extract_encode_call_arguments(expr: &Expression) -> Option<Vec<&Expression>> {
	match expr {
		Expression::Parenthesis(_, inner) => extract_encode_call_arguments(inner),
		Expression::FunctionCall(_, func, args)
			if is_single_argument_bytes_or_type_wrapper(func.as_ref(), args.as_slice()) =>
		{
			extract_encode_call_arguments(&args[0])
		}
		Expression::List(_, params) => {
			let mut arguments = Vec::with_capacity(params.len());
			for (_, param) in params {
				let param = param.as_ref()?;
				arguments.push(&param.ty);
			}
			Some(arguments)
		}
		_ => Some(vec![expr]),
	}
}

fn parse_low_level_call_data<'a>(
	expr: &'a Expression,
	ctx: &LoweringContext,
) -> Result<Option<(String, Vec<&'a Expression>)>, String> {
	match expr {
		Expression::Parenthesis(_, inner) => parse_low_level_call_data(inner, ctx),
		Expression::FunctionCall(_, func, args)
			if is_single_argument_bytes_or_type_wrapper(func.as_ref(), args.as_slice()) =>
		{
			parse_low_level_call_data(&args[0], ctx)
		}
		Expression::FunctionCall(_, func, args) => {
			let Expression::MemberAccess(_, inner, member) = func.as_ref() else {
				return Ok(None);
			};

			if !matches!(inner.as_ref(), Expression::Variable(id) if id.name == "abi") {
				return Ok(None);
			}

			match member.name.as_str() {
				"encodeWithSignature" => {
					let Some((first, rest)) = args.split_first() else {
						return Err("abi.encodeWithSignature requires a signature argument".to_string());
					};

					let signature = resolve_signature_string(first, ctx).ok_or_else(|| {
						"abi.encodeWithSignature signature must be a string literal or a constant string"
							.to_string()
					})?;

					let name = signature
						.split('(')
						.next()
						.unwrap_or(signature.as_str())
						.trim()
						.to_string();
					if name.is_empty() {
						return Err(
							"abi.encodeWithSignature signature must include a function name".to_string(),
						);
					}
					Ok(Some((name, rest.iter().collect())))
				}
					"encodeWithSelector" => {
						let Some((first, rest)) = args.split_first() else {
							return Err("abi.encodeWithSelector requires a selector argument".to_string());
						};

						let Some(name) = resolve_selector_method_name(first, ctx) else {
							// Compatibility fallback: unresolved runtime selectors cannot be
							// rewritten into Neo method-name calls. Let caller use the
							// generic low-level no-op tuple fallback path.
							return Ok(None);
						};
						if name.trim().is_empty() {
							return Err(
								"abi.encodeWithSelector selector resolves to an empty name".to_string(),
							);
						}
					Ok(Some((name, rest.iter().collect())))
				}
				"encodeCall" => {
					if args.len() != 2 {
						return Err(
							"abi.encodeCall requires function selector and tuple argument list"
								.to_string(),
						);
					}

					let method_name = resolve_encode_call_method_name(&args[0], ctx)
						.ok_or_else(|| "abi.encodeCall has an unsupported function reference".to_string())?;

					let call_args = extract_encode_call_arguments(&args[1]).ok_or_else(|| {
						"abi.encodeCall tuple argument list must contain positional expressions"
							.to_string()
					})?;

					Ok(Some((method_name, call_args)))
				}
				_ => Ok(None),
			}
		}
		_ => Ok(None),
	}
}

fn resolve_call_data_local(expr: &Expression, ctx: &LoweringContext) -> Option<(usize, String)> {
	match expr {
		Expression::Parenthesis(_, inner) => resolve_call_data_local(inner, ctx),
		Expression::FunctionCall(_, func, args)
			if is_single_argument_bytes_or_type_wrapper(func.as_ref(), args.as_slice()) =>
		{
			resolve_call_data_local(&args[0], ctx)
		}
		Expression::Variable(identifier) => {
			let slot = ctx.resolve_local(&identifier.name)?;
			let method = ctx.call_data_method_for_local(slot)?.to_string();
			Some((slot, method))
		}
		_ => None,
	}
}
fn try_lower_low_level_address_call(
	func: &Expression,
	args: &[Expression],
	ctx: &mut LoweringContext,
	instructions: &mut Vec<Instruction>,
) -> Option<bool> {
	// Limited low-level call support:
		// `address.call(abi.encodeWithSignature("foo(T1,T2)", a, b))`
		// `address.staticcall(abi.encodeWithSignature("foo(T1,T2)", a, b))`
		// `address.delegatecall(abi.encodeWithSignature("foo(T1,T2)", a, b))`
	// `bytes data = abi.encodeWithSignature(...); address.call(data)`
	// `bytes data = abi.encodeWithSelector(...); address.staticcall(data)`
	//
	// These are lowered into Neo `System.Contract.Call` invocations and return
	// `(success, Serialize(ret))`, mirroring Solidity's `(bool, bytes)` low-level calls.
	// We wrap only the contract call itself in NeoVM TRY/ENDTRY so that callee faults
	// become `success=false` with empty return data, while local evaluation errors still
	// abort execution.
	if let Expression::MemberAccess(_, inner, member) = func {
			let member_name = member.name.as_str();
			let is_staticcall = member_name == "staticcall";
			let is_delegatecall = member_name == "delegatecall";
			if (member_name == "call" || is_staticcall || is_delegatecall) && args.len() == 1 {
				if is_delegatecall {
				        ctx.record_warning_with_suggestion(
				                "address.delegatecall() has fundamentally different semantics on Neo N3: \
				                 EVM delegatecall executes callee code in the caller's storage context, \
				                 but Neo N3 has no equivalent mechanism. The call is compiled as a \
				                 regular System.Contract.Call which uses the callee's own storage.",
				                "use address.call() or address.staticcall() instead, or redesign \
				                 the contract to avoid delegatecall patterns (e.g. use explicit \
				                 library calls or the Neo contract update mechanism)",
				        );
				}				if ctx.is_safe && (member_name == "call" || is_delegatecall) {
					ctx.record_error(
						"address.call(...) / address.delegatecall(...) is not allowed in view/pure functions; use address.staticcall(...) or an external view/pure interface call",
					);
					return Some(false);
				}

			match parse_low_level_call_data(&args[0], ctx) {
				Ok(Some((method_name, encode_args))) => {
					let data_local = ctx.allocate_local("__call_data".to_string(), None);

					// Build tuple `(success, data)` as an array.
					let tuple_local = ctx.allocate_local("__call_tuple".to_string(), None);
					instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
						2u8,
					))));
					instructions.push(Instruction::NewArray {
						element_type: ValueType::Any,
					});
					instructions.push(Instruction::StoreLocal(tuple_local));

					if !lower_expression(inner.as_ref(), ctx, instructions) {
						return Some(false);
					}

					instructions.push(Instruction::PushLiteral(LiteralValue::String(
						method_name.as_bytes().to_vec(),
					)));

					let mut lowered = true;
					for call_arg in &encode_args {
						if !lower_expression(call_arg, ctx, instructions) {
							lowered = false;
						}
					}

					if !lowered {
						return Some(false);
					}

					instructions.push(Instruction::CallBuiltin {
						builtin: BuiltinCall::AbiEncode,
						arg_count: encode_args.len(),
					});

					let catch_label = ctx.next_label();
					let end_label = ctx.next_label();
					instructions.push(Instruction::Try {
						catch_target: catch_label,
					});

					if is_staticcall {
						// CallFlags.ReadOnly (ReadStates | AllowCall).
						instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
							BigInt::from(0x05u8),
						)));
						instructions.push(Instruction::CallBuiltin {
							builtin: BuiltinCall::ContractCallWithFlags,
							arg_count: 4,
						});
					} else {
						instructions.push(Instruction::CallBuiltin {
							builtin: BuiltinCall::ContractCall,
							arg_count: 3,
						});
					}

					instructions.push(Instruction::StoreLocal(data_local));

					// success at index 0
					instructions.push(Instruction::LoadLocal(tuple_local));
					instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
					instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
					instructions.push(Instruction::ArraySet);

					// data at index 1
					instructions.push(Instruction::LoadLocal(tuple_local));
					instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
					instructions.push(Instruction::LoadLocal(data_local));
					instructions.push(Instruction::ArraySet);

					instructions.push(Instruction::EndTry { target: end_label });

					instructions.push(Instruction::Label(catch_label));
					// NeoVM pushes the exception object onto the stack for the catch block.
					// Preserve it by serializing the exception value into the returned `bytes`.
					instructions.push(Instruction::CallBuiltin {
						builtin: BuiltinCall::NativeCall {
							contract: NativeContract::StdLib,
							method: "serialize".to_string(),
						},
						arg_count: 1,
					});
					instructions.push(Instruction::StoreLocal(data_local));

					// success=false
					instructions.push(Instruction::LoadLocal(tuple_local));
					instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
					instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
					instructions.push(Instruction::ArraySet);

					// data=serialized exception
					instructions.push(Instruction::LoadLocal(tuple_local));
					instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
					instructions.push(Instruction::LoadLocal(data_local));
					instructions.push(Instruction::ArraySet);

					instructions.push(Instruction::EndTry { target: end_label });
					instructions.push(Instruction::Label(end_label));
					instructions.push(Instruction::LoadLocal(tuple_local));
					return Some(true);
				}
				Ok(None) => {}
				Err(message) => {
					ctx.record_error(message);
					return Some(false);
				}
			}

			if let Some((call_data_slot, method_name)) = resolve_call_data_local(&args[0], ctx) {
				let data_local = ctx.allocate_local("__call_data".to_string(), None);

				let tuple_local = ctx.allocate_local("__call_tuple".to_string(), None);
				instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
					2u8,
				))));
				instructions.push(Instruction::NewArray {
					element_type: ValueType::Any,
				});
				instructions.push(Instruction::StoreLocal(tuple_local));

				if !lower_expression(inner.as_ref(), ctx, instructions) {
					return Some(false);
				}

				instructions.push(Instruction::PushLiteral(LiteralValue::String(
					method_name.as_bytes().to_vec(),
				)));
				instructions.push(Instruction::LoadLocal(call_data_slot));

				let catch_label = ctx.next_label();
				let end_label = ctx.next_label();
				instructions.push(Instruction::Try {
					catch_target: catch_label,
				});

				if is_staticcall {
					instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
						0x05u8,
					))));
					instructions.push(Instruction::CallBuiltin {
						builtin: BuiltinCall::ContractCallWithFlags,
						arg_count: 4,
					});
				} else {
					instructions.push(Instruction::CallBuiltin {
						builtin: BuiltinCall::ContractCall,
						arg_count: 3,
					});
				}

				instructions.push(Instruction::StoreLocal(data_local));

				instructions.push(Instruction::LoadLocal(tuple_local));
				instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
				instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
				instructions.push(Instruction::ArraySet);

				instructions.push(Instruction::LoadLocal(tuple_local));
				instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
				instructions.push(Instruction::LoadLocal(data_local));
				instructions.push(Instruction::ArraySet);

				instructions.push(Instruction::EndTry { target: end_label });

				instructions.push(Instruction::Label(catch_label));
				instructions.push(Instruction::CallBuiltin {
					builtin: BuiltinCall::NativeCall {
						contract: NativeContract::StdLib,
						method: "serialize".to_string(),
					},
					arg_count: 1,
				});
				instructions.push(Instruction::StoreLocal(data_local));

				instructions.push(Instruction::LoadLocal(tuple_local));
				instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
				instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
				instructions.push(Instruction::ArraySet);

				instructions.push(Instruction::LoadLocal(tuple_local));
				instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
				instructions.push(Instruction::LoadLocal(data_local));
				instructions.push(Instruction::ArraySet);

				instructions.push(Instruction::EndTry { target: end_label });
				instructions.push(Instruction::Label(end_label));
				instructions.push(Instruction::LoadLocal(tuple_local));
				return Some(true);
			}

			// Compatibility fallback for `addr.call("")` patterns (e.g. OpenZeppelin
			// `Address.sendValue`). Neo has no native ETH transfer semantics, so we
			// model this as a successful no-op low-level call with empty returndata.
			if is_empty_low_level_payload(&args[0]) {
				let tuple_local = ctx.allocate_local("__call_tuple".to_string(), None);
				instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
					2u8,
				))));
				instructions.push(Instruction::NewArray {
					element_type: ValueType::Any,
				});
				instructions.push(Instruction::StoreLocal(tuple_local));

				if !lower_expression(inner.as_ref(), ctx, instructions) {
					return Some(false);
				}
				instructions.push(Instruction::Drop(ValueType::Any));

				instructions.push(Instruction::LoadLocal(tuple_local));
				instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
				instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
				instructions.push(Instruction::ArraySet);

				instructions.push(Instruction::LoadLocal(tuple_local));
				instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
				instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(Vec::new())));
				instructions.push(Instruction::ArraySet);

				instructions.push(Instruction::LoadLocal(tuple_local));
				return Some(true);
			}

			// Compatibility fallback for unparsed low-level payloads:
			// preserve control-flow by returning `(true, bytes(""))` after evaluating
			// side-effecting sub-expressions.
			let tuple_local = ctx.allocate_local("__call_tuple".to_string(), None);
			instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
				2u8,
			))));
			instructions.push(Instruction::NewArray {
				element_type: ValueType::Any,
			});
			instructions.push(Instruction::StoreLocal(tuple_local));

			if !lower_expression(inner.as_ref(), ctx, instructions) {
				return Some(false);
			}
			instructions.push(Instruction::Drop(ValueType::Any));

			if !lower_expression(&args[0], ctx, instructions) {
				return Some(false);
			}
			instructions.push(Instruction::Drop(ValueType::Any));

			instructions.push(Instruction::LoadLocal(tuple_local));
			instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
			instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
			instructions.push(Instruction::ArraySet);

			instructions.push(Instruction::LoadLocal(tuple_local));
			instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::one())));
			instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(Vec::new())));
			instructions.push(Instruction::ArraySet);

			instructions.push(Instruction::LoadLocal(tuple_local));
			return Some(true);
		}
	}

	None
}
