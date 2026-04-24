use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{ArtifactsPlanConfig, ContractConfig, PlacementRulesConfig, ProjectConfig};
use crate::model::contract_validation;
use crate::model::prompt_validation;
use crate::model::scaffold_validation;
use crate::model::status_validation;
use crate::model::verify::{CheckResult, VerifyReport, VerifyStatus, VerifyTarget};

// ---------------------------------------------------------------------------
// UseCase public types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct ValidateProjectInput;

#[derive(Debug, Clone)]
pub struct ValidateProjectOutput {
    pub success: bool,
    pub structural_errors: usize,
    pub structural_warnings: usize,
    pub report: VerifyReport,
}

// ---------------------------------------------------------------------------
// UseCase
// ---------------------------------------------------------------------------

pub struct ValidateProjectUseCase;

impl ValidateProjectUseCase {
    /// Entry point used by the CLI adapter.
    pub fn execute(
        input: ValidateProjectInput,
    ) -> Result<ValidateProjectOutput, crate::app::error::ValidationError> {
        Self::execute_with_paths(
            input,
            "project.baton.yaml",
            "placement.rules.yaml",
            "artifacts.plan.yaml",
        )
    }

    /// Testable variant with injected config paths.
    pub(crate) fn execute_with_paths(
        _input: ValidateProjectInput,
        project_path: &str,
        placement_path: &str,
        artifacts_path: &str,
    ) -> Result<ValidateProjectOutput, crate::app::error::ValidationError> {
        // ── 1. Load configs ──────────────────────────────────────────────────
        let project = ProjectConfig::load(project_path).map_err(|e| {
            crate::app::error::ConfigLoadError::Load {
                path: project_path.to_string(),
                source: e,
            }
        })?;
        let placement = PlacementRulesConfig::load(placement_path).map_err(|e| {
            crate::app::error::ConfigLoadError::Load {
                path: placement_path.to_string(),
                source: e,
            }
        })?;
        let artifacts = ArtifactsPlanConfig::load(artifacts_path).map_err(|e| {
            crate::app::error::ConfigLoadError::Load {
                path: artifacts_path.to_string(),
                source: e,
            }
        })?;

        // ── 2. Structural validation (domain) ────────────────────────────────
        let context = crate::domain::project::ProjectContext::from_project_config(&project);
        let structural_result =
            crate::domain::validation::ArchitectureValidator::validate(&context, &placement, &artifacts);
        let structural_errors = structural_result.error_count();
        let structural_warnings = structural_result.warning_count();

        // ── 3. File-level verification (moved from commands/verify.rs) ────────
        let report = run_verify_checks(&project, &placement, &artifacts);

        let success = structural_errors == 0 && report.is_success();

        Ok(ValidateProjectOutput {
            success,
            structural_errors,
            structural_warnings,
            report,
        })
    }

    // Kept for backward compat with tests that injected a verifier closure.
    // Delegates to the new path-based variant; the `verify_runner` is ignored
    // because the logic is now internal to the use case.
    #[allow(dead_code)]
    pub fn execute_with_output(
        input: ValidateProjectInput,
        _output: &mut dyn crate::ports::OutputPort,
    ) -> Result<ValidateProjectOutput, crate::app::error::ValidationError> {
        Self::execute(input)
    }
}

// ---------------------------------------------------------------------------
// Verify check logic
// ---------------------------------------------------------------------------

fn run_verify_checks(
    project: &ProjectConfig,
    placement: &PlacementRulesConfig,
    artifacts: &ArtifactsPlanConfig,
) -> VerifyReport {
    let mut results: Vec<CheckResult> = Vec::new();
    let mut verified_contracts: HashSet<String> = HashSet::new();

    // Root file presence
    check_root_file("project.baton.yaml", &mut results);
    check_root_file("placement.rules.yaml", &mut results);
    check_root_file("artifacts.plan.yaml", &mut results);
    check_root_file("contracts.template.yaml", &mut results);

    // Per-artifact checks
    for artifact in &artifacts.artifacts {
        results.push(status_validation::validate_artifact_status(
            &artifact.name,
            artifact.status.as_deref(),
        ));

        let (role_checks, role_found) = check_role_consistency(
            &artifact.name,
            &artifact.role,
            artifact.path.as_deref(),
            placement,
        );
        results.extend(role_checks);
        if !role_found {
            continue;
        }

        match crate::generator::resolver::resolve_artifact_path(artifact, placement) {
            Ok(path) => {
                let role_config = placement.roles.get(&artifact.role);
                let contract_path = crate::generator::resolver::resolve_sidecar_path(
                    artifact,
                    &path,
                    role_config
                        .and_then(|r| r.sidecar.as_ref().and_then(|s| s.contract_dir.as_deref())),
                    "contract.yaml",
                );
                let prompt_path = crate::generator::resolver::resolve_sidecar_path(
                    artifact,
                    &path,
                    role_config
                        .and_then(|r| r.sidecar.as_ref().and_then(|s| s.prompt_dir.as_deref())),
                    "prompt.md",
                );

                results.extend(scaffold_validation::validate_scaffold_structure(
                    &artifact.name,
                    &path,
                    &contract_path,
                    &prompt_path,
                ));

                if contract_path.exists() {
                    let normalized = fs::canonicalize(&contract_path)
                        .unwrap_or_else(|_| contract_path.to_path_buf())
                        .to_string_lossy()
                        .to_string();
                    verified_contracts.insert(normalized.clone());

                    results.push(CheckResult {
                        check_id: "contract-exists".to_string(),
                        target: VerifyTarget::Contract {
                            artifact_name: artifact.name.clone(),
                            path: normalized,
                        },
                        status: VerifyStatus::Pass,
                        message: format!("Contract exists for {}", artifact.name),
                    });

                    match ContractConfig::load(&contract_path) {
                        Ok(contract_file) => {
                            let c = &contract_file.contract;
                            let mut mismatches = Vec::new();
                            if c.name != artifact.name {
                                mismatches.push(format!(
                                    "name: expectation '{}' vs actual '{}'",
                                    artifact.name, c.name
                                ));
                            }
                            if c.module != artifact.module {
                                mismatches.push(format!(
                                    "module: expectation '{}' vs actual '{}'",
                                    artifact.module, c.module
                                ));
                            }
                            if c.role != artifact.role {
                                mismatches.push(format!(
                                    "role: expectation '{}' vs actual '{}'",
                                    artifact.role, c.role
                                ));
                            }
                            let resolved_source_path = path.to_string_lossy().to_string();
                            if c.path != resolved_source_path {
                                mismatches.push(format!(
                                    "path: expectation '{}' vs actual '{}'",
                                    resolved_source_path, c.path
                                ));
                            }

                            let identity_status = if mismatches.is_empty() {
                                CheckResult {
                                    check_id: "contract-identity".to_string(),
                                    target: VerifyTarget::Contract {
                                        artifact_name: artifact.name.clone(),
                                        path: contract_path.to_string_lossy().to_string(),
                                    },
                                    status: VerifyStatus::Pass,
                                    message: format!(
                                        "Contract identity and path match plan for {}",
                                        artifact.name
                                    ),
                                }
                            } else {
                                CheckResult {
                                    check_id: "contract-identity".to_string(),
                                    target: VerifyTarget::Contract {
                                        artifact_name: artifact.name.clone(),
                                        path: contract_path.to_string_lossy().to_string(),
                                    },
                                    status: VerifyStatus::Fail,
                                    message: format!(
                                        "Contract identity mismatch for {}: {}",
                                        artifact.name,
                                        mismatches.join(", ")
                                    ),
                                }
                            };
                            results.push(identity_status);

                            let contract_path_str =
                                contract_path.to_string_lossy().to_string();
                            results.push(status_validation::validate_contract_status(
                                &artifact.name,
                                &contract_path_str,
                                Some(c.status.as_str()),
                            ));
                            results.push(
                                status_validation::validate_artifact_contract_status_consistency(
                                    &artifact.name,
                                    artifact.status.as_deref(),
                                    &contract_path_str,
                                    Some(c.status.as_str()),
                                ),
                            );

                            results.extend(contract_validation::validate_contract_fields(
                                &contract_path,
                                &artifact.name,
                            ));
                        }
                        Err(e) => {
                            results.push(CheckResult {
                                check_id: "contract-parse".to_string(),
                                target: VerifyTarget::Contract {
                                    artifact_name: artifact.name.clone(),
                                    path: contract_path.to_string_lossy().to_string(),
                                },
                                status: VerifyStatus::Fail,
                                message: format!(
                                    "Contract parsing failed for {}: {}",
                                    artifact.name, e
                                ),
                            });
                        }
                    }
                } else {
                    results.push(CheckResult {
                        check_id: "contract-exists".to_string(),
                        target: VerifyTarget::Contract {
                            artifact_name: artifact.name.clone(),
                            path: contract_path.to_string_lossy().to_string(),
                        },
                        status: VerifyStatus::Fail,
                        message: format!("Contract missing for {}", artifact.name),
                    });
                }

                if prompt_path.exists() {
                    results.push(CheckResult {
                        check_id: "prompt-exists".to_string(),
                        target: VerifyTarget::Prompt {
                            artifact_name: artifact.name.clone(),
                            path: prompt_path.to_string_lossy().to_string(),
                        },
                        status: VerifyStatus::Pass,
                        message: format!("Prompt exists for {}", artifact.name),
                    });
                    results.extend(prompt_validation::validate_prompt_identity(
                        &prompt_path,
                        &artifact.name,
                        &artifact.role,
                        &artifact.module,
                    ));
                } else {
                    results.push(CheckResult {
                        check_id: "prompt-exists".to_string(),
                        target: VerifyTarget::Prompt {
                            artifact_name: artifact.name.clone(),
                            path: prompt_path.to_string_lossy().to_string(),
                        },
                        status: VerifyStatus::Warn,
                        message: format!(
                            "Prompt missing for {} (Recommended for AI handoff)",
                            artifact.name
                        ),
                    });
                }
            }
            Err(e) => {
                results.push(CheckResult {
                    check_id: "path-resolution".to_string(),
                    target: VerifyTarget::Artifact { name: artifact.name.clone() },
                    status: VerifyStatus::Fail,
                    message: format!("Could not resolve path for {}: {}", artifact.name, e),
                });
            }
        }
    }

    // Orphaned contracts
    let all_contracts = find_all_contracts(".");
    for contract_path in all_contracts {
        let path_str = fs::canonicalize(&contract_path)
            .unwrap_or_else(|_| contract_path.to_path_buf())
            .to_string_lossy()
            .to_string();
        if !verified_contracts.contains(&path_str) {
            results.push(CheckResult {
                check_id: "orphaned-contract".to_string(),
                target: VerifyTarget::Contract {
                    artifact_name: "unknown".to_string(),
                    path: path_str.clone(),
                },
                status: VerifyStatus::Fail,
                message: format!(
                    "Orphaned contract found: {} (Not defined in artifacts.plan.yaml)",
                    path_str
                ),
            });
        }
    }

    let _ = project; // configs loaded for potential future use
    VerifyReport::new(results)
}

fn check_root_file(filename: &str, results: &mut Vec<CheckResult>) {
    let path = Path::new(filename);
    if path.exists() {
        results.push(CheckResult {
            check_id: "root-file-exists".to_string(),
            target: VerifyTarget::RootConfig {
                name: filename.to_string(),
                path: filename.to_string(),
            },
            status: VerifyStatus::Pass,
            message: format!("Found {}", filename),
        });
    } else {
        results.push(CheckResult {
            check_id: "root-file-exists".to_string(),
            target: VerifyTarget::RootConfig {
                name: filename.to_string(),
                path: filename.to_string(),
            },
            status: VerifyStatus::Fail,
            message: format!("Missing required file: {}", filename),
        });
    }
}

fn check_role_consistency(
    artifact_name: &str,
    artifact_role: &str,
    explicit_path: Option<&str>,
    placement: &PlacementRulesConfig,
) -> (Vec<CheckResult>, bool) {
    let mut results = Vec::new();

    let role_config = match placement.roles.get(artifact_role) {
        Some(rc) => {
            results.push(CheckResult {
                check_id: "role-defined".to_string(),
                target: VerifyTarget::Artifact { name: artifact_name.to_string() },
                status: VerifyStatus::Pass,
                message: format!(
                    "Role '{}' is defined in placement rules for '{}'",
                    artifact_role, artifact_name
                ),
            });
            rc
        }
        None => {
            results.push(CheckResult {
                check_id: "role-defined".to_string(),
                target: VerifyTarget::Artifact { name: artifact_name.to_string() },
                status: VerifyStatus::Fail,
                message: format!(
                    "Role '{}' used by artifact '{}' is not defined in placement rules",
                    artifact_role, artifact_name
                ),
            });
            return (results, false);
        }
    };

    if let Some(explicit) = explicit_path {
        let file_name = match &role_config.file_extension {
            Some(ext) => format!("{}.{}", artifact_name, ext),
            None => artifact_name.to_string(),
        };
        let mut expected = PathBuf::from(&role_config.path);
        expected.push(&file_name);
        let expected_str = expected.to_string_lossy().to_string();

        if explicit != expected_str {
            results.push(CheckResult {
                check_id: "role-path-match".to_string(),
                target: VerifyTarget::Artifact { name: artifact_name.to_string() },
                status: VerifyStatus::Warn,
                message: format!(
                    "Artifact '{}' has explicit path '{}' which deviates from role '{}' expected path '{}'",
                    artifact_name, explicit, artifact_role, expected_str
                ),
            });
        } else {
            results.push(CheckResult {
                check_id: "role-path-match".to_string(),
                target: VerifyTarget::Artifact { name: artifact_name.to_string() },
                status: VerifyStatus::Pass,
                message: format!(
                    "Explicit path for '{}' matches role-based expectation",
                    artifact_name
                ),
            });
        }
    }

    (results, true)
}

fn find_all_contracts<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut contracts = Vec::new();
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return contracts,
    };
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "target" || name == ".git" || name == "node_modules" {
                    continue;
                }
                contracts.extend(find_all_contracts(path));
            } else if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yaml" || ext == "yml" {
                        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                            if (file_name.ends_with(".contract.yaml")
                                || file_name.ends_with(".contract.yml"))
                                && file_name != "contracts.template.yaml"
                            {
                                contracts.push(path);
                            }
                        }
                    }
                }
            }
        }
    }
    contracts
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::placement::RolePlacement;
    use std::collections::HashMap;
    use tempfile::tempdir;

    fn make_placement(role: &str, path: &str, ext: Option<&str>) -> PlacementRulesConfig {
        let mut roles = HashMap::new();
        roles.insert(
            role.to_string(),
            RolePlacement {
                path: path.to_string(),
                file_extension: ext.map(|s| s.to_string()),
                sidecar: None,
            },
        );
        PlacementRulesConfig { roles }
    }

    #[test]
    fn role_defined_pass_when_role_exists() {
        let placement = make_placement("service", "src/services", Some("rs"));
        let (results, role_found) =
            check_role_consistency("UserService", "service", None, &placement);
        assert!(role_found);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].check_id, "role-defined");
        assert_eq!(results[0].status, VerifyStatus::Pass);
    }

    #[test]
    fn role_defined_fail_when_role_missing() {
        let placement = make_placement("service", "src/services", Some("rs"));
        let (results, role_found) =
            check_role_consistency("UserService", "unknown-role", None, &placement);
        assert!(!role_found);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].check_id, "role-defined");
        assert_eq!(results[0].status, VerifyStatus::Fail);
    }

    #[test]
    fn role_path_match_pass_when_explicit_path_matches_role() {
        let placement = make_placement("service", "src/services", Some("rs"));
        let (results, role_found) = check_role_consistency(
            "UserService",
            "service",
            Some("src/services/UserService.rs"),
            &placement,
        );
        assert!(role_found);
        assert_eq!(results.len(), 2);
        assert_eq!(results[1].check_id, "role-path-match");
        assert_eq!(results[1].status, VerifyStatus::Pass);
    }

    #[test]
    fn role_path_match_warn_when_explicit_path_deviates() {
        let placement = make_placement("service", "src/services", Some("rs"));
        let (results, role_found) = check_role_consistency(
            "UserService",
            "service",
            Some("custom/path/UserService.rs"),
            &placement,
        );
        assert!(role_found);
        assert_eq!(results.len(), 2);
        assert_eq!(results[1].check_id, "role-path-match");
        assert_eq!(results[1].status, VerifyStatus::Warn);
    }

    #[test]
    fn no_role_path_check_when_no_explicit_path() {
        let placement = make_placement("service", "src/services", Some("rs"));
        let (results, role_found) =
            check_role_consistency("UserService", "service", None, &placement);
        assert!(role_found);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].check_id, "role-defined");
    }

    #[test]
    fn validate_returns_explicit_config_error_when_project_missing() {
        let result = ValidateProjectUseCase::execute_with_paths(
            ValidateProjectInput,
            "missing-project.baton.yaml",
            "missing-placement.rules.yaml",
            "missing-artifacts.plan.yaml",
        );
        let err = result.expect_err("expected config load error");
        let msg = err.to_string();
        assert!(msg.contains("missing-project.baton.yaml"));
    }

    #[test]
    fn validate_completes_successfully_with_minimal_valid_config() {
        let temp = tempdir().expect("tempdir should be created");
        let root = temp.path();

        std::fs::write(
            root.join("project.baton.yaml"),
            r#"batonel:
  schema_version: "1"
project:
  name: sample-app
  architecture_style: layered
  language: generic
modules:
  - name: user
"#,
        )
        .expect("write project");

        std::fs::write(
            root.join("placement.rules.yaml"),
            r#"roles:
  usecase:
    path: "src/application/usecases"
    file_extension: rs
"#,
        )
        .expect("write placement");

        std::fs::write(
            root.join("artifacts.plan.yaml"),
            r#"artifacts:
  - name: create_user
    module: user
    role: usecase
"#,
        )
        .expect("write artifacts");

        let output = ValidateProjectUseCase::execute_with_paths(
            ValidateProjectInput,
            root.join("project.baton.yaml").to_str().unwrap(),
            root.join("placement.rules.yaml").to_str().unwrap(),
            root.join("artifacts.plan.yaml").to_str().unwrap(),
        )
        .expect("validate should complete");

        // Structural: no errors (user module is defined)
        assert_eq!(output.structural_errors, 0);
    }
}
