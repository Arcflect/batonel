use crate::config::{ArtifactsPlanConfig, ContractConfig, PlacementRulesConfig, ProjectConfig};
use crate::model::contract_validation;
use crate::model::prompt_validation;
use crate::model::verify::{CheckResult, VerifyReport, VerifyStatus, VerifyTarget};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub fn execute() {
    let mut results = Vec::new();
    let mut verified_contracts = HashSet::new();

    println!("Archflow Architectural Verification");
    println!("==================================");

    // 1. Root File Checks
    check_root_file("project.arch.yaml", &mut results);
    check_root_file("placement.rules.yaml", &mut results);
    check_root_file("artifacts.plan.yaml", &mut results);
    check_root_file("contracts.template.yaml", &mut results);

    // 2. Load Configs for deeper checks (only if root files exist)
    let project_config = ProjectConfig::load("project.arch.yaml").ok();
    let placement_config = PlacementRulesConfig::load("placement.rules.yaml").ok();
    let artifacts_config = ArtifactsPlanConfig::load("artifacts.plan.yaml").ok();

    if let (Some(project), Some(placement), Some(artifacts)) =
        (project_config, placement_config, artifacts_config)
    {
        println!("Project: {}", project.project.name);
        println!("Artifacts to verify: {}", artifacts.artifacts.len());
        println!();

        for artifact in &artifacts.artifacts {
            // Check: role consistency (role-defined + role-path-match)
            let (role_checks, role_found) =
                check_role_consistency(&artifact.name, &artifact.role, artifact.path.as_deref(), &placement);
            results.extend(role_checks);
            if !role_found {
                continue;
            }

            // Check Contract alignment
            match crate::generator::resolver::resolve_artifact_path(artifact, &placement) {
                Ok(path) => {
                    let role_config = placement.roles.get(&artifact.role);
                    let contract_path = crate::generator::resolver::resolve_sidecar_path(
                        artifact,
                        &path,
                        role_config.and_then(|r| r.sidecar.as_ref().and_then(|s| s.contract_dir.as_deref())),
                        "contract.yaml",
                    );

                    // Check physical contract existence
                    if contract_path.exists() {
                        let normalized_path = fs::canonicalize(&contract_path)
                            .unwrap_or_else(|_| contract_path.to_path_buf())
                            .to_string_lossy()
                            .to_string();
                        verified_contracts.insert(normalized_path.clone());

                        results.push(CheckResult {
                            check_id: "contract-exists".to_string(),
                            target: VerifyTarget::Contract { 
                                artifact_name: artifact.name.clone(),
                                path: normalized_path
                            },
                            status: VerifyStatus::Pass,
                            message: format!("Contract exists for {}", artifact.name),
                        });

                        // Check Identity Alignment
                        match ContractConfig::load(&contract_path) {
                            Ok(contract_file) => {
                                let c = &contract_file.contract;
                                // Detailed identity mismatch assessment
                                let mut mismatches = Vec::new();
                                if c.name != artifact.name {
                                    mismatches.push(format!("name: expectation '{}' vs actual '{}'", artifact.name, c.name));
                                }
                                if c.module != artifact.module {
                                    mismatches.push(format!("module: expectation '{}' vs actual '{}'", artifact.module, c.module));
                                }
                                if c.role != artifact.role {
                                    mismatches.push(format!("role: expectation '{}' vs actual '{}'", artifact.role, c.role));
                                }
                                
                                // Path alignment: contract.path should match the resolved artifact source path
                                let resolved_source_path = path.to_string_lossy().to_string();
                                if c.path != resolved_source_path {
                                    mismatches.push(format!("path: expectation '{}' vs actual '{}'", resolved_source_path, c.path));
                                }

                                if mismatches.is_empty() {
                                    results.push(CheckResult {
                                        check_id: "contract-identity".to_string(),
                                        target: VerifyTarget::Contract { 
                                            artifact_name: artifact.name.clone(),
                                            path: contract_path.to_string_lossy().to_string() 
                                        },
                                        status: VerifyStatus::Pass,
                                        message: format!("Contract identity and path match plan for {}", artifact.name),
                                    });
                                } else {
                                    results.push(CheckResult {
                                        check_id: "contract-identity".to_string(),
                                        target: VerifyTarget::Contract { 
                                            artifact_name: artifact.name.clone(),
                                            path: contract_path.to_string_lossy().to_string() 
                                        },
                                        status: VerifyStatus::Fail,
                                        message: format!(
                                            "Contract identity mismatch for {}: {}",
                                            artifact.name,
                                            mismatches.join(", ")
                                        ),
                                    });
                                }
                            }
                            Err(e) => {
                                results.push(CheckResult {
                                    check_id: "contract-parse".to_string(),
                                    target: VerifyTarget::Contract { 
                                        artifact_name: artifact.name.clone(),
                                        path: contract_path.to_string_lossy().to_string() 
                                    },
                                    status: VerifyStatus::Fail,
                                    message: format!("Contract parsing failed for {}: {}", artifact.name, e),
                                });
                            }
                        }

                        // Check Field Completeness
                        let field_results = contract_validation::validate_contract_fields(
                            &contract_path,
                            &artifact.name,
                        );
                        results.extend(field_results);
                    } else {
                        results.push(CheckResult {
                            check_id: "contract-exists".to_string(),
                            target: VerifyTarget::Contract { 
                                artifact_name: artifact.name.clone(),
                                path: contract_path.to_string_lossy().to_string() 
                            },
                            status: VerifyStatus::Fail,
                            message: format!("Contract missing for {}", artifact.name),
                        });
                    }

                    // Check Prompt presence
                    let prompt_path = crate::generator::resolver::resolve_sidecar_path(
                        artifact,
                        &path,
                        role_config.and_then(|r| r.sidecar.as_ref().and_then(|s| s.prompt_dir.as_deref())),
                        "prompt.md",
                    );
                    if prompt_path.exists() {
                        results.push(CheckResult {
                            check_id: "prompt-exists".to_string(),
                            target: VerifyTarget::Prompt { 
                                artifact_name: artifact.name.clone(),
                                path: prompt_path.to_string_lossy().to_string() 
                            },
                            status: VerifyStatus::Pass,
                            message: format!("Prompt exists for {}", artifact.name),
                        });

                        // Check prompt naming + prompt-to-contract identity alignment
                        let prompt_identity_results = prompt_validation::validate_prompt_identity(
                            &prompt_path,
                            &artifact.name,
                            &artifact.role,
                            &artifact.module,
                        );
                        results.extend(prompt_identity_results);
                    } else {
                        results.push(CheckResult {
                            check_id: "prompt-exists".to_string(),
                            target: VerifyTarget::Prompt { 
                                artifact_name: artifact.name.clone(),
                                path: prompt_path.to_string_lossy().to_string() 
                            },
                            status: VerifyStatus::Warn,
                            message: format!("Prompt missing for {} (Recommended for AI handoff)", artifact.name),
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
    } else {
        println!("Skipping artifact-level checks due to missing or invalid root configurations.");
        println!();
    }

    // 2.5 Reverse Check: Orphaned Contracts
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
                    artifact_name: "unknown".to_string(), // We don't know which artifact it was supposed to be
                    path: path_str.clone(),
                },
                status: VerifyStatus::Fail,
                message: format!("Orphaned contract found: {} (Not defined in artifacts.plan.yaml)", path_str),
            });
        }
    }

    // 3. Final Report
    let report = VerifyReport::new(results);
    render_report(&report);

    if !report.is_success() {
        std::process::exit(1);
    }
}

fn check_root_file(filename: &str, results: &mut Vec<CheckResult>) {
    let path = Path::new(filename);
    if path.exists() {
        results.push(CheckResult {
            check_id: "root-file-exists".to_string(),
            target: VerifyTarget::RootConfig { 
                name: filename.to_string(),
                path: filename.to_string()
            },
            status: VerifyStatus::Pass,
            message: format!("Found {}", filename),
        });
    } else {
        results.push(CheckResult {
            check_id: "root-file-exists".to_string(),
            target: VerifyTarget::RootConfig { 
                name: filename.to_string(),
                path: filename.to_string()
            },
            status: VerifyStatus::Fail,
            message: format!("Missing required file: {}", filename),
        });
    }
}

fn render_report(report: &VerifyReport) {
    for result in &report.results {
        let prefix = match result.status {
            VerifyStatus::Pass => "  [PASS] ",
            VerifyStatus::Fail => "  [FAIL] ",
            VerifyStatus::Warn => "  [WARN] ",
            VerifyStatus::Skip => "  [SKIP] ",
        };
        println!("{}{}", prefix, result.message);
    }

    let summary = report.summary();
    println!();
    println!("Summary:");
    println!("  Total:    {}", summary.total);
    println!("  Pass:     {}", summary.passes);
    println!("  Failures: {}", summary.failures);
    println!("  Warnings: {}", summary.warnings);
    println!("  Skips:    {}", summary.skips);
    println!();

    if report.is_success() {
        if summary.warnings > 0 {
            println!("Verification passed with warnings.");
        } else {
            println!("Verification successful.");
        }
    } else {
        println!("Verification failed.");
    }
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
                // Skip common ignore directories
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "target" || name == ".git" || name == "node_modules" {
                    continue;
                }
                contracts.extend(find_all_contracts(path));
            } else if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yaml" || ext == "yml" {
                        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                            if (file_name.ends_with(".contract.yaml") || file_name.ends_with(".contract.yml"))
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

/// Checks role-to-path consistency for a single artifact.
///
/// Returns `(results, role_found)` where `role_found` is `false` when the
/// artifact's role is missing from the placement rules — the caller should
/// skip further checks for that artifact.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::placement::RolePlacement;
    use std::collections::HashMap;

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
        // Only role-defined check, no role-path-match
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].check_id, "role-defined");
    }
}
