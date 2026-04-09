#[derive(Debug, Clone, Default)]
pub struct ValidateProjectInput;

#[derive(Debug, Clone)]
pub struct ValidateProjectOutput {
    pub success: bool,
}

pub struct ValidateProjectUseCase;

impl ValidateProjectUseCase {
    pub fn execute_with_output(
        _input: ValidateProjectInput,
        output: &mut dyn crate::ports::OutputPort,
    ) -> Result<ValidateProjectOutput, crate::app::error::ValidationError> {
        Self::execute_with_output_and_verifier(_input, output, || {
            crate::commands::verify::execute();
        })
    }

    fn execute_with_output_and_verifier<F>(
        _input: ValidateProjectInput,
        output: &mut dyn crate::ports::OutputPort,
        verify_runner: F,
    ) -> Result<ValidateProjectOutput, crate::app::error::ValidationError>
    where
        F: FnOnce(),
    {
        Self::execute_with_paths_output_and_verifier(
            _input,
            "project.arch.yaml",
            "placement.rules.yaml",
            "artifacts.plan.yaml",
            output,
            verify_runner,
        )
    }

    fn execute_with_paths_output_and_verifier<F>(
        _input: ValidateProjectInput,
        project_path: &str,
        placement_path: &str,
        artifacts_path: &str,
        output: &mut dyn crate::ports::OutputPort,
        verify_runner: F,
    ) -> Result<ValidateProjectOutput, crate::app::error::ValidationError>
    where
        F: FnOnce(),
    {
        let project = crate::config::ProjectConfig::load(project_path).map_err(|e| {
            crate::app::error::ConfigLoadError::Load {
                path: project_path.to_string(),
                source: e,
            }
        })?;
        let placement = crate::config::PlacementRulesConfig::load(placement_path)
            .map_err(|e| crate::app::error::ConfigLoadError::Load {
                path: placement_path.to_string(),
                source: e,
            })?;
        let artifacts = crate::config::ArtifactsPlanConfig::load(artifacts_path)
            .map_err(|e| crate::app::error::ConfigLoadError::Load {
                path: artifacts_path.to_string(),
                source: e,
            })?;

        let context = crate::domain::project::ProjectContext::from_project_config(&project);
        let result = crate::domain::validation::ArchitectureValidator::validate(
            &context,
            &placement,
            &artifacts,
        );
        if !result.is_valid() || result.warning_count() > 0 {
            output.write_line(
                crate::ports::OutputLevel::Warn,
                &format!(
                    "Structural validation: {} error(s), {} warning(s)",
                    result.error_count(),
                    result.warning_count()
                ),
            );
            for violation in &result.violations {
                let level = match violation.severity {
                    crate::domain::validation::ViolationSeverity::Error => {
                        crate::ports::OutputLevel::Error
                    }
                    crate::domain::validation::ViolationSeverity::Warn => {
                        crate::ports::OutputLevel::Warn
                    }
                };
                output.write_line(
                    level,
                    &format!(
                        "- [{}] {} ({})",
                        violation.rule_id, violation.message, violation.target
                    ),
                );
            }
        }

        verify_runner();
        Ok(ValidateProjectOutput { success: true })
    }
}

#[cfg(test)]
mod tests {
    use super::{ValidateProjectInput, ValidateProjectUseCase};
    use crate::ports::{OutputLevel, OutputPort};
    use tempfile::tempdir;

    #[derive(Default)]
    struct FakeOutput {
        lines: Vec<(OutputLevel, String)>,
    }

    impl OutputPort for FakeOutput {
        fn write_line(&mut self, level: OutputLevel, message: &str) {
            self.lines.push((level, message.to_string()));
        }
    }

    #[test]
    fn validate_returns_explicit_config_error_when_project_missing() {
        let mut output = FakeOutput::default();
        let result = ValidateProjectUseCase::execute_with_paths_output_and_verifier(
            ValidateProjectInput,
            "missing-project.arch.yaml",
            "missing-placement.rules.yaml",
            "missing-artifacts.plan.yaml",
            &mut output,
            || {},
        );

        let err = result.expect_err("expected config load error");
        let msg = err.to_string();
        assert!(msg.contains("missing-project.arch.yaml"));
    }

    #[test]
    fn validate_emits_structural_violations_to_output_port() {
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
  - name: create_order
    module: order
    role: usecase
"#,
        )
        .expect("write artifacts");

        let mut output = FakeOutput::default();
        let result = ValidateProjectUseCase::execute_with_paths_output_and_verifier(
            ValidateProjectInput,
            root.join("project.arch.yaml")
                .to_str()
                .expect("project path utf8"),
            root.join("placement.rules.yaml")
                .to_str()
                .expect("placement path utf8"),
            root.join("artifacts.plan.yaml")
                .to_str()
                .expect("artifacts path utf8"),
            &mut output,
            || {},
        )
        .expect("validate usecase should complete");

        assert!(result.success);
        assert!(
            output
                .lines
                .iter()
                .any(|(level, msg)| *level == OutputLevel::Warn
                    && msg.contains("Structural validation"))
        );
        assert!(
            output
                .lines
                .iter()
                .any(|(_, msg)| msg.contains("artifact-module-defined"))
        );
    }
}
