impl ExecutionContext {
    fn convert_item(&self, item: StackItem, target_code: u8) -> Result<StackItem, RuntimeError> {
        match target_code {
            0x00 => Ok(item), // Any/no-op
            0x20 => Ok(StackItem::Boolean(item.is_truthy())),
            0x21 | 0x22 => {
                let bytes = Self::stack_item_to_bytes(item);
                let mut buf = [0u8; 8];
                for (i, b) in bytes.iter().take(8).enumerate() {
                    buf[i] = *b;
                }
                Ok(StackItem::Integer(i64::from_le_bytes(buf)))
            }
            0x28 | 0x30 => Ok(StackItem::byte_array(Self::stack_item_to_bytes(item))),
            0x40 | 0x41 => match item {
                StackItem::Array(items) => Ok(StackItem::Array(items)),
                StackItem::Map(map) => Ok(StackItem::array(
                    map.borrow()
                        .iter()
                        .map(|(k, v)| {
                            StackItem::array(vec![StackItem::byte_array(k.clone()), v.clone()])
                        })
                        .collect(),
                )),
                other => Ok(StackItem::array(vec![other])),
            },
            0x48 => match item {
                StackItem::Map(map) => Ok(StackItem::Map(map)),
                StackItem::Array(items) => {
                    let mut map = std::collections::HashMap::new();
                    for pair in items.borrow().iter() {
                        let StackItem::Array(kv) = pair else {
                            continue;
                        };
                        let kv = kv.borrow();
                        if kv.len() < 2 {
                            continue;
                        }
                        let key = kv.first().cloned().unwrap_or(StackItem::Null);
                        let value = kv.last().cloned().unwrap_or(StackItem::Null);
                        map.insert(Self::stack_item_to_bytes(key), value);
                    }
                    Ok(StackItem::map(map))
                }
                other => {
                    let mut map = std::collections::HashMap::new();
                    map.insert(Vec::new(), other);
                    Ok(StackItem::map(map))
                }
            },
            0x80 => Ok(item), // iterator tokens already byte arrays; leave untouched
            _ => Ok(item),
        }
    }

    fn is_iterator_token(&self, item: &StackItem) -> bool {
        if let Some(id) = Self::iterator_id_from_item(item) {
            return self.iterators.contains_key(&id);
        }
        false
    }

}
