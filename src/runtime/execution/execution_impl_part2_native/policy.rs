impl ExecutionContext {
    fn invoke_native_policy(method: &str) -> StackItem {
        match method {
            "getfeeperbyte" | "getexecfeefactor" | "getstorageprice" => {
                StackItem::UnsignedInteger(0)
            }
            _ => StackItem::Null,
        }
    }
}
