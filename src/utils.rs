//! Shared Utility Functions
//!
//! Common utilities used across the compiler.

/// Convert a Solidity type string to its canonical form for ABI encoding.
///
/// Handles special cases like `struct Foo` and `enum Bar` by extracting
/// just the type name.
///
/// # Examples
/// ```
/// use neo_solidity::utils::canonical_param_type;
/// assert_eq!(canonical_param_type("uint256"), "uint256");
/// assert_eq!(canonical_param_type("struct MyStruct"), "MyStruct");
/// assert_eq!(canonical_param_type("enum MyEnum"), "MyEnum");
/// ```
pub fn canonical_param_type(ty: &str) -> String {
    let mut parts = ty.split_whitespace();
    match parts.next() {
        Some("struct" | "enum") => parts.next().unwrap_or_default().to_string(),
        Some(first) => first.to_string(),
        None => String::new(),
    }
}

/// Simple canonical type extraction (first word only).
///
/// Use this when struct/enum special handling is not needed.
pub fn canonical_param_type_simple(ty: &str) -> String {
    ty.split_whitespace().next().unwrap_or_default().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_param_type() {
        assert_eq!(canonical_param_type("uint256"), "uint256");
        assert_eq!(canonical_param_type("struct MyStruct"), "MyStruct");
        assert_eq!(canonical_param_type("enum MyEnum"), "MyEnum");
        assert_eq!(canonical_param_type("address payable"), "address");
        assert_eq!(canonical_param_type(""), "");
    }

    #[test]
    fn test_canonical_param_type_simple() {
        assert_eq!(canonical_param_type_simple("uint256"), "uint256");
        assert_eq!(canonical_param_type_simple("struct MyStruct"), "struct");
        assert_eq!(canonical_param_type_simple("address payable"), "address");
    }
}
