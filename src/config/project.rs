use crate::model::project::{Module, Project, Workspace};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

use super::error::ConfigError;
use super::raw::{RawBatonelMetadata, RawPresetReference, RawProjectConfig};

pub const SUPPORTED_PROJECT_SCHEMA_VERSION: &str = "1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatonelMetadata {
    pub schema_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<PresetReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetReference {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batonel: Option<BatonelMetadata>,
    pub project: Project,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Workspace>,
    pub modules: Vec<Module>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_yaml::Value>,
}

impl ProjectConfig {
    /// Loads the project configuration from a given file path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let contents = crate::config::loader::load_text(path)?;
        let raw = crate::config::parser::parse_project_raw(&contents, path)?;
        Self::from_raw(raw, path)
    }

    pub fn from_raw<P: AsRef<Path>>(raw: RawProjectConfig, path: P) -> Result<Self, ConfigError> {
        let config = ProjectConfig {
            batonel: raw.batonel.map(|a| BatonelMetadata {
                schema_version: a.schema_version,
                preset: a.preset.map(|p| PresetReference { id: p.id }),
            }),
            project: raw.project,
            workspace: raw.workspace,
            modules: raw.modules,
            metadata: raw.metadata,
        };

        config.validate(path)?;
        Ok(config)
    }

    pub fn has_module(&self, module_name: &str) -> bool {
        self.modules.iter().any(|module| module.name == module_name)
    }

    fn validate<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let path = path.as_ref().to_path_buf();

        let batonel = self.batonel.as_ref().ok_or_else(|| ConfigError::Validation {
            path: path.clone(),
            message: "missing required top-level key: batonel".to_string(),
        })?;

        if batonel.schema_version != SUPPORTED_PROJECT_SCHEMA_VERSION {
            return Err(ConfigError::Validation {
                path: path.clone(),
                message: format!(
                    "batonel.schema_version must be '{}' (got '{}')",
                    SUPPORTED_PROJECT_SCHEMA_VERSION, batonel.schema_version
                ),
            });
        }

        if let Some(preset) = &batonel.preset {
            if !is_kebab_case(&preset.id) {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: format!(
                        "batonel.preset.id must use lowercase kebab-case (got '{}')",
                        preset.id
                    ),
                });
            }
        }

        if !is_kebab_case(&self.project.name) {
            return Err(ConfigError::Validation {
                path: path.clone(),
                message: format!(
                    "project.name must use lowercase kebab-case (got '{}')",
                    self.project.name
                ),
            });
        }

        if self.modules.is_empty() {
            return Err(ConfigError::Validation {
                path: path.clone(),
                message: "modules must contain at least one module".to_string(),
            });
        }

        let mut module_names = BTreeSet::new();
        for module in &self.modules {
            if module.name.trim().is_empty() {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: "modules[].name cannot be empty".to_string(),
                });
            }

            if !is_lowercase_identifier(&module.name) {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: format!(
                        "modules[].name must use lowercase letters, digits, '-' or '_' (got '{}')",
                        module.name
                    ),
                });
            }

            if !module_names.insert(module.name.as_str()) {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: format!("duplicate module name detected: '{}'", module.name),
                });
            }

            if let Some(features) = &module.features {
                let mut feature_names = BTreeSet::new();
                for feature in features {
                    if feature.trim().is_empty() {
                        return Err(ConfigError::Validation {
                            path: path.clone(),
                            message: format!(
                                "modules[].features contains an empty value in module '{}'",
                                module.name
                            ),
                        });
                    }

                    if !is_lowercase_identifier(feature) {
                        return Err(ConfigError::Validation {
                            path: path.clone(),
                            message: format!(
                                "feature names must use lowercase letters, digits, '-' or '_' (got '{}')",
                                feature
                            ),
                        });
                    }

                    if !feature_names.insert(feature.as_str()) {
                        return Err(ConfigError::Validation {
                            path: path.clone(),
                            message: format!(
                                "duplicate feature '{}' detected in module '{}'",
                                feature, module.name
                            ),
                        });
                    }
                }
            }
        }

        if let Some(workspace) = &self.workspace {
            if workspace.enabled {
                if let Some(members) = &workspace.members {
                    let mut seen = BTreeSet::new();
                    for member in members {
                        if member.trim().is_empty() {
                            return Err(ConfigError::Validation {
                                path: path.clone(),
                                message: "workspace.members cannot contain empty entries".to_string(),
                            });
                        }
                        if !seen.insert(member.as_str()) {
                            return Err(ConfigError::Validation {
                                path: path.clone(),
                                message: format!(
                                    "duplicate workspace member detected: '{}'",
                                    member
                                ),
                            });
                        }
                    }
                }
            } else if workspace.members.as_ref().is_some_and(|members| !members.is_empty()) {
                return Err(ConfigError::Validation {
                    path,
                    message:
                        "workspace.members cannot be set when workspace.enabled is false"
                            .to_string(),
                });
            }
        }

        Ok(())
    }
}

impl From<PresetReference> for RawPresetReference {
    fn from(value: PresetReference) -> Self {
        RawPresetReference { id: value.id }
    }
}

impl From<BatonelMetadata> for RawBatonelMetadata {
    fn from(value: BatonelMetadata) -> Self {
        RawBatonelMetadata {
            schema_version: value.schema_version,
            preset: value.preset.map(Into::into),
        }
    }
}

fn is_kebab_case(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
}

fn is_lowercase_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_'
        })
        && !value.starts_with(['-', '_'])
        && !value.ends_with(['-', '_'])
}

#[cfg(test)]
mod tests {
    use super::ProjectConfig;
    use tempfile::tempdir;

    fn write_and_load(contents: &str) -> Result<ProjectConfig, crate::config::error::ConfigError> {
        let temp = tempdir().expect("tempdir should be created");
        let path = temp.path().join("project.baton.yaml");
        std::fs::write(&path, contents).expect("config should be written");
        ProjectConfig::load(&path)
    }

    #[test]
    fn load_accepts_versioned_batonel_metadata() {
        let config = write_and_load(
            r#"batonel:
  schema_version: "1"
  preset:
    id: generic-layered
project:
  name: sample-app
  architecture_style: layered
  language: generic
modules:
  - name: user
    features:
      - create_user
"#,
        )
        .expect("config should load");

        assert_eq!(config.batonel.as_ref().unwrap().schema_version, "1");
        assert!(config.has_module("user"));
    }

    #[test]
    fn load_rejects_missing_batonel_block() {
        let err = write_and_load(
            r#"project:
  name: sample-app
  architecture_style: layered
  language: generic
modules:
  - name: user
"#,
        )
        .expect_err("missing batonel should fail");

        assert!(err
            .to_string()
            .contains("missing required top-level key: batonel"));
    }

    #[test]
    fn load_rejects_unsupported_schema_version() {
        let err = write_and_load(
            r#"batonel:
  schema_version: "2"
project:
  name: sample-app
  architecture_style: layered
  language: generic
modules:
  - name: user
"#,
        )
        .expect_err("unsupported version should fail");

        assert!(err
            .to_string()
            .contains("batonel.schema_version must be '1' (got '2')"));
    }

    #[test]
    fn load_rejects_duplicate_module_names() {
        let err = write_and_load(
            r#"batonel:
  schema_version: "1"
project:
  name: sample-app
  architecture_style: layered
  language: generic
modules:
  - name: user
  - name: user
"#,
        )
        .expect_err("duplicate modules should fail");

        assert!(err.to_string().contains("duplicate module name detected: 'user'"));
    }
}
