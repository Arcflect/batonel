use crate::config::{ArtifactsPlanConfig, PlacementRulesConfig};
use crate::ports::{LlmPort, LlmRequest};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct DriftResult {
    has_drift: bool,
    deviations: Vec<String>,
}

pub fn execute(target: Option<&str>) {
    println!("Batonel Continuous Alignment (Drift Detection)");
    println!("==============================================");

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

    let artifacts: Vec<_> = if let Some(t) = target {
        artifacts_config.artifacts.into_iter().filter(|a| a.name == t).collect()
    } else {
        artifacts_config.artifacts
    };

    if artifacts.is_empty() {
        if let Some(t) = target {
            eprintln!("[!] artifact '{}' not found in artifacts.plan.yaml", t);
            std::process::exit(1);
        } else {
            println!("No artifacts to check.");
            return;
        }
    }

    let llm_adapter = crate::infra::llm::DummyLlmAdapter;
    let mut total_drift = 0;

    for artifact in artifacts {
        println!("\n[i] Checking artifact: {}", artifact.name);
        
        let path = match crate::generator::resolver::resolve_artifact_path(&artifact, &placement_config) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("  [!] skipping: resolving artifact path failed: {}", e);
                continue;
            }
        };

        if !path.exists() {
            println!("  [-] Implementation not found at {}", path.display());
            continue;
        }

        let role_config = placement_config.roles.get(&artifact.role);
        let contract_path = crate::generator::resolver::resolve_sidecar_path(
            &artifact,
            &path,
            role_config.and_then(|r| r.sidecar.as_ref().and_then(|s| s.contract_dir.as_deref())),
            "contract.yaml",
        );

        if !contract_path.exists() {
            println!("  [-] Contract not found at {}", contract_path.display());
            continue;
        }

        let code_content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("  [!] failed to read implementation code: {}", e);
                continue;
            }
        };

        let contract_content = match std::fs::read_to_string(&contract_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("  [!] failed to read contract: {}", e);
                continue;
            }
        };

        let prompt = format!(
            "You are an expert architecture validator.\n\
            Evaluate whether the following implementation code correctly fulfills the responsibilities and adheres to the must_not constraints defined in the architectural contract.\n\n\
            Contract:\n{}\n\n\
            Implementation Code:\n{}\n",
            contract_content,
            code_content
        );

        let system_prompt = "You are an expert architecture validator. Respond ONLY with a JSON object in this exact format:\n\
            {\n  \"has_drift\": true/false,\n  \"deviations\": [\"Explain any deviations or violations found. If has_drift is false, leave this array empty.\"]\n}";

        let request = LlmRequest {
            prompt,
            system_prompt: Some(system_prompt.to_string()),
            temperature: Some(0.0),
        };

        println!("  [~] Handing off to LLM for drift analysis...");
        
        match llm_adapter.complete(&request) {
            Ok(response) => {
                let extracted_json = crate::generator::ai_parser::AiResponseParser::extract_code_block(&response.content);
                match serde_json::from_str::<DriftResult>(&extracted_json) {
                    Ok(result) => {
                        if result.has_drift {
                            println!("  [!] Drift Detected!");
                            for dev in result.deviations {
                                println!("      - {}", dev);
                            }
                            total_drift += 1;
                        } else {
                            println!("  [+] Alignment Verified: Code adheres to the contract.");
                        }
                    }
                    Err(e) => {
                        eprintln!("  [!] failed to parse LLM drift analysis response: {}", e);
                        eprintln!("      Raw extracted response: {}", extracted_json);
                    }
                }
            }
            Err(e) => {
                eprintln!("  [!] LLM analysis failed: {}", e);
            }
        }
    }

    println!();
    if total_drift > 0 {
        eprintln!("[!] Drift analysis complete. {} artifact(s) drifted from their contracts.", total_drift);
        std::process::exit(1);
    } else {
        println!("[+] Drift analysis complete. All checked artifacts are aligned.");
    }
}
