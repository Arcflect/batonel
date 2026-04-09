use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig};
use crate::domain::project::ProjectContext;

use super::architecture_plan::{ArchitecturePlan, PlannedArtifact, PlannedArtifactStatus};

pub struct ArchitecturePlanner;

impl ArchitecturePlanner {
    pub fn plan(
        context: &ProjectContext,
        placement: &PlacementRulesConfig,
        artifacts: &ArtifactsPlanConfig,
    ) -> ArchitecturePlan {
        let mut planned = Vec::with_capacity(artifacts.artifacts.len());

        for artifact in &artifacts.artifacts {
            if !context.has_module(&artifact.module) {
                planned.push(PlannedArtifact {
                    name: artifact.name.clone(),
                    role: artifact.role.clone(),
                    status: PlannedArtifactStatus::Error,
                    resolved_path: None,
                    error_message: Some(format!(
                        "module '{}' is not defined in project.arch.yaml",
                        artifact.module
                    )),
                });
                continue;
            }

            match resolve_artifact_path(artifact, placement) {
                Ok(path) => planned.push(PlannedArtifact {
                    name: artifact.name.clone(),
                    role: artifact.role.clone(),
                    status: PlannedArtifactStatus::Planned,
                    resolved_path: Some(path),
                    error_message: None,
                }),
                Err(err) => planned.push(PlannedArtifact {
                    name: artifact.name.clone(),
                    role: artifact.role.clone(),
                    status: PlannedArtifactStatus::Error,
                    resolved_path: None,
                    error_message: Some(err),
                }),
            }
        }

        ArchitecturePlan {
            project_name: context.project_name.clone(),
            architecture_style: context.architecture_style.clone(),
            language: context.language.clone(),
            module_count: context.module_count(),
            artifacts: planned,
        }
    }
}

fn resolve_artifact_path(
    artifact: &crate::model::artifact::Artifact,
    placement: &PlacementRulesConfig,
) -> Result<String, String> {
    if let Some(explicit) = &artifact.path {
        return Ok(explicit.clone());
    }

    let role = placement
        .roles
        .get(&artifact.role)
        .ok_or_else(|| format!("Role '{}' not found in placement rules", artifact.role))?;

    if let Some(ext) = &role.file_extension {
        Ok(format!("{}/{}.{}", role.path.trim_end_matches('/'), artifact.name, ext))
    } else {
        Ok(format!("{}/{}", role.path.trim_end_matches('/'), artifact.name))
    }
}

#[cfg(test)]
mod tests {
    use super::ArchitecturePlanner;
    use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig};
    use crate::domain::planning::PlannedArtifactStatus;
    use crate::domain::project::ProjectContext;
    use crate::model::artifact::Artifact;
    use crate::model::placement::RolePlacement;
    use std::collections::HashMap;

    fn context() -> ProjectContext {
        ProjectContext::new(
            "demo".to_string(),
            "layered".to_string(),
            "rust".to_string(),
            ["user".to_string()].into_iter().collect(),
        )
    }

    #[test]
    fn planner_resolves_and_flags_errors() {
        let mut roles = HashMap::new();
        roles.insert(
            "usecase".to_string(),
            RolePlacement {
                path: "src/application/usecases".to_string(),
                file_extension: Some("rs".to_string()),
                sidecar: None,
            },
        );

        let artifacts = ArtifactsPlanConfig {
            artifacts: vec![
                Artifact {
                    name: "create_user".to_string(),
                    module: "user".to_string(),
                    role: "usecase".to_string(),
                    path: None,
                    inputs: None,
                    outputs: None,
                    status: None,
                    tags: None,
                },
                Artifact {
                    name: "create_order".to_string(),
                    module: "order".to_string(),
                    role: "usecase".to_string(),
                    path: None,
                    inputs: None,
                    outputs: None,
                    status: None,
                    tags: None,
                },
            ],
        };

        let plan = ArchitecturePlanner::plan(
            &context(),
            &PlacementRulesConfig { roles },
            &artifacts,
        );

        assert_eq!(plan.language, "rust");
        assert_eq!(plan.planned_count(), 1);
        assert_eq!(plan.error_count(), 1);
        assert_eq!(plan.artifacts[0].status, PlannedArtifactStatus::Planned);
        assert_eq!(
            plan.artifacts[0].resolved_path.as_deref(),
            Some("src/application/usecases/create_user.rs")
        );
    }
}
