use crate::config::{ArtifactsPlanConfig, GuardSidecarConfig, PlacementRulesConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardHookPoint {
    Init,
    Plan,
    Ci,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardSeverity {
    Error,
    Warn,
}

impl GuardSeverity {
    fn as_str(self) -> &'static str {
        match self {
            GuardSeverity::Error => "error",
            GuardSeverity::Warn => "warn",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GuardFinding {
    pub rule_id: &'static str,
    pub severity: GuardSeverity,
    pub target: String,
    pub message: String,
    pub remediation: String,
}

#[derive(Debug, Clone)]
pub struct GuardReport {
    pub findings: Vec<GuardFinding>,
}

impl GuardReport {
    pub fn has_errors(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == GuardSeverity::Error)
    }
}

pub fn execute_cli(hook: GuardHookPoint, strict: bool) {
    let report = run_hook(hook, None);
    render_report(&report);

    let should_fail = report.has_errors()
        || (strict
            && report
                .findings
                .iter()
                .any(|finding| finding.severity == GuardSeverity::Warn));
    if should_fail {
        std::process::exit(1);
    }
}

pub fn run_hook(hook: GuardHookPoint, init_planned_files: Option<&[String]>) -> GuardReport {
    let mut findings = Vec::new();

    let guard_config = match GuardSidecarConfig::load_optional("guard.sidecar.yaml") {
        Ok(Some(config)) => config,
        Ok(None) => {
            findings.push(GuardFinding {
                rule_id: "guard-rules-unavailable",
                severity: GuardSeverity::Warn,
                target: "guard.sidecar.yaml".to_string(),
                message: "guard rules file not found; fallback guard policy is applied".to_string(),
                remediation:
                    "Add guard.sidecar.yaml to customize guard behavior per repository."
                        .to_string(),
            });
            GuardSidecarConfig::default_fallback()
        }
        Err(err) => {
            findings.push(GuardFinding {
                rule_id: "guard-rules-unavailable",
                severity: GuardSeverity::Warn,
                target: "guard.sidecar.yaml".to_string(),
                message: format!(
                    "guard rules could not be loaded ({}); fallback guard policy is applied",
                    err
                ),
                remediation:
                    "Fix guard.sidecar.yaml parsing/validation issues to restore explicit guard rules."
                        .to_string(),
            });
            GuardSidecarConfig::default_fallback()
        }
    };

    if !is_hook_enabled(&guard_config, hook) {
        return GuardReport { findings };
    }

    if guard_config.checks.require_contracts_template
        && !file_exists_or_planned("contracts.template.yaml", init_planned_files)
    {
        findings.push(GuardFinding {
            rule_id: "guard-contract-template-required",
            severity: GuardSeverity::Error,
            target: "contracts.template.yaml".to_string(),
            message: "contracts.template.yaml is required by sidecar guard policy".to_string(),
            remediation:
                "Create contracts.template.yaml (or run `batonel init`) before continuing."
                    .to_string(),
        });
    }

    if matches!(hook, GuardHookPoint::Plan | GuardHookPoint::Ci) {
        let placement_config = match PlacementRulesConfig::load("placement.rules.yaml") {
            Ok(config) => Some(config),
            Err(err) => {
                findings.push(GuardFinding {
                    rule_id: "guard-placement-required",
                    severity: GuardSeverity::Error,
                    target: "placement.rules.yaml".to_string(),
                    message: format!(
                        "placement rules are required for guard sidecar checks: {}",
                        err
                    ),
                    remediation:
                        "Fix placement.rules.yaml so guard can resolve sidecar paths.".to_string(),
                });
                None
            }
        };

        let artifacts_config = match ArtifactsPlanConfig::load("artifacts.plan.yaml") {
            Ok(config) => Some(config),
            Err(err) => {
                findings.push(GuardFinding {
                    rule_id: "guard-artifacts-required",
                    severity: GuardSeverity::Error,
                    target: "artifacts.plan.yaml".to_string(),
                    message: format!(
                        "artifacts plan is required for guard sidecar checks: {}",
                        err
                    ),
                    remediation:
                        "Fix artifacts.plan.yaml so guard can evaluate sidecar coverage.".to_string(),
                });
                None
            }
        };

        if guard_config.checks.require_role_templates_for_artifact_roles
            && file_exists_or_planned("contracts.template.yaml", init_planned_files)
        {
            findings.extend(check_role_template_coverage(
                artifacts_config.as_ref(),
                "contracts.template.yaml",
            ));
        }

        if guard_config.checks.enforce_sidecar_suffixes {
            if let (Some(artifacts), Some(placement)) = (&artifacts_config, &placement_config) {
                findings.extend(check_sidecar_suffixes(artifacts, placement));
            }
        }
    }

    GuardReport { findings }
}

pub fn render_report(report: &GuardReport) {
    let errors = report
        .findings
        .iter()
        .filter(|finding| finding.severity == GuardSeverity::Error)
        .count();
    let warnings = report
        .findings
        .iter()
        .filter(|finding| finding.severity == GuardSeverity::Warn)
        .count();

    println!("Batonel Guard Sidecar Report");
    println!("============================");
    println!(
        "Summary: {} issue(s) detected (errors={}, warnings={})",
        report.findings.len(), errors, warnings
    );

    if report.findings.is_empty() {
        println!("Status: PASSED");
        return;
    }

    println!(
        "Status: {}",
        if errors > 0 {
            "FAILED"
        } else {
            "PASSED WITH WARNINGS"
        }
    );
    println!();

    for finding in &report.findings {
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

fn is_hook_enabled(config: &GuardSidecarConfig, hook: GuardHookPoint) -> bool {
    match hook {
        GuardHookPoint::Init => config.hooks.init,
        GuardHookPoint::Plan => config.hooks.plan,
        GuardHookPoint::Ci => config.hooks.ci,
    }
}

fn file_exists_or_planned(filename: &str, init_planned_files: Option<&[String]>) -> bool {
    std::path::Path::new(filename).exists()
        || init_planned_files
            .map(|entries| entries.iter().any(|entry| entry == filename))
            .unwrap_or(false)
}

fn check_role_template_coverage(
    artifacts_config: Option<&ArtifactsPlanConfig>,
    template_path: &str,
) -> Vec<GuardFinding> {
    let mut findings = Vec::new();
    let artifacts_config = match artifacts_config {
        Some(config) => config,
        None => return findings,
    };

    let templates_content = match std::fs::read_to_string(template_path) {
        Ok(content) => content,
        Err(err) => {
            findings.push(GuardFinding {
                rule_id: "guard-contract-template-readable",
                severity: GuardSeverity::Error,
                target: template_path.to_string(),
                message: format!(
                    "contract template file could not be read for sidecar guard checks: {}",
                    err
                ),
                remediation:
                    "Ensure contracts.template.yaml exists and is readable in repository root."
                        .to_string(),
            });
            return findings;
        }
    };

    let template_value = match serde_yaml::from_str::<serde_yaml::Value>(&templates_content) {
        Ok(value) => value,
        Err(err) => {
            findings.push(GuardFinding {
                rule_id: "guard-contract-template-parse",
                severity: GuardSeverity::Error,
                target: template_path.to_string(),
                message: format!("contract template file parse failed: {}", err),
                remediation:
                    "Fix contracts.template.yaml YAML syntax so role templates can be validated."
                        .to_string(),
            });
            return findings;
        }
    };

    let role_templates = template_value
        .get("role_templates")
        .and_then(serde_yaml::Value::as_mapping);

    for artifact in &artifacts_config.artifacts {
        if !has_role_template(role_templates, &artifact.role) {
            findings.push(GuardFinding {
                rule_id: "guard-role-template-coverage",
                severity: GuardSeverity::Error,
                target: format!("artifact:{}", artifact.name),
                message: format!(
                    "artifact role '{}' has no matching template in contracts.template.yaml",
                    artifact.role
                ),
                remediation: format!(
                    "Add role_templates.{} to contracts.template.yaml to preserve contract-first sidecar behavior.",
                    artifact.role
                ),
            });
        }
    }

    findings
}

fn has_role_template(
    role_templates: Option<&serde_yaml::Mapping>,
    role_name: &str,
) -> bool {
    role_templates
        .and_then(|mapping| mapping.get(serde_yaml::Value::String(role_name.to_string())))
        .is_some()
}

fn check_sidecar_suffixes(
    artifacts_config: &ArtifactsPlanConfig,
    placement_config: &PlacementRulesConfig,
) -> Vec<GuardFinding> {
    let mut findings = Vec::new();

    for artifact in &artifacts_config.artifacts {
        let role_config = match placement_config.roles.get(&artifact.role) {
            Some(config) => config,
            None => continue,
        };

        let artifact_path = match crate::generator::resolver::resolve_artifact_path(artifact, placement_config)
        {
            Ok(path) => path,
            Err(_) => continue,
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
        let prompt_path = crate::generator::resolver::resolve_sidecar_path(
            artifact,
            &artifact_path,
            role_config
                .sidecar
                .as_ref()
                .and_then(|sidecar| sidecar.prompt_dir.as_deref()),
            "prompt.md",
        );

        if !contract_path.to_string_lossy().ends_with(".contract.yaml") {
            findings.push(GuardFinding {
                rule_id: "guard-contract-sidecar-suffix",
                severity: GuardSeverity::Error,
                target: format!("artifact:{}", artifact.name),
                message: format!(
                    "contract sidecar path '{}' must end with '.contract.yaml'",
                    contract_path.to_string_lossy()
                ),
                remediation:
                    "Update sidecar path overrides to keep contract sidecar naming invariant."
                        .to_string(),
            });
        }

        if !prompt_path.to_string_lossy().ends_with(".prompt.md") {
            findings.push(GuardFinding {
                rule_id: "guard-prompt-sidecar-suffix",
                severity: GuardSeverity::Error,
                target: format!("artifact:{}", artifact.name),
                message: format!(
                    "prompt sidecar path '{}' must end with '.prompt.md'",
                    prompt_path.to_string_lossy()
                ),
                remediation:
                    "Update sidecar path overrides to keep prompt sidecar naming invariant."
                        .to_string(),
            });
        }
    }

    findings
}
