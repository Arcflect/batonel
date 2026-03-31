use serde::{Deserialize, Serialize};

/// Represents the subject of a verification check.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerifyTarget {
    /// Core project configuration files (e.g., project.arch.yaml)
    RootConfig { name: String, path: String },
    /// A logical artifact defined in the plan
    Artifact { name: String },
    /// A physical contract sidecar (.contract.yaml)
    Contract { artifact_name: String, path: String },
    /// A physical prompt sidecar (.prompt.md)
    Prompt { artifact_name: String, path: String },
    /// The actual implementation file (e.g., .rs, .go)
    SourceFile { artifact_name: String, path: String },
}

/// The outcome of a single verification check.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerifyStatus {
    /// Success: The check passed.
    Pass,
    /// Failure: A hard violation of the architectural model.
    Fail,
    /// Warning: A potential issue or soft violation.
    Warn,
    /// Skip: The check was not performed (e.g., prerequisite missing).
    Skip,
}

/// A single unit of validation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    /// Unique identifier for the check (e.g., "contract-exists")
    pub check_id: String,
    /// What was checked
    pub target: VerifyTarget,
    /// The outcome
    pub status: VerifyStatus,
    /// Human-readable diagnostic message
    pub message: String,
}

/// Aggregated statistics for a verification run.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifySummary {
    pub total: usize,
    pub passes: usize,
    pub failures: usize,
    pub warnings: usize,
    pub skips: usize,
}

/// The final report produced by a verification run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyReport {
    pub results: Vec<CheckResult>,
}

impl VerifyReport {
    pub fn new(results: Vec<CheckResult>) -> Self {
        VerifyReport { results }
    }

    pub fn summary(&self) -> VerifySummary {
        let mut summary = VerifySummary::default();
        summary.total = self.results.len();

        for result in &self.results {
            match result.status {
                VerifyStatus::Pass => summary.passes += 1,
                VerifyStatus::Fail => summary.failures += 1,
                VerifyStatus::Warn => summary.warnings += 1,
                VerifyStatus::Skip => summary.skips += 1,
            }
        }

        summary
    }

    pub fn is_success(&self) -> bool {
        self.results.iter().all(|r| r.status != VerifyStatus::Fail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_summary() {
        let results = vec![
            CheckResult {
                check_id: "check1".to_string(),
                target: VerifyTarget::Artifact { name: "a1".to_string() },
                status: VerifyStatus::Pass,
                message: "ok".to_string(),
            },
            CheckResult {
                check_id: "check2".to_string(),
                target: VerifyTarget::Artifact { name: "a2".to_string() },
                status: VerifyStatus::Fail,
                message: "error".to_string(),
            },
        ];

        let report = VerifyReport::new(results);
        let summary = report.summary();

        assert_eq!(summary.total, 2);
        assert_eq!(summary.passes, 1);
        assert_eq!(summary.failures, 1);
        assert!(!report.is_success());
    }
}
