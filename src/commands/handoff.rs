use crate::cli::OutputMode;
use crate::config::{ArtifactsPlanConfig, ContractConfig, PlacementRulesConfig};
use crate::model::prompt::Prompt;
use crate::ports::{LlmPort, LlmRequest};
use std::path::PathBuf;

pub fn execute(target: &str) {
    println!("Batonel Handoff Execution");
    println!("=========================");

    let (contract_path, target_path) = if target.ends_with(".yaml") || target.ends_with(".yml") {
        (PathBuf::from(target), None)
    } else {
        // Resolve target as an artifact name
        let placement_config = match PlacementRulesConfig::load("placement.rules.yaml") {
            Ok(config) => config,
            Err(e) => {
                eprintln!("[!] loading placement rules failed: {}", e);
                std::process::exit(1);
            }
        };

        let artifacts_config = match ArtifactsPlanConfig::load("artifacts.plan.yaml") {
            Ok(config) => config,
            Err(e) => {
                eprintln!("[!] loading artifacts plan failed: {}", e);
                std::process::exit(1);
            }
        };

        // Find artifact
        let artifact = match artifacts_config.artifacts.iter().find(|a| a.name == target) {
            Some(a) => a,
            None => {
                eprintln!("[!] artifact '{}' not found in artifacts.plan.yaml", target);
                std::process::exit(1);
            }
        };

        // Resolve path to the primary artifact
        let path =
            match crate::generator::resolver::resolve_artifact_path(artifact, &placement_config) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("[!] resolving artifact path failed: {}", e);
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

        (contract_path, Some(path))
    };

    println!("  [i] Resolving contract from: {}", contract_path.display());

    // Load contract
    let contract_config = match ContractConfig::load(&contract_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("[!] loading contract at {} failed: {}", contract_path.display(), e);
            std::process::exit(1);
        }
    };

    // Convert Contract to Prompt model
    let prompt_model: Prompt = (&contract_config.contract).into();

    // Serialize to markdown for the LLM
    let prompt_text = prompt_model.format_markdown(OutputMode::Standard);

    println!("  [i] Successfully generated prompt ({} bytes)", prompt_text.len());
    println!("  [~] Handing off to LLM execution engine...\n");

    let llm_adapter = crate::infra::llm::DummyLlmAdapter;
    
    let request = LlmRequest {
        prompt: prompt_text,
        system_prompt: Some("You are an expert AI code generator. Generate code that exactly matches the provided architectural contracts. Return ONLY the generated code wrapped in a single markdown code block.".to_string()),
        temperature: Some(0.2),
    };

    match llm_adapter.complete(&request) {
        Ok(response) => {
            let extracted_code = crate::generator::ai_parser::AiResponseParser::extract_code_block(&response.content);
            
            if let Some(target_file) = target_path {
                if !is_safe_to_write(&target_file) {
                    eprintln!("[!] Security violation: Attempted to write to unsafe path: {}", target_file.display());
                    std::process::exit(1);
                }

                if let Some(parent) = target_file.parent() {
                    if let Err(e) = std::fs::create_dir_all(parent) {
                        eprintln!("[!] failed to create directories for {}: {}", target_file.display(), e);
                        std::process::exit(1);
                    }
                }

                match std::fs::write(&target_file, &extracted_code) {
                    Ok(_) => {
                        println!("\n  [+] Handoff execution completed successfully.");
                        println!("  [+] Code written to: {}", target_file.display());
                    }
                    Err(e) => {
                        eprintln!("[!] failed to write to {}: {}", target_file.display(), e);
                        std::process::exit(1);
                    }
                }
            } else {
                println!("{}", extracted_code);
                println!("\n  [+] Handoff execution completed successfully. (No target path resolved, output printed to stdout)");
            }
        }
        Err(e) => {
            eprintln!("[!] LLM handoff failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn is_safe_to_write(path: &std::path::Path) -> bool {
    let reserved = [
        "project.baton.yaml",
        "placement.rules.yaml",
        "artifacts.plan.yaml",
        "contracts.template.yaml",
        "policy.profile.yaml",
    ];

    let path_str = path.to_string_lossy();
    
    // Disallow absolute paths and path traversal
    if path.is_absolute() || path_str.contains("..") {
        return false;
    }

    // Disallow overwriting Batonel core configs
    for r in reserved {
        if path_str.ends_with(r) {
            return false;
        }
    }
    
    true
}
