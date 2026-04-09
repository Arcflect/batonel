#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationSeverity {
    Error,
    Warn,
}

#[derive(Debug, Clone)]
pub struct Violation {
    pub rule_id: String,
    pub severity: ViolationSeverity,
    pub target: String,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    pub violations: Vec<Violation>,
}

impl ValidationResult {
    pub fn push(&mut self, violation: Violation) {
        self.violations.push(violation);
    }

    pub fn error_count(&self) -> usize {
        self.violations
            .iter()
            .filter(|v| v.severity == ViolationSeverity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.violations
            .iter()
            .filter(|v| v.severity == ViolationSeverity::Warn)
            .count()
    }

    pub fn is_valid(&self) -> bool {
        self.error_count() == 0
    }
}
