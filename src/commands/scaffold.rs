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

    println!("Archflow Scaffold Generation");
    println!("============================");
    println!("Project: {}", project_config.project.name);
    println!("Style:   {}", project_config.project.architecture_style);
    println!("Modules: {}", project_config.modules.len());
    println!();

    println!("Generating Artifacts:");
    let mut success_count = 0;
    let mut error_count = 0;

    for artifact in &artifacts_config.artifacts {
        if !project_config.has_module(&artifact.module) {
            eprintln!(
                "  [!] {} [{}]: Module Error: module '{}' is not defined in project.arch.yaml",
                artifact.name, artifact.role, artifact.module
            );
            error_count += 1;
            continue;
        }

        match crate::generator::resolve_artifact_path(artifact, &placement_config) {
            Ok(path) => {
                let role_config = placement_config.roles.get(&artifact.role);
                match crate::generator::scaffold::generate_artifact_with_sidecars(
                    artifact,
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
            Err(e) => {
                eprintln!(
                    "  [!] {} [{}]: Path Error: {}",
                    artifact.name, artifact.role, e
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
