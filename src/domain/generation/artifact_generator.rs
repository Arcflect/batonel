use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig};
use crate::domain::project::ProjectContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactGenerationStatus {
    Ready,
    Error,
}

#[derive(Debug, Clone)]
pub struct GenerationItem {
    pub artifact: crate::model::artifact::Artifact,
    pub status: ArtifactGenerationStatus,
    pub resolved_path: Option<std::path::PathBuf>,
    pub _error_message: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct GenerationPlan {
    pub items: Vec<GenerationItem>,
}

impl GenerationPlan {
    #[allow(dead_code)]
    pub fn ready_count(&self) -> usize {
        self.items
            .iter()
            .filter(|i| i.status == ArtifactGenerationStatus::Ready)
            .count()
    }

    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.items
            .iter()
            .filter(|i| i.status == ArtifactGenerationStatus::Error)
            .count()
    }
}

pub struct ArtifactGenerator;

impl ArtifactGenerator {
    pub fn build_plan(
        context: &ProjectContext,
        placement: &PlacementRulesConfig,
        artifacts: &ArtifactsPlanConfig,
    ) -> GenerationPlan {
        let mut items = Vec::with_capacity(artifacts.artifacts.len());

        for artifact in &artifacts.artifacts {
            if !context.has_module(&artifact.module) {
                items.push(GenerationItem {
                    artifact: artifact.clone(),
                    status: ArtifactGenerationStatus::Error,
                    resolved_path: None,
                    _error_message: Some(format!(
                        "module '{}' is not defined in project.baton.yaml",
                        artifact.module
                    )),
                });
                continue;
            }

            match resolve_artifact_path(artifact, placement) {
                Ok(path) => items.push(GenerationItem {
                    artifact: artifact.clone(),
                    status: ArtifactGenerationStatus::Ready,
                    resolved_path: Some(path),
                    _error_message: None,
                }),
                Err(err) => items.push(GenerationItem {
                    artifact: artifact.clone(),
                    status: ArtifactGenerationStatus::Error,
                    resolved_path: None,
                    _error_message: Some(err.to_string()),
                }),
            }
        }

        GenerationPlan { items }
    }
}

fn resolve_artifact_path(
    artifact: &crate::model::artifact::Artifact,
    placement: &PlacementRulesConfig,
) -> Result<std::path::PathBuf, String> {
    if let Some(explicit) = &artifact.path {
        return Ok(std::path::PathBuf::from(explicit));
    }

    let role = placement
        .roles
        .get(&artifact.role)
        .ok_or_else(|| format!("Role '{}' not found in placement rules", artifact.role))?;

    let mut path = std::path::PathBuf::from(role.path.trim_end_matches('/'));
    let filename = match role.file_extension.as_deref() {
        Some(ext) => format!("{}.{}", artifact.name, ext),
        None => artifact.name.clone(),
    };
    path.push(filename);
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::{ArtifactGenerationStatus, ArtifactGenerator};
    use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig};
    use crate::domain::project::ProjectContext;
    use crate::model::artifact::Artifact;

    #[test]
    fn generation_plan_marks_errors_for_unknown_module() {
        let context = ProjectContext::new(
            "demo".to_string(),
            "layered".to_string(),
            "rust".to_string(),
            ["user".to_string()].into_iter().collect(),
        );

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

        let plan = ArtifactGenerator::build_plan(&context, &PlacementRulesConfig { roles: Default::default() }, &artifacts);
        assert_eq!(plan.ready_count(), 0);
        assert_eq!(plan.error_count(), 1);
        assert_eq!(plan.items[0].status, ArtifactGenerationStatus::Error);
    }
}
