#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanRenderFormat {
    Text,
    Json,
    Markdown,
}

#[derive(Debug, Clone)]
pub struct PlanArchitectureInput {
    pub format: PlanRenderFormat,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GuardFindingOutput {
    pub rule_id: String,
    pub severity: String,
    pub target: String,
    pub message: String,
    pub remediation: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlanArchitectureOutput {
    pub plan: crate::domain::planning::ArchitecturePlan,
    pub guard_findings: Vec<GuardFindingOutput>,
    pub guard_errors: usize,
    pub guard_warnings: usize,
    pub success: bool,
}

pub struct PlanArchitectureUseCase;

impl PlanArchitectureUseCase {
    pub fn execute(
        input: PlanArchitectureInput,
    ) -> Result<PlanArchitectureOutput, crate::app::error::PlanBuildError> {
        Self::execute_with_paths_and_guard(
            input,
            "project.arch.yaml",
            "placement.rules.yaml",
            "artifacts.plan.yaml",
            || {
                crate::commands::guard::run_hook(crate::commands::guard::GuardHookPoint::Plan, None)
            },
        )
    }

    fn execute_with_paths_and_guard<F>(
        input: PlanArchitectureInput,
        project_path: &str,
        placement_path: &str,
        artifacts_path: &str,
        guard_runner: F,
    ) -> Result<PlanArchitectureOutput, crate::app::error::PlanBuildError>
    where
        F: FnOnce() -> crate::commands::guard::GuardReport,
    {
        let _requested_format = input.format;

        let guard_report = guard_runner();

        let project = crate::config::ProjectConfig::load(project_path).map_err(|e| {
            crate::app::error::ConfigLoadError::Load {
                path: project_path.to_string(),
                source: e,
            }
        })?;
        let placement = crate::config::PlacementRulesConfig::load(placement_path).map_err(|e| {
            crate::app::error::ConfigLoadError::Load {
                path: placement_path.to_string(),
                source: e,
            }
        })?;
        let artifacts = crate::config::ArtifactsPlanConfig::load(artifacts_path).map_err(|e| {
            crate::app::error::ConfigLoadError::Load {
                path: artifacts_path.to_string(),
                source: e,
            }
        })?;

        let context = crate::domain::project::ProjectContext::from_project_config(&project);
        let plan = crate::domain::planning::ArchitecturePlanner::plan(&context, &placement, &artifacts);

        let guard_findings: Vec<GuardFindingOutput> = guard_report
            .findings
            .iter()
            .map(|f| GuardFindingOutput {
                rule_id: f.rule_id.to_string(),
                severity: match f.severity {
                    crate::commands::guard::GuardSeverity::Error => "error".to_string(),
                    crate::commands::guard::GuardSeverity::Warn => "warn".to_string(),
                },
                target: f.target.clone(),
                message: f.message.clone(),
                remediation: f.remediation.clone(),
            })
            .collect();

        let guard_errors = guard_findings
            .iter()
            .filter(|f| f.severity == "error")
            .count();
        let guard_warnings = guard_findings
            .iter()
            .filter(|f| f.severity == "warn")
            .count();

        let success = plan.error_count() == 0 && guard_errors == 0;

        Ok(PlanArchitectureOutput {
            plan,
            guard_findings,
            guard_errors,
            guard_warnings,
            success,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{PlanArchitectureInput, PlanArchitectureUseCase, PlanRenderFormat};
    use tempfile::tempdir;

    #[test]
    fn returns_config_load_error_with_target_path() {
        let result = PlanArchitectureUseCase::execute_with_paths_and_guard(
            PlanArchitectureInput {
                format: PlanRenderFormat::Json,
            },
            "missing-project.arch.yaml",
            "missing-placement.rules.yaml",
            "missing-artifacts.plan.yaml",
            || crate::commands::guard::GuardReport { findings: vec![] },
        );

        let err = result.expect_err("expected missing config error");
        assert!(err.to_string().contains("missing-project.arch.yaml"));
    }

    #[test]
    fn builds_plan_successfully_with_minimal_valid_configuration() {
        let temp = tempdir().expect("tempdir should be created");
        let root = temp.path();

        std::fs::write(
            root.join("project.arch.yaml"),
            r#"archflow:
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

        let output = PlanArchitectureUseCase::execute_with_paths_and_guard(
            PlanArchitectureInput {
                format: PlanRenderFormat::Text,
            },
            root.join("project.arch.yaml")
                .to_str()
                .expect("project path utf8"),
            root.join("placement.rules.yaml")
                .to_str()
                .expect("placement path utf8"),
            root.join("artifacts.plan.yaml")
                .to_str()
                .expect("artifacts path utf8"),
            || crate::commands::guard::GuardReport { findings: vec![] },
        )
        .expect("plan should succeed");

        assert!(output.success, "plan success should be true: {output:?}");
        assert_eq!(output.guard_errors, 0);
        assert_eq!(output.plan.error_count(), 0);
        assert_eq!(output.plan.artifacts.len(), 1);
    }
}
