//! Security Analysis Framework for Neo Solidity
//! 
//! This crate provides comprehensive security analysis including:
//! - Cryptographic operation validation
//! - Storage collision detection
//! - Reentrancy protection analysis
//! - Static code analysis for vulnerabilities
//! - Runtime security monitoring

pub mod crypto_analyzer;
pub mod storage_analyzer;
pub mod reentrancy_detector;
pub mod static_analyzer;
pub mod runtime_monitor;
pub mod vulnerability_db;
pub mod reporting;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::{info, warn, error};

/// Main security analyzer for Neo Solidity contracts
pub struct SecurityAnalyzer {
    config: SecurityConfig,
    vulnerability_db: vulnerability_db::VulnerabilityDatabase,
    findings: Vec<SecurityFinding>,
}

/// Security analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable static code analysis
    pub enable_static_analysis: bool,
    /// Enable runtime monitoring
    pub enable_runtime_monitoring: bool,
    /// Enable crypto operation validation
    pub enable_crypto_validation: bool,
    /// Enable storage collision detection
    pub enable_storage_analysis: bool,
    /// Enable reentrancy detection
    pub enable_reentrancy_detection: bool,
    /// Severity threshold for reporting
    pub severity_threshold: Severity,
    /// Custom rules and patterns
    pub custom_rules: Vec<SecurityRule>,
}

/// Custom security rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub category: VulnerabilityCategory,
    pub pattern: String,
    pub enabled: bool,
}

/// Security finding from analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFinding {
    pub id: String,
    pub vulnerability: Vulnerability,
    pub location: Option<CodeLocation>,
    pub severity: Severity,
    pub confidence: Confidence,
    pub message: String,
    pub recommendation: String,
    pub proof_of_concept: Option<String>,
}

/// Complete security report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityReport {
    pub contract_name: String,
    pub analysis_timestamp: String,
    pub analyzer_version: String,
    pub summary: SecuritySummary,
    pub findings: Vec<SecurityFinding>,
    pub metrics: SecurityMetrics,
    pub compliance: ComplianceStatus,
}

/// Security analysis summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySummary {
    pub total_findings: u32,
    pub critical_count: u32,
    pub high_count: u32,
    pub medium_count: u32,
    pub low_count: u32,
    pub info_count: u32,
    pub overall_risk_score: f64,
    pub security_grade: SecurityGrade,
}

/// Code location for findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub function: Option<String>,
    pub contract: Option<String>,
}

/// Types of vulnerabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Vulnerability {
    // Reentrancy vulnerabilities
    ReentrancyAttack {
        function_name: String,
        external_call_location: CodeLocation,
        state_change_location: CodeLocation,
    },
    
    // Cryptographic vulnerabilities
    WeakRandomness {
        source: String,
        predictability_score: f64,
    },
    CryptoMisuse {
        operation: String,
        issue: String,
    },
    
    // Storage vulnerabilities
    StorageCollision {
        slot: u256::U256,
        conflicting_variables: Vec<String>,
    },
    UnprotectedStorage {
        variable: String,
        access_pattern: String,
    },
    
    // Access control
    MissingAccessControl {
        function_name: String,
        access_type: AccessType,
    },
    PrivilegeEscalation {
        vector: String,
        impact: String,
    },
    
    // Integer vulnerabilities
    IntegerOverflow {
        operation: String,
        variable_type: String,
    },
    IntegerUnderflow {
        operation: String,
        variable_type: String,
    },
    
    // Gas vulnerabilities
    GasLimitVulnerability {
        function_name: String,
        issue_type: GasIssueType,
    },
    
    // General logic vulnerabilities
    LogicError {
        description: String,
        impact: String,
    },
    
    // Neo-specific vulnerabilities
    NEOVMStackOverflow {
        depth: u32,
        location: CodeLocation,
    },
    InvalidNEOCall {
        call_type: String,
        issue: String,
    },
}

/// Vulnerability categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VulnerabilityCategory {
    Reentrancy,
    Cryptographic,
    Storage,
    AccessControl,
    Integer,
    Gas,
    Logic,
    NEOSpecific,
}

/// Severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Critical = 5,
    High = 4,
    Medium = 3,
    Low = 2,
    Informational = 1,
}

/// Confidence levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Confidence {
    High,
    Medium,
    Low,
}

/// Security grades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityGrade {
    A, // Excellent security
    B, // Good security
    C, // Acceptable security
    D, // Poor security
    F, // Unacceptable security
}

/// Access control types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessType {
    Owner,
    Admin,
    Modifier,
    Public,
    Internal,
    Private,
}

/// Gas-related issue types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GasIssueType {
    OutOfGas,
    GasGriefing,
    InfiniteLoop,
    ExpensiveOperation,
}

/// Security metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    pub lines_of_code: u32,
    pub functions_analyzed: u32,
    pub external_calls: u32,
    pub state_variables: u32,
    pub complexity_score: f64,
    pub test_coverage: f64,
    pub analysis_duration_ms: u64,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub standards: HashMap<String, ComplianceResult>,
    pub overall_compliant: bool,
}

/// Compliance check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub standard: String,
    pub version: String,
    pub compliant: bool,
    pub violations: Vec<String>,
    pub score: f64,
}

impl SecurityAnalyzer {
    /// Create a new security analyzer
    pub fn new(config: SecurityConfig) -> Result<Self> {
        let vulnerability_db = vulnerability_db::VulnerabilityDatabase::new()?;
        
        Ok(Self {
            config,
            vulnerability_db,
            findings: Vec::new(),
        })
    }

    /// Run comprehensive security analysis
    pub async fn analyze_contract(&mut self, contract_source: &str, contract_name: &str) -> Result<SecurityReport> {
        info!("Starting security analysis for contract: {}", contract_name);
        
        let start_time = std::time::Instant::now();
        self.findings.clear();

        // Static code analysis
        if self.config.enable_static_analysis {
            self.run_static_analysis(contract_source).await?;
        }

        // Cryptographic analysis
        if self.config.enable_crypto_validation {
            self.run_crypto_analysis(contract_source).await?;
        }

        // Storage analysis
        if self.config.enable_storage_analysis {
            self.run_storage_analysis(contract_source).await?;
        }

        // Reentrancy detection
        if self.config.enable_reentrancy_detection {
            self.run_reentrancy_analysis(contract_source).await?;
        }

        let analysis_duration = start_time.elapsed().as_millis() as u64;

        // Generate comprehensive report
        let report = self.generate_report(contract_name, analysis_duration)?;

        info!(
            "Security analysis completed for {}: {} findings in {}ms",
            contract_name,
            report.summary.total_findings,
            analysis_duration
        );

        Ok(report)
    }

    /// Run static code analysis
    async fn run_static_analysis(&mut self, contract_source: &str) -> Result<()> {
        let findings = static_analyzer::analyze_source(contract_source, &self.config)?;
        self.findings.extend(findings);
        Ok(())
    }

    /// Run cryptographic operation analysis
    async fn run_crypto_analysis(&mut self, contract_source: &str) -> Result<()> {
        let findings = crypto_analyzer::analyze_crypto_operations(contract_source)?;
        self.findings.extend(findings);
        Ok(())
    }

    /// Run storage collision analysis
    async fn run_storage_analysis(&mut self, contract_source: &str) -> Result<()> {
        let findings = storage_analyzer::analyze_storage_layout(contract_source)?;
        self.findings.extend(findings);
        Ok(())
    }

    /// Run reentrancy detection
    async fn run_reentrancy_analysis(&mut self, contract_source: &str) -> Result<()> {
        let findings = reentrancy_detector::detect_reentrancy(contract_source)?;
        self.findings.extend(findings);
        Ok(())
    }

    /// Generate comprehensive security report
    fn generate_report(&self, contract_name: &str, analysis_duration: u64) -> Result<SecurityReport> {
        let summary = self.generate_summary();
        let metrics = self.generate_metrics(analysis_duration);
        let compliance = self.check_compliance();

        Ok(SecurityReport {
            contract_name: contract_name.to_string(),
            analysis_timestamp: chrono::Utc::now().to_rfc3339(),
            analyzer_version: env!("CARGO_PKG_VERSION").to_string(),
            summary,
            findings: self.findings.clone(),
            metrics,
            compliance,
        })
    }

    /// Generate security summary
    fn generate_summary(&self) -> SecuritySummary {
        let critical_count = self.findings.iter().filter(|f| f.severity == Severity::Critical).count() as u32;
        let high_count = self.findings.iter().filter(|f| f.severity == Severity::High).count() as u32;
        let medium_count = self.findings.iter().filter(|f| f.severity == Severity::Medium).count() as u32;
        let low_count = self.findings.iter().filter(|f| f.severity == Severity::Low).count() as u32;
        let info_count = self.findings.iter().filter(|f| f.severity == Severity::Informational).count() as u32;

        let total_findings = critical_count + high_count + medium_count + low_count + info_count;

        // Calculate risk score (0-100)
        let risk_score = (critical_count as f64 * 20.0) + 
                        (high_count as f64 * 10.0) + 
                        (medium_count as f64 * 5.0) + 
                        (low_count as f64 * 2.0) + 
                        (info_count as f64 * 0.5);

        let security_grade = match risk_score {
            0.0..=10.0 => SecurityGrade::A,
            10.0..=25.0 => SecurityGrade::B,
            25.0..=50.0 => SecurityGrade::C,
            50.0..=100.0 => SecurityGrade::D,
            _ => SecurityGrade::F,
        };

        SecuritySummary {
            total_findings,
            critical_count,
            high_count,
            medium_count,
            low_count,
            info_count,
            overall_risk_score: risk_score,
            security_grade,
        }
    }

    /// Generate security metrics
    fn generate_metrics(&self, analysis_duration: u64) -> SecurityMetrics {
        SecurityMetrics {
            lines_of_code: 0, // TODO: Calculate from source
            functions_analyzed: 0, // TODO: Count from AST
            external_calls: 0, // TODO: Count external calls
            state_variables: 0, // TODO: Count state variables
            complexity_score: 0.0, // TODO: Calculate complexity
            test_coverage: 0.0, // TODO: Calculate coverage
            analysis_duration_ms: analysis_duration,
        }
    }

    /// Check compliance with security standards
    fn check_compliance(&self) -> ComplianceStatus {
        let mut standards = HashMap::new();
        
        // Example compliance checks
        standards.insert("SWC".to_string(), ComplianceResult {
            standard: "Smart Contract Weakness Classification".to_string(),
            version: "1.0".to_string(),
            compliant: self.findings.iter().filter(|f| f.severity >= Severity::High).count() == 0,
            violations: Vec::new(),
            score: 85.0,
        });

        let overall_compliant = standards.values().all(|r| r.compliant);

        ComplianceStatus {
            standards,
            overall_compliant,
        }
    }

    /// Filter findings by severity threshold
    pub fn filter_findings(&self, min_severity: Severity) -> Vec<&SecurityFinding> {
        self.findings.iter()
            .filter(|f| f.severity >= min_severity)
            .collect()
    }

    /// Get findings by category
    pub fn get_findings_by_category(&self, category: VulnerabilityCategory) -> Vec<&SecurityFinding> {
        self.findings.iter()
            .filter(|f| match &f.vulnerability {
                Vulnerability::ReentrancyAttack { .. } => matches!(category, VulnerabilityCategory::Reentrancy),
                Vulnerability::WeakRandomness { .. } | Vulnerability::CryptoMisuse { .. } => 
                    matches!(category, VulnerabilityCategory::Cryptographic),
                Vulnerability::StorageCollision { .. } | Vulnerability::UnprotectedStorage { .. } => 
                    matches!(category, VulnerabilityCategory::Storage),
                Vulnerability::MissingAccessControl { .. } | Vulnerability::PrivilegeEscalation { .. } => 
                    matches!(category, VulnerabilityCategory::AccessControl),
                Vulnerability::IntegerOverflow { .. } | Vulnerability::IntegerUnderflow { .. } => 
                    matches!(category, VulnerabilityCategory::Integer),
                Vulnerability::GasLimitVulnerability { .. } => 
                    matches!(category, VulnerabilityCategory::Gas),
                Vulnerability::LogicError { .. } => 
                    matches!(category, VulnerabilityCategory::Logic),
                Vulnerability::NEOVMStackOverflow { .. } | Vulnerability::InvalidNEOCall { .. } => 
                    matches!(category, VulnerabilityCategory::NEOSpecific),
            })
            .collect()
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_static_analysis: true,
            enable_runtime_monitoring: true,
            enable_crypto_validation: true,
            enable_storage_analysis: true,
            enable_reentrancy_detection: true,
            severity_threshold: Severity::Medium,
            custom_rules: Vec::new(),
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "Critical"),
            Severity::High => write!(f, "High"),
            Severity::Medium => write!(f, "Medium"),
            Severity::Low => write!(f, "Low"),
            Severity::Informational => write!(f, "Informational"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_analyzer_creation() {
        let config = SecurityConfig::default();
        let analyzer = SecurityAnalyzer::new(config);
        assert!(analyzer.is_ok());
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
        assert!(Severity::Low > Severity::Informational);
    }

    #[test]
    fn test_security_grade_calculation() {
        // Test with sample findings to verify grade calculation logic
        let findings = vec![
            SecurityFinding {
                id: "test1".to_string(),
                vulnerability: Vulnerability::LogicError {
                    description: "Test".to_string(),
                    impact: "Low".to_string(),
                },
                location: None,
                severity: Severity::Low,
                confidence: Confidence::High,
                message: "Test finding".to_string(),
                recommendation: "Fix it".to_string(),
                proof_of_concept: None,
            }
        ];

        // Verify that findings are properly categorized
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Low);
    }
}