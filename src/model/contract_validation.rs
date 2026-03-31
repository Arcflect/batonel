use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::model::verify::{CheckResult, VerifyStatus, VerifyTarget};

/// A lenient version of `Contract` where all required fields are `Option<T>`.
///
/// Used only during field-completeness validation so that we can emit a
/// precise, per-field diagnostic instead of a generic serde parse error.
#[derive(Debug, Deserialize)]
struct RawContract {
    pub name: Option<String>,
    pub module: Option<String>,
    pub role: Option<String>,
    pub path: Option<String>,
    pub responsibilities: Option<Vec<String>>,
    pub must_not: Option<Vec<String>>,
    pub status: Option<String>,
}

/// Validates that every required field in a contract file is present and
/// non-empty.  Returns one `CheckResult` per required field.
///
/// This is called only after the contract file is known to exist (the
/// `contract-exists` check has already passed).  A serde parse failure at
/// this stage is itself surfaced as a Fail result.
pub fn validate_contract_fields<P: AsRef<Path>>(
    contract_path: P,
    artifact_name: &str,
) -> Vec<CheckResult> {
    let contract_path = contract_path.as_ref();
    let mut results = Vec::new();

    let make_target = || VerifyTarget::Contract {
        artifact_name: artifact_name.to_string(),
        path: contract_path.to_string_lossy().to_string(),
    };

    // If we cannot read or parse the raw contract we emit a single Fail and
    // bail — the `contract-parse` check in verify.rs already handles proper
    // parse failures from the strict model, so this is a safety net.
    let contents = match fs::read_to_string(contract_path) {
        Ok(s) => s,
        Err(e) => {
            results.push(CheckResult {
                check_id: "contract-fields".to_string(),
                target: make_target(),
                status: VerifyStatus::Fail,
                message: format!(
                    "Could not read contract for field validation ({}): {}",
                    artifact_name, e
                ),
            });
            return results;
        }
    };

    let raw: RawContract = match serde_yaml::from_str(&contents) {
        Ok(r) => r,
        Err(e) => {
            results.push(CheckResult {
                check_id: "contract-fields".to_string(),
                target: make_target(),
                status: VerifyStatus::Fail,
                message: format!(
                    "Could not parse contract for field validation ({}): {}",
                    artifact_name, e
                ),
            });
            return results;
        }
    };

    // Helper closures — avoid repetition for the two value types.
    let check_string = |field: &str, value: &Option<String>| -> CheckResult {
        let (status, message) = match value {
            Some(s) if !s.trim().is_empty() => (
                VerifyStatus::Pass,
                format!("Contract field '{}' present for {}", field, artifact_name),
            ),
            Some(_) => (
                VerifyStatus::Fail,
                format!(
                    "Contract field '{}' is empty in {} — must have a non-empty value",
                    field, artifact_name
                ),
            ),
            None => (
                VerifyStatus::Fail,
                format!(
                    "Contract field '{}' missing in {} — required field not found",
                    field, artifact_name
                ),
            ),
        };
        CheckResult {
            check_id: "contract-fields".to_string(),
            target: make_target(),
            status,
            message,
        }
    };

    let check_vec = |field: &str, value: &Option<Vec<String>>| -> CheckResult {
        let (status, message) = match value {
            Some(v) if !v.is_empty() => (
                VerifyStatus::Pass,
                format!("Contract field '{}' present for {}", field, artifact_name),
            ),
            Some(_) => (
                VerifyStatus::Fail,
                format!(
                    "Contract field '{}' is an empty list in {} — must have at least one entry",
                    field, artifact_name
                ),
            ),
            None => (
                VerifyStatus::Fail,
                format!(
                    "Contract field '{}' missing in {} — required field not found",
                    field, artifact_name
                ),
            ),
        };
        CheckResult {
            check_id: "contract-fields".to_string(),
            target: make_target(),
            status,
            message,
        }
    };

    results.push(check_string("name", &raw.name));
    results.push(check_string("module", &raw.module));
    results.push(check_string("role", &raw.role));
    results.push(check_string("path", &raw.path));
    results.push(check_vec("responsibilities", &raw.responsibilities));
    results.push(check_vec("must_not", &raw.must_not));
    results.push(check_string("status", &raw.status));

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_contract(yaml: &str) -> NamedTempFile {
        let mut f = NamedTempFile::new().expect("failed to create tempfile");
        f.write_all(yaml.as_bytes()).expect("failed to write tempfile");
        f
    }

    fn failures(results: &[CheckResult]) -> Vec<&CheckResult> {
        results
            .iter()
            .filter(|r| r.status == VerifyStatus::Fail)
            .collect()
    }

    fn passes(results: &[CheckResult]) -> Vec<&CheckResult> {
        results
            .iter()
            .filter(|r| r.status == VerifyStatus::Pass)
            .collect()
    }

    #[test]
    fn test_all_fields_present() {
        let yaml = r#"
name: create_user
module: user
role: usecase
path: src/application/usecases/create_user.rs
responsibilities:
  - Handle user creation
must_not:
  - Access database directly
status: planned
"#;
        let f = write_contract(yaml);
        let results = validate_contract_fields(f.path(), "create_user");

        assert_eq!(results.len(), 7, "expected one result per required field");
        assert!(
            failures(&results).is_empty(),
            "expected no failures, got: {:?}",
            failures(&results)
        );
        assert_eq!(passes(&results).len(), 7);
    }

    #[test]
    fn test_missing_path() {
        let yaml = r#"
name: create_user
module: user
role: usecase
responsibilities:
  - Handle user creation
must_not:
  - Access database directly
status: planned
"#;
        let f = write_contract(yaml);
        let results = validate_contract_fields(f.path(), "create_user");

        let failed: Vec<_> = failures(&results);
        assert_eq!(failed.len(), 1, "expected exactly one failure");
        assert!(
            failed[0].message.contains("'path'"),
            "failure message should mention 'path', got: {}",
            failed[0].message
        );
    }

    #[test]
    fn test_empty_responsibilities() {
        let yaml = r#"
name: create_user
module: user
role: usecase
path: src/application/usecases/create_user.rs
responsibilities: []
must_not:
  - Access database directly
status: planned
"#;
        let f = write_contract(yaml);
        let results = validate_contract_fields(f.path(), "create_user");

        let failed: Vec<_> = failures(&results);
        assert_eq!(failed.len(), 1);
        assert!(failed[0].message.contains("'responsibilities'"));
    }

    #[test]
    fn test_multiple_missing_fields() {
        // Only 'name' is present; 6 others are missing.
        let yaml = "name: create_user\n";
        let f = write_contract(yaml);
        let results = validate_contract_fields(f.path(), "create_user");

        let failed: Vec<_> = failures(&results);
        assert_eq!(
            failed.len(),
            6,
            "expected 6 failures (all fields except name), got: {:?}",
            failed.iter().map(|r| &r.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_empty_string_field() {
        let yaml = r#"
name: ""
module: user
role: usecase
path: src/application/usecases/create_user.rs
responsibilities:
  - Handle user creation
must_not:
  - Access database directly
status: planned
"#;
        let f = write_contract(yaml);
        let results = validate_contract_fields(f.path(), "create_user");

        let failed: Vec<_> = failures(&results);
        assert_eq!(failed.len(), 1);
        assert!(failed[0].message.contains("'name'"));
        assert!(failed[0].message.contains("empty"));
    }
}
