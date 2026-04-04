use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig, ProjectConfig};

pub fn execute() {
    let project_config = match ProjectConfig::load("project.arch.yaml") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading project: {}", e);
            std::process::exit(1);
        }
    };

    let placement_config = match PlacementRulesConfig::load("placement.rules.yaml") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading placement rules: {}", e);
            std::process::exit(1);
        }
    };

    let artifacts_config = match ArtifactsPlanConfig::load("artifacts.plan.yaml") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading artifacts plan: {}", e);
            std::process::exit(1);
        }
    };

    let (lines, error_count) = build_plan_output(&project_config, &placement_config, &artifacts_config);
    for line in lines {
        println!("{}", line);
    }

    if error_count > 0 {
        std::process::exit(1);
    }
}

fn build_plan_output(
    project_config: &ProjectConfig,
    placement_config: &PlacementRulesConfig,
    artifacts_config: &ArtifactsPlanConfig,
) -> (Vec<String>, usize) {
    let mut lines = vec![
        "Archflow Implementation Plan".to_string(),
        "============================".to_string(),
        format!("Project: {}", project_config.project.name),
        format!("Style:   {}", project_config.project.architecture_style),
        format!("Modules: {}", project_config.modules.len()),
        String::new(),
        "Planned Artifacts:".to_string(),
    ];

    let mut success_count = 0;
    let mut error_count = 0;

    for artifact in &artifacts_config.artifacts {
        match crate::generator::resolve_artifact_path(artifact, placement_config) {
            Ok(path) => {
                lines.push(format!(
                    "  - {} [{}] -> {}",
                    artifact.name,
                    artifact.role,
                    path.display()
                ));
                success_count += 1;
            }
            Err(e) => {
                lines.push(format!(
                    "  ! {} [{}]: Error: {}",
                    artifact.name,
                    artifact.role,
                    e
                ));
                error_count += 1;
            }
        }
    }

    lines.push(String::new());
    lines.push(format!(
        "Plan result: {} planned, {} errors.",
        success_count, error_count
    ));

    (lines, error_count)
}

#[cfg(test)]
mod tests {
    use super::build_plan_output;
    use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig, ProjectConfig};
    use crate::model::artifact::Artifact;
    use crate::model::placement::RolePlacement;
    use crate::model::project::{Module, Project};
    use std::collections::HashMap;

    #[test]
    fn build_plan_output_keeps_success_and_error_lines_in_input_order() {
        let project_config = ProjectConfig {
            project: Project {
                name: "demo".to_string(),
                architecture_style: "layered".to_string(),
                language: "rust".to_string(),
            },
            workspace: None,
            modules: vec![Module {
                name: "user".to_string(),
                features: None,
            }],
            metadata: None,
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
        let placement_config = PlacementRulesConfig { roles };

        let artifacts_config = ArtifactsPlanConfig {
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
                    name: "user_view".to_string(),
                    module: "user".to_string(),
                    role: "missing-role".to_string(),
                    path: None,
                    inputs: None,
                    outputs: None,
                    status: None,
                    tags: None,
                },
            ],
        };

        let (lines, error_count) = build_plan_output(&project_config, &placement_config, &artifacts_config);

        assert_eq!(error_count, 1);
        assert!(lines[7].contains("create_user [usecase] -> src/application/usecases/create_user.rs"));
        assert!(lines[8].contains("user_view [missing-role]: Error: Role 'missing-role' not found in placement rules"));
        assert_eq!(lines.last().expect("summary line must exist"), "Plan result: 1 planned, 1 errors.");
    }
}
