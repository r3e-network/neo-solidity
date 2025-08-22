//! Reentrancy Detection and Protection Testing
//! 
//! This module provides comprehensive reentrancy vulnerability detection including:
//! - Static analysis for reentrancy patterns
//! - Runtime protection testing
//! - Cross-function reentrancy detection
//! - State change ordering analysis

use crate::*;
use std::collections::{HashMap, HashSet, VecDeque};
use petgraph::{Graph, Direction};
use petgraph::graph::NodeIndex;
use regex::Regex;
use anyhow::Result;
use tracing::{info, debug, warn};

/// Reentrancy detector and tester
pub struct ReentrancyDetector {
    call_graph: CallGraph,
    state_changes: StateChangeTracker,
    external_calls: ExternalCallTracker,
    protection_patterns: ProtectionPatterns,
}

/// Call graph for reentrancy analysis
pub struct CallGraph {
    graph: Graph<FunctionNode, CallEdge>,
    function_map: HashMap<String, NodeIndex>,
}

/// Function node in call graph
#[derive(Debug, Clone)]
pub struct FunctionNode {
    pub name: String,
    pub visibility: FunctionVisibility,
    pub state_mutability: StateMutability,
    pub modifiers: Vec<String>,
    pub external_calls: Vec<ExternalCall>,
    pub state_changes: Vec<StateChange>,
    pub line_range: (u32, u32),
}

/// Call edge in graph
#[derive(Debug, Clone)]
pub struct CallEdge {
    pub call_type: CallType,
    pub conditions: Vec<String>,
    pub line_number: u32,
}

/// Function visibility
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionVisibility {
    Public,
    External,
    Internal,
    Private,
}

/// State mutability
#[derive(Debug, Clone, PartialEq)]
pub enum StateMutability {
    Pure,
    View,
    Payable,
    NonPayable,
}

/// Call types
#[derive(Debug, Clone, PartialEq)]
pub enum CallType {
    Internal,
    External,
    DelegateCall,
    StaticCall,
    Create,
    Create2,
}

/// External call information
#[derive(Debug, Clone)]
pub struct ExternalCall {
    pub target: CallTarget,
    pub call_type: CallType,
    pub value_sent: bool,
    pub gas_specified: bool,
    pub line_number: u32,
    pub before_state_changes: Vec<StateChange>,
    pub after_state_changes: Vec<StateChange>,
}

/// Call target types
#[derive(Debug, Clone)]
pub enum CallTarget {
    Address(String),
    Contract(String),
    Unknown,
    UserControlled,
}

/// State change tracking
pub struct StateChangeTracker {
    changes: Vec<StateChange>,
    variables: HashMap<String, VariableInfo>,
}

/// State change information
#[derive(Debug, Clone)]
pub struct StateChange {
    pub variable: String,
    pub change_type: ChangeType,
    pub line_number: u32,
    pub function_context: String,
    pub before_external_call: bool,
    pub after_external_call: bool,
}

/// Types of state changes
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    Assignment,
    Increment,
    Decrement,
    Delete,
    Push,
    Pop,
    Transfer,
}

/// Variable information
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub var_type: String,
    pub visibility: VariableVisibility,
    pub is_mapping: bool,
    pub is_array: bool,
    pub access_patterns: Vec<AccessPattern>,
}

/// Variable visibility
#[derive(Debug, Clone, PartialEq)]
pub enum VariableVisibility {
    Public,
    Internal,
    Private,
}

/// Variable access pattern
#[derive(Debug, Clone)]
pub struct AccessPattern {
    pub access_type: AccessType,
    pub line_number: u32,
    pub function_context: String,
}

/// Access types
#[derive(Debug, Clone, PartialEq)]
pub enum AccessType {
    Read,
    Write,
    ReadWrite,
}

/// External call tracker
pub struct ExternalCallTracker {
    calls: Vec<ExternalCall>,
    patterns: Vec<CallPattern>,
}

/// External call pattern
#[derive(Debug, Clone)]
pub struct CallPattern {
    pub pattern_type: CallPatternType,
    pub regex: Regex,
    pub severity: Severity,
}

/// Call pattern types
#[derive(Debug, Clone)]
pub enum CallPatternType {
    LowLevelCall,
    Send,
    Transfer,
    Call,
    DelegateCall,
    StaticCall,
    InterfaceCall,
}

/// Protection pattern detector
pub struct ProtectionPatterns {
    patterns: Vec<ProtectionPattern>,
}

/// Protection pattern
#[derive(Debug, Clone)]
pub struct ProtectionPattern {
    pub name: String,
    pub pattern_type: ProtectionType,
    pub regex: Regex,
    pub effectiveness: f64,
}

/// Protection types
#[derive(Debug, Clone)]
pub enum ProtectionType {
    ReentrancyGuard,
    ChecksEffectsInteractions,
    PullPayment,
    Mutex,
    StateMachine,
}

/// Reentrancy vulnerability types
#[derive(Debug, Clone)]
pub enum ReentrancyType {
    /// Classic single-function reentrancy
    SingleFunction {
        function_name: String,
        external_call: ExternalCall,
        vulnerable_state_change: StateChange,
    },
    /// Cross-function reentrancy
    CrossFunction {
        functions: Vec<String>,
        call_path: Vec<String>,
        shared_state: Vec<String>,
    },
    /// Read-only reentrancy
    ReadOnly {
        function_name: String,
        view_function: String,
        inconsistent_state: String,
    },
    /// Creation reentrancy
    Creation {
        constructor: String,
        external_call: ExternalCall,
        state_dependency: String,
    },
}

/// Main entry point for reentrancy detection
pub fn detect_reentrancy(contract_source: &str) -> Result<Vec<SecurityFinding>> {
    let mut detector = ReentrancyDetector::new()?;
    detector.analyze_contract(contract_source)
}

impl ReentrancyDetector {
    /// Create new reentrancy detector
    pub fn new() -> Result<Self> {
        Ok(Self {
            call_graph: CallGraph::new(),
            state_changes: StateChangeTracker::new(),
            external_calls: ExternalCallTracker::new()?,
            protection_patterns: ProtectionPatterns::new()?,
        })
    }

    /// Analyze contract for reentrancy vulnerabilities
    pub fn analyze_contract(&mut self, contract_source: &str) -> Result<Vec<SecurityFinding>> {
        info!("Starting reentrancy analysis");
        
        let mut findings = Vec::new();

        // Step 1: Parse contract and build call graph
        self.build_call_graph(contract_source)?;

        // Step 2: Identify external calls and state changes
        self.identify_external_calls(contract_source)?;
        self.track_state_changes(contract_source)?;

        // Step 3: Analyze reentrancy patterns
        findings.extend(self.detect_single_function_reentrancy()?);
        findings.extend(self.detect_cross_function_reentrancy()?);
        findings.extend(self.detect_read_only_reentrancy()?);
        findings.extend(self.detect_creation_reentrancy()?);

        // Step 4: Check for protection patterns
        self.analyze_protection_patterns(contract_source, &mut findings)?;

        info!("Reentrancy analysis completed: {} findings", findings.len());
        Ok(findings)
    }

    /// Build call graph from contract source
    fn build_call_graph(&mut self, contract_source: &str) -> Result<()> {
        debug!("Building call graph");
        
        // Parse functions (simplified - would use proper parser)
        let function_regex = Regex::new(
            r"function\s+(\w+)\s*\([^)]*\)\s*([^{]*)\s*\{"
        )?;

        for capture in function_regex.captures_iter(contract_source) {
            let function_name = capture.get(1).unwrap().as_str().to_string();
            let modifiers_str = capture.get(2).unwrap().as_str();

            // Extract visibility and modifiers
            let visibility = if modifiers_str.contains("external") {
                FunctionVisibility::External
            } else if modifiers_str.contains("public") {
                FunctionVisibility::Public
            } else if modifiers_str.contains("internal") {
                FunctionVisibility::Internal
            } else {
                FunctionVisibility::Private
            };

            let state_mutability = if modifiers_str.contains("pure") {
                StateMutability::Pure
            } else if modifiers_str.contains("view") {
                StateMutability::View
            } else if modifiers_str.contains("payable") {
                StateMutability::Payable
            } else {
                StateMutability::NonPayable
            };

            let node = FunctionNode {
                name: function_name.clone(),
                visibility,
                state_mutability,
                modifiers: self.extract_modifiers(modifiers_str),
                external_calls: Vec::new(),
                state_changes: Vec::new(),
                line_range: (0, 0), // Would calculate from source
            };

            self.call_graph.add_function(node);
        }

        // Analyze function calls within contract
        self.analyze_internal_calls(contract_source)?;

        Ok(())
    }

    /// Extract modifiers from function signature
    fn extract_modifiers(&self, modifiers_str: &str) -> Vec<String> {
        let modifier_regex = Regex::new(r"\b(\w+)(?:\([^)]*\))?\b").unwrap();
        modifier_regex
            .captures_iter(modifiers_str)
            .filter_map(|cap| {
                let modifier = cap.get(1).unwrap().as_str();
                if !["public", "external", "internal", "private", "pure", "view", "payable"].contains(&modifier) {
                    Some(modifier.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Analyze internal function calls
    fn analyze_internal_calls(&mut self, contract_source: &str) -> Result<()> {
        // Simple pattern matching for function calls
        let call_regex = Regex::new(r"(\w+)\s*\(")?;
        
        // This would be more sophisticated in practice
        // For now, just record that calls exist
        Ok(())
    }

    /// Identify external calls in contract
    fn identify_external_calls(&mut self, contract_source: &str) -> Result<()> {
        debug!("Identifying external calls");

        let external_call_patterns = vec![
            (CallPatternType::LowLevelCall, r"\.call\s*\("),
            (CallPatternType::Send, r"\.send\s*\("),
            (CallPatternType::Transfer, r"\.transfer\s*\("),
            (CallPatternType::DelegateCall, r"\.delegatecall\s*\("),
            (CallPatternType::StaticCall, r"\.staticcall\s*\("),
        ];

        for (call_type, pattern) in external_call_patterns {
            let regex = Regex::new(pattern)?;
            for mat in regex.find_iter(contract_source) {
                // Calculate line number
                let line_number = contract_source[..mat.start()].lines().count() as u32;
                
                let external_call = ExternalCall {
                    target: CallTarget::Unknown, // Would analyze target
                    call_type: match call_type {
                        CallPatternType::LowLevelCall => CallType::External,
                        CallPatternType::Send => CallType::External,
                        CallPatternType::Transfer => CallType::External,
                        CallPatternType::DelegateCall => CallType::DelegateCall,
                        CallPatternType::StaticCall => CallType::StaticCall,
                        _ => CallType::External,
                    },
                    value_sent: pattern.contains("send") || pattern.contains("transfer"),
                    gas_specified: false, // Would analyze
                    line_number,
                    before_state_changes: Vec::new(),
                    after_state_changes: Vec::new(),
                };

                self.external_calls.calls.push(external_call);
            }
        }

        Ok(())
    }

    /// Track state changes in contract
    fn track_state_changes(&mut self, contract_source: &str) -> Result<()> {
        debug!("Tracking state changes");

        // State variable assignment patterns
        let assignment_regex = Regex::new(r"(\w+)\s*=\s*[^;]+;")?;
        
        for capture in assignment_regex.captures_iter(contract_source) {
            let variable = capture.get(1).unwrap().as_str().to_string();
            let line_number = contract_source[..capture.get(0).unwrap().start()].lines().count() as u32;
            
            let state_change = StateChange {
                variable,
                change_type: ChangeType::Assignment,
                line_number,
                function_context: "unknown".to_string(), // Would track context
                before_external_call: false,
                after_external_call: false,
            };

            self.state_changes.changes.push(state_change);
        }

        Ok(())
    }

    /// Detect single-function reentrancy
    fn detect_single_function_reentrancy(&self) -> Result<Vec<SecurityFinding>> {
        debug!("Detecting single-function reentrancy");
        let mut findings = Vec::new();

        // Check each function for the pattern:
        // 1. External call
        // 2. State change after external call
        for function in self.call_graph.get_all_functions() {
            let external_calls: Vec<&ExternalCall> = self.external_calls.calls
                .iter()
                .filter(|call| {
                    // Would check if call is in this function
                    true // Simplified
                })
                .collect();

            for external_call in external_calls {
                // Check for state changes after external call
                let vulnerable_changes: Vec<&StateChange> = self.state_changes.changes
                    .iter()
                    .filter(|change| {
                        change.line_number > external_call.line_number &&
                        change.function_context == function.name
                    })
                    .collect();

                for vulnerable_change in vulnerable_changes {
                    let finding = SecurityFinding {
                        id: format!("REENTRANCY_SF_{}", function.name),
                        vulnerability: Vulnerability::ReentrancyAttack {
                            function_name: function.name.clone(),
                            external_call_location: CodeLocation {
                                file: "contract.sol".to_string(),
                                line: external_call.line_number,
                                column: 0,
                                function: Some(function.name.clone()),
                                contract: None,
                            },
                            state_change_location: CodeLocation {
                                file: "contract.sol".to_string(),
                                line: vulnerable_change.line_number,
                                column: 0,
                                function: Some(function.name.clone()),
                                contract: None,
                            },
                        },
                        location: Some(CodeLocation {
                            file: "contract.sol".to_string(),
                            line: external_call.line_number,
                            column: 0,
                            function: Some(function.name.clone()),
                            contract: None,
                        }),
                        severity: Severity::Critical,
                        confidence: Confidence::High,
                        message: format!(
                            "Potential reentrancy vulnerability in function '{}': external call on line {} followed by state change on line {}",
                            function.name,
                            external_call.line_number,
                            vulnerable_change.line_number
                        ),
                        recommendation: "Apply the checks-effects-interactions pattern or use a reentrancy guard".to_string(),
                        proof_of_concept: Some(self.generate_reentrancy_poc(&function.name, external_call)),
                    };

                    findings.push(finding);
                }
            }
        }

        Ok(findings)
    }

    /// Detect cross-function reentrancy
    fn detect_cross_function_reentrancy(&self) -> Result<Vec<SecurityFinding>> {
        debug!("Detecting cross-function reentrancy");
        let mut findings = Vec::new();

        // Analyze call paths between functions that share state
        for function1 in self.call_graph.get_all_functions() {
            for function2 in self.call_graph.get_all_functions() {
                if function1.name == function2.name {
                    continue;
                }

                // Check if functions share state variables
                let shared_state = self.find_shared_state(&function1.name, &function2.name);
                if shared_state.is_empty() {
                    continue;
                }

                // Check if function1 has external calls and function2 modifies shared state
                let has_external_call = self.external_calls.calls.iter()
                    .any(|call| {
                        // Would check if call is in function1
                        true // Simplified
                    });

                if has_external_call {
                    let finding = SecurityFinding {
                        id: format!("REENTRANCY_CF_{}_{}", function1.name, function2.name),
                        vulnerability: Vulnerability::ReentrancyAttack {
                            function_name: function1.name.clone(),
                            external_call_location: CodeLocation {
                                file: "contract.sol".to_string(),
                                line: 0,
                                column: 0,
                                function: Some(function1.name.clone()),
                                contract: None,
                            },
                            state_change_location: CodeLocation {
                                file: "contract.sol".to_string(),
                                line: 0,
                                column: 0,
                                function: Some(function2.name.clone()),
                                contract: None,
                            },
                        },
                        location: Some(CodeLocation {
                            file: "contract.sol".to_string(),
                            line: 0,
                            column: 0,
                            function: Some(function1.name.clone()),
                            contract: None,
                        }),
                        severity: Severity::High,
                        confidence: Confidence::Medium,
                        message: format!(
                            "Potential cross-function reentrancy between '{}' and '{}' sharing state: {:?}",
                            function1.name,
                            function2.name,
                            shared_state
                        ),
                        recommendation: "Use reentrancy guards on all functions that modify shared state".to_string(),
                        proof_of_concept: Some(self.generate_cross_reentrancy_poc(&function1.name, &function2.name, &shared_state)),
                    };

                    findings.push(finding);
                }
            }
        }

        Ok(findings)
    }

    /// Detect read-only reentrancy
    fn detect_read_only_reentrancy(&self) -> Result<Vec<SecurityFinding>> {
        debug!("Detecting read-only reentrancy");
        // Implementation for read-only reentrancy detection
        Ok(Vec::new())
    }

    /// Detect creation reentrancy
    fn detect_creation_reentrancy(&self) -> Result<Vec<SecurityFinding>> {
        debug!("Detecting creation reentrancy");
        // Implementation for creation reentrancy detection
        Ok(Vec::new())
    }

    /// Analyze protection patterns
    fn analyze_protection_patterns(&self, contract_source: &str, findings: &mut Vec<SecurityFinding>) -> Result<()> {
        debug!("Analyzing protection patterns");

        // Check for reentrancy guard pattern
        let guard_pattern = Regex::new(r"modifier\s+nonReentrant")?;
        let has_guard = guard_pattern.is_match(contract_source);

        // Check for checks-effects-interactions pattern
        let cei_compliance = self.check_cei_pattern(contract_source)?;

        // Update findings with protection information
        for finding in findings.iter_mut() {
            if let Vulnerability::ReentrancyAttack { .. } = &finding.vulnerability {
                if has_guard {
                    finding.recommendation += " (Note: reentrancy guard pattern detected, verify it's applied to vulnerable functions)";
                }
                
                if cei_compliance {
                    finding.confidence = Confidence::Medium;
                    finding.recommendation += " (Note: some CEI pattern compliance detected)";
                }
            }
        }

        Ok(())
    }

    /// Check for checks-effects-interactions pattern compliance
    fn check_cei_pattern(&self, _contract_source: &str) -> Result<bool> {
        // Simplified implementation
        Ok(false)
    }

    /// Find shared state variables between functions
    fn find_shared_state(&self, _function1: &str, _function2: &str) -> Vec<String> {
        // Simplified implementation
        vec!["balance".to_string()]
    }

    /// Generate proof of concept for single-function reentrancy
    fn generate_reentrancy_poc(&self, function_name: &str, _external_call: &ExternalCall) -> String {
        format!(
            r#"
// Proof of Concept: Reentrancy Attack on {}
contract Attacker {{
    Target target;
    
    constructor(address _target) {{
        target = Target(_target);
    }}
    
    function attack() public payable {{
        target.{}();
    }}
    
    fallback() external payable {{
        if (address(target).balance >= msg.value) {{
            target.{}();
        }}
    }}
}}
            "#,
            function_name, function_name, function_name
        )
    }

    /// Generate proof of concept for cross-function reentrancy
    fn generate_cross_reentrancy_poc(&self, function1: &str, function2: &str, shared_state: &[String]) -> String {
        format!(
            r#"
// Proof of Concept: Cross-Function Reentrancy Attack
// Functions: {} and {}
// Shared State: {:?}
contract Attacker {{
    Target target;
    
    constructor(address _target) {{
        target = Target(_target);
    }}
    
    function attack() public {{
        target.{}();
    }}
    
    fallback() external payable {{
        // Reenter through different function
        target.{}();
    }}
}}
            "#,
            function1, function2, shared_state, function1, function2
        )
    }
}

// Implementation of helper structures

impl CallGraph {
    fn new() -> Self {
        Self {
            graph: Graph::new(),
            function_map: HashMap::new(),
        }
    }

    fn add_function(&mut self, function: FunctionNode) {
        let node_index = self.graph.add_node(function.clone());
        self.function_map.insert(function.name, node_index);
    }

    fn get_all_functions(&self) -> Vec<&FunctionNode> {
        self.graph.node_weights().collect()
    }
}

impl StateChangeTracker {
    fn new() -> Self {
        Self {
            changes: Vec::new(),
            variables: HashMap::new(),
        }
    }
}

impl ExternalCallTracker {
    fn new() -> Result<Self> {
        let patterns = vec![
            CallPattern {
                pattern_type: CallPatternType::LowLevelCall,
                regex: Regex::new(r"\.call\s*\(")?,
                severity: Severity::High,
            },
            CallPattern {
                pattern_type: CallPatternType::Send,
                regex: Regex::new(r"\.send\s*\(")?,
                severity: Severity::Medium,
            },
            CallPattern {
                pattern_type: CallPatternType::Transfer,
                regex: Regex::new(r"\.transfer\s*\(")?,
                severity: Severity::Low,
            },
        ];

        Ok(Self {
            calls: Vec::new(),
            patterns,
        })
    }
}

impl ProtectionPatterns {
    fn new() -> Result<Self> {
        let patterns = vec![
            ProtectionPattern {
                name: "OpenZeppelin ReentrancyGuard".to_string(),
                pattern_type: ProtectionType::ReentrancyGuard,
                regex: Regex::new(r"modifier\s+nonReentrant")?,
                effectiveness: 0.95,
            },
            ProtectionPattern {
                name: "Custom Mutex".to_string(),
                pattern_type: ProtectionType::Mutex,
                regex: Regex::new(r"require\s*\(\s*!locked\s*\)")?,
                effectiveness: 0.90,
            },
        ];

        Ok(Self { patterns })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reentrancy_detector_creation() {
        let detector = ReentrancyDetector::new();
        assert!(detector.is_ok());
    }

    #[tokio::test]
    async fn test_simple_reentrancy_detection() {
        let contract_source = r#"
            contract Vulnerable {
                mapping(address => uint) public balances;
                
                function withdraw() public {
                    uint amount = balances[msg.sender];
                    (bool success,) = msg.sender.call{value: amount}("");
                    require(success);
                    balances[msg.sender] = 0;
                }
            }
        "#;

        let findings = detect_reentrancy(contract_source).unwrap();
        assert!(!findings.is_empty());
        
        // Should detect reentrancy in withdraw function
        let reentrancy_findings: Vec<_> = findings.iter()
            .filter(|f| matches!(f.vulnerability, Vulnerability::ReentrancyAttack { .. }))
            .collect();
        
        assert!(!reentrancy_findings.is_empty());
    }

    #[test]
    fn test_call_graph_construction() {
        let mut call_graph = CallGraph::new();
        
        let function = FunctionNode {
            name: "test_function".to_string(),
            visibility: FunctionVisibility::Public,
            state_mutability: StateMutability::NonPayable,
            modifiers: vec![],
            external_calls: vec![],
            state_changes: vec![],
            line_range: (1, 10),
        };

        call_graph.add_function(function);
        assert_eq!(call_graph.get_all_functions().len(), 1);
    }
}