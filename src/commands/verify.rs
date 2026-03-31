use crate::config::{ArtifactsPlanConfig, ContractConfig, PlacementRulesConfig, ProjectConfig};
use crate::model::verify::{CheckResult, VerifyReport, VerifyStatus, VerifyTarget};
use std::path::Path;

pub fn execute() {
    let mut results = Vec::new();

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
                        results.push(CheckResult {
                            check_id: "contract-exists".to_string(),
                            target: VerifyTarget::Contract { 
                                artifact_name: artifact.name.clone(),
                                path: contract_path.to_string_lossy().to_string() 
                            },
                            status: VerifyStatus::Pass,
                            message: format!("Contract exists for {}", artifact.name),
                        });

                        // Check Identity Alignment
                        match ContractConfig::load(&contract_path) {
                            Ok(contract_file) => {
                                let c = &contract_file.contract;
                                if c.name == artifact.name && c.role == artifact.role && c.module == artifact.module {
                                    results.push(CheckResult {
                                        check_id: "contract-identity".to_string(),
                                        target: VerifyTarget::Contract { 
                                            artifact_name: artifact.name.clone(),
                                            path: contract_path.to_string_lossy().to_string() 
                                        },
                                        status: VerifyStatus::Pass,
                                        message: format!("Contract identity matches plan for {}", artifact.name),
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
                                            "Contract identity mismatch for {}: Plan({}/{}/{}) vs Contract({}/{}/{})",
                                            artifact.name,
                                            artifact.name, artifact.role, artifact.module,
                                            c.name, c.role, c.module
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
