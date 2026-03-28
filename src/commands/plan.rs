use crate::config::ProjectConfig;

pub fn execute() {
    println!("Loading project architecture...");

    match ProjectConfig::load("project.arch.yaml") {
        Ok(config) => {
            println!("Successfully loaded project: {}", config.project.name);
            println!("Plan command executed (stub)");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
