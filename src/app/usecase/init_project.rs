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
    pub fn execute(
        input: InitProjectInput,
    ) -> Result<InitProjectOutput, crate::app::error::AppError> {
        let resolved_preset = if let Some(preset_id) = input.preset.as_deref() {
            let trimmed = preset_id.trim();
            if trimmed.is_empty() {
                return Err(crate::app::error::PresetResolutionError::EmptyPresetId.into());
            }
            if !is_kebab_case(trimmed) {
                return Err(crate::app::error::PresetResolutionError::InvalidPresetId {
                    preset_id: trimmed.to_string(),
                }
                .into());
            }
            Some(trimmed.to_string())
        } else {
            crate::domain::preset::PresetResolver::resolve(None, None, None).map(|preset| preset.id)
        };

        crate::commands::init::execute(
            input.preset.as_deref(),
            input.project_name.as_deref(),
            input.dry_run,
        );
        Ok(InitProjectOutput {
            success: true,
            resolved_preset,
        })
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
