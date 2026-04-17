use crate::config::{
    ArtifactsPlanConfig, ContractConfig, PlacementRulesConfig, PolicyProfileConfig, ProjectConfig,
    override_policy,
};
use crate::model::artifact::Artifact;
use crate::model::placement::RolePlacement;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warn,
}

impl Severity {
    fn as_str(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warn => "warn",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    pub rule_id: String,
    pub severity: Severity,
    pub target: String,
    pub message: String,
    pub remediation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub repository: String,
    pub findings: Vec<AuditFinding>,
    pub errors: usize,
    pub warnings: usize,
    pub timestamp: String,
}

pub fn execute(strict: bool, evidence_export: Option<String>) {
    let report = match run_for_root(Path::new(".")) {
        Ok(report) => report,
        Err(err) => {
            eprintln!("[!] {}", err);
            std::process::exit(1);
        }
    };

    render_report(&report.findings);

    if let Some(export_path) = evidence_export {
        match serde_json::to_string_pretty(&report) {
            Ok(json_content) => {
                if let Err(e) = std::fs::write(&export_path, json_content) {
                    eprintln!("[!] Failed to write audit evidence to {}: {}", export_path, e);
                } else {
                    println!("\n[i] Audit evidence successfully exported to: {}", export_path);
                }
            }
            Err(e) => {
                eprintln!("[!] Failed to serialize audit evidence: {}", e);
            }
        }
    }

    if report.errors > 0 || (strict && report.warnings > 0) {
        std::process::exit(1);
    }
}

pub fn run_for_root(root: &Path) -> Result<AuditReport, String> {
    let mut findings = Vec::new();

    // Resolve effective policy by loading org/team/project layers in precedence order.
    let effective_policy = match override_policy::resolve(
        Some(&root.join(override_policy::ORG_POLICY_PATH)),
        Some(&root.join(override_policy::TEAM_POLICY_PATH)),
        Some(&root.join(override_policy::PROJECT_POLICY_PATH)),
    ) {
        Ok(ep) => ep,
        Err(err) => {
            findings.push(AuditFinding {
                rule_id: "policy-profile-valid".to_string(),
                severity: Severity::Error,
                target: "policy.profile.yaml".to_string(),
                message: format!("policy resolution failed: {}; hint: run `batonel policy-resolve` to diagnose", err),
                remediation: "Fix the policy layer files identified by `batonel policy-resolve`."
                    .to_string(),
            });
            return Ok(build_report(root, findings));
        }
    };
    let policy_config = effective_policy.to_policy_profile_config();

    for required_file in &policy_config.required_files {
        check_required_root_file(root, required_file, &mut findings, &policy_config);
    }

    if findings.iter().any(|f| f.severity == Severity::Error) {
        return Ok(build_report(root, findings));
    }

    let project_config = match ProjectConfig::load(root.join("project.baton.yaml")) {
        Ok(config) => config,
        Err(err) => {
            findings.push(AuditFinding {
                rule_id: "project-config-valid".to_string(),
                severity: Severity::Error,
                target: "project.baton.yaml".to_string(),
                message: format!("project configuration is invalid: {}", err),
                remediation: "Fix project.baton.yaml based on the validation message and rerun `batonel audit`.".to_string(),
            });
            return Ok(build_report(root, findings));
        }
    };

    let placement_config = match PlacementRulesConfig::load(root.join("placement.rules.yaml")) {
        Ok(config) => config,
        Err(err) => {
            findings.push(AuditFinding {
                rule_id: "placement-config-valid".to_string(),
                severity: Severity::Error,
                target: "placement.rules.yaml".to_string(),
                message: format!("placement rules are invalid: {}", err),
                remediation: "Fix placement.rules.yaml and ensure each role has a valid path and optional extension.".to_string(),
            });
            return Ok(build_report(root, findings));
        }
    };

    let artifacts_config = match ArtifactsPlanConfig::load(root.join("artifacts.plan.yaml")) {
        Ok(config) => config,
        Err(err) => {
            findings.push(AuditFinding {
                rule_id: "artifacts-plan-valid".to_string(),
                severity: Severity::Error,
                target: "artifacts.plan.yaml".to_string(),
                message: format!("artifacts plan is invalid: {}", err),
                remediation: "Fix artifacts.plan.yaml and ensure each artifact has name/module/role.".to_string(),
            });
            return Ok(build_report(root, findings));
        }
    };

    findings.extend(run_policy_audit(
        root,
        &policy_config,
        &project_config,
        &placement_config,
        &artifacts_config,
    ));

    Ok(build_report(root, findings))
}

fn build_report(root: &Path, findings: Vec<AuditFinding>) -> AuditReport {
    let errors = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Error)
        .count();
    let warnings = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Warn)
        .count();

    AuditReport {
        repository: root.display().to_string(),
        findings,
        errors,
        warnings,
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

fn check_required_root_file(
    root: &Path,
    filename: &str,
    findings: &mut Vec<AuditFinding>,
    policy_config: &PolicyProfileConfig,
) {
    if policy_config.is_overridden("required-root-file", filename) {
        return;
    }

    if !root.join(filename).exists() {
        findings.push(AuditFinding {
            rule_id: "required-root-file".to_string(),
            severity: Severity::Error,
            target: filename.to_string(),
            message: format!("missing required root file: {}", filename),
            remediation: format!("Run `batonel init` to generate missing files, or add {} manually.", filename),
        });
    }
}

fn run_policy_audit(
    root: &Path,
    policy_config: &PolicyProfileConfig,
    project_config: &ProjectConfig,
    placement_config: &PlacementRulesConfig,
    artifacts_config: &ArtifactsPlanConfig,
) -> Vec<AuditFinding> {
    let mut findings = Vec::new();

    for module in &project_config.modules {
        if !matches_naming_rule(&module.name, &policy_config.naming.module) {
            let target = format!("module:{}", module.name);
            if !policy_config.is_overridden("module-name-policy", &target) {
                findings.push(AuditFinding {
                    rule_id: "module-name-policy".to_string(),
                    severity: Severity::Error,
                    target,
                    message: format!(
                        "module '{}' does not satisfy naming rule '{:?}'",
                        module.name, policy_config.naming.module
                    ),
                    remediation:
                        "Rename the module to satisfy policy naming rules or add a targeted override in policy.profile.yaml."
                            .to_string(),
                });
            }
        }
    }

    for artifact in &artifacts_config.artifacts {
        let artifact_target = format!("artifact:{}", artifact.name);

        if !matches_naming_rule(&artifact.name, &policy_config.naming.artifact)
            && !policy_config.is_overridden("artifact-name-policy", &artifact_target)
        {
            findings.push(AuditFinding {
                rule_id: "artifact-name-policy".to_string(),
                severity: Severity::Error,
                target: artifact_target.clone(),
                message: format!(
                    "artifact '{}' does not satisfy naming rule '{:?}'",
                    artifact.name, policy_config.naming.artifact
                ),
                remediation:
                    "Rename the artifact in artifacts.plan.yaml or add a targeted override in policy.profile.yaml."
                        .to_string(),
            });
        }

        if !project_config.has_module(&artifact.module) {
            if !policy_config.is_overridden("artifact-module-defined", &artifact_target) {
                findings.push(AuditFinding {
                    rule_id: "artifact-module-defined".to_string(),
                    severity: Severity::Error,
                    target: artifact_target.clone(),
                    message: format!(
                        "artifact '{}' references undefined module '{}'",
                        artifact.name, artifact.module
                    ),
                    remediation: format!(
                        "Add module '{}' to project.baton.yaml or update artifact '{}' to an existing module.",
                        artifact.module, artifact.name
                    ),
                });
            }
        }

        let role_config = match placement_config.roles.get(&artifact.role) {
            Some(role_config) => role_config,
            None => {
                if !policy_config.is_overridden("artifact-role-defined", &artifact_target) {
                    findings.push(AuditFinding {
                        rule_id: "artifact-role-defined".to_string(),
                        severity: Severity::Error,
                        target: artifact_target.clone(),
                        message: format!(
                            "artifact '{}' uses undefined role '{}'",
                            artifact.name, artifact.role
                        ),
                        remediation: format!(
                            "Define role '{}' in placement.rules.yaml or change artifact '{}' role.",
                            artifact.role, artifact.name
                        ),
                    });
                }
                continue;
            }
        };

        findings.extend(check_policy_forbidden_dependencies(
            root,
            policy_config,
            artifact,
            role_config,
            placement_config,
        ));

        if let Some(explicit_path) = artifact.path.as_deref() {
            let expected = expected_role_path(artifact, role_config);
            if explicit_path != expected
                && !policy_config.is_overridden("artifact-path-aligns-role", &artifact_target)
            {
                findings.push(AuditFinding {
                    rule_id: "artifact-path-aligns-role".to_string(),
                    severity: Severity::Warn,
                    target: artifact_target,
                    message: format!(
                        "explicit path '{}' deviates from role '{}' expected path '{}'",
                        explicit_path, artifact.role, expected
                    ),
                    remediation: "Either align artifact.path with role defaults or document the exception in project policy.".to_string(),
                });
            }
        }
    }

    findings
}

fn check_policy_forbidden_dependencies(
    root: &Path,
    policy_config: &PolicyProfileConfig,
    artifact: &Artifact,
    role_config: &RolePlacement,
    placement_config: &PlacementRulesConfig,
) -> Vec<AuditFinding> {
    let mut findings = Vec::new();
    let forbidden_entries = match policy_config.forbidden_entries_for_role(&artifact.role) {
        Some(entries) => entries,
        None => return findings,
    };

    let artifact_target = format!("artifact:{}", artifact.name);
    if policy_config.is_overridden("policy-forbidden-dependencies-covered", &artifact_target) {
        return findings;
    }

    let artifact_path = match crate::generator::resolver::resolve_artifact_path(artifact, placement_config) {
        Ok(path) => path,
        Err(_) => return findings,
    };
    let contract_path = crate::generator::resolver::resolve_sidecar_path(
        artifact,
        &artifact_path,
        role_config
            .sidecar
            .as_ref()
            .and_then(|sidecar| sidecar.contract_dir.as_deref()),
        "contract.yaml",
    );

    let contract = match ContractConfig::load(root.join(&contract_path)) {
        Ok(config) => config.contract,
        Err(_) => return findings,
    };

    let contract_forbidden = contract.forbidden_dependencies.unwrap_or_default();
    for required_entry in forbidden_entries {
        if !contract_forbidden.iter().any(|entry| entry == required_entry) {
            findings.push(AuditFinding {
                rule_id: "policy-forbidden-dependencies-covered".to_string(),
                severity: Severity::Error,
                target: artifact_target.clone(),
                message: format!(
                    "role '{}' requires forbidden dependency '{}' in contract, but '{}' does not declare it",
                    artifact.role, required_entry, artifact.name
                ),
                remediation:
                    "Add the required forbidden dependency entry to the artifact contract, or add a targeted override in policy.profile.yaml."
                        .to_string(),
            });
        }
    }

    findings
}

fn matches_naming_rule(value: &str, rule: &crate::config::policy::NamingRule) -> bool {
    match rule {
        crate::config::policy::NamingRule::KebabCase => is_kebab_case(value),
        crate::config::policy::NamingRule::LowercaseIdentifier => is_lowercase_identifier(value),
    }
}

fn is_kebab_case(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
}

fn is_lowercase_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_'
        })
        && !value.starts_with(['-', '_'])
        && !value.ends_with(['-', '_'])
}

fn expected_role_path(artifact: &Artifact, role: &RolePlacement) -> String {
    let mut path = role.path.clone();
    if !path.ends_with('/') {
        path.push('/');
    }
    match role.file_extension.as_deref() {
        Some(ext) => format!("{}{}.{}", path, artifact.name, ext),
        None => format!("{}{}", path, artifact.name),
    }
}

fn render_report(findings: &[AuditFinding]) {
    let errors = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Error)
        .count();
    let warnings = findings
        .iter()
        .filter(|finding| finding.severity == Severity::Warn)
        .count();

    println!("Batonel Audit Report");
    println!("=====================");
    println!(
        "Summary: {} issue(s) detected (errors={}, warnings={})",
        findings.len(), errors, warnings
    );

    if findings.is_empty() {
        println!("Status: PASSED");
        return;
    }

    println!("Status: {}", if errors > 0 { "FAILED" } else { "PASSED WITH WARNINGS" });
    println!();

    for finding in findings {
        println!(
            "- [{}][{}] {}",
            finding.rule_id,
            finding.severity.as_str(),
            finding.message
        );
        println!("  target: {}", finding.target);
        println!("  remediation: {}", finding.remediation);
    }
}

#[cfg(test)]
mod tests {
    use super::{run_policy_audit, Severity};
    use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig, PolicyProfileConfig, ProjectConfig};
    use crate::model::artifact::Artifact;
    use crate::model::placement::RolePlacement;
    use crate::model::project::{Module, Project};
    use std::collections::HashMap;

    fn base_project() -> ProjectConfig {
        ProjectConfig {
            batonel: Some(crate::config::project::BatonelMetadata {
                schema_version: crate::config::project::SUPPORTED_PROJECT_SCHEMA_VERSION.to_string(),
                preset: None,
            }),
            project: Project {
                name: "demo-app".to_string(),
                architecture_style: "layered".to_string(),
                language: "generic".to_string(),
            },
            workspace: None,
            modules: vec![Module {
                name: "user".to_string(),
                features: None,
            }],
            metadata: None,
        }
    }

    #[test]
    fn baseline_audit_reports_undefined_module_and_role() {
        let project = base_project();
        let placement = PlacementRulesConfig { roles: HashMap::new() };
        let policy = PolicyProfileConfig::default_minimum();
        let artifacts = ArtifactsPlanConfig {
            artifacts: vec![Artifact {
                name: "create_order".to_string(),
                module: "order".to_string(),
                role: "usecase".to_string(),
                path: None,
                inputs: None,
                outputs: None,
                status: None,
                tags: None,
            }],
        };

        let findings = run_policy_audit(
            std::path::Path::new("."),
            &policy,
            &project,
            &placement,
            &artifacts,
        );

        assert_eq!(findings.len(), 2);
        assert!(findings.iter().any(|f| f.rule_id == "artifact-module-defined" && f.severity == Severity::Error));
        assert!(findings.iter().any(|f| f.rule_id == "artifact-role-defined" && f.severity == Severity::Error));
    }

    #[test]
    fn baseline_audit_warns_for_path_deviation() {
        let project = base_project();
        let policy = PolicyProfileConfig::default_minimum();
        let mut roles = HashMap::new();
        roles.insert(
            "usecase".to_string(),
            RolePlacement {
                path: "src/application/usecases".to_string(),
                file_extension: Some("rs".to_string()),
                sidecar: None,
            },
        );
        let placement = PlacementRulesConfig { roles };
        let artifacts = ArtifactsPlanConfig {
            artifacts: vec![Artifact {
                name: "create_user".to_string(),
                module: "user".to_string(),
                role: "usecase".to_string(),
                path: Some("custom/path/create_user.rs".to_string()),
                inputs: None,
                outputs: None,
                status: None,
                tags: None,
            }],
        };

        let findings = run_policy_audit(
            std::path::Path::new("."),
            &policy,
            &project,
            &placement,
            &artifacts,
        );

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "artifact-path-aligns-role");
        assert_eq!(findings[0].severity, Severity::Warn);
        assert!(!findings[0].remediation.is_empty());
    }

    #[test]
    fn baseline_audit_honors_policy_override() {
        let project = base_project();
        let placement = PlacementRulesConfig { roles: HashMap::new() };
        let mut policy = PolicyProfileConfig::default_minimum();
        policy.overrides.push(crate::config::policy::PolicyOverride {
            rule_id: "artifact-module-defined".to_string(),
            targets: vec!["artifact:create_order".to_string()],
            reason: "legacy migration".to_string(),
            expires_at: None,
        });
        let artifacts = ArtifactsPlanConfig {
            artifacts: vec![Artifact {
                name: "create_order".to_string(),
                module: "order".to_string(),
                role: "usecase".to_string(),
                path: None,
                inputs: None,
                outputs: None,
                status: None,
                tags: None,
            }],
        };

        let findings = run_policy_audit(
            std::path::Path::new("."),
            &policy,
            &project,
            &placement,
            &artifacts,
        );

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "artifact-role-defined");
    }
}
