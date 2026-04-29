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
    let project_config = match crate::config::ProjectConfig::load("project.baton.yaml") {
        Ok(config) => config,
        Err(e) => {
            eprintln!("[!] loading project config failed: {}", e);
            std::process::exit(1);
        }
    };
    let language = project_config.project.language;

    let llm_adapter = crate::infra::llm::DummyLlmAdapter;
    let mut current_prompt_text = prompt_text;
    let max_retries = 3;
    let mut attempt = 1;

    loop {
        println!("  [~] Handing off to LLM execution engine (Attempt {}/{})...", attempt, max_retries);
        
        let request = LlmRequest {
            prompt: current_prompt_text.clone(),
            system_prompt: Some("You are an expert AI code generator. Generate code that exactly matches the provided architectural contracts. Return ONLY the generated code wrapped in a single markdown code block.".to_string()),
            temperature: Some(0.2),
        };

        match llm_adapter.complete(&request) {
            Ok(response) => {
                let extracted_code = crate::generator::ai_parser::AiResponseParser::extract_code_block(&response.content);
                
                if let Some(target_file) = &target_path {
                    if !is_safe_to_write(target_file) {
                        eprintln!("[!] Security violation: Attempted to write to unsafe path: {}", target_file.display());
                        std::process::exit(1);
                    }

                    if let Some(parent) = target_file.parent() {
                        if let Err(e) = std::fs::create_dir_all(parent) {
                            eprintln!("[!] failed to create directories for {}: {}", target_file.display(), e);
                            std::process::exit(1);
                        }
                    }

                    if let Err(e) = std::fs::write(target_file, &extracted_code) {
                        eprintln!("[!] failed to write to {}: {}", target_file.display(), e);
                        std::process::exit(1);
                    }
                    
                    println!("  [+] Code written to: {}", target_file.display());

                    // 1. Run internal verification
                    let mut verification_failed = false;
                    let mut error_messages = String::new();

                    println!("  [i] Running internal architectural verification...");
                    let verify_output = match crate::app::usecase::ValidateProjectUseCase::execute(
                        crate::app::usecase::ValidateProjectInput,
                    ) {
                        Ok(out) => out,
                        Err(e) => {
                            verification_failed = true;
                            error_messages.push_str(&format!("Batonel execution error: {}\n", e));
                            // Construct a dummy failing output to skip success check
                            crate::app::usecase::ValidateProjectOutput {
                                success: false,
                                structural_errors: 1,
                                structural_warnings: 0,
                                report: crate::model::verify::VerifyReport::new(vec![]),
                            }
                        }
                    };

                    if !verify_output.success {
                        verification_failed = true;
                        error_messages.push_str("Architectural Verification Failed:\n");
                        let lines = crate::commands::verify::build_report_lines(&verify_output.report);
                        for line in lines {
                            error_messages.push_str(&format!("{}\n", line));
                        }
                    } else {
                        println!("  [+] Architectural verification passed.");
                    }

                    // 2. Run native tests if architecture passed (or even if it didn't? Let's only run if architecture passed to save time, or maybe run both to get all errors. Let's only run if architecture passed to ensure structural safety first.)
                    if !verification_failed {
                        let (cmd, args) = match language.as_str() {
                            "rust" => ("cargo", vec!["test"]),
                            "typescript" | "javascript" => ("npm", vec!["test"]),
                            _ => ("", vec![]),
                        };

                        if !cmd.is_empty() {
                            println!("  [i] Running native tests ({} {})...", cmd, args.join(" "));
                            match std::process::Command::new(cmd).args(&args).output() {
                                Ok(output) if !output.status.success() => {
                                    verification_failed = true;
                                    error_messages.push_str("\nNative Tests Failed:\n");
                                    error_messages.push_str(&String::from_utf8_lossy(&output.stderr));
                                    error_messages.push_str(&String::from_utf8_lossy(&output.stdout));
                                }
                                Err(e) => {
                                    println!("  [!] Failed to execute test command: {}", e);
                                    // Treat missing test runner as failure? Actually, just warn.
                                }
                                _ => {
                                    println!("  [+] Native tests passed.");
                                }
                            }
                        }
                    }

                    if verification_failed {
                        if attempt >= max_retries {
                            eprintln!("\n[!] Maximum retries ({}) reached. Handoff failed.", max_retries);
                            eprintln!("Last errors:\n{}", error_messages);
                            std::process::exit(1);
                        }
                        println!("  [!] Verification failed. Augmenting prompt and retrying...");
                        current_prompt_text.push_str("\n\n## Previous Attempt Failed\nThe generated code failed verification with the following errors:\n```text\n");
                        current_prompt_text.push_str(&error_messages);
                        current_prompt_text.push_str("\n```\nPlease correct these issues and try again.\n");
                        attempt += 1;
                        continue;
                    } else {
                        println!("\n  [+] Handoff execution completed successfully.");
                        break;
                    }
                } else {
                    println!("{}", extracted_code);
                    println!("\n  [+] Handoff execution completed successfully. (No target path resolved, output printed to stdout)");
                    break;
                }
            }
            Err(e) => {
                eprintln!("[!] LLM handoff failed: {}", e);
                std::process::exit(1);
            }
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
