use crate::model::placement::RolePlacement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::error::ConfigError;
use super::raw::{RawPlacementRulesConfig, RawRolePlacement, RawSidecarPlacement};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacementRulesConfig {
    pub roles: HashMap<String, RolePlacement>,
}

impl PlacementRulesConfig {
    /// Loads the placement rules configuration from a given file path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let contents = crate::config::loader::load_text(path)?;
        let raw = crate::config::parser::parse_placement_raw(&contents, path)?;
        Ok(Self::from_raw(raw))
    }

    pub fn from_raw(raw: RawPlacementRulesConfig) -> Self {
        let roles = raw
            .roles
            .into_iter()
            .map(|(name, role)| {
                let sidecar = role.sidecar.map(|s| crate::model::placement::SidecarPlacement {
                    contract_dir: s.contract_dir,
                    prompt_dir: s.prompt_dir,
                });
                (
                    name,
                    RolePlacement {
                        path: role.path,
                        file_extension: role.file_extension,
                        sidecar,
                    },
                )
            })
            .collect();
        Self { roles }
    }
}

impl From<crate::model::placement::SidecarPlacement> for RawSidecarPlacement {
    fn from(value: crate::model::placement::SidecarPlacement) -> Self {
        RawSidecarPlacement {
            contract_dir: value.contract_dir,
            prompt_dir: value.prompt_dir,
        }
    }
}

impl From<RolePlacement> for RawRolePlacement {
    fn from(value: RolePlacement) -> Self {
        RawRolePlacement {
            path: value.path,
            file_extension: value.file_extension,
            sidecar: value.sidecar.map(Into::into),
        }
    }
}
