impl ExecutionContext {
    fn invoke_native_stdlib(method: &str, params: StackItem) -> StackItem {
        match method {
            "serialize" => {
                if let StackItem::Array(args) = params {
                    let value = args.borrow().first().cloned().unwrap_or(StackItem::Null);
                    let bytes = serde_json::to_vec(&value).unwrap_or_default();
                    StackItem::byte_array(bytes)
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            "deserialize" => {
                if let StackItem::Array(args) = params {
                    let input = args
                        .borrow()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| StackItem::byte_array(Vec::new()));
                    let bytes = Self::stack_item_to_bytes(input);
                    serde_json::from_slice::<StackItem>(&bytes).unwrap_or(StackItem::Null)
                } else {
                    StackItem::Null
                }
            }
            "jsonserialize" => {
                if let StackItem::Array(args) = params {
                    let value = args.borrow().first().cloned().unwrap_or(StackItem::Null);
                    let json = serde_json::to_string(&value).unwrap_or_default();
                    StackItem::byte_array(json.into_bytes())
                } else {
                    StackItem::byte_array(Vec::new())
                }
            }
            "jsondeserialize" => {
                if let StackItem::Array(args) = params {
                    let input = args
                        .borrow()
                        .first()
                        .cloned()
                        .unwrap_or_else(|| StackItem::byte_array(Vec::new()));
                    let bytes = Self::stack_item_to_bytes(input);
                    let json_str = String::from_utf8(bytes).unwrap_or_default();
                    serde_json::from_str::<StackItem>(&json_str).unwrap_or(StackItem::Null)
                } else {
                    StackItem::Null
                }
            }
            _ => StackItem::Null,
        }
    }
}
