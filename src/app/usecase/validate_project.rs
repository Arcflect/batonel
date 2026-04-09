#[derive(Debug, Clone, Default)]
pub struct ValidateProjectInput;

#[derive(Debug, Clone)]
pub struct ValidateProjectOutput {
    pub success: bool,
}

pub struct ValidateProjectUseCase;

impl ValidateProjectUseCase {
    pub fn execute(_input: ValidateProjectInput) -> ValidateProjectOutput {
        if let (Ok(project), Ok(placement), Ok(artifacts)) = (
            crate::config::ProjectConfig::load("project.arch.yaml"),
            crate::config::PlacementRulesConfig::load("placement.rules.yaml"),
            crate::config::ArtifactsPlanConfig::load("artifacts.plan.yaml"),
        ) {
            let context = crate::domain::project::ProjectContext::from_project_config(&project);
            let result = crate::domain::validation::ArchitectureValidator::validate(
                &context,
                &placement,
                &artifacts,
            );
            if !result.is_valid() || result.warning_count() > 0 {
                eprintln!(
                    "[!] Structural validation: {} error(s), {} warning(s)",
                    result.error_count(),
                    result.warning_count()
                );
                for violation in &result.violations {
                    let level = match violation.severity {
                        crate::domain::validation::ViolationSeverity::Error => "error",
                        crate::domain::validation::ViolationSeverity::Warn => "warn",
                    };
                    eprintln!(
                        "    - [{}][{}] {} ({})",
                        violation.rule_id, level, violation.message, violation.target
                    );
                }
            }
        }

        crate::commands::verify::execute();
        ValidateProjectOutput { success: true }
    }
}
