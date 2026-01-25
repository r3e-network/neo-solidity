//! Stack item types for execution context.
//!
//! StackItem represents values that can be pushed onto the evaluation stack
//! during NeoVM execution.

use serde::{Deserialize, Serialize};

/// Stack item in execution context
#[derive(Debug, Clone)]
pub enum StackItem {
    Integer(i64),
    UnsignedInteger(u64),
    ByteArray(std::rc::Rc<std::cell::RefCell<Vec<u8>>>),
    Array(std::rc::Rc<std::cell::RefCell<Vec<StackItem>>>),
    Map(std::rc::Rc<std::cell::RefCell<std::collections::HashMap<Vec<u8>, StackItem>>>),
    Boolean(bool),
    Null,
}

impl StackItem {
    pub fn byte_array(bytes: Vec<u8>) -> Self {
        Self::ByteArray(std::rc::Rc::new(std::cell::RefCell::new(bytes)))
    }

    pub fn array(items: Vec<StackItem>) -> Self {
        Self::Array(std::rc::Rc::new(std::cell::RefCell::new(items)))
    }

    pub fn map(map: std::collections::HashMap<Vec<u8>, StackItem>) -> Self {
        Self::Map(std::rc::Rc::new(std::cell::RefCell::new(map)))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum StackItemSerde {
    Integer(i64),
    UnsignedInteger(u64),
    ByteArray(Vec<u8>),
    Array(Vec<StackItemSerde>),
    Map(Vec<(Vec<u8>, StackItemSerde)>),
    Boolean(bool),
    Null,
}

impl StackItemSerde {
    fn from_item(item: &StackItem) -> Self {
        match item {
            StackItem::Integer(value) => StackItemSerde::Integer(*value),
            StackItem::UnsignedInteger(value) => StackItemSerde::UnsignedInteger(*value),
            StackItem::ByteArray(bytes) => StackItemSerde::ByteArray(bytes.borrow().clone()),
            StackItem::Array(items) => StackItemSerde::Array(
                items
                    .borrow()
                    .iter()
                    .map(StackItemSerde::from_item)
                    .collect(),
            ),
            StackItem::Map(map) => StackItemSerde::Map(
                map.borrow()
                    .iter()
                    .map(|(k, v)| (k.clone(), StackItemSerde::from_item(v)))
                    .collect(),
            ),
            StackItem::Boolean(value) => StackItemSerde::Boolean(*value),
            StackItem::Null => StackItemSerde::Null,
        }
    }

    fn into_item(self) -> StackItem {
        match self {
            StackItemSerde::Integer(value) => StackItem::Integer(value),
            StackItemSerde::UnsignedInteger(value) => StackItem::UnsignedInteger(value),
            StackItemSerde::ByteArray(bytes) => StackItem::byte_array(bytes),
            StackItemSerde::Array(items) => {
                let converted = items.into_iter().map(StackItemSerde::into_item).collect();
                StackItem::array(converted)
            }
            StackItemSerde::Map(entries) => {
                let mut map = std::collections::HashMap::new();
                for (key, value) in entries {
                    map.insert(key, value.into_item());
                }
                StackItem::map(map)
            }
            StackItemSerde::Boolean(value) => StackItem::Boolean(value),
            StackItemSerde::Null => StackItem::Null,
        }
    }
}

impl serde::Serialize for StackItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        StackItemSerde::from_item(self).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for StackItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        StackItemSerde::deserialize(deserializer).map(StackItemSerde::into_item)
    }
}

impl PartialEq for StackItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StackItem::Integer(a), StackItem::Integer(b)) => a == b,
            (StackItem::UnsignedInteger(a), StackItem::UnsignedInteger(b)) => a == b,
            (StackItem::Boolean(a), StackItem::Boolean(b)) => a == b,
            (StackItem::Null, StackItem::Null) => true,
            (StackItem::ByteArray(a), StackItem::ByteArray(b)) => a.borrow().eq(&*b.borrow()),
            (StackItem::Array(a), StackItem::Array(b)) => a.borrow().eq(&*b.borrow()),
            (StackItem::Map(a), StackItem::Map(b)) => a.borrow().eq(&*b.borrow()),
            _ => false,
        }
    }
}
