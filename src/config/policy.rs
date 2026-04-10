use serde::{Deserialize, Serialize};
#[cfg(test)]
use std::collections::HashSet;
#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::path::Path;

#[cfg(test)]
use super::error::ConfigError;

pub const SUPPORTED_POLICY_PROFILE_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyProfileConfig {
    pub version: u32,
    pub required_files: Vec<String>,
    pub naming: NamingPolicy,
    #[serde(default)]
    pub forbidden_dependencies: Vec<RoleForbiddenDependencyPolicy>,
    #[serde(default)]
    pub overrides: Vec<PolicyOverride>,
    #[serde(default)]
    pub governance_roles: Vec<GovernanceRoleBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingPolicy {
    pub module: NamingRule,
    pub artifact: NamingRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NamingRule {
    KebabCase,
    LowercaseIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleForbiddenDependencyPolicy {
    pub role: String,
    pub forbidden_entries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyOverride {
    pub rule_id: String,
    pub targets: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRoleBinding {
    pub role: String,
    pub members: Vec<String>,
}

impl PolicyProfileConfig {
    #[cfg(test)]
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default_minimum());
        }

        let contents = fs::read_to_string(path).map_err(|e| ConfigError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

        let config: PolicyProfileConfig =
            serde_yaml::from_str(&contents).map_err(|e| ConfigError::Parse {
                path: path.to_path_buf(),
                source: e,
            })?;

        config.validate(path)?;
        Ok(config)
    }

    pub fn default_minimum() -> Self {
        Self {
            version: SUPPORTED_POLICY_PROFILE_VERSION,
            required_files: vec![
                "project.arch.yaml".to_string(),
                "placement.rules.yaml".to_string(),
                "artifacts.plan.yaml".to_string(),
                "contracts.template.yaml".to_string(),
            ],
            naming: NamingPolicy {
                module: NamingRule::LowercaseIdentifier,
                artifact: NamingRule::LowercaseIdentifier,
            },
            forbidden_dependencies: vec![],
            overrides: vec![],
            governance_roles: vec![],
        }
    }

    pub fn is_overridden(&self, rule_id: &str, target: &str) -> bool {
        self.overrides.iter().any(|entry| {
            entry.rule_id == rule_id && entry.targets.iter().any(|candidate| candidate == target)
        })
    }

    pub fn forbidden_entries_for_role(&self, role: &str) -> Option<&[String]> {
        self.forbidden_dependencies
            .iter()
            .find(|policy| policy.role == role)
            .map(|policy| policy.forbidden_entries.as_slice())
    }

    #[cfg(test)]
    fn validate<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let path = path.as_ref().to_path_buf();

        if self.version != SUPPORTED_POLICY_PROFILE_VERSION {
            return Err(ConfigError::Validation {
                path: path.clone(),
                message: format!(
                    "policy profile version must be '{}' (got '{}')",
                    SUPPORTED_POLICY_PROFILE_VERSION, self.version
                ),
            });
        }

        if self.required_files.is_empty() {
            return Err(ConfigError::Validation {
                path: path.clone(),
                message: "required_files must contain at least one file".to_string(),
            });
        }

        let mut required_file_set = HashSet::new();
        for filename in &self.required_files {
            if filename.trim().is_empty() {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: "required_files cannot contain empty values".to_string(),
                });
            }
            if !required_file_set.insert(filename.as_str()) {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: format!("duplicate required file '{}'", filename),
                });
            }
        }

        for dependency_policy in &self.forbidden_dependencies {
            if dependency_policy.role.trim().is_empty() {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: "forbidden_dependencies[].role cannot be empty".to_string(),
                });
            }

            if dependency_policy.forbidden_entries.is_empty() {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: format!(
                        "forbidden_entries for role '{}' cannot be empty",
                        dependency_policy.role
                    ),
                });
            }
        }

        for override_entry in &self.overrides {
            if override_entry.rule_id.trim().is_empty() {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: "overrides[].rule_id cannot be empty".to_string(),
                });
            }
            if override_entry.targets.is_empty() {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: format!(
                        "override '{}' must define at least one target",
                        override_entry.rule_id
                    ),
                });
            }
            if override_entry.reason.trim().is_empty() {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: format!(
                        "override '{}' must provide reason",
                        override_entry.rule_id
                    ),
                });
            }
        }

        for role_binding in &self.governance_roles {
            if role_binding.role.trim().is_empty() {
                return Err(ConfigError::Validation {
                    path: path.clone(),
                    message: "governance_roles[].role cannot be empty".to_string(),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PolicyProfileConfig;
    use tempfile::tempdir;

    #[test]
    fn load_or_default_returns_default_when_missing() {
        let temp = tempdir().expect("tempdir should be created");
        let path = temp.path().join("policy.profile.yaml");

        let config = PolicyProfileConfig::load_or_default(&path).expect("default should load");
        assert_eq!(config.version, 1);
        assert!(config
            .required_files
            .iter()
            .any(|filename| filename == "project.arch.yaml"));
    }

    #[test]
    fn load_or_default_rejects_invalid_override() {
        let temp = tempdir().expect("tempdir should be created");
        let path = temp.path().join("policy.profile.yaml");
        std::fs::write(
            &path,
            r#"version: 1
required_files:
  - project.arch.yaml
naming:
  module: lowercase-identifier
  artifact: lowercase-identifier
overrides:
  - rule_id: artifact-path-aligns-role
    targets: []
    reason: ""
"#,
        )
        .expect("policy should be written");

        let err = PolicyProfileConfig::load_or_default(&path)
            .expect_err("invalid override should fail");
        assert!(err.to_string().contains("must define at least one target"));
    }
}
