//! Documentation Generator
//!
//! Generates API documentation from source code.

/// Documentation item
#[derive(Debug, Clone)]
pub struct DocItem {
    pub name: String,
    pub kind: DocKind,
    pub description: String,
    pub params: Vec<DocParam>,
    pub returns: Option<String>,
}

/// Documentation kind
#[derive(Debug, Clone, Copy)]
pub enum DocKind {
    Contract,
    Function,
    Event,
    Modifier,
    Variable,
}

/// Parameter documentation
#[derive(Debug, Clone)]
pub struct DocParam {
    pub name: String,
    pub type_name: String,
    pub description: String,
}

impl DocItem {
    pub fn new(name: impl Into<String>, kind: DocKind) -> Self {
        Self {
            name: name.into(),
            kind,
            description: String::new(),
            params: Vec::new(),
            returns: None,
        }
    }
}
