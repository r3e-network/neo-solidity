//! Canonical intermediate representation for Neo Solidity.

use crate::solidity::{
    ContractMetadata, FunctionKind as MetadataFunctionKind, FunctionMetadata, ParameterMetadata,
    StateVariableMetadata,
};
use crate::storage_key::compute_state_slot;
use hex::decode as hex_decode;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive, Zero};
use sha3::{Digest, Keccak256};
use solang_parser::pt::{
    Expression, HexLiteral as PtHexLiteral, Statement, StorageLocation as PtStorageLocation,
    StringLiteral as PtStringLiteral, Type as PtType,
};
use std::collections::{HashMap, HashSet};

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
    pub local_count: u16,
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
pub enum RuntimeValue {
    MsgSender,
    BlockTimestamp,
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
    LoadStorageDynamic,
    LoadLocal(usize),
    StoreLocal(usize),
    LoadMappingElement {
        state_index: usize,
        key_types: Vec<ValueType>,
    },
    StoreMappingElement {
        state_index: usize,
        key_types: Vec<ValueType>,
    },
    LoadStructField {
        state_index: usize,
        key_types: Vec<ValueType>,
        field_key: [u8; 32],
        field_type: ValueType,
    },
    StoreStructField {
        state_index: usize,
        key_types: Vec<ValueType>,
        field_key: [u8; 32],
        field_type: ValueType,
    },
    LoadRuntimeValue(RuntimeValue),
    GetSize,
    CallFunction {
        name: String,
        arg_count: usize,
    },
    CallBuiltin {
        builtin: BuiltinCall,
        arg_count: usize,
    },
    EmitEvent {
        event_index: usize,
        arg_count: usize,
    },
    EmitEventByName {
        name: String,
        arg_count: usize,
    },
    NewArray {
        element_type: ValueType,
    },
    ArrayGet,
    ArraySet,
    BitwiseNot,
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
    Struct {
        name: String,
        fields: Vec<StructField>,
    },
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub name: String,
    pub ty: ValueType,
    pub key: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinCall {
    RuntimeNotify,
    RuntimeCheckWitness,
    AbiEncode,
    AbiEncodePacked,
    AbiEncodeWithSignature,
    AbiDecode,
    Keccak256,
    StorageFind,
    TypeOf,
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

        let function_names: HashSet<String> = metadata
            .methods
            .iter()
            .map(|function| function.name.clone())
            .collect();

        let mut functions = Vec::new();
        let mut errors = Vec::new();
        for function in &metadata.methods {
            match Function::from_metadata(
                function,
                &state_index_map,
                &state_types,
                &event_index_map,
                &function_names,
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
        function_names: &HashSet<String>,
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
            &parameters,
            state_index_map,
            state_types,
            event_index_map,
            function_names,
        );

        let mut instructions: Vec<Instruction> = Vec::new();
        let mut return_slots: Vec<Option<usize>> = Vec::new();
        for (ret_param, value_type) in metadata.return_parameters.iter().zip(returns.iter()) {
            if let Some(name) = &ret_param.name {
                let slot = ctx.allocate_local(name.clone(), Some(value_type.clone()));
                if push_default_for_value_type(value_type, &mut instructions) {
                    instructions.push(Instruction::StoreLocal(slot));
                }
                return_slots.push(Some(slot));
            } else {
                return_slots.push(None);
            }
        }
        let mut returned = false;

        if let Some(body) = &metadata.body {
            returned = lower_statement(body, &mut ctx, &mut instructions);
        }

        let local_count = ctx.local_count;
        if !ctx.errors.is_empty() {
            return Err(ctx.errors);
        }

        if !returned {
            match metadata.kind {
                MetadataFunctionKind::Constructor => instructions.push(Instruction::ReturnVoid),
                _ if returns.is_empty() => instructions.push(Instruction::ReturnVoid),
                _ => {
                    if return_slots.is_empty() {
                        if let Some(ret_ty) = returns.first() {
                            instructions.push(Instruction::ReturnDefault(ret_ty.clone()));
                        } else {
                            instructions.push(Instruction::ReturnVoid);
                        }
                    } else {
                        for (slot, value_type) in return_slots.iter().zip(returns.iter()) {
                            if let Some(index) = slot {
                                instructions.push(Instruction::LoadLocal(*index));
                            } else {
                                push_default_for_value_type(value_type, &mut instructions);
                            }
                        }
                        instructions.push(Instruction::Return);
                    }
                }
            };
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
            local_count,
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
            crate::type_system::NeoType::Struct { name, fields } => ValueType::Struct {
                name: name.clone(),
                fields: fields
                    .iter()
                    .map(|field| StructField {
                        name: field.name.clone(),
                        ty: ValueType::from_neotype(field.ty.as_ref()),
                        key: compute_state_slot(&format!("{}::{}", name, field.name)),
                    })
                    .collect(),
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
            let int_part = parse_decimal_bigint(integer)?;
            if fraction.trim().is_empty() {
                Some(LiteralValue::Integer(int_part))
            } else if let Ok(exp) = fraction.trim().parse::<u32>() {
                Some(LiteralValue::Integer(int_part * pow10(exp)))
            } else {
                None
            }
        }
        Expression::HexNumberLiteral(_, value, _) => {
            parse_hex_bigint(value).map(LiteralValue::Integer)
        }
        Expression::RationalNumberLiteral(_, numer, denom, _, _) => {
            let num = parse_decimal_bigint(numer)?;
            let den = parse_decimal_bigint(denom)?;
            if den.is_zero() {
                return None;
            }
            Some(LiteralValue::Integer(num / den))
        }
        Expression::StringLiteral(parts) => Some(LiteralValue::String(string_literal_bytes(parts))),
        Expression::HexLiteral(parts) => decode_hex_segments(parts).map(LiteralValue::ByteArray),
        Expression::AddressLiteral(_, value) => decode_hex_bytes(value).map(LiteralValue::Address),
        Expression::Parenthesis(_, inner) => literal_from_expression(inner),
        _ => None,
    }
}

fn infer_literal_array_element_type(elements: &[Expression]) -> ValueType {
    if elements.is_empty() {
        return ValueType::Any;
    }

    let mut ty = ValueType::Any;
    for expr in elements {
        match literal_from_expression(expr) {
            Some(LiteralValue::Boolean(_)) => ty = ValueType::Boolean,
            Some(LiteralValue::Integer(_)) => {
                ty = ValueType::Integer {
                    signed: false,
                    bits: 256,
                }
            }
            Some(LiteralValue::String(_)) => ty = ValueType::String,
            Some(LiteralValue::ByteArray(_)) => ty = ValueType::ByteArray { fixed_len: None },
            Some(LiteralValue::Address(_)) => ty = ValueType::Address,
            None => {
                ty = ValueType::Any;
                break;
            }
        }
    }
    ty
}

fn infer_type_from_expression(expr: &Expression, ctx: &LoweringContext) -> Option<ValueType> {
    match expr {
        Expression::Type(_, ty) => value_type_from_ptype(ty),
        Expression::ArrayLiteral(_, elements) => Some(ValueType::Array(Box::new(
            infer_literal_array_element_type(elements),
        ))),
        Expression::ArraySubscript(_, array, _) => {
            if let Some(ValueType::Array(inner)) = infer_type_from_expression(array, ctx) {
                Some(*inner.clone())
            } else {
                None
            }
        }
        Expression::Variable(identifier) => ctx.variable_type(&identifier.name),
        Expression::MemberAccess(_, inner, _) => infer_type_from_expression(inner, ctx),
        _ => None,
    }
}

fn value_type_from_ptype(ty: &PtType) -> Option<ValueType> {
    match ty {
        PtType::Bool => Some(ValueType::Boolean),
        PtType::Address | PtType::AddressPayable => Some(ValueType::Address),
        PtType::Uint(bits) => Some(ValueType::Integer {
            signed: false,
            bits: *bits,
        }),
        PtType::Int(bits) => Some(ValueType::Integer {
            signed: true,
            bits: *bits,
        }),
        PtType::String => Some(ValueType::String),
        PtType::Bytes(len) => Some(ValueType::ByteArray {
            fixed_len: Some(*len as u16),
        }),
        PtType::DynamicBytes => Some(ValueType::ByteArray { fixed_len: None }),
        _ => None,
    }
}

fn infer_array_element_type(expr: &Expression, ctx: &LoweringContext) -> Option<ValueType> {
    match infer_type_from_expression(expr, ctx) {
        Some(ValueType::Array(inner)) => Some(*inner.clone()),
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

fn pow10(exp: u32) -> BigInt {
    let ten = BigInt::from(10u8);
    ten.pow(exp)
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
    param_types: &'a [ValueType],
    state_index_map: &'a HashMap<String, usize>,
    state_types: &'a [ValueType],
    event_index_map: &'a HashMap<String, usize>,
    function_names: &'a HashSet<String>,
    local_index_map: HashMap<String, Vec<usize>>,
    local_types: HashMap<usize, ValueType>,
    scope_stack: Vec<Vec<String>>,
    storage_aliases: HashMap<String, StorageReference>,
    local_count: u16,
    label_counter: usize,
    loop_stack: Vec<LoopLabels>,
    errors: Vec<String>,
}

impl<'a> LoweringContext<'a> {
    fn new(
        function_name: &str,
        param_index_map: HashMap<String, usize>,
        param_types: &'a [ValueType],
        state_index_map: &'a HashMap<String, usize>,
        state_types: &'a [ValueType],
        event_index_map: &'a HashMap<String, usize>,
        function_names: &'a HashSet<String>,
    ) -> Self {
        Self {
            function_name: function_name.to_string(),
            param_index_map,
            param_types,
            state_index_map,
            state_types,
            event_index_map,
            function_names,
            local_index_map: HashMap::new(),
            local_types: HashMap::new(),
            scope_stack: vec![Vec::new()],
            storage_aliases: HashMap::new(),
            local_count: 0,
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

fn load_expression(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let mut tmp = Vec::new();
    if lower_expression(expr, ctx, &mut tmp) {
        instructions.append(&mut tmp);
    }
}

fn push_default_for_type(ty: &PtType, instructions: &mut Vec<Instruction>) {
    match ty {
        PtType::Address | PtType::AddressPayable => {
            instructions.push(Instruction::PushLiteral(LiteralValue::Address(vec![
                0u8;
                20
            ])));
        }
        PtType::Bool => instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false))),
        PtType::String => {
            instructions.push(Instruction::PushLiteral(LiteralValue::String(Vec::new())))
        }
        PtType::Uint(_) | PtType::Int(_) => {
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
        }
        PtType::Bytes(len) => instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
            vec![0u8; *len as usize],
        ))),
        PtType::DynamicBytes => instructions.push(Instruction::PushLiteral(
            LiteralValue::ByteArray(Vec::new()),
        )),
        _ => instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::zero(),
        ))),
    }
}

fn push_default_for_value_type(
    value_type: &ValueType,
    instructions: &mut Vec<Instruction>,
) -> bool {
    match value_type {
        ValueType::Integer { .. } => {
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            true
        }
        ValueType::Boolean => {
            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
            true
        }
        ValueType::String => {
            instructions.push(Instruction::PushLiteral(LiteralValue::String(Vec::new())));
            true
        }
        ValueType::Address => {
            instructions.push(Instruction::PushLiteral(LiteralValue::Address(vec![
                0u8;
                20
            ])));
            true
        }
        ValueType::ByteArray { fixed_len } => {
            let bytes = fixed_len
                .map(|len| vec![0u8; len as usize])
                .unwrap_or_else(Vec::new);
            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(bytes)));
            true
        }
        ValueType::Array(_)
        | ValueType::Mapping { .. }
        | ValueType::Struct { .. }
        | ValueType::Any => {
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            true
        }
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
    value_type: ValueType,
}

#[derive(Clone)]
struct StorageReference {
    state_index: usize,
    key_expressions: Vec<Expression>,
    key_types: Vec<ValueType>,
    value_type: ValueType,
    field_path: Vec<StorageReferenceField>,
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
        }
    }
}

fn resolve_storage_reference(
    expression: &Expression,
    ctx: &LoweringContext,
) -> Option<StorageReference> {
    if let Some(mapping) = resolve_mapping_access(expression, ctx) {
        return Some(mapping.to_storage_reference());
    }

    match expression {
        Expression::Variable(identifier) => ctx.storage_alias(&identifier.name).cloned(),
        Expression::MemberAccess(_, inner, member) => {
            let mut base = resolve_storage_reference(inner, ctx)?;
            let field = find_struct_field(&base.value_type, &member.name)?;
            base.field_path.push(StorageReferenceField {
                key: field.key,
                ty: field.ty.clone(),
            });
            base.value_type = field.ty.clone();
            Some(base)
        }
        _ => None,
    }
}

fn emit_storage_load(
    reference: &StorageReference,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let mut success = true;
    for expr in reference.key_expressions.iter().rev() {
        if !lower_expression(expr, ctx, instructions) {
            success = false;
        }
    }

    if !success {
        return false;
    }

    if let Some(field) = reference.field_path.last() {
        instructions.push(Instruction::LoadStructField {
            state_index: reference.state_index,
            key_types: reference.key_types.clone(),
            field_key: field.key,
            field_type: field.ty.clone(),
        });
    } else {
        instructions.push(Instruction::LoadMappingElement {
            state_index: reference.state_index,
            key_types: reference.key_types.clone(),
        });
    }

    true
}

fn emit_storage_store(
    reference: &StorageReference,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let mut success = true;
    for expr in reference.key_expressions.iter().rev() {
        if !lower_expression(expr, ctx, instructions) {
            success = false;
        }
    }

    if !success {
        return false;
    }

    if let Some(field) = reference.field_path.last() {
        instructions.push(Instruction::StoreStructField {
            state_index: reference.state_index,
            key_types: reference.key_types.clone(),
            field_key: field.key,
            field_type: field.ty.clone(),
        });
    } else {
        instructions.push(Instruction::StoreMappingElement {
            state_index: reference.state_index,
            key_types: reference.key_types.clone(),
        });
    }

    true
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
                    value_type: current_type,
                });
            }
            _ => return None,
        }
    }
}

fn resolve_builtin_call(expr: &Expression) -> Option<BuiltinCall> {
    if let Expression::MemberAccess(_, inner, member) = expr {
        if let Expression::Variable(base) = inner.as_ref() {
            match (base.name.as_str(), member.name.as_str()) {
                ("Runtime", "notify") => return Some(BuiltinCall::RuntimeNotify),
                ("Runtime", "checkWitness") => return Some(BuiltinCall::RuntimeCheckWitness),
                ("abi", "encode") => return Some(BuiltinCall::AbiEncode),
                ("abi", "encodePacked") => return Some(BuiltinCall::AbiEncodePacked),
                ("abi", "encodeWithSignature") => return Some(BuiltinCall::AbiEncodeWithSignature),
                ("abi", "decode") => return Some(BuiltinCall::AbiDecode),
                ("Storage", "find") => return Some(BuiltinCall::StorageFind),
                _ => {}
            }
        }
    }
    if let Expression::Variable(identifier) = expr {
        if identifier.name == "keccak256" {
            return Some(BuiltinCall::Keccak256);
        }
        if identifier.name == "type" {
            return Some(BuiltinCall::TypeOf);
        }
    }
    None
}

fn lower_statement(
    statement: &Statement,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    match statement {
        Statement::Block { statements, .. } => {
            ctx.enter_scope();
            let mut returned = false;
            for stmt in statements {
                if lower_statement(stmt, ctx, instructions) {
                    returned = true;
                    break;
                }
            }
            ctx.exit_scope();
            returned
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
            ctx.enter_scope();
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
            ctx.exit_scope();
            false
        }
        Statement::Expression(_, expr) => {
            if let Expression::Assign(_, lhs, rhs) = expr {
                lower_assignment(lhs, rhs, ctx, instructions);
            } else if let Expression::FunctionCall(_, func, args) = expr {
                if let Expression::Variable(identifier) = func.as_ref() {
                    if identifier.name == "require" {
                        lower_require(args, ctx, instructions);
                        return false;
                    }
                }
                if lower_expression(expr, ctx, instructions) {
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
            } else if lower_expression(expr, ctx, instructions) {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
            false
        }
        Statement::VariableDefinition(_, decl, init) => {
            if let Some(ident) = &decl.name {
                if ctx.is_local_in_current_scope(&ident.name) {
                    ctx.record_error(format!("local variable '{}' redeclared", ident.name));
                } else {
                    let is_storage_reference =
                        matches!(decl.storage, Some(PtStorageLocation::Storage(_)));
                    let inferred_type = if is_storage_reference {
                        None
                    } else {
                        infer_type_from_expression(&decl.ty, ctx)
                    };
                    let slot = ctx.allocate_local(ident.name.clone(), inferred_type);
                    if let Some(initializer) = init {
                        if is_storage_reference {
                            if let Some(reference) = resolve_storage_reference(initializer, ctx) {
                                ctx.set_storage_alias(ident.name.clone(), reference);
                            } else if lower_expression(initializer, ctx, instructions) {
                                instructions.push(Instruction::Drop(ValueType::Any));
                            }
                        } else if lower_expression(initializer, ctx, instructions) {
                            instructions.push(Instruction::StoreLocal(slot));
                        }
                    } else if !is_storage_reference {
                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                            BigInt::from(0u8),
                        )));
                        instructions.push(Instruction::StoreLocal(slot));
                    }
                }
            } else {
                ctx.record_error("variable declaration missing identifier");
            }
            false
        }
        Statement::Emit(_, call) => {
            lower_emit(call, ctx, instructions);
            false
        }
        Statement::Assembly { .. } => {
            lower_special_assembly(ctx, instructions);
            false
        }
        Statement::Try(_, expr, _, _) => {
            load_expression(expr, ctx, instructions);
            instructions.push(Instruction::Drop(ValueType::Any));
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
    if let Expression::Variable(identifier) = lhs {
        if ctx.storage_alias(&identifier.name).is_some() {
            if let Some(source_reference) = resolve_storage_reference(rhs, ctx) {
                ctx.set_storage_alias(identifier.name.clone(), source_reference);
                return;
            }
        }
    }

    if let Some(reference) = resolve_storage_reference(lhs, ctx) {
        if let ValueType::Struct { name, fields } = &reference.value_type {
            if let Expression::NamedFunctionCall(_, func, args) = rhs {
                if let Expression::Variable(identifier) = func.as_ref() {
                    if identifier.name.eq_ignore_ascii_case(name) {
                        for field in fields {
                            let mut field_reference = reference.clone();
                            field_reference.field_path.push(StorageReferenceField {
                                key: field.key,
                                ty: field.ty.clone(),
                            });
                            field_reference.value_type = field.ty.clone();

                            let success = if let Some(arg) =
                                args.iter().find(|arg| arg.name.name == field.name)
                            {
                                lower_expression(&arg.expr, ctx, instructions)
                            } else {
                                push_default_for_value_type(&field.ty, instructions)
                            };

                            if success && !emit_storage_store(&field_reference, ctx, instructions) {
                                instructions.push(Instruction::Drop(ValueType::Any));
                            }
                        }

                        return;
                    }
                }
            }
        }

        let success = lower_expression(rhs, ctx, instructions);
        if success {
            if !emit_storage_store(&reference, ctx, instructions) {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
        } else {
            instructions.push(Instruction::Drop(ValueType::Any));
        }
        return;
    }

    if let Expression::List(_, params) = lhs {
        load_expression(rhs, ctx, instructions);
        instructions.push(Instruction::Drop(ValueType::Any));

        for (_, param) in params {
            if let Some(parameter) = param {
                if let Some(name) = &parameter.name {
                    let index = ctx.ensure_local(&name.name);
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                    instructions.push(Instruction::StoreLocal(index));
                }
            }
        }

        return;
    }

    if matches!(lhs, Expression::ArraySubscript(_, _, Some(_))) {
        lower_array_store(lhs, rhs, ctx, instructions);
        return;
    }

    if let Expression::Variable(identifier) = lhs {
        if let Some(index) = ctx.resolve_local(&identifier.name) {
            if lower_expression(rhs, ctx, instructions) {
                instructions.push(Instruction::StoreLocal(index));
            }
            return;
        }
        if let Some(index) = ctx.state_index_map.get(&identifier.name) {
            if lower_expression(rhs, ctx, instructions) {
                instructions.push(Instruction::StoreState(*index));
            }
            return;
        }

        let index = ctx.ensure_local(&identifier.name);
        if lower_expression(rhs, ctx, instructions) {
            instructions.push(Instruction::StoreLocal(index));
        }
        return;
    }

    // Fallback: evaluate RHS (if possible) and drop to allow compilation to continue.
    if lower_expression(rhs, ctx, instructions) {
        instructions.push(Instruction::Drop(ValueType::Any));
    }
}

fn find_struct_field<'a>(value_type: &'a ValueType, field_name: &str) -> Option<&'a StructField> {
    match value_type {
        ValueType::Struct { fields, .. } => fields.iter().find(|field| field.name == field_name),
        _ => None,
    }
}

fn lower_compound_assignment(
    lhs: &Expression,
    rhs: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    op: BinaryOperator,
) -> bool {
    let mut load_lhs = Vec::new();

    if !lower_expression(lhs, ctx, &mut load_lhs) {
        return false;
    }

    if let Some(mapping) = resolve_mapping_access(lhs, ctx) {
        let mut key_instrs = Vec::new();
        for key_expr in mapping.key_expressions.iter().rev() {
            if !lower_expression(key_expr, ctx, &mut key_instrs) {
                ctx.record_error("failed to lower mapping key in compound assignment");
                return false;
            }
        }

        if !lower_expression(rhs, ctx, instructions) {
            return false;
        }

        instructions.append(&mut load_lhs.clone());
        instructions.push(Instruction::BinaryOp(op));
        instructions.append(&mut key_instrs);
        instructions.push(Instruction::StoreMappingElement {
            state_index: mapping.state_index,
            key_types: mapping.key_types.clone(),
        });
        return true;
    }

    if let Expression::Variable(identifier) = lhs {
        let store_instr = if let Some(local) = ctx.resolve_local(&identifier.name) {
            Instruction::StoreLocal(local)
        } else if let Some(state) = ctx.state_index_map.get(&identifier.name) {
            Instruction::StoreState(*state)
        } else {
            let local = ctx.ensure_local(&identifier.name);
            Instruction::StoreLocal(local)
        };

        if !lower_expression(rhs, ctx, instructions) {
            return false;
        }

        instructions.append(&mut load_lhs);
        instructions.push(Instruction::BinaryOp(op));
        instructions.push(store_instr);
        return true;
    }

    true
}

fn lower_array_store(
    target: &Expression,
    rhs: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    if let Expression::ArraySubscript(_, array, Some(index)) = target {
        let checkpoint = instructions.len();
        if lower_expression(array, ctx, instructions)
            && lower_expression(index, ctx, instructions)
            && lower_expression(rhs, ctx, instructions)
        {
            instructions.push(Instruction::ArraySet);
            return;
        }
        instructions.truncate(checkpoint);
    }

    load_expression(rhs, ctx, instructions);
    instructions.push(Instruction::Drop(ValueType::Any));
}

fn lower_post_inc_dec(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    increment: bool,
) -> bool {
    // Post-increment/decrement semantics:
    // 1. Load the original value (this will be the result)
    // 2. Perform the increment/decrement and store back
    // 3. The original value remains on the stack as the expression result

    // Step 1: Load original value onto stack (this is the return value)
    if !lower_expression(expr, ctx, instructions) {
        return false;
    }

    // Step 2: Perform compound assignment (x = x + 1 or x = x - 1)
    // This modifies the variable but we don't need its result on stack
    let one = Expression::NumberLiteral(Default::default(), "1".to_string(), "".to_string(), None);
    let op = if increment {
        BinaryOperator::Add
    } else {
        BinaryOperator::Sub
    };

    if !lower_compound_assignment(expr, &one, ctx, instructions, op) {
        return false;
    }

    // The original value loaded in Step 1 is already on the stack as the result
    // Do NOT load the value again - that was the bug!
    true
}

fn lower_pre_inc_dec(
    expr: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    increment: bool,
) -> bool {
    let one = Expression::NumberLiteral(Default::default(), "1".to_string(), "".to_string(), None);
    let op = if increment {
        BinaryOperator::Add
    } else {
        BinaryOperator::Sub
    };

    if !lower_compound_assignment(expr, &one, ctx, instructions, op) {
        return false;
    }

    lower_expression(expr, ctx, instructions)
}

fn lower_emit(expr: &Expression, ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>) {
    if let Expression::FunctionCall(_, func, args) = expr {
        if let Expression::Variable(identifier) = func.as_ref() {
            let original_len = instructions.len();
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                identifier.name.as_bytes().to_vec(),
            )));
            let mut success = true;
            for arg in args {
                if !lower_expression(arg, ctx, instructions) {
                    success = false;
                }
            }

            if success {
                if let Some(index) = ctx.event_index_map.get(&identifier.name) {
                    instructions.push(Instruction::EmitEvent {
                        event_index: *index,
                        arg_count: args.len(),
                    });
                } else {
                    instructions.push(Instruction::EmitEventByName {
                        name: identifier.name.clone(),
                        arg_count: args.len(),
                    });
                }
                return;
            }

            instructions.truncate(original_len);
        }
    }
}

fn lower_special_assembly(ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>) -> bool {
    match ctx.function_name.as_str() {
        "extsload" | "exttload" => {
            lower_extsload_single(ctx, instructions)
                || lower_extsload_range(ctx, instructions)
                || lower_extsload_slots(ctx, instructions)
        }
        _ => false,
    }
}

fn lower_extsload_single(ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>) -> bool {
    let slot_index = match ctx.param_index_map.get("slot").copied() {
        Some(index) if ctx.param_index_map.len() == 1 => index,
        _ => return false,
    };

    instructions.push(Instruction::LoadParameter(slot_index));
    instructions.push(Instruction::LoadStorageDynamic);
    instructions.push(Instruction::Return);
    true
}

fn lower_extsload_range(ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>) -> bool {
    let start_index = match ctx.param_index_map.get("startSlot").copied() {
        Some(index) => index,
        None => return false,
    };
    let count_index = match ctx.param_index_map.get("nSlots").copied() {
        Some(index) => index,
        None => return false,
    };

    if ctx.param_index_map.len() != 2 {
        return false;
    }

    let start_local = ctx.allocate_local("__extsload_start".to_string(), None);
    instructions.push(Instruction::LoadParameter(start_index));
    instructions.push(Instruction::StoreLocal(start_local));

    let count_local = ctx.allocate_local("__extsload_count".to_string(), None);
    instructions.push(Instruction::LoadParameter(count_index));
    instructions.push(Instruction::StoreLocal(count_local));

    let array_element_type = ValueType::ByteArray {
        fixed_len: Some(32),
    };
    let array_value_type = ValueType::Array(Box::new(array_element_type.clone()));
    let array_local = ctx.allocate_local(
        "__extsload_array".to_string(),
        Some(array_value_type.clone()),
    );
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::NewArray {
        element_type: array_element_type,
    });
    instructions.push(Instruction::StoreLocal(array_local));

    let index_local = ctx.allocate_local("__extsload_index".to_string(), None);
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::StoreLocal(index_local));

    let value_local = ctx.allocate_local("__extsload_value".to_string(), None);

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();

    instructions.push(Instruction::Label(loop_label));
    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: end_label });

    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::LoadStorageDynamic);
    instructions.push(Instruction::StoreLocal(value_local));

    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::LoadLocal(value_local));
    instructions.push(Instruction::ArraySet);

    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(1u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(index_local));

    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(1u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(start_local));

    instructions.push(Instruction::Jump { target: loop_label });
    instructions.push(Instruction::Label(end_label));
    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::Return);
    true
}

fn lower_extsload_slots(ctx: &mut LoweringContext, instructions: &mut Vec<Instruction>) -> bool {
    let slots_index = match ctx.param_index_map.get("slots").copied() {
        Some(index) if ctx.param_index_map.len() == 1 => index,
        _ => return false,
    };

    let slots_local = ctx.allocate_local("__extsload_slots".to_string(), None);
    instructions.push(Instruction::LoadParameter(slots_index));
    instructions.push(Instruction::StoreLocal(slots_local));

    let count_local = ctx.allocate_local("__extsload_count".to_string(), None);
    instructions.push(Instruction::LoadLocal(slots_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(count_local));

    let slots_array_element = ValueType::ByteArray {
        fixed_len: Some(32),
    };
    let slots_array_type = ValueType::Array(Box::new(slots_array_element.clone()));
    let array_local = ctx.allocate_local(
        "__extsload_array".to_string(),
        Some(slots_array_type.clone()),
    );
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::NewArray {
        element_type: slots_array_element,
    });
    instructions.push(Instruction::StoreLocal(array_local));

    let index_local = ctx.allocate_local("__extsload_index".to_string(), None);
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::StoreLocal(index_local));

    let value_local = ctx.allocate_local("__extsload_value".to_string(), None);

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();

    instructions.push(Instruction::Label(loop_label));
    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::LoadLocal(count_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: end_label });

    instructions.push(Instruction::LoadLocal(slots_local));
    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::ArrayGet);
    instructions.push(Instruction::LoadStorageDynamic);
    instructions.push(Instruction::StoreLocal(value_local));

    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::LoadLocal(value_local));
    instructions.push(Instruction::ArraySet);

    instructions.push(Instruction::LoadLocal(index_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(1u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(index_local));

    instructions.push(Instruction::Jump { target: loop_label });
    instructions.push(Instruction::Label(end_label));
    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::Return);
    true
}

fn lower_require(
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    if args.is_empty() {
        ctx.record_error("require() expects at least one argument");
        return;
    }

    let ok_label = ctx.next_label();
    if lower_expression(&args[0], ctx, instructions) {
        instructions.push(Instruction::JumpIf { target: ok_label });
    }

    instructions.push(Instruction::Abort);
    instructions.push(Instruction::Label(ok_label));
}

fn lower_logical_or(
    left: &Expression,
    right: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let false_label = ctx.next_label();
    let end_label = ctx.next_label();

    if !lower_expression(left, ctx, instructions) {
        return false;
    }

    instructions.push(Instruction::JumpIf {
        target: false_label,
    });
    instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
    instructions.push(Instruction::Jump { target: end_label });
    instructions.push(Instruction::Label(false_label));

    if !lower_expression(right, ctx, instructions) {
        return false;
    }

    instructions.push(Instruction::Label(end_label));
    true
}

fn lower_logical_and(
    left: &Expression,
    right: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let false_label = ctx.next_label();
    let end_label = ctx.next_label();

    if !lower_expression(left, ctx, instructions) {
        return false;
    }

    instructions.push(Instruction::JumpIf {
        target: false_label,
    });

    if !lower_expression(right, ctx, instructions) {
        return false;
    }

    instructions.push(Instruction::Jump { target: end_label });
    instructions.push(Instruction::Label(false_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
    instructions.push(Instruction::Label(end_label));
    true
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
        Expression::ShiftLeft(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Shl)
        }
        Expression::ShiftRight(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::Shr)
        }
        Expression::BitwiseAnd(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::BitAnd)
        }
        Expression::BitwiseOr(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::BitOr)
        }
        Expression::BitwiseXor(_, left, right) => {
            lower_binary_expr(left, right, ctx, instructions, BinaryOperator::BitXor)
        }
        Expression::AssignAdd(_, lhs, rhs) => {
            lower_compound_assignment(lhs, rhs, ctx, instructions, BinaryOperator::Add)
        }
        Expression::AssignSubtract(_, lhs, rhs) => {
            lower_compound_assignment(lhs, rhs, ctx, instructions, BinaryOperator::Sub)
        }
        Expression::AssignShiftLeft(_, lhs, rhs) => {
            lower_compound_assignment(lhs, rhs, ctx, instructions, BinaryOperator::Shl)
        }
        Expression::AssignShiftRight(_, lhs, rhs) => {
            lower_compound_assignment(lhs, rhs, ctx, instructions, BinaryOperator::Shr)
        }
        Expression::AssignAnd(_, lhs, rhs) => {
            lower_compound_assignment(lhs, rhs, ctx, instructions, BinaryOperator::BitAnd)
        }
        Expression::AssignOr(_, lhs, rhs) => {
            lower_compound_assignment(lhs, rhs, ctx, instructions, BinaryOperator::BitOr)
        }
        Expression::AssignXor(_, lhs, rhs) => {
            lower_compound_assignment(lhs, rhs, ctx, instructions, BinaryOperator::BitXor)
        }
        Expression::AssignMultiply(_, lhs, rhs) => {
            lower_compound_assignment(lhs, rhs, ctx, instructions, BinaryOperator::Mul)
        }
        Expression::AssignDivide(_, lhs, rhs) => {
            lower_compound_assignment(lhs, rhs, ctx, instructions, BinaryOperator::Div)
        }
        Expression::AssignModulo(_, lhs, rhs) => {
            lower_compound_assignment(lhs, rhs, ctx, instructions, BinaryOperator::Mod)
        }
        Expression::Assign(_, lhs, rhs) => {
            lower_assignment(lhs, rhs, ctx, instructions);
            true
        }
        Expression::PostIncrement(_, inner) => lower_post_inc_dec(inner, ctx, instructions, true),
        Expression::PostDecrement(_, inner) => lower_post_inc_dec(inner, ctx, instructions, false),
        Expression::PreIncrement(_, inner) => lower_pre_inc_dec(inner, ctx, instructions, true),
        Expression::PreDecrement(_, inner) => lower_pre_inc_dec(inner, ctx, instructions, false),
        Expression::Delete(_, target) => {
            load_expression(target, ctx, instructions);
            instructions.push(Instruction::Drop(ValueType::Any));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            true
        }
        Expression::Not(_, inner) => {
            if lower_expression(inner, ctx, instructions) {
                instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
                true
            } else {
                false
            }
        }
        Expression::BitwiseNot(_, inner) => {
            if lower_expression(inner, ctx, instructions) {
                instructions.push(Instruction::BitwiseNot);
                true
            } else {
                false
            }
        }
        Expression::Power(_, left, right) => {
            if let (Some(LiteralValue::Integer(base)), Some(LiteralValue::Integer(exp_lit))) = (
                literal_from_expression(left),
                literal_from_expression(right),
            ) {
                if let Some(exp) = exp_lit.to_u32() {
                    let mut result = BigInt::one();
                    for _ in 0..exp {
                        result *= &base;
                    }
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(result)));
                    return true;
                }
            }

            let base_local = ctx.allocate_local("__pow_base".to_string(), None);
            let exp_local = ctx.allocate_local("__pow_exp".to_string(), None);
            let result_local = ctx.allocate_local("__pow_result".to_string(), None);

            if !lower_expression(left, ctx, instructions) {
                return false;
            }
            instructions.push(Instruction::StoreLocal(base_local));

            if !lower_expression(right, ctx, instructions) {
                return false;
            }
            instructions.push(Instruction::StoreLocal(exp_local));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::StoreLocal(result_local));

            let loop_label = ctx.next_label();
            let end_label = ctx.next_label();
            let mul_label = ctx.next_label();
            let skip_mul_label = ctx.next_label();

            instructions.push(Instruction::Label(loop_label));
            instructions.push(Instruction::LoadLocal(exp_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf { target: end_label });

            instructions.push(Instruction::LoadLocal(exp_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::BitAnd));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Ne));
            instructions.push(Instruction::JumpIf { target: mul_label });
            instructions.push(Instruction::Jump {
                target: skip_mul_label,
            });

            instructions.push(Instruction::Label(mul_label));
            instructions.push(Instruction::LoadLocal(result_local));
            instructions.push(Instruction::LoadLocal(base_local));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
            instructions.push(Instruction::StoreLocal(result_local));

            instructions.push(Instruction::Label(skip_mul_label));
            instructions.push(Instruction::LoadLocal(base_local));
            instructions.push(Instruction::LoadLocal(base_local));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
            instructions.push(Instruction::StoreLocal(base_local));

            instructions.push(Instruction::LoadLocal(exp_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Shr));
            instructions.push(Instruction::StoreLocal(exp_local));

            instructions.push(Instruction::Jump { target: loop_label });
            instructions.push(Instruction::Label(end_label));
            instructions.push(Instruction::LoadLocal(result_local));
            true
        }
        Expression::UnaryPlus(_, inner) => lower_expression(inner, ctx, instructions),
        Expression::Negate(_, inner) => {
            if lower_expression(inner, ctx, instructions) {
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(-1),
                )));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
                true
            } else {
                false
            }
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
            if identifier.name == "this" {
                instructions.push(Instruction::PushLiteral(LiteralValue::Address(vec![
                    0u8;
                    20
                ])));
                return true;
            }
            if identifier.name == "block" || identifier.name == "msg" {
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::zero(),
                )));
                return true;
            }
            if let Some(alias) = ctx.storage_alias(&identifier.name).cloned() {
                return emit_storage_load(&alias, ctx, instructions);
            }
            if let Some(index) = ctx.param_index_map.get(&identifier.name) {
                instructions.push(Instruction::LoadParameter(*index));
                true
            } else if let Some(index) = ctx.resolve_local(&identifier.name) {
                instructions.push(Instruction::LoadLocal(index));
                true
            } else if let Some(index) = ctx.state_index_map.get(&identifier.name) {
                instructions.push(Instruction::LoadState(*index));
                true
            } else {
                // Unknown identifier - push zero as fallback (covers type names and undefined vars)
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::zero(),
                )));
                true
            }
        }
        Expression::ArraySubscript(_, _, None) => {
            instructions.push(Instruction::PushLiteral(
                LiteralValue::ByteArray(Vec::new()),
            ));
            true
        }
        Expression::ArraySubscript(_, array, Some(index)) => {
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
            } else if lower_expression(array, ctx, instructions)
                && lower_expression(index, ctx, instructions)
            {
                instructions.push(Instruction::ArrayGet);
                true
            } else {
                false
            }
        }
        Expression::ArraySlice(_, array, start, end) => {
            let array_local = ctx.allocate_local("__slice_array".to_string(), None);
            if !lower_expression(array, ctx, instructions) {
                return false;
            }
            instructions.push(Instruction::StoreLocal(array_local));

            let start_local = ctx.allocate_local("__slice_start".to_string(), None);
            if let Some(start_expr) = start {
                if !lower_expression(start_expr, ctx, instructions) {
                    return false;
                }
            } else {
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::zero(),
                )));
            }
            instructions.push(Instruction::StoreLocal(start_local));

            let end_local = ctx.allocate_local("__slice_end".to_string(), None);
            if let Some(end_expr) = end {
                if !lower_expression(end_expr, ctx, instructions) {
                    return false;
                }
            } else {
                instructions.push(Instruction::LoadLocal(array_local));
                instructions.push(Instruction::GetSize);
            }
            instructions.push(Instruction::StoreLocal(end_local));

            // Clamp start to >= 0
            let clamp_start_label = ctx.next_label();
            let clamp_start_done = ctx.next_label();
            instructions.push(Instruction::LoadLocal(start_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf {
                target: clamp_start_label,
            });
            instructions.push(Instruction::Jump {
                target: clamp_start_done,
            });
            instructions.push(Instruction::Label(clamp_start_label));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(start_local));
            instructions.push(Instruction::Label(clamp_start_done));

            // Clamp end to array length
            let size_local = ctx.allocate_local("__slice_size".to_string(), None);
            instructions.push(Instruction::LoadLocal(array_local));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::StoreLocal(size_local));

            let clamp_end_label = ctx.next_label();
            let clamp_end_done = ctx.next_label();
            instructions.push(Instruction::LoadLocal(end_local));
            instructions.push(Instruction::LoadLocal(size_local));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Gt));
            instructions.push(Instruction::JumpIf {
                target: clamp_end_label,
            });
            instructions.push(Instruction::Jump {
                target: clamp_end_done,
            });
            instructions.push(Instruction::Label(clamp_end_label));
            instructions.push(Instruction::LoadLocal(size_local));
            instructions.push(Instruction::StoreLocal(end_local));
            instructions.push(Instruction::Label(clamp_end_done));

            let len_local = ctx.allocate_local("__slice_len".to_string(), None);
            instructions.push(Instruction::LoadLocal(end_local));
            instructions.push(Instruction::LoadLocal(start_local));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Sub));
            instructions.push(Instruction::StoreLocal(len_local));

            let clamp_label = ctx.next_label();
            let clamp_done = ctx.next_label();
            instructions.push(Instruction::LoadLocal(len_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf {
                target: clamp_label,
            });
            instructions.push(Instruction::Jump { target: clamp_done });
            instructions.push(Instruction::Label(clamp_label));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(len_local));
            instructions.push(Instruction::Label(clamp_done));

            let element_type = infer_array_element_type(array, ctx).unwrap_or(ValueType::Any);
            let slice_array_type = ValueType::Array(Box::new(element_type.clone()));
            let out_local = ctx.allocate_local("__slice_out".to_string(), Some(slice_array_type));
            instructions.push(Instruction::LoadLocal(len_local));
            instructions.push(Instruction::NewArray { element_type });
            instructions.push(Instruction::StoreLocal(out_local));

            let idx_local = ctx.allocate_local("__slice_index".to_string(), None);
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(idx_local));

            let loop_label = ctx.next_label();
            let end_label = ctx.next_label();

            instructions.push(Instruction::Label(loop_label));
            instructions.push(Instruction::LoadLocal(idx_local));
            instructions.push(Instruction::LoadLocal(len_local));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf { target: end_label });

            instructions.push(Instruction::LoadLocal(out_local));
            instructions.push(Instruction::LoadLocal(idx_local));
            instructions.push(Instruction::LoadLocal(array_local));
            instructions.push(Instruction::LoadLocal(start_local));
            instructions.push(Instruction::LoadLocal(idx_local));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::LoadLocal(idx_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(1u8),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(idx_local));

            instructions.push(Instruction::Jump { target: loop_label });
            instructions.push(Instruction::Label(end_label));
            instructions.push(Instruction::LoadLocal(out_local));
            true
        }
        Expression::ArrayLiteral(_, elements) => {
            let element_type = infer_literal_array_element_type(elements);
            let array_local = ctx.allocate_local(
                "__array_literal".to_string(),
                Some(ValueType::Array(Box::new(element_type.clone()))),
            );
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(elements.len()),
            )));
            instructions.push(Instruction::NewArray { element_type });
            instructions.push(Instruction::StoreLocal(array_local));

            for (index, element) in elements.iter().enumerate() {
                instructions.push(Instruction::LoadLocal(array_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(index as u64),
                )));
                if !lower_expression(element, ctx, instructions) {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                }
                instructions.push(Instruction::ArraySet);
            }

            instructions.push(Instruction::LoadLocal(array_local));
            true
        }
        Expression::Or(_, left, right) => lower_logical_or(left, right, ctx, instructions),
        Expression::And(_, left, right) => lower_logical_and(left, right, ctx, instructions),
        Expression::FunctionCallBlock(_, call, block) => {
            load_expression(call, ctx, instructions);
            instructions.push(Instruction::Drop(ValueType::Any));
            if let Statement::Block { statements, .. } = block.as_ref() {
                for stmt in statements {
                    lower_statement(stmt, ctx, instructions);
                }
            }
            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
            true
        }
        Expression::NamedFunctionCall(_, func, args) => {
            if !matches!(func.as_ref(), Expression::Variable(_)) {
                load_expression(func, ctx, instructions);
                instructions.push(Instruction::Drop(ValueType::Any));
            }
            for arg in args {
                load_expression(&arg.expr, ctx, instructions);
                instructions.push(Instruction::Drop(ValueType::Any));
            }
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            true
        }
        Expression::FunctionCall(_, func, args) => {
            if let Expression::FunctionCallBlock(_, inner_call, block) = func.as_ref() {
                load_expression(inner_call, ctx, instructions);
                instructions.push(Instruction::Drop(ValueType::Any));

                if let Statement::Block { statements, .. } = block.as_ref() {
                    for stmt in statements {
                        lower_statement(stmt, ctx, instructions);
                    }
                }

                instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
                return true;
            }
            if let Expression::Type(_, ty) = func.as_ref() {
                match ty {
                    PtType::Address | PtType::AddressPayable => {
                        if args.is_empty() {
                            instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                                vec![0u8; 20],
                            )));
                        } else if lower_expression(&args[0], ctx, instructions) {
                        } else {
                            instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                                vec![0u8; 20],
                            )));
                        }
                        return true;
                    }
                    PtType::Uint(_) | PtType::Int(_) => {
                        if args.is_empty() || !lower_expression(&args[0], ctx, instructions) {
                            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                                BigInt::zero(),
                            )));
                        }
                        return true;
                    }
                    _ => {}
                }
            }
            if let Some(builtin) = resolve_builtin_call(func.as_ref()) {
                let (min_args, max_args) = match builtin {
                    BuiltinCall::RuntimeNotify => (2, Some(2)),
                    BuiltinCall::RuntimeCheckWitness => (1, Some(1)),
                    BuiltinCall::AbiEncode | BuiltinCall::AbiEncodePacked => (1, None),
                    BuiltinCall::AbiEncodeWithSignature => (1, None),
                    BuiltinCall::AbiDecode => (2, None),
                    BuiltinCall::Keccak256 => (1, None),
                    BuiltinCall::StorageFind => (1, None),
                    BuiltinCall::TypeOf => (1, None),
                };

                if args.len() < min_args || max_args.is_some_and(|max| args.len() > max) {
                    ctx.record_error(format!(
                        "builtin call requires between {} and {} argument(s), got {}",
                        min_args,
                        max_args
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "∞".to_string()),
                        args.len()
                    ));
                    return false;
                }

                let mut success = true;
                if !matches!(builtin, BuiltinCall::TypeOf) {
                    for arg in args {
                        if !lower_expression(arg, ctx, instructions) {
                            success = false;
                        }
                    }
                }

                if success {
                    match builtin {
                        BuiltinCall::RuntimeNotify | BuiltinCall::RuntimeCheckWitness => {
                            instructions.push(Instruction::CallBuiltin {
                                builtin,
                                arg_count: args.len(),
                            });
                        }
                        BuiltinCall::AbiEncode | BuiltinCall::AbiEncodePacked => {
                            for _ in args {
                                instructions.push(Instruction::Drop(ValueType::Any));
                            }
                            instructions
                                .push(Instruction::PushLiteral(LiteralValue::ByteArray(vec![])));
                        }
                        BuiltinCall::AbiEncodeWithSignature => {
                            let mut selector = Vec::new();
                            if let Some(Expression::StringLiteral(parts)) = args.first() {
                                let bytes = string_literal_bytes(parts);
                                let mut hasher = Keccak256::new();
                                hasher.update(&bytes);
                                let digest = hasher.finalize();
                                selector.extend_from_slice(&digest[..4]);
                            }
                            for _ in args {
                                instructions.push(Instruction::Drop(ValueType::Any));
                            }
                            instructions
                                .push(Instruction::PushLiteral(LiteralValue::ByteArray(selector)));
                        }
                        BuiltinCall::AbiDecode => {
                            for _ in args {
                                instructions.push(Instruction::Drop(ValueType::Any));
                            }
                            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                                BigInt::zero(),
                            )));
                        }
                        BuiltinCall::TypeOf => {
                            for _ in args {
                                instructions.push(Instruction::Drop(ValueType::Any));
                            }
                            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                                BigInt::zero(),
                            )));
                        }
                        BuiltinCall::Keccak256 => {
                            for _ in args {
                                instructions.push(Instruction::Drop(ValueType::Any));
                            }
                            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                                vec![0u8; 32],
                            )));
                        }
                        BuiltinCall::StorageFind => {
                            for _ in args {
                                instructions.push(Instruction::Drop(ValueType::Any));
                            }
                            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                                Vec::new(),
                            )));
                        }
                    }
                }
                success
            } else if let Expression::MemberAccess(_, _, _) = func.as_ref() {
                let mut success = true;
                for arg in args {
                    if !lower_expression(arg, ctx, instructions) {
                        success = false;
                    }
                }

                if success {
                    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                        BigInt::zero(),
                    )));
                }
                success
            } else if let Expression::Variable(identifier) = func.as_ref() {
                if identifier.name == "require" {
                    ctx.record_error("require() cannot be used as an expression");
                    return false;
                }

                if ctx.function_names.contains(&identifier.name) {
                    let mut success = true;
                    for arg in args {
                        if !lower_expression(arg, ctx, instructions) {
                            success = false;
                        }
                    }

                    if success {
                        instructions.push(Instruction::CallFunction {
                            name: identifier.name.clone(),
                            arg_count: args.len(),
                        });
                    }

                    return success;
                }

                for arg in args {
                    load_expression(arg, ctx, instructions);
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::zero(),
                )));
                true
            } else {
                for arg in args {
                    load_expression(arg, ctx, instructions);
                    instructions.push(Instruction::Drop(ValueType::Any));
                }
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::zero(),
                )));
                true
            }
        }
        Expression::New(_, expr) => {
            load_expression(expr, ctx, instructions);
            instructions.push(Instruction::Drop(ValueType::Any));
            instructions.push(Instruction::PushLiteral(
                LiteralValue::ByteArray(Vec::new()),
            ));
            true
        }
        Expression::Type(_, ty) => {
            push_default_for_type(ty, instructions);
            true
        }
        Expression::List(_, _) => {
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            true
        }
        Expression::ConditionalOperator(_, condition, then_expr, else_expr) => {
            // Ternary operator: condition ? then_expr : else_expr
            // Semantics: evaluate condition, then evaluate ONLY one branch based on result

            // Generate unique labels for this ternary expression
            let else_label = ctx.next_label();
            let end_label = ctx.next_label();

            // Step 1: Evaluate condition
            if !lower_expression(condition, ctx, instructions) {
                return false;
            }

            // Step 2: Jump to else branch if condition is false (zero)
            // Note: JumpIf jumps when condition is TRUE, so we need to negate
            // We use a NOT + JumpIf pattern, or we can use the existing JumpIf
            // which jumps on non-zero. For "jump if zero", we negate first.
            instructions.push(Instruction::BitwiseNot); // NOT: 0 -> -1 (truthy), non-zero -> 0
            instructions.push(Instruction::JumpIf { target: else_label });

            // Step 3: Evaluate then branch (condition was true/non-zero)
            if !lower_expression(then_expr, ctx, instructions) {
                return false;
            }
            instructions.push(Instruction::Jump { target: end_label });

            // Step 4: Else branch label
            instructions.push(Instruction::Label(else_label));

            // Step 5: Evaluate else branch (condition was false/zero)
            if !lower_expression(else_expr, ctx, instructions) {
                return false;
            }

            // Step 6: End label - result is on stack from whichever branch executed
            instructions.push(Instruction::Label(end_label));

            true
        }
        Expression::Parenthesis(_, inner) => lower_expression(inner, ctx, instructions),
        Expression::MemberAccess(_, inner, member) => {
            match member.name.as_str() {
                "sender" => {
                    if let Expression::Variable(base) = inner.as_ref() {
                        if base.name == "msg" {
                            instructions
                                .push(Instruction::LoadRuntimeValue(RuntimeValue::MsgSender));
                            return true;
                        }
                    }
                }
                "timestamp" => {
                    if let Expression::Variable(base) = inner.as_ref() {
                        if base.name == "block" {
                            instructions
                                .push(Instruction::LoadRuntimeValue(RuntimeValue::BlockTimestamp));
                            return true;
                        }
                    }
                }
                "length" => {
                    if lower_expression(inner, ctx, instructions) {
                        instructions.push(Instruction::GetSize);
                        return true;
                    }
                    return false;
                }
                "code" => {
                    if lower_expression(inner, ctx, instructions) {
                        instructions.push(Instruction::Drop(ValueType::Any));
                        instructions
                            .push(Instruction::PushLiteral(LiteralValue::ByteArray(vec![])));
                        return true;
                    }
                    return false;
                }
                "max" => {
                    if let Expression::Type(_, PtType::Uint(bits)) = inner.as_ref() {
                        let mut value = BigInt::one();
                        value <<= *bits as usize;
                        value -= BigInt::one();
                        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(value)));
                        return true;
                    }
                }
                "interfaceId" => {
                    if let Expression::Type(_, _) = inner.as_ref() {
                        instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(vec![
                            0, 0, 0, 0,
                        ])));
                        return true;
                    }
                }
                _ => {}
            }

            if let Some(reference) = resolve_storage_reference(expr, ctx) {
                return emit_storage_load(&reference, ctx, instructions);
            }

            load_expression(inner, ctx, instructions);
            instructions.push(Instruction::Drop(ValueType::Any));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            true
        }
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

#[cfg(test)]
mod tests;
