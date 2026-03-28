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
        match crate::generator::resolve_artifact_path(artifact, &placement_config) {
            Ok(path) => println!("  - {} -> {}", artifact.name, path.display()),
            Err(e) => eprintln!("  - {}: Error: {}", artifact.name, e),
        }
    }

    println!("\nScaffold command executed (stub)");
}
