#[derive(Debug, Clone, Default)]
pub struct GenerateArtifactsInput;

#[derive(Debug, Clone)]
pub struct GenerateArtifactsOutput {
    pub success: bool,
    pub generated_count: usize,
    pub error_count: usize,
}

pub struct GenerateArtifactsUseCase;

impl GenerateArtifactsUseCase {
    pub fn execute(
        input: GenerateArtifactsInput,
    ) -> Result<GenerateArtifactsOutput, crate::app::error::GenerationError> {
        Self::execute_with_paths(
            input,
            "project.baton.yaml",
            "placement.rules.yaml",
            "artifacts.plan.yaml",
        )
    }

    /// Testable variant with injected config paths.
    pub(crate) fn execute_with_paths(
        _input: GenerateArtifactsInput,
        project_path: &str,
        placement_path: &str,
        artifacts_path: &str,
    ) -> Result<GenerateArtifactsOutput, crate::app::error::GenerationError> {
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
        let generation_plan =
            crate::domain::generation::ArtifactGenerator::build_plan(&context, &placement, &artifacts);

        let mut generated_count = 0usize;
        let mut error_count = 0usize;

        for item in generation_plan.items {
            match item.status {
                crate::domain::generation::ArtifactGenerationStatus::Ready => {
                    let artifact = item.artifact;
                    let path = match item.resolved_path {
                        Some(p) => p,
                        None => {
                            error_count += 1;
                            continue;
                        }
                    };
                    let role_config = placement.roles.get(&artifact.role);
                    match crate::generator::scaffold::generate_artifact_with_sidecars(
                        &artifact,
                        &path,
                        role_config,
                    ) {
                        Ok(_) => generated_count += 1,
                        Err(_) => error_count += 1,
                    }
                }
                crate::domain::generation::ArtifactGenerationStatus::Error => {
                    error_count += 1;
                }
            }
        }

        Ok(GenerateArtifactsOutput {
            success: error_count == 0,
            generated_count,
            error_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{GenerateArtifactsInput, GenerateArtifactsUseCase};
    use tempfile::tempdir;

    #[test]
    fn returns_config_load_error_when_files_missing() {
        let result = GenerateArtifactsUseCase::execute_with_paths(
            GenerateArtifactsInput,
            "missing-project.baton.yaml",
            "missing-placement.rules.yaml",
            "missing-artifacts.plan.yaml",
        );
        let err = result.expect_err("expected config load error");
        assert!(err.to_string().contains("missing-project.baton.yaml"));
    }

    #[test]
    fn generates_returns_output_for_minimal_valid_config() {
        let temp = tempdir().expect("tempdir should be created");
        let root = temp.path();

        std::fs::write(
            root.join("project.baton.yaml"),
            r#"batonel:
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

        // execute_with_paths loads configs and runs the plan; the actual file
        // writes are CWD-relative so we don't assert on success here — that is
        // validated by the E2E workflow (batonel-verify-example.yml).
        let output = GenerateArtifactsUseCase::execute_with_paths(
            GenerateArtifactsInput,
            root.join("project.baton.yaml").to_str().unwrap(),
            root.join("placement.rules.yaml").to_str().unwrap(),
            root.join("artifacts.plan.yaml").to_str().unwrap(),
        )
        .expect("execute_with_paths should not return a config error");

        // generated_count + error_count == number of artifacts in the plan
        assert_eq!(output.generated_count + output.error_count, 1);
    }
}
