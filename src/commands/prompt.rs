use crate::config::{ArtifactsPlanConfig, ContractConfig, PlacementRulesConfig};
use crate::model::prompt::Prompt;
use std::path::PathBuf;

pub fn execute(target: &str) {
    let contract_path = if target.ends_with(".yaml") || target.ends_with(".yml") {
        PathBuf::from(target)
    } else {
        // Resolve target as an artifact name
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

        // Find artifact
        let artifact = match artifacts_config.artifacts.iter().find(|a| a.name == target) {
            Some(a) => a,
            None => {
                eprintln!(
                    "Error: Artifact '{}' not found in artifacts.plan.yaml",
                    target
                );
                std::process::exit(1);
            }
        };

        // Resolve path to the primary artifact
        let path =
            match crate::generator::resolver::resolve_artifact_path(artifact, &placement_config) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error resolving artifact path: {}", e);
                    std::process::exit(1);
                }
            };

        let role_config = placement_config.roles.get(&artifact.role);

        let contract_path = crate::generator::resolver::resolve_sidecar_path(
            artifact,
            &path,
            role_config.and_then(|r| r.sidecar.as_ref().and_then(|s| s.contract_dir.as_deref())),
            "contract.yaml",
        );

        contract_path
    };

    // Load contract
    let contract_config = match ContractConfig::load(&contract_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!(
                "Error loading contract at {}: {}",
                contract_path.display(),
                e
            );
            std::process::exit(1);
        }
    };

    // Convert Contract to Prompt model
    let prompt: Prompt = (&contract_config.contract).into();

    // Serialize to markdown
    let output = prompt.format_markdown();

    // Output cleanly to stdout
    println!("{}", output);
}
