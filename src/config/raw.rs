use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawPresetReference {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawArchflowMetadata {
    pub schema_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<RawPresetReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawProjectConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archflow: Option<RawArchflowMetadata>,
    pub project: crate::model::project::Project,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<crate::model::project::Workspace>,
    pub modules: Vec<crate::model::project::Module>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawArtifact {
    pub name: String,
    pub module: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawArtifactsPlanConfig {
    pub artifacts: Vec<RawArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawSidecarPlacement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawRolePlacement {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sidecar: Option<RawSidecarPlacement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawPlacementRulesConfig {
    pub roles: HashMap<String, RawRolePlacement>,
}
