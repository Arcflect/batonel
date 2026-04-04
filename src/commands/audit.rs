use crate::config::{
    ArtifactsPlanConfig, ContractConfig, PlacementRulesConfig, PolicyProfileConfig, ProjectConfig,
};
use crate::model::artifact::Artifact;
use crate::model::placement::RolePlacement;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Severity {
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

#[derive(Debug, Clone)]
struct AuditFinding {
    rule_id: &'static str,
    severity: Severity,
    target: String,
    message: String,
    remediation: String,
}

pub fn execute(strict: bool) {
    let mut findings = Vec::new();

    let policy_config = match PolicyProfileConfig::load_or_default("policy.profile.yaml") {
        Ok(config) => config,
        Err(err) => {
            findings.push(AuditFinding {
                rule_id: "policy-profile-valid",
                severity: Severity::Error,
                target: "policy.profile.yaml".to_string(),
                message: format!("policy profile is invalid: {}", err),
                remediation:
                    "Fix policy.profile.yaml or remove it to fallback to the minimum default policy profile."
                        .to_string(),
            });
            render_report(&findings);
            std::process::exit(1);
        }
    };

    for required_file in &policy_config.required_files {
        check_required_root_file(required_file, &mut findings, &policy_config);
    }

    if findings.iter().any(|f| f.severity == Severity::Error) {
        render_report(&findings);
        std::process::exit(1);
    }

    let project_config = match ProjectConfig::load("project.arch.yaml") {
        Ok(config) => config,
        Err(err) => {
            findings.push(AuditFinding {
                rule_id: "project-config-valid",
                severity: Severity::Error,
                target: "project.arch.yaml".to_string(),
                message: format!("project configuration is invalid: {}", err),
                remediation: "Fix project.arch.yaml based on the validation message and rerun `archflow audit`.".to_string(),
            });
            render_report(&findings);
            std::process::exit(1);
        }
    };

    let placement_config = match PlacementRulesConfig::load("placement.rules.yaml") {
        Ok(config) => config,
        Err(err) => {
            findings.push(AuditFinding {
                rule_id: "placement-config-valid",
                severity: Severity::Error,
                target: "placement.rules.yaml".to_string(),
                message: format!("placement rules are invalid: {}", err),
                remediation: "Fix placement.rules.yaml and ensure each role has a valid path and optional extension.".to_string(),
            });
            render_report(&findings);
            std::process::exit(1);
        }
    };

    let artifacts_config = match ArtifactsPlanConfig::load("artifacts.plan.yaml") {
        Ok(config) => config,
        Err(err) => {
            findings.push(AuditFinding {
                rule_id: "artifacts-plan-valid",
                severity: Severity::Error,
                target: "artifacts.plan.yaml".to_string(),
                message: format!("artifacts plan is invalid: {}", err),
                remediation: "Fix artifacts.plan.yaml and ensure each artifact has name/module/role.".to_string(),
            });
            render_report(&findings);
            std::process::exit(1);
        }
    };

    findings.extend(run_policy_audit(
        &policy_config,
        &project_config,
        &placement_config,
        &artifacts_config,
    ));

    render_report(&findings);

    let has_errors = findings
        .iter()
        .any(|finding| finding.severity == Severity::Error);
    let has_warnings = findings
        .iter()
        .any(|finding| finding.severity == Severity::Warn);

    if has_errors || (strict && has_warnings) {
        std::process::exit(1);
    }
}

fn check_required_root_file(
    filename: &str,
    findings: &mut Vec<AuditFinding>,
    policy_config: &PolicyProfileConfig,
) {
    if policy_config.is_overridden("required-root-file", filename) {
        return;
    }

    if !Path::new(filename).exists() {
        findings.push(AuditFinding {
            rule_id: "required-root-file",
            severity: Severity::Error,
            target: filename.to_string(),
            message: format!("missing required root file: {}", filename),
            remediation: format!("Run `archflow init` to generate missing files, or add {} manually.", filename),
        });
    }
}

fn run_policy_audit(
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
                    rule_id: "module-name-policy",
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
                rule_id: "artifact-name-policy",
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
                    rule_id: "artifact-module-defined",
                    severity: Severity::Error,
                    target: artifact_target.clone(),
                    message: format!(
                        "artifact '{}' references undefined module '{}'",
                        artifact.name, artifact.module
                    ),
                    remediation: format!(
                        "Add module '{}' to project.arch.yaml or update artifact '{}' to an existing module.",
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
                        rule_id: "artifact-role-defined",
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
                    rule_id: "artifact-path-aligns-role",
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

    let contract = match ContractConfig::load(&contract_path) {
        Ok(config) => config.contract,
        Err(_) => return findings,
    };

    let contract_forbidden = contract.forbidden_dependencies.unwrap_or_default();
    for required_entry in forbidden_entries {
        if !contract_forbidden.iter().any(|entry| entry == required_entry) {
            findings.push(AuditFinding {
                rule_id: "policy-forbidden-dependencies-covered",
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

    println!("Archflow Audit Report");
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
            archflow: Some(crate::config::project::ArchflowMetadata {
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

        let findings = run_policy_audit(&policy, &project, &placement, &artifacts);

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

        let findings = run_policy_audit(&policy, &project, &placement, &artifacts);

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

        let findings = run_policy_audit(&policy, &project, &placement, &artifacts);

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "artifact-role-defined");
    }
}
