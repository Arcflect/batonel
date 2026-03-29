use serde::{Deserialize, Serialize};


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

impl From<&crate::model::contract::Contract> for Prompt {
    fn from(contract: &crate::model::contract::Contract) -> Self {
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

impl Prompt {
    pub fn format_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# Artifact Prompt: {}\n\n", self.artifact_name));
        out.push_str(&format!(
            "Implement the `{}` artifact.\n\n",
            self.artifact_name
        ));
        out.push_str(&format!("## Role\n{}\n\n", self.role));
        out.push_str(&format!("## Module\n{}\n\n", self.module));

        if !self.responsibilities.is_empty() {
            out.push_str("## Responsibilities\n");
            for r in &self.responsibilities {
                out.push_str(&format!("- {}\n", r));
            }
            out.push('\n');
        }

        if !self.must_not.is_empty() {
            out.push_str("## Must not\n");
            for m in &self.must_not {
                out.push_str(&format!("- {}\n", m));
            }
            out.push('\n');
        }

        if let Some(deps) = &self.allowed_dependencies {
            if !deps.is_empty() {
                out.push_str("## Allowed dependencies\n");
                for d in deps {
                    out.push_str(&format!("- {}\n", d));
                }
                out.push('\n');
            }
        }

        if let Some(deps) = &self.forbidden_dependencies {
            if !deps.is_empty() {
                out.push_str("## Forbidden dependencies\n");
                for d in deps {
                    out.push_str(&format!("- {}\n", d));
                }
                out.push('\n');
            }
        }

        if let Some(inputs) = &self.inputs {
            if !inputs.is_empty() {
                out.push_str("## Inputs\n");
                for i in inputs {
                    out.push_str(&format!("- {}\n", i));
                }
                out.push('\n');
            }
        }

        if let Some(outputs) = &self.outputs {
            if !outputs.is_empty() {
                out.push_str("## Outputs\n");
                for o in outputs {
                    out.push_str(&format!("- {}\n", o));
                }
                out.push('\n');
            }
        }

        if let Some(criteria) = &self.completion_criteria {
            if !criteria.is_empty() {
                out.push_str("## Completion criteria\n");
                for c in criteria {
                    out.push_str(&format!("- {}\n", c));
                }
                out.push('\n');
            }
        }

        out
    }
}
