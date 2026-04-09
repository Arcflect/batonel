use crate::model::artifact::Artifact;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::error::ConfigError;
use super::raw::{RawArtifact, RawArtifactsPlanConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactsPlanConfig {
    pub artifacts: Vec<Artifact>,
}

impl ArtifactsPlanConfig {
    /// Loads the artifacts plan configuration from a given file path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let contents = crate::config::loader::load_text(path)?;
        let raw = crate::config::parser::parse_artifacts_raw(&contents, path)?;
        Ok(Self::from_raw(raw))
    }

    pub fn from_raw(raw: RawArtifactsPlanConfig) -> Self {
        let artifacts = raw
            .artifacts
            .into_iter()
            .map(|a| Artifact {
                name: a.name,
                module: a.module,
                role: a.role,
                path: a.path,
                inputs: a.inputs,
                outputs: a.outputs,
                status: a.status,
                tags: a.tags,
            })
            .collect();
        Self { artifacts }
    }
}

impl From<Artifact> for RawArtifact {
    fn from(value: Artifact) -> Self {
        RawArtifact {
            name: value.name,
            module: value.module,
            role: value.role,
            path: value.path,
            inputs: value.inputs,
            outputs: value.outputs,
            status: value.status,
            tags: value.tags,
        }
    }
}
