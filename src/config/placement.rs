use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::error::ConfigError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacementRulesConfig {
    pub roles: HashMap<String, RolePlacement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarPlacement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePlacement {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sidecar: Option<SidecarPlacement>,
}

impl PlacementRulesConfig {
    /// Loads the placement rules configuration from a given file path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ConfigError::NotFound(path.to_path_buf()));
        }

        let contents = fs::read_to_string(path).map_err(|e| ConfigError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

        let config: PlacementRulesConfig =
            serde_yaml::from_str(&contents).map_err(|e| ConfigError::Parse {
                path: path.to_path_buf(),
                source: e,
            })?;

        Ok(config)
    }
}
