//! Error Decoding Framework
//! 
//! This module provides comprehensive error decoding capabilities including:
//! - ABI error decoding
//! - Panic code interpretation
//! - Custom error extraction
//! - Revert reason parsing
//! - Stack trace enhancement

use super::*;
use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Error decoder for contract execution errors
pub struct ErrorDecoder {
    panic_codes: HashMap<u32, PanicInfo>,
    custom_errors: HashMap<[u8; 4], CustomErrorInfo>,
    builtin_errors: HashMap<[u8; 4], BuiltinErrorInfo>,
}

/// Panic code information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanicInfo {
    pub code: u32,
    pub name: String,
    pub description: String,
    pub category: PanicCategory,
}

/// Categories of panic errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PanicCategory {
    AssertionFailure,
    ArithmeticOverflow,
    DivisionByZero,
    ArrayBounds,
    Storage,
    Internal,
}

/// Custom error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomErrorInfo {
    pub selector: [u8; 4],
    pub name: String,
    pub signature: String,
    pub parameters: Vec<ErrorParameter>,
    pub contract: Option<String>,
}

/// Builtin error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinErrorInfo {
    pub selector: [u8; 4],
    pub name: String,
    pub description: String,
}

/// Error parameter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorParameter {
    pub name: String,
    pub type_name: String,
    pub indexed: bool,
}

/// Decoded error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedError {
    pub error_type: DecodedErrorType,
    pub message: String,
    pub selector: Option<[u8; 4]>,
    pub parameters: Vec<DecodedParameter>,
    pub raw_data: Vec<u8>,
    pub source_location: Option<SourceLocation>,
    pub suggestions: Vec<String>,
}

/// Types of decoded errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecodedErrorType {
    Panic {
        code: u32,
        category: PanicCategory,
    },
    Custom {
        name: String,
        contract: Option<String>,
    },
    Builtin {
        name: String,
    },
    Revert {
        reason: Option<String>,
    },
    Unknown {
        selector: Option<[u8; 4]>,
    },
}

/// Decoded parameter value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedParameter {
    pub name: String,
    pub type_name: String,
    pub value: ParameterValue,
    pub display_value: String,
}

/// Parameter value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterValue {
    Uint(u64),
    Int(i64),
    Address(String),
    Bool(bool),
    Bytes(Vec<u8>),
    String(String),
    Array(Vec<ParameterValue>),
    Tuple(Vec<ParameterValue>),
}

impl ErrorDecoder {
    /// Create new error decoder
    pub fn new() -> Result<Self> {
        let panic_codes = Self::initialize_panic_codes();
        let builtin_errors = Self::initialize_builtin_errors();
        
        Ok(Self {
            panic_codes,
            custom_errors: HashMap::new(),
            builtin_errors,
        })
    }

    /// Decode error data into human-readable format
    pub fn decode_error(&self, error_data: &[u8]) -> Result<String> {
        let decoded = self.decode_error_detailed(error_data)?;
        Ok(self.format_error(&decoded))
    }

    /// Decode error data with full details
    pub fn decode_error_detailed(&self, error_data: &[u8]) -> Result<DecodedError> {
        if error_data.is_empty() {
            return Ok(DecodedError {
                error_type: DecodedErrorType::Unknown { selector: None },
                message: "Empty error data".to_string(),
                selector: None,
                parameters: Vec::new(),
                raw_data: error_data.to_vec(),
                source_location: None,
                suggestions: vec!["Check for out-of-gas or invalid state transitions".to_string()],
            });
        }

        // Check for panic errors (0x4e487b71 selector)
        if error_data.len() >= 4 && &error_data[0..4] == &[0x4e, 0x48, 0x7b, 0x71] {
            return self.decode_panic_error(error_data);
        }

        // Check for Error(string) (0x08c379a0 selector)
        if error_data.len() >= 4 && &error_data[0..4] == &[0x08, 0xc3, 0x79, 0xa0] {
            return self.decode_string_error(error_data);
        }

        // Check if it's a known builtin error
        if error_data.len() >= 4 {
            let selector = [error_data[0], error_data[1], error_data[2], error_data[3]];
            
            if let Some(builtin_info) = self.builtin_errors.get(&selector) {
                return self.decode_builtin_error(error_data, builtin_info);
            }

            if let Some(custom_info) = self.custom_errors.get(&selector) {
                return self.decode_custom_error(error_data, custom_info);
            }
        }

        // Unknown error format
        Ok(DecodedError {
            error_type: DecodedErrorType::Unknown {
                selector: if error_data.len() >= 4 {
                    Some([error_data[0], error_data[1], error_data[2], error_data[3]])
                } else {
                    None
                },
            },
            message: "Unknown error format".to_string(),
            selector: if error_data.len() >= 4 {
                Some([error_data[0], error_data[1], error_data[2], error_data[3]])
            } else {
                None
            },
            parameters: Vec::new(),
            raw_data: error_data.to_vec(),
            source_location: None,
            suggestions: vec![
                "This may be a custom error not in the decoder database".to_string(),
                "Check contract ABI for error definitions".to_string(),
                "Verify the error data format".to_string(),
            ],
        })
    }

    /// Register custom error for decoding
    pub fn register_custom_error(&mut self, error_info: CustomErrorInfo) {
        self.custom_errors.insert(error_info.selector, error_info);
    }

    /// Register multiple custom errors from ABI
    pub fn register_errors_from_abi(&mut self, abi_json: &str) -> Result<()> {
        let abi: serde_json::Value = serde_json::from_str(abi_json)?;
        
        if let Some(abi_array) = abi.as_array() {
            for item in abi_array {
                if let Some(item_type) = item.get("type").and_then(|t| t.as_str()) {
                    if item_type == "error" {
                        let error_info = self.parse_error_from_abi_item(item)?;
                        self.register_custom_error(error_info);
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Decode panic error (0x4e487b71)
    fn decode_panic_error(&self, error_data: &[u8]) -> Result<DecodedError> {
        if error_data.len() < 36 {
            anyhow::bail!("Invalid panic error data length");
        }

        // Extract panic code (bytes 4-35, but we only need the last 4 bytes)
        let panic_code = u32::from_be_bytes([
            error_data[32], error_data[33], error_data[34], error_data[35]
        ]);

        let panic_info = self.panic_codes.get(&panic_code).cloned().unwrap_or_else(|| {
            PanicInfo {
                code: panic_code,
                name: "Unknown Panic".to_string(),
                description: format!("Unknown panic code: {}", panic_code),
                category: PanicCategory::Internal,
            }
        });

        Ok(DecodedError {
            error_type: DecodedErrorType::Panic {
                code: panic_code,
                category: panic_info.category.clone(),
            },
            message: format!("Panic: {} ({})", panic_info.name, panic_info.description),
            selector: Some([0x4e, 0x48, 0x7b, 0x71]),
            parameters: vec![
                DecodedParameter {
                    name: "code".to_string(),
                    type_name: "uint256".to_string(),
                    value: ParameterValue::Uint(panic_code as u64),
                    display_value: panic_code.to_string(),
                }
            ],
            raw_data: error_data.to_vec(),
            source_location: None,
            suggestions: self.get_panic_suggestions(&panic_info),
        })
    }

    /// Decode string error (0x08c379a0)
    fn decode_string_error(&self, error_data: &[u8]) -> Result<DecodedError> {
        if error_data.len() < 68 {
            return Ok(DecodedError {
                error_type: DecodedErrorType::Revert { reason: None },
                message: "Revert with empty reason".to_string(),
                selector: Some([0x08, 0xc3, 0x79, 0xa0]),
                parameters: Vec::new(),
                raw_data: error_data.to_vec(),
                source_location: None,
                suggestions: vec!["Check require() or revert() statements in the contract".to_string()],
            });
        }

        // Parse ABI-encoded string
        // Skip selector (4 bytes) + offset (32 bytes) + length (32 bytes)
        let length_start = 36;
        if error_data.len() < length_start + 32 {
            anyhow::bail!("Invalid string error data");
        }

        let string_length = u32::from_be_bytes([
            error_data[length_start + 28],
            error_data[length_start + 29],
            error_data[length_start + 30],
            error_data[length_start + 31],
        ]) as usize;

        let string_start = length_start + 32;
        if error_data.len() < string_start + string_length {
            anyhow::bail!("Invalid string error data length");
        }

        let reason = String::from_utf8(error_data[string_start..string_start + string_length].to_vec())
            .unwrap_or_else(|_| "Invalid UTF-8 in error message".to_string());

        Ok(DecodedError {
            error_type: DecodedErrorType::Revert {
                reason: Some(reason.clone()),
            },
            message: format!("Revert: {}", reason),
            selector: Some([0x08, 0xc3, 0x79, 0xa0]),
            parameters: vec![
                DecodedParameter {
                    name: "reason".to_string(),
                    type_name: "string".to_string(),
                    value: ParameterValue::String(reason.clone()),
                    display_value: reason,
                }
            ],
            raw_data: error_data.to_vec(),
            source_location: None,
            suggestions: vec![
                "Check the condition in require() statement".to_string(),
                "Verify input parameters are valid".to_string(),
                "Check contract state before function call".to_string(),
            ],
        })
    }

    /// Decode builtin error
    fn decode_builtin_error(&self, error_data: &[u8], builtin_info: &BuiltinErrorInfo) -> Result<DecodedError> {
        Ok(DecodedError {
            error_type: DecodedErrorType::Builtin {
                name: builtin_info.name.clone(),
            },
            message: format!("Builtin Error: {} - {}", builtin_info.name, builtin_info.description),
            selector: Some(builtin_info.selector),
            parameters: Vec::new(), // Would decode parameters if any
            raw_data: error_data.to_vec(),
            source_location: None,
            suggestions: vec![format!("See documentation for {}", builtin_info.name)],
        })
    }

    /// Decode custom error
    fn decode_custom_error(&self, error_data: &[u8], custom_info: &CustomErrorInfo) -> Result<DecodedError> {
        let parameters = self.decode_error_parameters(&error_data[4..], &custom_info.parameters)?;

        Ok(DecodedError {
            error_type: DecodedErrorType::Custom {
                name: custom_info.name.clone(),
                contract: custom_info.contract.clone(),
            },
            message: format!("Custom Error: {}", custom_info.name),
            selector: Some(custom_info.selector),
            parameters,
            raw_data: error_data.to_vec(),
            source_location: None,
            suggestions: vec![
                format!("Check contract logic for {} error condition", custom_info.name),
                "Review error parameters for context".to_string(),
            ],
        })
    }

    /// Decode error parameters from ABI-encoded data
    fn decode_error_parameters(&self, data: &[u8], parameter_specs: &[ErrorParameter]) -> Result<Vec<DecodedParameter>> {
        let mut parameters = Vec::new();
        let mut offset = 0;

        for spec in parameter_specs {
            let (param, new_offset) = self.decode_single_parameter(data, offset, spec)?;
            parameters.push(param);
            offset = new_offset;
        }

        Ok(parameters)
    }

    /// Decode a single parameter
    fn decode_single_parameter(&self, data: &[u8], offset: usize, spec: &ErrorParameter) -> Result<(DecodedParameter, usize)> {
        // Simplified parameter decoding - would implement full ABI decoding
        match spec.type_name.as_str() {
            "uint256" => {
                if data.len() < offset + 32 {
                    anyhow::bail!("Insufficient data for uint256");
                }
                
                let mut bytes = [0u8; 32];
                bytes.copy_from_slice(&data[offset..offset + 32]);
                let value = u64::from_be_bytes([
                    bytes[24], bytes[25], bytes[26], bytes[27],
                    bytes[28], bytes[29], bytes[30], bytes[31]
                ]);

                Ok((DecodedParameter {
                    name: spec.name.clone(),
                    type_name: spec.type_name.clone(),
                    value: ParameterValue::Uint(value),
                    display_value: value.to_string(),
                }, offset + 32))
            }
            "address" => {
                if data.len() < offset + 32 {
                    anyhow::bail!("Insufficient data for address");
                }
                
                let address_bytes = &data[offset + 12..offset + 32];
                let address = format!("0x{}", hex::encode(address_bytes));

                Ok((DecodedParameter {
                    name: spec.name.clone(),
                    type_name: spec.type_name.clone(),
                    value: ParameterValue::Address(address.clone()),
                    display_value: address,
                }, offset + 32))
            }
            "bool" => {
                if data.len() < offset + 32 {
                    anyhow::bail!("Insufficient data for bool");
                }
                
                let value = data[offset + 31] != 0;

                Ok((DecodedParameter {
                    name: spec.name.clone(),
                    type_name: spec.type_name.clone(),
                    value: ParameterValue::Bool(value),
                    display_value: value.to_string(),
                }, offset + 32))
            }
            _ => {
                // Default to bytes for unknown types
                let bytes = if data.len() >= offset + 32 {
                    data[offset..offset + 32].to_vec()
                } else {
                    data[offset..].to_vec()
                };

                Ok((DecodedParameter {
                    name: spec.name.clone(),
                    type_name: spec.type_name.clone(),
                    value: ParameterValue::Bytes(bytes.clone()),
                    display_value: format!("0x{}", hex::encode(&bytes)),
                }, offset + 32))
            }
        }
    }

    /// Parse error from ABI JSON item
    fn parse_error_from_abi_item(&self, item: &serde_json::Value) -> Result<CustomErrorInfo> {
        let name = item.get("name").and_then(|n| n.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing error name"))?;

        let inputs = item.get("inputs").and_then(|i| i.as_array())
            .unwrap_or(&vec![]);

        let mut parameters = Vec::new();
        let mut signature = format!("{}(", name);

        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                signature.push(',');
            }

            let param_type = input.get("type").and_then(|t| t.as_str())
                .ok_or_else(|| anyhow::anyhow!("Missing parameter type"))?;
            
            let param_name = input.get("name").and_then(|n| n.as_str())
                .unwrap_or(&format!("param{}", i));

            signature.push_str(param_type);

            parameters.push(ErrorParameter {
                name: param_name.to_string(),
                type_name: param_type.to_string(),
                indexed: false,
            });
        }

        signature.push(')');

        // Calculate selector from signature
        let selector = self.calculate_error_selector(&signature);

        Ok(CustomErrorInfo {
            selector,
            name: name.to_string(),
            signature,
            parameters,
            contract: None, // Would extract from context if available
        })
    }

    /// Calculate error selector from signature
    fn calculate_error_selector(&self, signature: &str) -> [u8; 4] {
        use sha3::{Digest, Keccak256};
        let hash = Keccak256::digest(signature.as_bytes());
        [hash[0], hash[1], hash[2], hash[3]]
    }

    /// Format decoded error for display
    fn format_error(&self, decoded: &DecodedError) -> String {
        let mut result = decoded.message.clone();

        if !decoded.parameters.is_empty() {
            result.push_str(" with parameters:");
            for param in &decoded.parameters {
                result.push_str(&format!("\n  {}: {} = {}", param.name, param.type_name, param.display_value));
            }
        }

        if !decoded.suggestions.is_empty() {
            result.push_str("\n\nSuggestions:");
            for suggestion in &decoded.suggestions {
                result.push_str(&format!("\n  • {}", suggestion));
            }
        }

        result
    }

    /// Get suggestions for panic errors
    fn get_panic_suggestions(&self, panic_info: &PanicInfo) -> Vec<String> {
        match panic_info.code {
            0x01 => vec![
                "Check assert() conditions - they should always be true".to_string(),
                "Review contract invariants and preconditions".to_string(),
            ],
            0x11 => vec![
                "Check arithmetic operations for overflow/underflow".to_string(),
                "Use SafeMath library or Solidity 0.8+ built-in checks".to_string(),
                "Verify input ranges are valid".to_string(),
            ],
            0x12 => vec![
                "Check for division by zero or modulo by zero".to_string(),
                "Validate denominators before arithmetic operations".to_string(),
            ],
            0x21 => vec![
                "Check enum values are within valid range".to_string(),
                "Verify type conversion is valid".to_string(),
            ],
            0x22 => vec![
                "Check storage encoding is correct".to_string(),
                "Verify storage layout matches expectations".to_string(),
            ],
            0x31 => vec![
                "Check array bounds - index may be out of range".to_string(),
                "Verify array length before accessing elements".to_string(),
            ],
            0x32 => vec![
                "Check array allocation size".to_string(),
                "Verify memory allocation is reasonable".to_string(),
            ],
            0x41 => vec![
                "Check for memory allocation issues".to_string(),
                "Verify contract has sufficient memory".to_string(),
            ],
            0x51 => vec![
                "Check for uninitialized function pointer calls".to_string(),
                "Verify function variables are properly initialized".to_string(),
            ],
            _ => vec![
                format!("Unknown panic code {} - check Solidity documentation", panic_info.code),
                "Review recent contract changes for potential issues".to_string(),
            ],
        }
    }

    /// Initialize standard panic codes
    fn initialize_panic_codes() -> HashMap<u32, PanicInfo> {
        let mut codes = HashMap::new();

        codes.insert(0x01, PanicInfo {
            code: 0x01,
            name: "Assertion Error".to_string(),
            description: "assert() condition failed".to_string(),
            category: PanicCategory::AssertionFailure,
        });

        codes.insert(0x11, PanicInfo {
            code: 0x11,
            name: "Arithmetic Overflow/Underflow".to_string(),
            description: "Arithmetic operation overflowed or underflowed".to_string(),
            category: PanicCategory::ArithmeticOverflow,
        });

        codes.insert(0x12, PanicInfo {
            code: 0x12,
            name: "Division by Zero".to_string(),
            description: "Division or modulo by zero".to_string(),
            category: PanicCategory::DivisionByZero,
        });

        codes.insert(0x21, PanicInfo {
            code: 0x21,
            name: "Invalid Enum Value".to_string(),
            description: "Converting value to enum type with invalid value".to_string(),
            category: PanicCategory::Internal,
        });

        codes.insert(0x22, PanicInfo {
            code: 0x22,
            name: "Invalid Storage Encoding".to_string(),
            description: "Incorrectly encoded storage byte array accessed".to_string(),
            category: PanicCategory::Storage,
        });

        codes.insert(0x31, PanicInfo {
            code: 0x31,
            name: "Array Index Out of Bounds".to_string(),
            description: "Array index is out of bounds".to_string(),
            category: PanicCategory::ArrayBounds,
        });

        codes.insert(0x32, PanicInfo {
            code: 0x32,
            name: "Array Too Large".to_string(),
            description: "Attempting to allocate too large array".to_string(),
            category: PanicCategory::ArrayBounds,
        });

        codes.insert(0x41, PanicInfo {
            code: 0x41,
            name: "Out of Memory".to_string(),
            description: "Too much memory allocated".to_string(),
            category: PanicCategory::Internal,
        });

        codes.insert(0x51, PanicInfo {
            code: 0x51,
            name: "Uninitialized Function".to_string(),
            description: "Calling uninitialized internal function variable".to_string(),
            category: PanicCategory::Internal,
        });

        codes
    }

    /// Initialize builtin error selectors
    fn initialize_builtin_errors() -> HashMap<[u8; 4], BuiltinErrorInfo> {
        let mut errors = HashMap::new();

        errors.insert([0x08, 0xc3, 0x79, 0xa0], BuiltinErrorInfo {
            selector: [0x08, 0xc3, 0x79, 0xa0],
            name: "Error".to_string(),
            description: "Standard string error from require() or revert()".to_string(),
        });

        errors.insert([0x4e, 0x48, 0x7b, 0x71], BuiltinErrorInfo {
            selector: [0x4e, 0x48, 0x7b, 0x71],
            name: "Panic".to_string(),
            description: "Panic error with error code".to_string(),
        });

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_decoder_creation() {
        let decoder = ErrorDecoder::new();
        assert!(decoder.is_ok());
    }

    #[test]
    fn test_panic_error_decoding() {
        let decoder = ErrorDecoder::new().unwrap();
        
        // Panic error with code 0x11 (arithmetic overflow)
        let error_data = [
            0x4e, 0x48, 0x7b, 0x71, // Panic selector
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, // Panic code 0x11
        ];

        let result = decoder.decode_error_detailed(&error_data).unwrap();
        
        match result.error_type {
            DecodedErrorType::Panic { code, .. } => {
                assert_eq!(code, 0x11);
            }
            _ => panic!("Expected panic error"),
        }
    }

    #[test]
    fn test_string_error_decoding() {
        let decoder = ErrorDecoder::new().unwrap();
        
        // Error(string) with message "Test error"
        let mut error_data = vec![
            0x08, 0xc3, 0x79, 0xa0, // Error(string) selector
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, // Offset
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, // Length (10 bytes)
        ];
        
        error_data.extend_from_slice(b"Test error");
        error_data.resize((error_data.len() + 31) & !31, 0); // Pad to 32-byte boundary

        let result = decoder.decode_error_detailed(&error_data).unwrap();
        
        match result.error_type {
            DecodedErrorType::Revert { reason: Some(msg) } => {
                assert_eq!(msg, "Test error");
            }
            _ => panic!("Expected revert error with reason"),
        }
    }

    #[test]
    fn test_custom_error_registration() {
        let mut decoder = ErrorDecoder::new().unwrap();
        
        let custom_error = CustomErrorInfo {
            selector: [0x12, 0x34, 0x56, 0x78],
            name: "TestError".to_string(),
            signature: "TestError(uint256)".to_string(),
            parameters: vec![
                ErrorParameter {
                    name: "value".to_string(),
                    type_name: "uint256".to_string(),
                    indexed: false,
                }
            ],
            contract: Some("TestContract".to_string()),
        };

        decoder.register_custom_error(custom_error);
        
        assert!(decoder.custom_errors.contains_key(&[0x12, 0x34, 0x56, 0x78]));
    }

    #[test]
    fn test_empty_error_data() {
        let decoder = ErrorDecoder::new().unwrap();
        let result = decoder.decode_error_detailed(&[]).unwrap();
        
        match result.error_type {
            DecodedErrorType::Unknown { selector: None } => assert!(true),
            _ => panic!("Expected unknown error with no selector"),
        }
    }
}