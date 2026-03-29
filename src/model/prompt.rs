use serde::{Deserialize, Serialize};

use super::contract::Contract;

/// The internal Prompt representation used during Phase 3 generation.
/// This model acts strictly down-stream from contracts; it is not a parallel source of truth.
/// It translates the semantic constraints of a contract into an AI-ready delivery payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub artifact_name: String,
    pub role: String,
    pub module: String,

    // Core constraints (inherited from Contract or Role expectations)
    pub responsibilities: Vec<String>,
    pub must_not: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_dependencies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_dependencies: Option<Vec<String>>,

    // Artifact interface boundaries (downstream from Artifact definition)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,

    // Final AI fulfillment checks (derived from contract intent)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_criteria: Option<Vec<String>>,
}

impl From<&Contract> for Prompt {
    fn from(contract: &Contract) -> Self {
        // Derive default completion criteria from the basic architectural boundaries
        // This keeps the prompt generation deterministic and faithful to the contract rules
        let completion_criteria = Some(vec![
            "The artifact focuses exclusively on its defined responsibilities.".to_string(),
            "The implementation respects forbidden dependencies and architectural rules."
                .to_string(),
            "No infrastructure-specific logic is leaked into pure domain or usecase layers."
                .to_string(),
        ]);

        Prompt {
            artifact_name: contract.name.clone(),
            role: contract.role.clone(),
            module: contract.module.clone(),
            responsibilities: contract.responsibilities.clone(),
            must_not: contract.must_not.clone(),
            allowed_dependencies: contract.allowed_dependencies.clone(),
            forbidden_dependencies: contract.forbidden_dependencies.clone(),
            inputs: contract.inputs.clone(),
            outputs: contract.outputs.clone(),
            completion_criteria,
        }
    }
}
