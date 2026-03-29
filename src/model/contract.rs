use serde::{Deserialize, Serialize};

/// The internal Contract representation.
/// This acts as the authoritative truth from which Prompts are derived.
/// It defines structural and logic constraints for an individual Artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub name: String,
    pub module: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    pub responsibilities: Vec<String>,
    pub must_not: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_dependencies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_dependencies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_size: Option<String>,

    pub status: String,
}
