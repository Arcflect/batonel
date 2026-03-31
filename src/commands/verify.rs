use crate::config::{ArtifactsPlanConfig, ContractConfig, PlacementRulesConfig, ProjectConfig};
use crate::model::contract_validation;
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
