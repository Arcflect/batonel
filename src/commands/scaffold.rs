use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig, ProjectConfig};
use crate::domain::generation::{ArtifactGenerationStatus, ArtifactGenerator};
use crate::domain::project::ProjectContext;

pub fn execute() {
    let project_config = match ProjectConfig::load("project.baton.yaml") {
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

    println!("Batonel Scaffold Generation");
    println!("============================");
    println!("Project: {}", project_config.project.name);
    println!("Style:   {}", project_config.project.architecture_style);
    println!("Modules: {}", project_config.modules.len());
    println!();

    println!("Generating Artifacts:");
    let context = ProjectContext::from_project_config(&project_config);
    let generation_plan = ArtifactGenerator::build_plan(&context, &placement_config, &artifacts_config);

    let mut success_count = 0;
    let mut error_count = 0;
    for item in generation_plan.items {
        match item.status {
            ArtifactGenerationStatus::Ready => {
                let artifact = item.artifact;
                let path = match item.resolved_path {
                    Some(path) => path,
                    None => {
                        eprintln!(
                            "  [!] {} [{}]: Planning Error: missing resolved path",
                            artifact.name, artifact.role
                        );
                        error_count += 1;
                        continue;
                    }
                };
                let role_config = placement_config.roles.get(&artifact.role);
                match crate::generator::scaffold::generate_artifact_with_sidecars(
                    &artifact,
                    &path,
                    role_config,
                ) {
                    Ok(_) => {
                        println!(
                            "  [+] {} [{}] -> {}",
                            artifact.name,
                            artifact.role,
                            path.display()
                        );
                        success_count += 1;
                    }
                    Err(e) => {
                        eprintln!(
                            "  [!] {} [{}]: Generation Error: {}",
                            artifact.name, artifact.role, e
                        );
                        error_count += 1;
                    }
                }
            }
            ArtifactGenerationStatus::Error => {
                let artifact = item.artifact;
                eprintln!(
                    "  [!] {} [{}]: Planning Error: {}",
                    artifact.name,
                    artifact.role,
                    item.error_message.as_deref().unwrap_or("unknown error")
                );
                error_count += 1;
            }
        }
    }

    println!();
    println!(
        "Scaffold result: {} generated, {} errors.",
        success_count, error_count
    );

    if error_count > 0 {
        std::process::exit(1);
    }
}
