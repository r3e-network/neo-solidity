impl ExecutionContext {
    fn invoke_native_role_management(method: &str) -> StackItem {
        match method {
            "designateasrole" | "isdesignated" => StackItem::Boolean(true),
            _ => StackItem::Null,
        }
    }
}
