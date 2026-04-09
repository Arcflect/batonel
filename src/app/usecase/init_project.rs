#[derive(Debug, Clone)]
pub struct InitProjectInput {
    pub preset: Option<String>,
    pub project_name: Option<String>,
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct InitProjectOutput {
    pub success: bool,
    pub resolved_preset: Option<String>,
}

pub struct InitProjectUseCase;

impl InitProjectUseCase {
    pub fn execute(input: InitProjectInput) -> InitProjectOutput {
        let resolved_preset = crate::domain::preset::PresetResolver::resolve(
            input.preset.as_deref(),
            None,
            None,
        )
        .map(|preset| preset.id);

        crate::commands::init::execute(
            input.preset.as_deref(),
            input.project_name.as_deref(),
            input.dry_run,
        );
        InitProjectOutput {
            success: true,
            resolved_preset,
        }
    }
}
