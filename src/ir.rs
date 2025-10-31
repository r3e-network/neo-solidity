//! Canonical intermediate representation for Neo Solidity.

use crate::solidity::{
    ContractMetadata, FunctionKind as MetadataFunctionKind, FunctionMetadata, ParameterMetadata,
    StateVariableMetadata,
};
use crate::storage_key::compute_state_slot;
use hex::decode as hex_decode;
use num_bigint::BigInt;
use solang_parser::pt::{
    Expression, HexLiteral as PtHexLiteral, Statement, StringLiteral as PtStringLiteral,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Module {
    pub functions: Vec<Function>,
    pub state_variables: Vec<StateVariable>,
    pub events: Vec<Event>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub kind: FunctionKind,
    pub parameters: Vec<ValueType>,
    pub returns: Vec<ValueType>,
    pub basic_blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionKind {
    Constructor,
    Regular,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateVariable {
    pub name: Option<String>,
    pub ty: ValueType,
    pub is_constant: bool,
    pub is_immutable: bool,
    pub storage_key: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicBlock {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Drop(ValueType),
    LoadParameter(usize),
    PushLiteral(LiteralValue),
    Return,
    ReturnVoid,
    ReturnDefault(ValueType),
    BinaryOp(BinaryOperator),
    LoadState(usize),
    StoreState(usize),
    LoadMappingElement {
        state_index: usize,
        key_types: Vec<ValueType>,
    },
    StoreMappingElement {
        state_index: usize,
        key_types: Vec<ValueType>,
    },
    EmitEvent {
        event_index: usize,
    },
    Jump {
        target: usize,
    },
    JumpIf {
        target: usize,
    },
    Label(usize),
    Abort,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralValue {
    Integer(BigInt),
    Boolean(bool),
    String(Vec<u8>),
    ByteArray(Vec<u8>),
    Address(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    Integer {
        signed: bool,
        bits: u16,
    },
    Boolean,
    String,
    Address,
    ByteArray {
        fixed_len: Option<u16>,
    },
    Array(Box<ValueType>),
    Mapping {
        key: Box<ValueType>,
        value: Box<ValueType>,
    },
    Any,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

impl Module {
    pub fn from_contract(metadata: &ContractMetadata) -> Result<Self, Vec<String>> {
        let state_variables: Vec<StateVariable> = metadata
            .state_variables
            .iter()
            .map(StateVariable::from_metadata)
            .collect();

        let mut state_index_map = HashMap::new();
        for (index, state) in state_variables.iter().enumerate() {
            if let Some(name) = &state.name {
                state_index_map.insert(name.clone(), index);
            }
        }

        let events: Vec<Event> = metadata.events.iter().map(Event::from_metadata).collect();

        let mut event_index_map = HashMap::new();
        for (index, event) in events.iter().enumerate() {
            event_index_map.insert(event.name.clone(), index);
        }

        let state_types: Vec<ValueType> = state_variables
            .iter()
            .map(|state| state.ty.clone())
            .collect();

        let mut functions = Vec::new();
        let mut errors = Vec::new();
        for function in &metadata.methods {
            match Function::from_metadata(
                function,
                &state_index_map,
                &state_types,
                &event_index_map,
            ) {
                Ok(func) => functions.push(func),
                Err(mut function_errors) => errors.append(&mut function_errors),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            functions,
            state_variables,
            events,
        })
    }
}

impl Function {
    fn from_metadata(
        metadata: &FunctionMetadata,
        state_index_map: &HashMap<String, usize>,
        state_types: &[ValueType],
        event_index_map: &HashMap<String, usize>,
    ) -> Result<Self, Vec<String>> {
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
            param_index_map,
            state_index_map,
            state_types,
            event_index_map,
        );

        let mut instructions: Vec<Instruction> = Vec::new();
        let mut returned = false;

        if let Some(body) = &metadata.body {
            returned = lower_statement(body, &mut ctx, &mut instructions);
        }

        let LoweringContext { errors, .. } = ctx;
        if !errors.is_empty() {
            return Err(errors);
        }

        if !returned {
            if matches!(metadata.kind, MetadataFunctionKind::Constructor) {
                instructions.push(Instruction::ReturnVoid);
            } else if let Some(ret_ty) = returns.first() {
                instructions.push(Instruction::ReturnDefault(ret_ty.clone()));
            } else {
                instructions.push(Instruction::ReturnVoid);
            }
        }

        Ok(Self {
            name: metadata.name.clone(),
            kind: match metadata.kind {
                MetadataFunctionKind::Constructor => FunctionKind::Constructor,
                MetadataFunctionKind::Regular => FunctionKind::Regular,
            },
            parameters,
            returns,
            basic_blocks: vec![BasicBlock { instructions }],
        })
    }
}

impl StateVariable {
    fn from_metadata(symbol: &StateVariableMetadata) -> Self {
        let storage_key = symbol
            .name
            .as_deref()
            .map(|name| compute_state_slot(name).to_vec())
            .unwrap_or_default();
        Self {
            name: symbol.name.clone(),
            ty: ValueType::from_neotype(
                &symbol
                    .neo_type
                    .clone()
                    .unwrap_or(crate::type_system::NeoType::Any),
            ),
            is_constant: symbol.is_constant,
            is_immutable: symbol.is_immutable,
            storage_key,
        }
    }
}

impl Event {
    fn from_metadata(event: &crate::solidity::EventMetadata) -> Self {
        Self {
            name: event.name.clone(),
        }
    }
}

impl ValueType {
    fn from_neotype(neotype: &crate::type_system::NeoType) -> Self {
        match neotype {
            crate::type_system::NeoType::Integer { signed, bits } => ValueType::Integer {
                signed: *signed,
                bits: *bits,
            },
            crate::type_system::NeoType::Boolean => ValueType::Boolean,
            crate::type_system::NeoType::String => ValueType::String,
            crate::type_system::NeoType::Address => ValueType::Address,
            crate::type_system::NeoType::ByteArray { fixed_len } => ValueType::ByteArray {
                fixed_len: *fixed_len,
            },
            crate::type_system::NeoType::Array(element) => {
                ValueType::Array(Box::new(ValueType::from_neotype(element)))
            }
            crate::type_system::NeoType::Mapping { key, value } => ValueType::Mapping {
                key: Box::new(ValueType::from_neotype(key.as_ref())),
                value: Box::new(ValueType::from_neotype(value.as_ref())),
            },
            crate::type_system::NeoType::Any => ValueType::Any,
        }
    }

    fn from_parameter(param: &ParameterMetadata) -> Self {
        match &param.neo_type {
            Some(neo_type) => ValueType::from_neotype(neo_type),
            None => ValueType::Any,
        }
    }
}

fn literal_from_expression(expr: &Expression) -> Option<LiteralValue> {
    match expr {
        Expression::BoolLiteral(_, value) => Some(LiteralValue::Boolean(*value)),
        Expression::NumberLiteral(_, integer, fraction, _) => {
            if fraction.trim().is_empty() {
                parse_decimal_bigint(integer).map(LiteralValue::Integer)
            } else {
                None
            }
        }
        Expression::HexNumberLiteral(_, value, _) => {
            parse_hex_bigint(value).map(LiteralValue::Integer)
        }
        Expression::StringLiteral(parts) => Some(LiteralValue::String(string_literal_bytes(parts))),
        Expression::HexLiteral(parts) => decode_hex_segments(parts).map(LiteralValue::ByteArray),
        Expression::AddressLiteral(_, value) => decode_hex_bytes(value).map(LiteralValue::Address),
        Expression::Parenthesis(_, inner) => literal_from_expression(inner),
        _ => None,
    }
}

fn string_literal_bytes(parts: &[PtStringLiteral]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for part in parts {
        bytes.extend_from_slice(part.string.as_bytes());
    }
    bytes
}

fn decode_hex_segments(parts: &[PtHexLiteral]) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    for part in parts {
        let segment = part.hex.trim();
        let inner = segment
            .strip_prefix("hex")
            .and_then(|s| s.trim().strip_prefix('\"'))
            .and_then(|s| s.strip_suffix('\"'))
            .unwrap_or(segment);
        let cleaned: String = inner.chars().filter(|c| !c.is_whitespace()).collect();
        bytes.extend(hex_decode(&cleaned).ok()?);
    }
    Some(bytes)
}

fn decode_hex_bytes(value: &str) -> Option<Vec<u8>> {
    let cleaned = value.trim();
    if let Some(inner) = cleaned.strip_prefix("0x") {
        hex_decode(inner).ok()
    } else {
        hex_decode(cleaned).ok()
    }
}

fn parse_decimal_bigint(value: &str) -> Option<BigInt> {
    let sanitized: String = value.chars().filter(|c| *c != '_').collect();
    BigInt::parse_bytes(sanitized.as_bytes(), 10)
}

fn parse_hex_bigint(value: &str) -> Option<BigInt> {
    let sanitized = value.trim_start_matches("0x");
    BigInt::parse_bytes(sanitized.as_bytes(), 16)
}

fn build_parameter_index_map(metadata: &FunctionMetadata) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for (index, param) in metadata.parameters.iter().enumerate() {
        if let Some(name) = &param.name {
            map.insert(name.clone(), index);
        }
    }
    map
}

struct LoweringContext<'a> {
    function_name: String,
    param_index_map: HashMap<String, usize>,
    state_index_map: &'a HashMap<String, usize>,
    state_types: &'a [ValueType],
    event_index_map: &'a HashMap<String, usize>,
    label_counter: usize,
    loop_stack: Vec<LoopLabels>,
    errors: Vec<String>,
}

impl<'a> LoweringContext<'a> {
    fn new(
        function_name: &str,
        param_index_map: HashMap<String, usize>,
        state_index_map: &'a HashMap<String, usize>,
        state_types: &'a [ValueType],
        event_index_map: &'a HashMap<String, usize>,
    ) -> Self {
        Self {
            function_name: function_name.to_string(),
            param_index_map,
            state_index_map,
            state_types,
            event_index_map,
            label_counter: 0,
            loop_stack: Vec::new(),
            errors: Vec::new(),
        }
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

    fn state_type(&self, index: usize) -> Option<&ValueType> {
        self.state_types.get(index)
    }
}

struct LoopLabels {
    continue_label: usize,
    break_label: usize,
}

struct MappingAccess<'a> {
    state_index: usize,
    key_expressions: Vec<&'a Expression>,
    key_types: Vec<ValueType>,
}

fn resolve_mapping_access<'a>(
    expression: &'a Expression,
    ctx: &LoweringContext,
) -> Option<MappingAccess<'a>> {
    let mut keys: Vec<&'a Expression> = Vec::new();
    let mut current = expression;

    loop {
        match current {
            Expression::ArraySubscript(_, inner, maybe_index) => {
                let index_expr = maybe_index.as_ref()?.as_ref();
                keys.insert(0, index_expr);
                current = inner;
            }
            Expression::Variable(identifier) => {
                let state_index = *ctx.state_index_map.get(&identifier.name)?;
                let mut current_type = ctx.state_type(state_index)?.clone();
                let mut key_types = Vec::with_capacity(keys.len());

                for _key_expr in &keys {
                    match current_type {
                        ValueType::Mapping { ref key, ref value } => {
                            key_types.push((**key).clone());
                            current_type = (**value).clone();
                        }
                        _ => return None,
                    }
                }

                return Some(MappingAccess {
                    state_index,
                    key_expressions: keys,
                    key_types,
                });
            }
            _ => return None,
        }
    }
}

fn lower_statement(
    statement: &Statement,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    match statement {
        Statement::Block { statements, .. } => {
            for stmt in statements {
                if lower_statement(stmt, ctx, instructions) {
                    return true;
                }
            }
            false
        }
        Statement::If(_, condition, then_stmt, else_stmt) => {
            let else_label = ctx.next_label();
            let end_label = if else_stmt.is_some() {
                ctx.next_label()
            } else {
                else_label
            };

            lower_expression(condition, ctx, instructions);
            instructions.push(Instruction::JumpIf { target: else_label });

            let then_returns = lower_statement(then_stmt, ctx, instructions);

            if let Some(else_stmt) = else_stmt {
                instructions.push(Instruction::Jump { target: end_label });
                instructions.push(Instruction::Label(else_label));
                let else_returns = lower_statement(else_stmt, ctx, instructions);
                instructions.push(Instruction::Label(end_label));
                then_returns && else_returns
            } else {
                instructions.push(Instruction::Label(else_label));
                false
            }
        }
        Statement::While(_, condition, body) => {
            let start_label = ctx.next_label();
            let end_label = ctx.next_label();

            instructions.push(Instruction::Label(start_label));
            lower_expression(condition, ctx, instructions);
            instructions.push(Instruction::JumpIf { target: end_label });
            ctx.push_loop(start_label, end_label);
            lower_statement(body, ctx, instructions);
            ctx.pop_loop();
            instructions.push(Instruction::Jump {
                target: start_label,
            });
            instructions.push(Instruction::Label(end_label));
            false
        }
        Statement::DoWhile(_, body, condition) => {
            let start_label = ctx.next_label();
            let condition_label = ctx.next_label();
            let end_label = ctx.next_label();

            instructions.push(Instruction::Label(start_label));
            ctx.push_loop(condition_label, end_label);
            lower_statement(body, ctx, instructions);
            ctx.pop_loop();
            instructions.push(Instruction::Label(condition_label));
            lower_expression(condition, ctx, instructions);
            instructions.push(Instruction::JumpIf { target: end_label });
            instructions.push(Instruction::Jump {
                target: start_label,
            });
            instructions.push(Instruction::Label(end_label));
            false
        }
        Statement::For(_, init, condition, post, body) => {
            if let Some(init_stmt) = init.as_deref() {
                lower_statement(init_stmt, ctx, instructions);
            }

            let condition_label = ctx.next_label();
            let post_label = ctx.next_label();
            let end_label = ctx.next_label();

            instructions.push(Instruction::Label(condition_label));

            if let Some(cond_expr) = condition.as_deref() {
                lower_expression(cond_expr, ctx, instructions);
                instructions.push(Instruction::JumpIf { target: end_label });
            }

            if let Some(body_stmt) = body.as_deref() {
                ctx.push_loop(post_label, end_label);
                lower_statement(body_stmt, ctx, instructions);
                ctx.pop_loop();
            }

            instructions.push(Instruction::Label(post_label));
            if let Some(post_expr) = post.as_deref() {
                if lower_expression(post_expr, ctx, instructions) {
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
            }

            instructions.push(Instruction::Jump {
                target: condition_label,
            });
            instructions.push(Instruction::Label(end_label));
            false
        }
        Statement::Expression(_, expr) => {
            if let Expression::Assign(_, lhs, rhs) = expr {
                lower_assignment(lhs, rhs, ctx, instructions);
            } else if lower_expression(expr, ctx, instructions) {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
            false
        }
        Statement::Emit(_, call) => {
            lower_emit(call, ctx, instructions);
            false
        }
        Statement::Break(_) => {
            if let Some(label) = ctx.break_target() {
                instructions.push(Instruction::Jump { target: label });
            }
            false
        }
        Statement::Continue(_) => {
            if let Some(label) = ctx.continue_target() {
                instructions.push(Instruction::Jump { target: label });
            }
            false
        }
        Statement::Return(_, expr) => {
            if let Some(expression) = expr {
                if lower_expression(expression, ctx, instructions) {
                    instructions.push(Instruction::Return);
                    return true;
                }
            } else {
                instructions.push(Instruction::ReturnVoid);
                return true;
            }
            false
        }
        Statement::Revert(_, Some(_), _)
        | Statement::RevertNamedArgs(_, Some(_), _)
        | Statement::Revert(_, None, _)
        | Statement::RevertNamedArgs(_, None, _) => {
            instructions.push(Instruction::Abort);
            true
        }
        _ => {
            ctx.record_error(format!("unsupported statement '{:?}'", statement));
            false
        }
    }
}

fn lower_assignment(
    lhs: &Expression,
    rhs: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    if let Some(mapping) = resolve_mapping_access(lhs, ctx) {
        let mut success = lower_expression(rhs, ctx, instructions);

        for key_expr in mapping.key_expressions.iter().rev() {
            if !lower_expression(key_expr, ctx, instructions) {
                success = false;
            }
        }

        if success {
            instructions.push(Instruction::StoreMappingElement {
                state_index: mapping.state_index,
                key_types: mapping.key_types.clone(),
            });
        }

        return;
    }

    if let Expression::Variable(identifier) = lhs {
        if let Some(index) = ctx.state_index_map.get(&identifier.name) {
            if lower_expression(rhs, ctx, instructions) {
                instructions.push(Instruction::StoreState(*index));
            }
            return;
        }
    }

    ctx.record_error(format!("assignment target '{:?}' is not supported", lhs));
}

fn lower_emit(expr: &Expression, ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>) {
    if let Expression::FunctionCall(_, func, _args) = expr {
        if let Expression::Variable(identifier) = func.as_ref() {
            if let Some(index) = ctx.event_index_map.get(&identifier.name) {
                instructions.push(Instruction::EmitEvent {
                    event_index: *index,
                });
                return;
            }
        }
    }

    ctx.record_error("event emission is only supported for direct event invocations");
}

fn lower_expression(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    match expr {
        Expression::Add(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Add)
        }
        Expression::Subtract(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Sub)
        }
        Expression::Multiply(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Mul)
        }
        Expression::Divide(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Div)
        }
        Expression::Modulo(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Mod)
        }
        Expression::Less(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Lt)
        }
        Expression::LessEqual(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Le)
        }
        Expression::More(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Gt)
        }
        Expression::MoreEqual(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Ge)
        }
        Expression::Equal(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Eq)
        }
        Expression::NotEqual(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Ne)
        }
        Expression::Variable(identifier) => {
            if let Some(index) = ctx.param_index_map.get(&identifier.name) {
                instructions.push(Instruction::LoadParameter(*index));
                true
            } else if let Some(index) = ctx.state_index_map.get(&identifier.name) {
                instructions.push(Instruction::LoadState(*index));
                true
            } else {
                ctx.record_error(format!(
                    "identifier '{}' cannot be resolved in this context",
                    identifier.name
                ));
                false
            }
        }
        Expression::ArraySubscript(_, _, None) => {
            ctx.record_error("array slicing is not supported");
            false
        }
        Expression::ArraySubscript(_, _, Some(_)) => {
            if let Some(mapping) = resolve_mapping_access(expr, ctx) {
                let mut success = true;
                for key_expr in mapping.key_expressions.iter().rev() {
                    if !lower_expression(key_expr, ctx, instructions) {
                        success = false;
                    }
                }

                if success {
                    instructions.push(Instruction::LoadMappingElement {
                        state_index: mapping.state_index,
                        key_types: mapping.key_types.clone(),
                    });
                }

                success
            } else {
                ctx.record_error("array subscript expression is not supported");
                false
            }
        }
        Expression::Parenthesis(_, inner) => lower_expression(inner, ctx, instructions),
        _ => {
            if let Some(literal) = literal_from_expression(expr) {
                instructions.push(Instruction::PushLiteral(literal));
                true
            } else {
                ctx.record_error(format!("unsupported expression '{:?}'", expr));
                false
            }
        }
    }
}

fn lower_binary_expr(
    left: &Expression,
    right: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
) -> bool {
    if lower_expression(left, ctx, instructions) && lower_expression(right, ctx, instructions) {
        instructions.push(Instruction::BinaryOp(operator));
        true
    } else {
        false
    }
}
