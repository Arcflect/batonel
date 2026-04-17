use crate::config::{GuardSidecarConfig, PlacementRulesConfig};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentSeverity {
    Error,
    Warn,
}

impl AlignmentSeverity {
    fn as_str(self) -> &'static str {
        match self {
            AlignmentSeverity::Error => "error",
            AlignmentSeverity::Warn => "warn",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AlignmentFinding {
    pub rule_id: &'static str,
    pub severity: AlignmentSeverity,
    pub target: String,
    pub message: String,
    pub remediation: String,
}

#[derive(Debug, Clone)]
pub struct AlignmentReport {
    pub preset_name: String,
    pub preset_version: String,
    pub findings: Vec<AlignmentFinding>,
}

impl AlignmentReport {
    pub fn has_errors(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.severity == AlignmentSeverity::Error)
    }
}

// ---------------------------------------------------------------------------
// Minimal manifest types (preset.yaml)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct MinimalManifest {
    name: String,
    #[serde(default)]
    includes: MinimalIncludes,
    #[serde(default)]
    package: MinimalPackage,
}

#[derive(Debug, Deserialize, Default)]
struct MinimalIncludes {
    #[serde(default)]
    required: Vec<String>,
    #[serde(default)]
    optional: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
struct MinimalPackage {
    #[serde(default)]
    version: String,
}

// ---------------------------------------------------------------------------
// Public entry points
// ---------------------------------------------------------------------------

pub fn execute_cli(preset_dir: &str, strict: bool) {
    let preset_dir_path = PathBuf::from(preset_dir);
    let report = run_alignment_check(&preset_dir_path);
    render_report(&report);

    let should_fail = report.has_errors()
        || (strict
            && report
                .findings
                .iter()
                .any(|finding| finding.severity == AlignmentSeverity::Warn));
    if should_fail {
        std::process::exit(1);
    }
}

/// Run all contract-first and sidecar-first alignment checks against a preset directory.
pub fn run_alignment_check(preset_dir: &Path) -> AlignmentReport {
    let manifest_path = preset_dir.join("preset.yaml");
    let manifest = match load_minimal_manifest(&manifest_path) {
        Ok(manifest) => manifest,
        Err(err) => {
            return AlignmentReport {
                preset_name: preset_dir
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "(unknown)".to_string()),
                preset_version: "unknown".to_string(),
                findings: vec![AlignmentFinding {
                    rule_id: "preset-manifest-unreadable",
                    severity: AlignmentSeverity::Error,
                    target: "preset.yaml".to_string(),
                    message: format!("failed to load preset manifest: {}", err),
                    remediation: "Ensure preset.yaml exists and is valid YAML.".to_string(),
                }],
            };
        }
    };

    let mut findings = Vec::new();

    // Collect all listed files (required + optional) for presence cross-checks.
    let all_includes: Vec<&String> = manifest
        .includes
        .required
        .iter()
        .chain(manifest.includes.optional.iter())
        .collect();

    // -----------------------------------------------------------------------
    // CONTRACT-FIRST checks
    // -----------------------------------------------------------------------

    // Rule: contracts.template.yaml must be in includes.required.
    let contracts_in_required = manifest
        .includes
        .required
        .iter()
        .any(|entry| entry == "contracts.template.yaml");
    if !contracts_in_required {
        let in_optional = all_includes
            .iter()
            .any(|entry| *entry == "contracts.template.yaml");
        if in_optional {
            findings.push(AlignmentFinding {
                rule_id: "preset-contracts-template-in-required",
                severity: AlignmentSeverity::Error,
                target: "preset.yaml: includes.required".to_string(),
                message: "contracts.template.yaml is listed as optional but must be required for contract-first alignment".to_string(),
                remediation: "Move contracts.template.yaml from includes.optional to includes.required.".to_string(),
            });
        } else {
            findings.push(AlignmentFinding {
                rule_id: "preset-contracts-template-in-required",
                severity: AlignmentSeverity::Error,
                target: "preset.yaml: includes.required".to_string(),
                message: "contracts.template.yaml is absent from preset includes; contract-first behavior requires it".to_string(),
                remediation: "Add contracts.template.yaml to includes.required in preset.yaml.".to_string(),
            });
        }
    }

    // Rule: placement.rules.yaml must be in includes.required.
    if !manifest
        .includes
        .required
        .iter()
        .any(|entry| entry == "placement.rules.yaml")
    {
        findings.push(AlignmentFinding {
            rule_id: "preset-placement-rules-in-required",
            severity: AlignmentSeverity::Error,
            target: "preset.yaml: includes.required".to_string(),
            message: "placement.rules.yaml must be in includes.required for sidecar path resolution".to_string(),
            remediation: "Add placement.rules.yaml to includes.required in preset.yaml.".to_string(),
        });
    }

    // Rule: contracts.template.yaml content checks.
    let contracts_template_path = preset_dir.join("contracts.template.yaml");
    let role_template_names = if contracts_template_path.exists() {
        check_contracts_template(&contracts_template_path, &mut findings)
    } else {
        Vec::new()
    };

    // Rule: placement.rules.yaml content checks and coverage vs role_templates.
    let placement_path = preset_dir.join("placement.rules.yaml");
    if placement_path.exists() {
        check_placement_rules(
            &placement_path,
            &role_template_names,
            &mut findings,
        );
    }

    // -----------------------------------------------------------------------
    // SIDECAR-FIRST checks
    // -----------------------------------------------------------------------

    // Rule: guard.sidecar.yaml should appear in preset includes.
    let guard_included = all_includes.iter().any(|entry| *entry == "guard.sidecar.yaml");
    if !guard_included {
        findings.push(AlignmentFinding {
            rule_id: "preset-guard-sidecar-present",
            severity: AlignmentSeverity::Warn,
            target: "preset.yaml: includes".to_string(),
            message: "guard.sidecar.yaml is not in preset includes; sidecar-first behavior benefits from embedded guard rules".to_string(),
            remediation: "Add guard.sidecar.yaml to includes.optional or includes.required in preset.yaml.".to_string(),
        });
    }

    // Rule: guard.sidecar.yaml checks when the file is present.
    let guard_sidecar_path = preset_dir.join("guard.sidecar.yaml");
    if guard_sidecar_path.exists() {
        check_guard_sidecar(&guard_sidecar_path, &mut findings);
    }

    AlignmentReport {
        preset_name: manifest.name,
        preset_version: manifest.package.version,
        findings,
    }
}

pub fn render_report(report: &AlignmentReport) {
    let errors = report
        .findings
        .iter()
        .filter(|finding| finding.severity == AlignmentSeverity::Error)
        .count();
    let warnings = report
        .findings
        .iter()
        .filter(|finding| finding.severity == AlignmentSeverity::Warn)
        .count();

    println!("Batonel Preset Alignment Report");
    println!("================================");
    println!(
        "Preset: {} ({})",
        report.preset_name, report.preset_version
    );
    println!(
        "Summary: {} issue(s) detected (errors={}, warnings={})",
        report.findings.len(),
        errors,
        warnings
    );

    if report.findings.is_empty() {
        println!("Status: PASSED");
        return;
    }

    println!(
        "Status: {}",
        if errors > 0 { "FAILED" } else { "PASSED WITH WARNINGS" }
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

// ---------------------------------------------------------------------------
// Internal alignment checks
// ---------------------------------------------------------------------------

/// Check contracts.template.yaml content.
/// Returns the list of role template names found (for cross-checking placement roles).
fn check_contracts_template(path: &Path, findings: &mut Vec<AlignmentFinding>) -> Vec<String> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            findings.push(AlignmentFinding {
                rule_id: "preset-contracts-template-readable",
                severity: AlignmentSeverity::Error,
                target: "contracts.template.yaml".to_string(),
                message: format!("contracts.template.yaml could not be read: {}", err),
                remediation: "Ensure contracts.template.yaml is readable.".to_string(),
            });
            return Vec::new();
        }
    };

    let value = match serde_yaml::from_str::<serde_yaml::Value>(&content) {
        Ok(value) => value,
        Err(err) => {
            findings.push(AlignmentFinding {
                rule_id: "preset-contracts-template-parse",
                severity: AlignmentSeverity::Error,
                target: "contracts.template.yaml".to_string(),
                message: format!("contracts.template.yaml parse failed: {}", err),
                remediation: "Fix contracts.template.yaml YAML syntax.".to_string(),
            });
            return Vec::new();
        }
    };

    let role_templates = value
        .get("role_templates")
        .and_then(serde_yaml::Value::as_mapping);

    let is_empty = role_templates.map(|mapping| mapping.is_empty()).unwrap_or(true);
    if is_empty {
        findings.push(AlignmentFinding {
            rule_id: "preset-contracts-template-has-role-templates",
            severity: AlignmentSeverity::Error,
            target: "contracts.template.yaml: role_templates".to_string(),
            message: "contracts.template.yaml defines no role_templates; contract-first behavior requires at least one".to_string(),
            remediation: "Add role_templates entries to contracts.template.yaml.".to_string(),
        });
        return Vec::new();
    }

    let templates = role_templates.unwrap();
    let mut names = Vec::new();
    for (key, template_value) in templates {
        let role_name = key.as_str().unwrap_or("(unknown)");
        names.push(role_name.to_string());

        let responsibilities = template_value
            .get("responsibilities")
            .and_then(serde_yaml::Value::as_sequence);
        let has_responsibilities = responsibilities.map(|list| !list.is_empty()).unwrap_or(false);
        if !has_responsibilities {
            findings.push(AlignmentFinding {
                rule_id: "preset-role-template-has-responsibilities",
                severity: AlignmentSeverity::Warn,
                target: format!("contracts.template.yaml: role_templates.{}", role_name),
                message: format!(
                    "role template '{}' defines no responsibilities; it provides no contract guidance",
                    role_name
                ),
                remediation: format!(
                    "Add responsibilities to role_templates.{} in contracts.template.yaml.",
                    role_name
                ),
            });
        }
    }

    names
}

/// Check placement.rules.yaml content and coverage against known role templates.
fn check_placement_rules(
    path: &Path,
    known_role_template_names: &[String],
    findings: &mut Vec<AlignmentFinding>,
) {
    let config = match PlacementRulesConfig::load(path) {
        Ok(config) => config,
        Err(err) => {
            findings.push(AlignmentFinding {
                rule_id: "preset-placement-rules-parse",
                severity: AlignmentSeverity::Error,
                target: "placement.rules.yaml".to_string(),
                message: format!("placement.rules.yaml could not be parsed: {}", err),
                remediation: "Fix placement.rules.yaml YAML syntax.".to_string(),
            });
            return;
        }
    };

    for (role_name, role_config) in &config.roles {
        // Each placement role should have file_extension for sidecar suffix enforcement.
        if role_config.file_extension.is_none() {
            findings.push(AlignmentFinding {
                rule_id: "preset-placement-file-extension-present",
                severity: AlignmentSeverity::Warn,
                target: format!("placement.rules.yaml: roles.{}", role_name),
                message: format!(
                    "placement role '{}' has no file_extension; sidecar suffix enforcement cannot resolve source paths without it",
                    role_name
                ),
                remediation: format!(
                    "Add file_extension to role '{}' in placement.rules.yaml.",
                    role_name
                ),
            });
        }

        // Each placement role should have a matching role_template.
        // Only emit this check when we successfully loaded role_templates (list not empty).
        if !known_role_template_names.is_empty()
            && !known_role_template_names
                .iter()
                .any(|name| name == role_name)
        {
            findings.push(AlignmentFinding {
                rule_id: "preset-placement-role-in-template",
                severity: AlignmentSeverity::Warn,
                target: format!("contracts.template.yaml: role_templates.{}", role_name),
                message: format!(
                    "placement role '{}' has no matching role_template in contracts.template.yaml; this weakens contract-first behavior",
                    role_name
                ),
                remediation: format!(
                    "Add role_templates.{} to contracts.template.yaml.",
                    role_name
                ),
            });
        }
    }
}

/// Check guard.sidecar.yaml settings for sidecar-first alignment.
fn check_guard_sidecar(path: &Path, findings: &mut Vec<AlignmentFinding>) {
    let guard_config = match GuardSidecarConfig::load_optional(path) {
        Ok(Some(config)) => config,
        Ok(None) => return, // file not present at path (shouldn't happen since caller checked)
        Err(err) => {
            findings.push(AlignmentFinding {
                rule_id: "preset-guard-sidecar-parse",
                severity: AlignmentSeverity::Warn,
                target: "guard.sidecar.yaml".to_string(),
                message: format!(
                    "guard.sidecar.yaml exists but could not be parsed: {}",
                    err
                ),
                remediation: "Fix guard.sidecar.yaml YAML syntax.".to_string(),
            });
            return;
        }
    };

    if !guard_config.hooks.ci {
        findings.push(AlignmentFinding {
            rule_id: "preset-guard-ci-hook-enabled",
            severity: AlignmentSeverity::Warn,
            target: "guard.sidecar.yaml: hooks.ci".to_string(),
            message: "guard.sidecar.yaml has hooks.ci disabled; CI gate enforcement is recommended for sidecar-first behavior".to_string(),
            remediation: "Set hooks.ci: true in guard.sidecar.yaml.".to_string(),
        });
    }

    if !guard_config.checks.require_contracts_template {
        findings.push(AlignmentFinding {
            rule_id: "preset-guard-checks-require-contracts-template",
            severity: AlignmentSeverity::Warn,
            target: "guard.sidecar.yaml: checks.require_contracts_template".to_string(),
            message: "guard check require_contracts_template is disabled; this weakens contract-first behavior".to_string(),
            remediation: "Set checks.require_contracts_template: true in guard.sidecar.yaml."
                .to_string(),
        });
    }

    if !guard_config.checks.enforce_sidecar_suffixes {
        findings.push(AlignmentFinding {
            rule_id: "preset-guard-checks-enforce-sidecar-suffixes",
            severity: AlignmentSeverity::Warn,
            target: "guard.sidecar.yaml: checks.enforce_sidecar_suffixes".to_string(),
            message: "guard check enforce_sidecar_suffixes is disabled; sidecar outputs are not enforced as first-class".to_string(),
            remediation: "Set checks.enforce_sidecar_suffixes: true in guard.sidecar.yaml."
                .to_string(),
        });
    }
}

fn load_minimal_manifest(path: &Path) -> Result<MinimalManifest, String> {
    let contents = fs::read_to_string(path)
        .map_err(|err| format!("failed to read '{}': {}", path.display(), err))?;
    serde_yaml::from_str::<MinimalManifest>(&contents)
        .map_err(|err| format!("failed to parse '{}': {}", path.display(), err))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write_file(dir: &Path, name: &str, content: &str) {
        fs::write(dir.join(name), content).expect("failed to write test file");
    }

    fn minimal_valid_preset_yaml() -> &'static str {
        r#"
name: test-preset
summary: Test preset
architecture_style: layered
ecosystem: generic
includes:
  required:
    - project.baton.yaml
    - placement.rules.yaml
    - contracts.template.yaml
package:
  version: 0.1.0
  visibility: public
  compatibility:
    min_batonel_version: 0.1.0
    max_batonel_version: 0.1.99
    project_schema_version: "1"
    policy_profile_version: 1
"#
    }

    fn minimal_contracts_template_yaml() -> &'static str {
        r#"
role_templates:
  service:
    responsibilities:
      - "Execute one operation"
    must_not:
      - "Access infrastructure directly"
    implementation_size: "small"
"#
    }

    fn minimal_placement_rules_yaml() -> &'static str {
        r#"
roles:
  service:
    path: "src/application/services/"
    file_extension: rs
"#
    }

    fn valid_guard_sidecar_yaml() -> &'static str {
        r#"
version: 1
hooks:
  init: true
  plan: true
  ci: true
checks:
  require_contracts_template: true
  require_role_templates_for_artifact_roles: true
  enforce_sidecar_suffixes: true
"#
    }

    #[test]
    fn alignment_check_passes_fully_valid_preset() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path();

        write_file(preset_dir, "preset.yaml", minimal_valid_preset_yaml());
        write_file(
            preset_dir,
            "contracts.template.yaml",
            minimal_contracts_template_yaml(),
        );
        write_file(
            preset_dir,
            "placement.rules.yaml",
            minimal_placement_rules_yaml(),
        );
        write_file(
            preset_dir,
            "guard.sidecar.yaml",
            valid_guard_sidecar_yaml(),
        );

        // guard.sidecar.yaml is in optional includes to avoid preset-guard-sidecar-present warning
        let preset_yaml_with_guard = r#"
name: test-preset
summary: Test preset
architecture_style: layered
ecosystem: generic
includes:
  required:
    - project.baton.yaml
    - placement.rules.yaml
    - contracts.template.yaml
  optional:
    - guard.sidecar.yaml
package:
  version: 0.1.0
  visibility: public
  compatibility:
    min_batonel_version: 0.1.0
    max_batonel_version: 0.1.99
    project_schema_version: "1"
    policy_profile_version: 1
"#;
        write_file(preset_dir, "preset.yaml", preset_yaml_with_guard);

        let report = run_alignment_check(preset_dir);
        assert!(
            !report.has_errors(),
            "Expected no errors, got: {:?}",
            report.findings
        );
        assert!(
            report.findings.is_empty(),
            "Expected no findings, got: {:?}",
            report.findings
        );
    }

    #[test]
    fn alignment_check_detects_contracts_template_in_optional() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path();

        let preset_yaml = r#"
name: test-preset
summary: Test preset
architecture_style: layered
ecosystem: generic
includes:
  required:
    - project.baton.yaml
    - placement.rules.yaml
  optional:
    - contracts.template.yaml
package:
  version: 0.1.0
  visibility: public
  compatibility:
    min_batonel_version: 0.1.0
    max_batonel_version: 0.1.99
    project_schema_version: "1"
    policy_profile_version: 1
"#;
        write_file(preset_dir, "preset.yaml", preset_yaml);
        write_file(
            preset_dir,
            "contracts.template.yaml",
            minimal_contracts_template_yaml(),
        );
        write_file(
            preset_dir,
            "placement.rules.yaml",
            minimal_placement_rules_yaml(),
        );

        let report = run_alignment_check(preset_dir);
        assert!(report.has_errors());
        let rule_ids: Vec<&str> = report.findings.iter().map(|f| f.rule_id).collect();
        assert!(
            rule_ids.contains(&"preset-contracts-template-in-required"),
            "Expected preset-contracts-template-in-required, got: {:?}",
            rule_ids
        );
    }

    #[test]
    fn alignment_check_detects_empty_role_templates() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path();

        write_file(preset_dir, "preset.yaml", minimal_valid_preset_yaml());
        write_file(
            preset_dir,
            "contracts.template.yaml",
            "role_templates: {}\n",
        );
        write_file(
            preset_dir,
            "placement.rules.yaml",
            minimal_placement_rules_yaml(),
        );

        let report = run_alignment_check(preset_dir);
        assert!(report.has_errors());
        let rule_ids: Vec<&str> = report.findings.iter().map(|f| f.rule_id).collect();
        assert!(
            rule_ids.contains(&"preset-contracts-template-has-role-templates"),
            "Expected preset-contracts-template-has-role-templates, got: {:?}",
            rule_ids
        );
    }

    #[test]
    fn alignment_check_warns_on_guard_ci_disabled() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path();

        let preset_yaml_with_guard = r#"
name: test-preset
summary: Test preset
architecture_style: layered
ecosystem: generic
includes:
  required:
    - project.baton.yaml
    - placement.rules.yaml
    - contracts.template.yaml
  optional:
    - guard.sidecar.yaml
package:
  version: 0.1.0
  visibility: public
  compatibility:
    min_batonel_version: 0.1.0
    max_batonel_version: 0.1.99
    project_schema_version: "1"
    policy_profile_version: 1
"#;
        write_file(preset_dir, "preset.yaml", preset_yaml_with_guard);
        write_file(
            preset_dir,
            "contracts.template.yaml",
            minimal_contracts_template_yaml(),
        );
        write_file(
            preset_dir,
            "placement.rules.yaml",
            minimal_placement_rules_yaml(),
        );
        write_file(
            preset_dir,
            "guard.sidecar.yaml",
            r#"
version: 1
hooks:
  init: true
  plan: true
  ci: false
checks:
  require_contracts_template: true
  require_role_templates_for_artifact_roles: true
  enforce_sidecar_suffixes: true
"#,
        );

        let report = run_alignment_check(preset_dir);
        assert!(!report.has_errors(), "Expected no errors but got: {:?}", report.findings);
        let rule_ids: Vec<&str> = report.findings.iter().map(|f| f.rule_id).collect();
        assert!(
            rule_ids.contains(&"preset-guard-ci-hook-enabled"),
            "Expected preset-guard-ci-hook-enabled warning, got: {:?}",
            rule_ids
        );
    }

    #[test]
    fn alignment_check_warns_when_guard_sidecar_absent_from_includes() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path();

        write_file(preset_dir, "preset.yaml", minimal_valid_preset_yaml());
        write_file(
            preset_dir,
            "contracts.template.yaml",
            minimal_contracts_template_yaml(),
        );
        write_file(
            preset_dir,
            "placement.rules.yaml",
            minimal_placement_rules_yaml(),
        );
        // guard.sidecar.yaml not listed in includes

        let report = run_alignment_check(preset_dir);
        let rule_ids: Vec<&str> = report.findings.iter().map(|f| f.rule_id).collect();
        assert!(
            rule_ids.contains(&"preset-guard-sidecar-present"),
            "Expected preset-guard-sidecar-present warning, got: {:?}",
            rule_ids
        );
    }

    #[test]
    fn alignment_check_warns_on_placement_role_without_template() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path();

        write_file(preset_dir, "preset.yaml", minimal_valid_preset_yaml());
        write_file(
            preset_dir,
            "contracts.template.yaml",
            minimal_contracts_template_yaml(), // only has "service" role template
        );
        // placement has "service" and "controller" — controller has no role template
        write_file(
            preset_dir,
            "placement.rules.yaml",
            r#"
roles:
  service:
    path: "src/application/services/"
    file_extension: rs
  controller:
    path: "src/interfaces/"
    file_extension: rs
"#,
        );

        let report = run_alignment_check(preset_dir);
        let rule_ids: Vec<&str> = report.findings.iter().map(|f| f.rule_id).collect();
        assert!(
            rule_ids.contains(&"preset-placement-role-in-template"),
            "Expected preset-placement-role-in-template warning, got: {:?}",
            rule_ids
        );
    }

    #[test]
    fn alignment_check_warns_on_placement_role_without_file_extension() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path();

        write_file(preset_dir, "preset.yaml", minimal_valid_preset_yaml());
        write_file(
            preset_dir,
            "contracts.template.yaml",
            minimal_contracts_template_yaml(),
        );
        write_file(
            preset_dir,
            "placement.rules.yaml",
            r#"
roles:
  service:
    path: "src/application/services/"
"#, // no file_extension
        );

        let report = run_alignment_check(preset_dir);
        let rule_ids: Vec<&str> = report.findings.iter().map(|f| f.rule_id).collect();
        assert!(
            rule_ids.contains(&"preset-placement-file-extension-present"),
            "Expected preset-placement-file-extension-present warning, got: {:?}",
            rule_ids
        );
    }
}
