use std::path::Path;

use super::error::ConfigError;
use super::raw::{RawArtifactsPlanConfig, RawPlacementRulesConfig, RawProjectConfig};

/// Parse raw project config from YAML text.
pub fn parse_project_raw(contents: &str, path: &Path) -> Result<RawProjectConfig, ConfigError> {
    serde_yaml::from_str(contents).map_err(|e| ConfigError::Parse {
        path: path.to_path_buf(),
        source: e,
    })
}

/// Parse raw artifacts plan config from YAML text.
pub fn parse_artifacts_raw(
    contents: &str,
    path: &Path,
) -> Result<RawArtifactsPlanConfig, ConfigError> {
    serde_yaml::from_str(contents).map_err(|e| ConfigError::Parse {
        path: path.to_path_buf(),
        source: e,
    })
}

/// Parse raw placement rules config from YAML text.
pub fn parse_placement_raw(
    contents: &str,
    path: &Path,
) -> Result<RawPlacementRulesConfig, ConfigError> {
    serde_yaml::from_str(contents).map_err(|e| ConfigError::Parse {
        path: path.to_path_buf(),
        source: e,
    })
}
