#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructFieldMetadata {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructTypeMetadata {
    pub name: String,
    pub fields: Vec<StructFieldMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumTypeMetadata {
    pub name: String,
    pub variants: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeoType {
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
    Array(Box<NeoType>),
    Mapping {
        key: Box<NeoType>,
        value: Box<NeoType>,
    },
    Struct {
        name: String,
        fields: Vec<StructFieldType>,
    },
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructFieldType {
    pub name: String,
    pub ty: Box<NeoType>,
}

#[derive(Debug, Error)]
pub enum TypeParseError {
    #[error("unsupported Solidity type '{0}'")]
    Unsupported(String),
}

