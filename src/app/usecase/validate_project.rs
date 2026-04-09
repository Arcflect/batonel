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
        let project = crate::config::ProjectConfig::load("project.arch.yaml").map_err(|e| {
            crate::app::error::ConfigLoadError::Load {
                path: "project.arch.yaml".to_string(),
                source: e,
            }
        })?;
        let placement = crate::config::PlacementRulesConfig::load("placement.rules.yaml")
            .map_err(|e| crate::app::error::ConfigLoadError::Load {
                path: "placement.rules.yaml".to_string(),
                source: e,
            })?;
        let artifacts = crate::config::ArtifactsPlanConfig::load("artifacts.plan.yaml")
            .map_err(|e| crate::app::error::ConfigLoadError::Load {
                path: "artifacts.plan.yaml".to_string(),
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

        crate::commands::verify::execute();
        Ok(ValidateProjectOutput { success: true })
    }
}
