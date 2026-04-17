use crate::config::{ArtifactsPlanConfig, ContractConfig, PlacementRulesConfig, ProjectConfig};
use crate::model::contract_validation;
use crate::model::prompt_validation;
use crate::model::scaffold_validation;
use crate::model::status_validation;
use crate::model::verify::{CheckResult, VerifyReport, VerifyStatus, VerifyTarget};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub fn execute() {
    let mut results = Vec::new();
    let mut verified_contracts = HashSet::new();

    println!("Batonel Architectural Verification");
    println!("==================================");

    // 1. Root File Checks
    check_root_file("project.baton.yaml", &mut results);
    check_root_file("placement.rules.yaml", &mut results);
    check_root_file("artifacts.plan.yaml", &mut results);
    check_root_file("contracts.template.yaml", &mut results);

    // 2. Load Configs for deeper checks (only if root files exist)
    let project_config = ProjectConfig::load("project.baton.yaml").ok();
    let placement_config = PlacementRulesConfig::load("placement.rules.yaml").ok();
    let artifacts_config = ArtifactsPlanConfig::load("artifacts.plan.yaml").ok();

    if let (Some(project), Some(placement), Some(artifacts)) =
        (project_config, placement_config, artifacts_config)
    {
        println!("Project: {}", project.project.name);
        println!("Artifacts to verify: {}", artifacts.artifacts.len());
        println!();

        for artifact in &artifacts.artifacts {
            // Check artifact status validity
            results.push(status_validation::validate_artifact_status(
                &artifact.name,
                artifact.status.as_deref(),
            ));

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
                    let prompt_path = crate::generator::resolver::resolve_sidecar_path(
                        artifact,
                        &path,
                        role_config.and_then(|r| r.sidecar.as_ref().and_then(|s| s.prompt_dir.as_deref())),
                        "prompt.md",
                    );

                    let scaffold_results = scaffold_validation::validate_scaffold_structure(
                        &artifact.name,
                        &path,
                        &contract_path,
                        &prompt_path,
                    );
                    results.extend(scaffold_results);

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

                                // Check contract status validity and artifact-contract status consistency
                                let contract_path_str = contract_path.to_string_lossy().to_string();
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
    for line in build_report_lines(report) {
        println!("{}", line);
    }
}

fn build_report_lines(report: &VerifyReport) -> Vec<String> {
    let summary = report.summary();
    let mut lines = vec![
        "Verification Report".to_string(),
        "===================".to_string(),
        format!("Status: {}", overall_status_label(report, &summary)),
        format!(
            "Checks: total={} pass={} fail={} warn={} skip={}",
            summary.total, summary.passes, summary.failures, summary.warnings, summary.skips
        ),
    ];

    let failures: Vec<&CheckResult> = report
        .results
        .iter()
        .filter(|result| result.status == VerifyStatus::Fail)
        .collect();
    let warnings: Vec<&CheckResult> = report
        .results
        .iter()
        .filter(|result| result.status == VerifyStatus::Warn)
        .collect();

    if !failures.is_empty() {
        lines.push(String::new());
        lines.push("Failures".to_string());
        lines.push("--------".to_string());
        lines.extend(render_grouped_results(&failures));
    }

    if !warnings.is_empty() {
        lines.push(String::new());
        lines.push("Warnings".to_string());
        lines.push("--------".to_string());
        lines.extend(render_grouped_results(&warnings));
    }

    if failures.is_empty() && warnings.is_empty() {
        lines.push(String::new());
        lines.push("No failures or warnings detected.".to_string());
    }

    if summary.skips > 0 {
        lines.push(String::new());
        lines.push(format!(
            "Skipped checks: {} (hidden from detailed output)",
            summary.skips
        ));
    }

    lines
}

fn overall_status_label(report: &VerifyReport, summary: &crate::model::verify::VerifySummary) -> &'static str {
    if !report.is_success() {
        "FAILED"
    } else if summary.warnings > 0 {
        "PASSED WITH WARNINGS"
    } else {
        "PASSED"
    }
}

fn render_grouped_results(results: &[&CheckResult]) -> Vec<String> {
    let mut grouped: BTreeMap<String, Vec<&CheckResult>> = BTreeMap::new();

    for result in results {
        grouped
            .entry(format_target(&result.target))
            .or_default()
            .push(*result);
    }

    let mut lines = Vec::new();
    for (target, target_results) in grouped {
        lines.push(format!("{}", target));
        for result in target_results {
            lines.push(format!("  - [{}] {}", result.check_id, result.message));
        }
    }

    lines
}

fn format_target(target: &VerifyTarget) -> String {
    match target {
        VerifyTarget::RootConfig { name, .. } => format!("Root config: {}", name),
        VerifyTarget::Artifact { name } => format!("Artifact: {}", name),
        VerifyTarget::Contract {
            artifact_name,
            path,
        } => format!("Contract: {} ({})", artifact_name, path),
        VerifyTarget::Prompt {
            artifact_name,
            path,
        } => format!("Prompt: {} ({})", artifact_name, path),
        VerifyTarget::SourceFile {
            artifact_name,
            path,
        } => format!("Source file: {} ({})", artifact_name, path),
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
                // Skip dependency/build directories while walking for contracts.
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

    #[test]
    fn report_lines_group_failures_and_warnings() {
        let report = VerifyReport::new(vec![
            CheckResult {
                check_id: "root-file-exists".to_string(),
                target: VerifyTarget::RootConfig {
                    name: "project.baton.yaml".to_string(),
                    path: "project.baton.yaml".to_string(),
                },
                status: VerifyStatus::Fail,
                message: "Missing required file: project.baton.yaml".to_string(),
            },
            CheckResult {
                check_id: "prompt-exists".to_string(),
                target: VerifyTarget::Prompt {
                    artifact_name: "user".to_string(),
                    path: "src/user.prompt.md".to_string(),
                },
                status: VerifyStatus::Warn,
                message: "Prompt missing for user".to_string(),
            },
        ]);

        let lines = build_report_lines(&report);
        let output = lines.join("\n");

        assert!(output.contains("Status: FAILED"));
        assert!(output.contains("Failures"));
        assert!(output.contains("Warnings"));
        assert!(output.contains("Root config: project.baton.yaml"));
        assert!(output.contains("[root-file-exists] Missing required file: project.baton.yaml"));
        assert!(output.contains("Prompt: user (src/user.prompt.md)"));
        assert!(output.contains("[prompt-exists] Prompt missing for user"));
    }

    #[test]
    fn report_lines_show_clean_success_summary() {
        let report = VerifyReport::new(vec![CheckResult {
            check_id: "role-defined".to_string(),
            target: VerifyTarget::Artifact {
                name: "user".to_string(),
            },
            status: VerifyStatus::Pass,
            message: "Role is valid".to_string(),
        }]);

        let lines = build_report_lines(&report);
        let output = lines.join("\n");

        assert!(output.contains("Status: PASSED"));
        assert!(output.contains("No failures or warnings detected."));
        assert!(!output.contains("Failures"));
        assert!(!output.contains("Warnings"));
    }
}
