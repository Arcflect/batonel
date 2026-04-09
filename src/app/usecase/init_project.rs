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

#[cfg(test)]
mod tests {
    use super::{is_kebab_case, InitProjectInput, InitProjectUseCase};

    #[test]
    fn kebab_case_validator_accepts_and_rejects_expected_patterns() {
        assert!(is_kebab_case("rust-clean-hexagonal"));
        assert!(is_kebab_case("preset1"));
        assert!(!is_kebab_case(""));
        assert!(!is_kebab_case("Rust-Clean"));
        assert!(!is_kebab_case("bad__name"));
        assert!(!is_kebab_case("-starts-with-dash"));
        assert!(!is_kebab_case("ends-with-dash-"));
        assert!(!is_kebab_case("double--dash"));
    }

    #[test]
    fn execute_rejects_empty_preset_id_before_running_init_flow() {
        let result = InitProjectUseCase::execute(InitProjectInput {
            preset: Some("   ".to_string()),
            project_name: None,
            dry_run: true,
        });

        let err = result.expect_err("empty preset id should be rejected");
        assert!(err.to_string().contains("preset id must not be empty"));
    }

    #[test]
    fn execute_rejects_non_kebab_case_preset_id() {
        let result = InitProjectUseCase::execute(InitProjectInput {
            preset: Some("Rust_Clean".to_string()),
            project_name: None,
            dry_run: true,
        });

        let err = result.expect_err("invalid preset id should be rejected");
        assert!(err.to_string().contains("kebab-case"));
    }
}
