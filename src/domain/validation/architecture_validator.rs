use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig};
use crate::domain::project::ProjectContext;

use super::validation_result::{ValidationResult, Violation, ViolationSeverity};

pub struct ArchitectureValidator;

impl ArchitectureValidator {
    pub fn validate(
        context: &ProjectContext,
        placement: &PlacementRulesConfig,
        artifacts: &ArtifactsPlanConfig,
    ) -> ValidationResult {
        let mut result = ValidationResult::default();

        for artifact in &artifacts.artifacts {
            let target = format!("artifact:{}", artifact.name);

            if !context.has_module(&artifact.module) {
                result.push(Violation {
                    _rule_id: "artifact-module-defined".to_string(),
                    severity: ViolationSeverity::Error,
                    _target: target.clone(),
                    _message: format!(
                        "artifact '{}' references undefined module '{}'",
                        artifact.name, artifact.module
                    ),
                });
            }

            if !placement.roles.contains_key(&artifact.role) {
                result.push(Violation {
                    _rule_id: "artifact-role-defined".to_string(),
                    severity: ViolationSeverity::Error,
                    _target: target,
                    _message: format!(
                        "artifact '{}' uses undefined role '{}'",
                        artifact.name, artifact.role
                    ),
                });
                continue;
            }

            if let Some(explicit_path) = artifact.path.as_deref() {
                let role = placement
                    .roles
                    .get(&artifact.role)
                    .expect("checked by contains_key above");
                let expected = expected_role_path(artifact, role);
                if explicit_path != expected {
                    result.push(Violation {
                        _rule_id: "artifact-path-aligns-role".to_string(),
                        severity: ViolationSeverity::Warn,
                        _target: format!("artifact:{}", artifact.name),
                        _message: format!(
                            "explicit path '{}' deviates from role '{}' expected path '{}'",
                            explicit_path, artifact.role, expected
                        ),
                    });
                }
            }
        }

        result
    }
}

fn expected_role_path(artifact: &crate::model::artifact::Artifact, role: &crate::model::placement::RolePlacement) -> String {
    let path = role.path.trim_end_matches('/');
    match role.file_extension.as_deref() {
        Some(ext) => format!("{}/{}.{}", path, artifact.name, ext),
        None => format!("{}/{}", path, artifact.name),
    }
}

#[cfg(test)]
mod tests {
    use super::ArchitectureValidator;
    use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig};
    use crate::domain::project::ProjectContext;
    use crate::domain::validation::ViolationSeverity;
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
    fn validator_reports_module_and_role_violations() {
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

        let result = ArchitectureValidator::validate(&context(), &PlacementRulesConfig { roles: Default::default() }, &artifacts);
        assert_eq!(result.error_count(), 2);
        assert_eq!(result.warning_count(), 0);
        assert!(!result.is_valid());
        assert_eq!(result.violations[0].severity, ViolationSeverity::Error);
    }

    #[test]
    fn validator_reports_warn_for_explicit_path_deviation() {
        let artifacts = ArtifactsPlanConfig {
            artifacts: vec![Artifact {
                name: "create_user".to_string(),
                module: "user".to_string(),
                role: "usecase".to_string(),
                path: Some("src/custom/path.rs".to_string()),
                inputs: None,
                outputs: None,
                status: None,
                tags: None,
            }],
        };

        let mut roles = HashMap::new();
        roles.insert(
            "usecase".to_string(),
            RolePlacement {
                path: "src/application/usecases".to_string(),
                file_extension: Some("rs".to_string()),
                sidecar: None,
            },
        );

        let result = ArchitectureValidator::validate(
            &context(),
            &PlacementRulesConfig { roles },
            &artifacts,
        );
        assert_eq!(result.error_count(), 0);
        assert_eq!(result.warning_count(), 1);
        assert_eq!(result.violations[0].severity, ViolationSeverity::Warn);
    }

    #[test]
    fn validator_accepts_explicit_path_when_it_matches_role_resolution() {
        let artifacts = ArtifactsPlanConfig {
            artifacts: vec![Artifact {
                name: "create_user".to_string(),
                module: "user".to_string(),
                role: "usecase".to_string(),
                path: Some("src/application/usecases/create_user.rs".to_string()),
                inputs: None,
                outputs: None,
                status: None,
                tags: None,
            }],
        };

        let mut roles = HashMap::new();
        roles.insert(
            "usecase".to_string(),
            RolePlacement {
                path: "src/application/usecases".to_string(),
                file_extension: Some("rs".to_string()),
                sidecar: None,
            },
        );

        let result = ArchitectureValidator::validate(
            &context(),
            &PlacementRulesConfig { roles },
            &artifacts,
        );
        assert_eq!(result.error_count(), 0);
        assert_eq!(result.warning_count(), 0);
        assert!(result.is_valid());
    }
}
