use crate::config::{PlacementRulesConfig, ProjectConfig};

pub fn execute() {
    println!("Loading project architecture...");

    match ProjectConfig::load("project.arch.yaml") {
        Ok(config) => {
            println!("Successfully loaded project: {}", config.project.name);
        }
        Err(e) => {
            eprintln!("Error loading project: {}", e);
            std::process::exit(1);
        }
    }

    println!("Loading placement rules...");
    match PlacementRulesConfig::load("placement.rules.yaml") {
        Ok(config) => {
            println!("Successfully loaded {} placement rules", config.roles.len());
            println!("Scaffold command executed (stub)");
        }
        Err(e) => {
            eprintln!("Error loading placement rules: {}", e);
            std::process::exit(1);
        }
    }
}
