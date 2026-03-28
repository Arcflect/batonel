use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::Path;

use super::error::ConfigError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: Project,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Workspace>,
    pub modules: Vec<Module>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub architecture_style: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
}

impl ProjectConfig {
    /// Loads the project configuration from a given file path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ConfigError::NotFound(path.to_path_buf()));
        }

        let contents = fs::read_to_string(path).map_err(|e| ConfigError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

        let config: ProjectConfig =
            serde_yaml::from_str(&contents).map_err(|e| ConfigError::Parse {
                path: path.to_path_buf(),
                source: e,
            })?;

        Ok(config)
    }
}
