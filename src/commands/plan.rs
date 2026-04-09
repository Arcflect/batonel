use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig, ProjectConfig};
use crate::domain::planning::{ArchitecturePlanner, PlannedArtifactStatus};
use crate::domain::project::ProjectContext;

#[allow(dead_code)]
pub fn execute() {
    let guard_report = crate::commands::guard::run_hook(crate::commands::guard::GuardHookPoint::Plan, None);
    crate::commands::guard::render_report(&guard_report);

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

    let context = ProjectContext::from_project_config(&project_config);
    let plan = ArchitecturePlanner::plan(&context, &placement_config, &artifacts_config);
    let (lines, mut error_count) = build_plan_output(&plan);
    for line in lines {
        println!("{}", line);
    }

    if guard_report.has_errors() {
        error_count += 1;
    }

    if error_count > 0 {
        std::process::exit(1);
    }
}

#[allow(dead_code)]
fn build_plan_output(plan: &crate::domain::planning::ArchitecturePlan) -> (Vec<String>, usize) {
    let mut lines = vec![
        "Archflow Implementation Plan".to_string(),
        "============================".to_string(),
        format!("Project: {}", plan.project_name),
        format!("Style:   {}", plan.architecture_style),
        format!("Lang:    {}", plan.language),
        format!("Modules: {}", plan.module_count),
        String::new(),
        "Planned Artifacts:".to_string(),
    ];

    for artifact in &plan.artifacts {
        match artifact.status {
            PlannedArtifactStatus::Planned => lines.push(format!(
                "  - {} [{}] -> {}",
                artifact.name,
                artifact.role,
                artifact
                    .resolved_path
                    .as_deref()
                    .unwrap_or("<unresolved>")
            )),
            PlannedArtifactStatus::Error => lines.push(format!(
                "  ! {} [{}]: Error: {}",
                artifact.name,
                artifact.role,
                artifact.error_message.as_deref().unwrap_or("unknown error")
            )),
        }
    }

    let success_count = plan.planned_count();
    let error_count = plan.error_count();

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
    use crate::domain::planning::{ArchitecturePlan, PlannedArtifact, PlannedArtifactStatus};

    #[test]
    fn build_plan_output_keeps_success_and_error_lines_in_input_order() {
        let plan = ArchitecturePlan {
            project_name: "demo".to_string(),
            architecture_style: "layered".to_string(),
            language: "rust".to_string(),
            module_count: 1,
            artifacts: vec![
                PlannedArtifact {
                    name: "create_user".to_string(),
                    role: "usecase".to_string(),
                    status: PlannedArtifactStatus::Planned,
                    resolved_path: Some("src/application/usecases/create_user.rs".to_string()),
                    error_message: None,
                },
                PlannedArtifact {
                    name: "user_view".to_string(),
                    role: "missing-role".to_string(),
                    status: PlannedArtifactStatus::Error,
                    resolved_path: None,
                    error_message: Some("Role 'missing-role' not found in placement rules".to_string()),
                },
            ],
        };

        let (lines, error_count) = build_plan_output(&plan);

        assert_eq!(error_count, 1);
        assert!(lines.iter().any(|line| {
            line.contains("create_user [usecase] -> src/application/usecases/create_user.rs")
        }));
        assert!(lines.iter().any(|line| {
            line.contains("user_view [missing-role]: Error: Role 'missing-role' not found in placement rules")
        }));
        assert_eq!(lines.last().expect("summary line must exist"), "Plan result: 1 planned, 1 errors.");
    }

    #[test]
    fn build_plan_output_reports_unknown_artifact_module_before_path_resolution() {
        let plan = ArchitecturePlan {
            project_name: "demo-app".to_string(),
            architecture_style: "layered".to_string(),
            language: "rust".to_string(),
            module_count: 1,
            artifacts: vec![PlannedArtifact {
                name: "create_order".to_string(),
                role: "usecase".to_string(),
                status: PlannedArtifactStatus::Error,
                resolved_path: None,
                error_message: Some("module 'order' is not defined in project.arch.yaml".to_string()),
            }],
        };

        let (lines, error_count) = build_plan_output(&plan);

        assert_eq!(error_count, 1);
        assert!(lines
            .iter()
            .any(|line| line.contains("module 'order' is not defined in project.arch.yaml")));
    }
}
