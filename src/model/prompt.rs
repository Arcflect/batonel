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
    /// Creates a new Prompt from a given Contract.
    /// 
    /// This implementation follows the "Strict Projection" rule (ADR-0010),
    /// ensuring all prompt content is directly traceable to either 
    /// the Contract fields or authoritative Role-based defaults.
    fn from(contract: &crate::model::contract::Contract) -> Self {
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
            completion_criteria: derive_completion_criteria(contract),
        }
    }
}

/// Helper to derive completion criteria based on the authoritative contract-to-role mapping.
/// If explicit criteria are provided in the contract, they take precedence.
fn derive_completion_criteria(contract: &crate::model::contract::Contract) -> Option<Vec<String>> {
    match contract.completion_criteria.as_ref() {
        Some(criteria) if !criteria.is_empty() => Some(criteria.clone()),
        _ => {
            let criteria_items = match contract.role.as_str() {
                "entity" => vec![
                    "The entity strictly protects its domain invariants.",
                    "Methods represent business rules, not just generic getters/setters.",
                    "No application, transport, or persistence details leak into this layer.",
                ],
                "usecase" => vec![
                    "The usecase implements exactly one application flow.",
                    "It coordinates domain behavior through ports but does not implement infrastructure natively.",
                    "No HTTP or database logic is present.",
                ],
                "repository_port" => vec![
                    "The abstraction focuses purely on the repository intent (e.g., retrieving aggregates).",
                    "It is fully decoupled from specific SQL, ORM, or database terminology.",
                ],
                r if r.contains("handler") => vec![
                    "The handler cleanly translates transport models into application requests.",
                    "It invokes the application layer but embeds zero core business rules locally.",
                ],
                r if r == "repository_impl" || r == "repository" => vec![
                    "The implementation fulfills an outbound port.",
                    "It safely translates between raw persistence data and pure upstream domain models.",
                ],
                _ => vec![
                    "The artifact focuses exclusively on its defined responsibilities.",
                    "The implementation respects forbidden dependencies and architectural rules.",
                ],
            };
            Some(criteria_items.into_iter().map(String::from).collect())
        }
    }
}

impl Prompt {
    pub fn format_markdown(&self, mode: crate::cli::OutputMode) -> String {
        match mode {
            crate::cli::OutputMode::Standard => self.format_standard(),
            crate::cli::OutputMode::Compact => self.format_compact(),
        }
    }

    fn format_standard(&self) -> String {
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

    fn format_compact(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("**Artifact:** `{}` (Role: `{}`, Module: `{}`)\n\n", self.artifact_name, self.role, self.module));

        if !self.responsibilities.is_empty() {
            out.push_str("**Responsibilities:**\n");
            for r in &self.responsibilities {
                out.push_str(&format!("- {}\n", r));
            }
            out.push('\n');
        }

        if !self.must_not.is_empty() {
            out.push_str("**Must Not:**\n");
            for m in &self.must_not {
                out.push_str(&format!("- {}\n", m));
            }
            out.push('\n');
        }

        if let Some(deps) = &self.allowed_dependencies {
            if !deps.is_empty() {
                out.push_str(&format!("**Dependencies Allowed:** {}\n\n", deps.join(", ")));
            }
        }

        if let Some(deps) = &self.forbidden_dependencies {
            if !deps.is_empty() {
                out.push_str(&format!("**Forbidden Dependencies:** {}\n\n", deps.join(", ")));
            }
        }

        if let Some(inputs) = &self.inputs {
            if !inputs.is_empty() {
                out.push_str(&format!("**Inputs:** {}\n\n", inputs.join(", ")));
            }
        }

        if let Some(outputs) = &self.outputs {
            if !outputs.is_empty() {
                out.push_str(&format!("**Outputs:** {}\n\n", outputs.join(", ")));
            }
        }

        if let Some(criteria) = &self.completion_criteria {
            if !criteria.is_empty() {
                out.push_str("**Completion Criteria:**\n");
                for c in criteria {
                    out.push_str(&format!("- {}\n", c));
                }
                out.push('\n');
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::contract::Contract;

    #[test]
    fn test_prompt_derivation_from_contract() {
        let contract = Contract {
            name: "test_artifact".to_string(),
            module: "test_module".to_string(),
            role: "entity".to_string(),
            path: "src/test.rs".to_string(),
            responsibilities: vec!["Do A".to_string()],
            must_not: vec!["Do B".to_string()],
            allowed_dependencies: Some(vec!["dep1".to_string()]),
            forbidden_dependencies: None,
            inputs: None,
            outputs: None,
            implementation_size: "small".to_string(),
            status: "planned".to_string(),
            completion_criteria: None,
        };

        let prompt = Prompt::from(&contract);

        assert_eq!(prompt.artifact_name, "test_artifact");
        assert_eq!(prompt.role, "entity");
        assert_eq!(prompt.module, "test_module");
        assert_eq!(prompt.responsibilities, vec!["Do A"]);
        
        // Verify role-based criteria injection
        let criteria = prompt.completion_criteria.unwrap();
        assert!(criteria.iter().any(|c| c.contains("domain invariants")));
    }

    #[test]
    fn test_prompt_precedence_of_explicit_criteria() {
        let contract = Contract {
            name: "test".to_string(),
            module: "test".to_string(),
            role: "entity".to_string(),
            path: "test.rs".to_string(),
            responsibilities: vec![],
            must_not: vec![],
            allowed_dependencies: None,
            forbidden_dependencies: None,
            inputs: None,
            outputs: None,
            implementation_size: "small".to_string(),
            status: "planned".to_string(),
            completion_criteria: Some(vec!["Manual rule".to_string()]),
        };

        let prompt = Prompt::from(&contract);
        let criteria = prompt.completion_criteria.unwrap();
        
        assert_eq!(criteria, vec!["Manual rule"]);
        assert!(!criteria.iter().any(|c| c.contains("domain invariants")));
    }
}
