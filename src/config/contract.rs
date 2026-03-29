use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::error::ConfigError;
use crate::model::contract::Contract;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractConfig {
    #[serde(flatten)]
    pub contract: Contract,
}

impl ContractConfig {
    /// Loads the contract configuration from a given file path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ConfigError::NotFound(path.to_path_buf()));
        }

        let contents = fs::read_to_string(path).map_err(|e| ConfigError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

        // The top-level YAML fields map directly into the Contract struct
        let contract: Contract =
            serde_yaml::from_str(&contents).map_err(|e| ConfigError::Parse {
                path: path.to_path_buf(),
                source: e,
            })?;

        Ok(ContractConfig { contract })
    }
}
