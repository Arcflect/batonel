use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig, ProjectConfig};
use crate::model::artifact::Artifact;
use crate::model::placement::RolePlacement;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FixClass {
    AutoFixable,
    ReviewRequired,
}

#[derive(Debug, Clone)]
pub struct FixFinding {
    pub rule_id: &'static str,
    pub class: FixClass,
    pub target: String,
    pub message: String,
    pub remediation: String,
    pub patch_preview: Option<String>,
}

/// Collect all fix candidates for the given project root without applying anything.
pub fn collect_findings(root: &Path) -> Vec<FixFinding> {
    let mut findings = Vec::new();

    let required_files = [
        "project.baton.yaml",
        "placement.rules.yaml",
        "artifacts.plan.yaml",
        "contracts.template.yaml",
    ];

    for filename in required_files
        .iter()
        .copied()
        .filter(|f| !root.join(f).exists())
    {
        findings.push(FixFinding {
            rule_id: "required-root-file",
            class: FixClass::AutoFixable,
            target: filename.to_string(),
            message: format!("missing required root file: {}", filename),
            remediation: "Generate missing root files via `batonel init` (non-destructive for existing files).".to_string(),
            patch_preview: None,
        });
    }

    let project_config = ProjectConfig::load(root.join("project.baton.yaml")).ok();
    let placement_config = PlacementRulesConfig::load(root.join("placement.rules.yaml")).ok();
    let artifacts_config = ArtifactsPlanConfig::load(root.join("artifacts.plan.yaml")).ok();

    if let (Some(project), Some(placement), Some(artifacts)) =
        (project_config, placement_config, artifacts_config)
    {
        findings.extend(classify_review_required_findings(&project, &placement, &artifacts));
    }

    findings
}

pub fn execute(dry_run: bool, apply: bool) {
    if dry_run && apply {
        eprintln!("[!] --dry-run and --apply cannot be used together");
        std::process::exit(1);
    }

    let effective_dry_run = !apply || dry_run;
    let findings = collect_findings(Path::new("."));

    render_fix_report(&findings, effective_dry_run, apply);

    let auto_fixable_count = findings
        .iter()
        .filter(|finding| finding.class == FixClass::AutoFixable)
        .count();
    let review_required_count = findings
        .iter()
        .filter(|finding| finding.class == FixClass::ReviewRequired)
        .count();

    if apply {
        if auto_fixable_count > 0 {
            crate::commands::init::execute(None, None, false);
        }

        if review_required_count > 0 {
            eprintln!(
                "\n[!] {} review-required finding(s) were not auto-applied.",
                review_required_count
            );
            eprintln!("    Review patch previews above and apply manually.");
            std::process::exit(1);
        }

        return;
    }

    if review_required_count > 0 {
        std::process::exit(1);
    }
}

fn classify_review_required_findings(
    project_config: &ProjectConfig,
    placement_config: &PlacementRulesConfig,
    artifacts_config: &ArtifactsPlanConfig,
) -> Vec<FixFinding> {
    let mut findings = Vec::new();

    for artifact in &artifacts_config.artifacts {
        if !project_config.has_module(&artifact.module) {
            findings.push(FixFinding {
                rule_id: "artifact-module-defined",
                class: FixClass::ReviewRequired,
                target: format!("artifact:{}", artifact.name),
                message: format!(
                    "artifact '{}' references undefined module '{}'",
                    artifact.name, artifact.module
                ),
                remediation: "Choose whether to add the module or rebind the artifact to an existing module.".to_string(),
                patch_preview: Some(module_patch_preview(&artifact.module)),
            });
        }

        let role_config = match placement_config.roles.get(&artifact.role) {
            Some(role) => role,
            None => {
                findings.push(FixFinding {
                    rule_id: "artifact-role-defined",
                    class: FixClass::ReviewRequired,
                    target: format!("artifact:{}", artifact.name),
                    message: format!(
                        "artifact '{}' uses undefined role '{}'",
                        artifact.name, artifact.role
                    ),
                    remediation: "Define the role in placement.rules.yaml or change the artifact role.".to_string(),
                    patch_preview: Some(role_patch_preview(&artifact.role)),
                });
                continue;
            }
        };

        if let Some(explicit_path) = artifact.path.as_deref() {
            let expected = expected_role_path(artifact, role_config);
            if explicit_path != expected {
                findings.push(FixFinding {
                    rule_id: "artifact-path-aligns-role",
                    class: FixClass::ReviewRequired,
                    target: format!("artifact:{}", artifact.name),
                    message: format!(
                        "explicit path '{}' deviates from role '{}' expected path '{}'",
                        explicit_path, artifact.role, expected
                    ),
                    remediation: "Confirm architectural intent before changing path alignment.".to_string(),
                    patch_preview: Some(path_patch_preview(&artifact.name, explicit_path, &expected)),
                });
            }
        }
    }

    findings
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

fn module_patch_preview(module_name: &str) -> String {
    format!(
        "--- a/project.baton.yaml\n+++ b/project.baton.yaml\n@@\n modules:\n   - name: <existing_module>\n+  - name: {}\n",
        module_name
    )
}

fn role_patch_preview(role_name: &str) -> String {
    format!(
        "--- a/placement.rules.yaml\n+++ b/placement.rules.yaml\n@@\n roles:\n+  {}:\n+    path: \"src/<replace-me>/\"\n+    file_extension: rs\n",
        role_name
    )
}

fn path_patch_preview(artifact_name: &str, old_path: &str, new_path: &str) -> String {
    format!(
        "--- a/artifacts.plan.yaml\n+++ b/artifacts.plan.yaml\n@@\n   - name: {}\n-    path: {}\n+    path: {}\n",
        artifact_name, old_path, new_path
    )
}

fn render_fix_report(findings: &[FixFinding], dry_run: bool, apply: bool) {
    let auto_fixable = findings
        .iter()
        .filter(|finding| finding.class == FixClass::AutoFixable)
        .count();
    let review_required = findings
        .iter()
        .filter(|finding| finding.class == FixClass::ReviewRequired)
        .count();

    println!("Batonel Conservative Fix");
    println!("=========================");
    println!(
        "Mode: {}",
        if apply {
            "apply (low-risk fixes only)"
        } else if dry_run {
            "dry-run preview"
        } else {
            "preview"
        }
    );
    println!(
        "Summary: total={} auto_fixable={} review_required={}",
        findings.len(), auto_fixable, review_required
    );

    if findings.is_empty() {
        println!("No fix candidates found.");
        return;
    }

    println!();
    println!("Auto-fixable findings");
    println!("---------------------");
    for finding in findings
        .iter()
        .filter(|finding| finding.class == FixClass::AutoFixable)
    {
        println!("- [{}] {}", finding.rule_id, finding.message);
        println!("  target: {}", finding.target);
        println!("  remediation: {}", finding.remediation);
    }

    println!();
    println!("Review-required findings");
    println!("------------------------");
    for finding in findings
        .iter()
        .filter(|finding| finding.class == FixClass::ReviewRequired)
    {
        println!("- [{}] {}", finding.rule_id, finding.message);
        println!("  target: {}", finding.target);
        println!("  remediation: {}", finding.remediation);
        if let Some(patch) = &finding.patch_preview {
            println!("  patch preview:");
            for line in patch.lines() {
                println!("    {}", line);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_review_required_findings, expected_role_path, FixClass};
    use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig, ProjectConfig};
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
    fn classify_flags_module_and_role_as_review_required() {
        let project = base_project();
        let placement = PlacementRulesConfig {
            roles: HashMap::new(),
        };
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

        let findings = classify_review_required_findings(&project, &placement, &artifacts);
        assert_eq!(findings.len(), 2);
        assert!(findings
            .iter()
            .all(|finding| finding.class == FixClass::ReviewRequired));
    }

    #[test]
    fn expected_role_path_handles_extension() {
        let artifact = Artifact {
            name: "create_user".to_string(),
            module: "user".to_string(),
            role: "usecase".to_string(),
            path: None,
            inputs: None,
            outputs: None,
            status: None,
            tags: None,
        };
        let role = RolePlacement {
            path: "src/application/usecases".to_string(),
            file_extension: Some("rs".to_string()),
            sidecar: None,
        };

        assert_eq!(
            expected_role_path(&artifact, &role),
            "src/application/usecases/create_user.rs"
        );
    }
}
