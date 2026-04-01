use std::path::Path;

use crate::model::verify::{CheckResult, VerifyStatus, VerifyTarget};

pub fn validate_scaffold_structure<P: AsRef<Path>>(
    artifact_name: &str,
    artifact_path: P,
    contract_path: P,
    prompt_path: P,
) -> Vec<CheckResult> {
    let artifact_path = artifact_path.as_ref();
    let contract_path = contract_path.as_ref();
    let prompt_path = prompt_path.as_ref();
    let mut results = Vec::new();

    results.push(validate_expected_directory(artifact_name, artifact_path));
    results.push(validate_source_file_presence(artifact_name, artifact_path));
    results.push(validate_sidecar_completeness(
        artifact_name,
        artifact_path,
        contract_path,
        prompt_path,
    ));

    results
}

fn validate_expected_directory(artifact_name: &str, artifact_path: &Path) -> CheckResult {
    match artifact_path.parent() {
        Some(parent) if parent.exists() => CheckResult {
            check_id: "scaffold-directory-exists".to_string(),
            target: VerifyTarget::SourceFile {
                artifact_name: artifact_name.to_string(),
                path: parent.to_string_lossy().to_string(),
            },
            status: VerifyStatus::Pass,
            message: format!(
                "Expected scaffold directory exists for {}: {}",
                artifact_name,
                parent.to_string_lossy()
            ),
        },
        Some(parent) => CheckResult {
            check_id: "scaffold-directory-exists".to_string(),
            target: VerifyTarget::SourceFile {
                artifact_name: artifact_name.to_string(),
                path: parent.to_string_lossy().to_string(),
            },
            status: VerifyStatus::Fail,
            message: format!(
                "Expected scaffold directory missing for {}: {}",
                artifact_name,
                parent.to_string_lossy()
            ),
        },
        None => CheckResult {
            check_id: "scaffold-directory-exists".to_string(),
            target: VerifyTarget::SourceFile {
                artifact_name: artifact_name.to_string(),
                path: artifact_path.to_string_lossy().to_string(),
            },
            status: VerifyStatus::Skip,
            message: format!(
                "Scaffold directory check skipped for {} because the artifact path is at the workspace root",
                artifact_name
            ),
        },
    }
}

fn validate_source_file_presence(artifact_name: &str, artifact_path: &Path) -> CheckResult {
    let exists = artifact_path.exists() && artifact_path.is_file();
    CheckResult {
        check_id: "scaffold-source-exists".to_string(),
        target: VerifyTarget::SourceFile {
            artifact_name: artifact_name.to_string(),
            path: artifact_path.to_string_lossy().to_string(),
        },
        status: if exists {
            VerifyStatus::Pass
        } else {
            VerifyStatus::Fail
        },
        message: if exists {
            format!("Expected scaffold source file exists for {}", artifact_name)
        } else {
            format!("Expected scaffold source file missing for {}", artifact_name)
        },
    }
}

fn validate_sidecar_completeness(
    artifact_name: &str,
    artifact_path: &Path,
    contract_path: &Path,
    prompt_path: &Path,
) -> CheckResult {
    let mut missing = Vec::new();
    if !(contract_path.exists() && contract_path.is_file()) {
        missing.push(format!("contract: {}", contract_path.to_string_lossy()));
    }
    if !(prompt_path.exists() && prompt_path.is_file()) {
        missing.push(format!("prompt: {}", prompt_path.to_string_lossy()));
    }

    CheckResult {
        check_id: "scaffold-sidecars-complete".to_string(),
        target: VerifyTarget::SourceFile {
            artifact_name: artifact_name.to_string(),
            path: artifact_path.to_string_lossy().to_string(),
        },
        status: if missing.is_empty() {
            VerifyStatus::Pass
        } else {
            VerifyStatus::Fail
        },
        message: if missing.is_empty() {
            format!("Expected scaffold sidecars exist for {}", artifact_name)
        } else {
            format!(
                "Expected scaffold sidecars missing for {}: {}",
                artifact_name,
                missing.join(", ")
            )
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn scaffold_checks_pass_when_structure_exists() {
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src/domain");
        fs::create_dir_all(&src_dir).unwrap();
        let artifact_path = src_dir.join("user.rs");
        let contract_path = src_dir.join("user.contract.yaml");
        let prompt_path = src_dir.join("user.prompt.md");
        fs::write(&artifact_path, "// placeholder\n").unwrap();
        fs::write(&contract_path, "name: user\n").unwrap();
        fs::write(&prompt_path, "<!-- prompt -->\n").unwrap();

        let results = validate_scaffold_structure("user", &artifact_path, &contract_path, &prompt_path);

        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.status == VerifyStatus::Pass));
    }

    #[test]
    fn scaffold_directory_check_fails_when_parent_missing() {
        let dir = tempdir().unwrap();
        let artifact_path = dir.path().join("missing/user.rs");
        let contract_path = dir.path().join("missing/user.contract.yaml");
        let prompt_path = dir.path().join("missing/user.prompt.md");

        let results = validate_scaffold_structure("user", &artifact_path, &contract_path, &prompt_path);

        assert_eq!(results[0].check_id, "scaffold-directory-exists");
        assert_eq!(results[0].status, VerifyStatus::Fail);
    }

    #[test]
    fn scaffold_source_check_fails_when_source_missing() {
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src/domain");
        fs::create_dir_all(&src_dir).unwrap();
        let artifact_path = src_dir.join("user.rs");
        let contract_path = src_dir.join("user.contract.yaml");
        let prompt_path = src_dir.join("user.prompt.md");
        fs::write(&contract_path, "name: user\n").unwrap();
        fs::write(&prompt_path, "<!-- prompt -->\n").unwrap();

        let results = validate_scaffold_structure("user", &artifact_path, &contract_path, &prompt_path);

        assert_eq!(results[1].check_id, "scaffold-source-exists");
        assert_eq!(results[1].status, VerifyStatus::Fail);
    }

    #[test]
    fn scaffold_sidecar_check_fails_when_prompt_missing() {
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("src/domain");
        fs::create_dir_all(&src_dir).unwrap();
        let artifact_path = src_dir.join("user.rs");
        let contract_path = src_dir.join("user.contract.yaml");
        let prompt_path = src_dir.join("user.prompt.md");
        fs::write(&artifact_path, "// placeholder\n").unwrap();
        fs::write(&contract_path, "name: user\n").unwrap();

        let results = validate_scaffold_structure("user", &artifact_path, &contract_path, &prompt_path);

        assert_eq!(results[2].check_id, "scaffold-sidecars-complete");
        assert_eq!(results[2].status, VerifyStatus::Fail);
    }
}