/// Result of semantic analysis
#[derive(Debug, Clone)]
pub struct SemanticResult {
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
    pub errors: Vec<String>,
    pub complexity_metrics: ComplexityMetrics,
    pub security_issues: Vec<SecurityIssue>,
    pub performance_metrics: PerformanceMetrics,
}

impl SemanticResult {
    /// Check if analysis found any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Check if analysis found any security issues
    pub fn has_security_issues(&self) -> bool {
        !self.security_issues.is_empty()
    }

    /// Get count of high/critical security issues
    pub fn critical_issue_count(&self) -> usize {
        self.security_issues
            .iter()
            .filter(|i| matches!(i.severity, Severity::High | Severity::Critical))
            .count()
    }

    /// Get total issue count
    pub fn total_issues(&self) -> usize {
        self.errors.len() + self.warnings.len() + self.security_issues.len()
    }
}

/// Code complexity metrics
#[derive(Debug, Clone, Default)]
pub struct ComplexityMetrics {
    pub cyclomatic: u32,
    pub function_count: u32,
    pub max_nesting_depth: u32,
}

impl ComplexityMetrics {
    /// Check if complexity is within acceptable limits
    pub fn is_acceptable(&self) -> bool {
        self.cyclomatic <= 15 && self.max_nesting_depth <= 5
    }

    /// Get complexity rating
    pub fn rating(&self) -> &'static str {
        match self.cyclomatic {
            0..=5 => "Simple",
            6..=10 => "Moderate",
            11..=20 => "Complex",
            _ => "Very Complex",
        }
    }
}

/// Security issue found during analysis
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub message: String,
    pub severity: Severity,
}

impl SecurityIssue {
    /// Create a new security issue
    pub fn new(message: impl Into<String>, severity: Severity) -> Self {
        Self {
            message: message.into(),
            severity,
        }
    }
}

/// Severity level for security issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    /// Get severity as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

/// Performance metrics from analysis
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    pub estimated_gas: u64,
    pub hot_paths: Vec<String>,
    pub optimization_opportunities: Vec<String>,
}

impl PerformanceMetrics {
    /// Check if there are optimization opportunities
    pub fn has_opportunities(&self) -> bool {
        !self.optimization_opportunities.is_empty()
    }
}

#[derive(Default)]
pub struct SemanticAnalyzer;

