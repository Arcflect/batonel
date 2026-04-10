/// Org/team override precedence model for Archflow policy profiles.
///
/// Precedence chain (highest to lowest priority):
///   org   →  team  →  project
///
/// Each level contributes its overrides and forbidden_dependency policies.
/// When the same rule_id + target is overridden at multiple levels, the highest-priority
/// level wins. The `resolve` command surfaces the full effective-policy resolution
/// so that governance is auditable and predictable.
use crate::config::policy::{
    GovernanceRoleBinding, NamingPolicy, NamingRule, PolicyOverride, PolicyProfileConfig,
    RoleForbiddenDependencyPolicy, SUPPORTED_POLICY_PROFILE_VERSION,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::config::error::ConfigError;

// ---------------------------------------------------------------------------
// Override level identifier
// ---------------------------------------------------------------------------

/// Identifies the source level of an override or policy control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverrideLevel {
    /// Applies to the entire organisation (highest priority).
    Org,
    /// Applies to a specific team within the organisation.
    Team,
    /// Applies to a single repository/project (lowest priority).
    Project,
    /// Built-in minimum default (no explicit configuration found).
    Default,
}

impl OverrideLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            OverrideLevel::Org => "org",
            OverrideLevel::Team => "team",
            OverrideLevel::Project => "project",
            OverrideLevel::Default => "default",
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            OverrideLevel::Org => 3,
            OverrideLevel::Team => 2,
            OverrideLevel::Project => 1,
            OverrideLevel::Default => 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Org-level policy override file
// ---------------------------------------------------------------------------

/// A single policy layer loaded from org, team, or project level.
///
/// Stored in any of:
///   `.archflow/org.policy.yaml`
///   `.archflow/team.policy.yaml`
///   `policy.profile.yaml`  (project level — existing format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyLayer {
    pub version: u32,
    /// Human-readable label for this layer (e.g. team name).
    #[serde(default)]
    pub label: String,
    /// Rule IDs this layer locks (cannot be overridden by lower levels).
    #[serde(default)]
    pub locked_rules: Vec<String>,
    /// Overrides contributed by this layer.
    #[serde(default)]
    pub overrides: Vec<PolicyOverride>,
    /// Forbidden dependency policies contributed by this layer.
    #[serde(default)]
    pub forbidden_dependencies: Vec<RoleForbiddenDependencyPolicy>,
    /// Naming rules contributed by this layer (if any).
    #[serde(default)]
    pub naming: Option<NamingPolicy>,
    /// Required files enforced by this layer (merged with lower levels).
    #[serde(default)]
    pub required_files: Vec<String>,
    /// Governance roles contributed by this layer.
    #[serde(default)]
    pub governance_roles: Vec<GovernanceRoleBinding>,
}

impl PolicyLayer {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Option<Self>, ConfigError> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(path).map_err(|e| ConfigError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

        let layer: PolicyLayer = serde_yaml::from_str(&contents).map_err(|e| ConfigError::Parse {
            path: path.to_path_buf(),
            source: e,
        })?;

        if layer.version != SUPPORTED_POLICY_PROFILE_VERSION {
            return Err(ConfigError::Validation {
                path: path.to_path_buf(),
                message: format!(
                    "policy layer version must be '{}' (got '{}')",
                    SUPPORTED_POLICY_PROFILE_VERSION, layer.version
                ),
            });
        }

        Ok(Some(layer))
    }
}

// ---------------------------------------------------------------------------
// Effective policy entry: tracks where each decision came from
// ---------------------------------------------------------------------------

/// A single override entry in the resolved effective policy, with its source.
#[derive(Debug, Clone)]
pub struct ResolvedOverride {
    pub rule_id: String,
    pub targets: Vec<String>,
    pub reason: String,
    pub expires_at: Option<String>,
    pub is_expired: bool,
    #[allow(dead_code)]
    pub source_level: OverrideLevel,
    pub source_label: String,
}

/// A forbidden-dependency entry in the resolved policy.
#[derive(Debug, Clone)]
pub struct ResolvedForbiddenDependency {
    pub role: String,
    pub forbidden_entries: Vec<String>,
    #[allow(dead_code)]
    pub source_level: OverrideLevel,
    pub source_label: String,
}

/// A required-files entry in the resolved policy.
#[derive(Debug, Clone)]
pub struct ResolvedRequiredFile {
    pub filename: String,
    #[allow(dead_code)]
    pub source_level: OverrideLevel,
    pub source_label: String,
}

/// A governance role binding in the resolved policy.
#[derive(Debug, Clone)]
pub struct ResolvedGovernanceRoleBinding {
    pub role: String,
    pub members: Vec<String>,
    #[allow(dead_code)]
    pub source_level: OverrideLevel,
    pub source_label: String,
}

/// A locked-rule entry: this rule cannot be overridden by lower levels.
#[derive(Debug, Clone)]
pub struct ResolvedLockedRule {
    pub rule_id: String,
    pub source_level: OverrideLevel,
    pub source_label: String,
}

/// The fully resolved effective policy including source attribution for each entry.
#[derive(Debug, Clone)]
pub struct EffectivePolicy {
    /// Naming rule for modules, with source attribution.
    pub module_naming: (NamingRule, OverrideLevel, String),
    /// Naming rule for artifacts, with source attribution.
    pub artifact_naming: (NamingRule, OverrideLevel, String),
    /// All resolved overrides (deduplicated by rule_id+target; highest level wins).
    pub overrides: Vec<ResolvedOverride>,
    /// All resolved forbidden-dependency policies (highest level wins per role).
    pub forbidden_dependencies: Vec<ResolvedForbiddenDependency>,
    /// Union of all required files from all levels.
    pub required_files: Vec<ResolvedRequiredFile>,
    /// Rules locked by org or team that cannot be suppressed at lower levels.
    pub locked_rules: Vec<ResolvedLockedRule>,
    /// All resolved governance roles (highest level wins per role).
    pub governance_roles: Vec<ResolvedGovernanceRoleBinding>,
}

impl EffectivePolicy {
    #[allow(dead_code)]
    /// Check whether the given override is allowed (i.e. the rule is not locked by a higher level).
    pub fn is_override_allowed(&self, rule_id: &str, requesting_level: &OverrideLevel) -> bool {
        !self.locked_rules.iter().any(|locked| {
            locked.rule_id == rule_id
                && locked.source_level.precedence() > requesting_level.precedence()
        })
    }

    /// Check whether a rule_id + target is effectively overridden.
    #[allow(dead_code)]
    pub fn is_overridden(&self, rule_id: &str, target: &str) -> bool {
        self.overrides.iter().any(|entry| {
            !entry.is_expired
                && entry.rule_id == rule_id
                && entry.targets.iter().any(|candidate| candidate == target)
        })
    }

    /// Get forbidden entries for a role (highest-precedence source wins).
    #[allow(dead_code)]
    pub fn forbidden_entries_for_role(&self, role: &str) -> Option<&[String]> {
        self.forbidden_dependencies
            .iter()
            .find(|dep| dep.role == role)
            .map(|dep| dep.forbidden_entries.as_slice())
    }

    /// Produce a `PolicyProfileConfig` compatible view for use by existing audit code.
    pub fn to_policy_profile_config(&self) -> PolicyProfileConfig {
        let naming = rule_pair(&self.module_naming.0, &self.artifact_naming.0);
        PolicyProfileConfig {
            version: SUPPORTED_POLICY_PROFILE_VERSION,
            required_files: self.required_files.iter().map(|r| r.filename.clone()).collect(),
            naming,
            forbidden_dependencies: self
                .forbidden_dependencies
                .iter()
                .map(|dep| RoleForbiddenDependencyPolicy {
                    role: dep.role.clone(),
                    forbidden_entries: dep.forbidden_entries.clone(),
                })
                .collect(),
            overrides: self
                .overrides
                .iter()
                .map(|entry| PolicyOverride {
                    rule_id: entry.rule_id.clone(),
                    targets: entry.targets.clone(),
                    reason: entry.reason.clone(),
                    expires_at: entry.expires_at.clone(),
                })
                .collect(),
            governance_roles: self
                .governance_roles
                .iter()
                .map(|role| GovernanceRoleBinding {
                    role: role.role.clone(),
                    members: role.members.clone(),
                })
                .collect(),
        }
    }
}

fn rule_pair(module: &NamingRule, artifact: &NamingRule) -> NamingPolicy {
    NamingPolicy {
        module: module.clone(),
        artifact: artifact.clone(),
    }
}

// ---------------------------------------------------------------------------
// Resolution engine
// ---------------------------------------------------------------------------

/// Standard lookup paths for each policy level.
pub const ORG_POLICY_PATH: &str = ".archflow/org.policy.yaml";
pub const TEAM_POLICY_PATH: &str = ".archflow/team.policy.yaml";
pub const PROJECT_POLICY_PATH: &str = "policy.profile.yaml";

/// Load all layers and resolve the effective policy.
///
/// Call this with paths to the org, team, and project policy files.
/// Any path that is `None` or points to a non-existent file is skipped.
pub fn resolve(
    org_path: Option<&Path>,
    team_path: Option<&Path>,
    project_path: Option<&Path>,
) -> Result<EffectivePolicy, String> {
    // Build a list of (level, optional_layer) tuples from highest to lowest priority.
    let layer_specs: Vec<(OverrideLevel, Option<&Path>)> = vec![
        (OverrideLevel::Org, org_path),
        (OverrideLevel::Team, team_path),
        (OverrideLevel::Project, project_path),
    ];

    let mut loaded: Vec<(OverrideLevel, PolicyLayer)> = Vec::new();

    for (level, path_opt) in &layer_specs {
        if let Some(path) = path_opt {
            match PolicyLayer::load(path) {
                Ok(Some(layer)) => loaded.push((level.clone(), layer)),
                Ok(None) => {}
                Err(err) => {
                    return Err(format!(
                        "failed to load {} policy layer from '{}': {}",
                        level.as_str(),
                        path.display(),
                        err
                    ));
                }
            }
        }
    }

    // If nothing was loaded, resolve from default.
    let project_default = PolicyProfileConfig::default_minimum();

    // Determine which rules are locked (by org or team level).
    let mut locked_rules: Vec<ResolvedLockedRule> = Vec::new();
    let mut seen_locked: std::collections::HashSet<String> = Default::default();
    for (level, layer) in &loaded {
        if level == &OverrideLevel::Project {
            continue; // projects cannot lock rules
        }
        for rule_id in &layer.locked_rules {
            if seen_locked.insert(rule_id.clone()) {
                locked_rules.push(ResolvedLockedRule {
                    rule_id: rule_id.clone(),
                    source_level: level.clone(),
                    source_label: friendly_label(level, &layer.label),
                });
            }
        }
    }

    // Resolve overrides: highest-precedence level wins for each (rule_id, target) pair.
    let mut resolved_overrides: Vec<ResolvedOverride> = Vec::new();
    // Track (rule_id, target) pairs we have already committed.
    let mut seen_override: std::collections::HashSet<(String, String)> = Default::default();

    for (level, layer) in &loaded {
        for entry in &layer.overrides {
            // Entries at this level are blocked if the rule is locked by a higher-priority level.
            if !is_allowed_by_locks(&locked_rules, &entry.rule_id, level) {
                continue;
            }

            let mut new_targets = Vec::new();
            for target in &entry.targets {
                let key = (entry.rule_id.clone(), target.clone());
                if seen_override.insert(key) {
                    new_targets.push(target.clone());
                }
            }
            if !new_targets.is_empty() {
                let mut is_expired = false;
                if let Some(expires_at) = &entry.expires_at {
                    if let Ok(expiration_date) = chrono::NaiveDate::parse_from_str(expires_at, "%Y-%m-%d") {
                        let today = chrono::Utc::now().naive_utc().date();
                        if expiration_date < today {
                            is_expired = true;
                        }
                    }
                }

                resolved_overrides.push(ResolvedOverride {
                    rule_id: entry.rule_id.clone(),
                    targets: new_targets,
                    reason: entry.reason.clone(),
                    expires_at: entry.expires_at.clone(),
                    is_expired,
                    source_level: level.clone(),
                    source_label: friendly_label(level, &layer.label),
                });
            }
        }
    }

    // Resolve forbidden dependencies: highest-precedence level wins per role.
    let mut resolved_forbidden: Vec<ResolvedForbiddenDependency> = Vec::new();
    let mut seen_role: std::collections::HashSet<String> = Default::default();

    for (level, layer) in &loaded {
        for dep in &layer.forbidden_dependencies {
            if seen_role.insert(dep.role.clone()) {
                resolved_forbidden.push(ResolvedForbiddenDependency {
                    role: dep.role.clone(),
                    forbidden_entries: dep.forbidden_entries.clone(),
                    source_level: level.clone(),
                    source_label: friendly_label(level, &layer.label),
                });
            }
        }
    }

    // Resolve governance roles: highest-precedence level wins per role.
    let mut resolved_roles: Vec<ResolvedGovernanceRoleBinding> = Vec::new();
    let mut seen_gov_role: std::collections::HashSet<String> = Default::default();

    for (level, layer) in &loaded {
        for role_binding in &layer.governance_roles {
            if seen_gov_role.insert(role_binding.role.clone()) {
                resolved_roles.push(ResolvedGovernanceRoleBinding {
                    role: role_binding.role.clone(),
                    members: role_binding.members.clone(),
                    source_level: level.clone(),
                    source_label: friendly_label(level, &layer.label),
                });
            }
        }
    }

    // Resolve required files: union of all levels (no deduplication).
    let mut resolved_required: Vec<ResolvedRequiredFile> = Vec::new();
    let mut seen_file: std::collections::HashSet<String> = Default::default();

    // Default minimum files come first.
    for filename in &project_default.required_files {
        if seen_file.insert(filename.clone()) {
            resolved_required.push(ResolvedRequiredFile {
                filename: filename.clone(),
                source_level: OverrideLevel::Default,
                source_label: "default".to_string(),
            });
        }
    }

    // Layers contribute in reverse order (project → team → org) so that org additions
    // appear as org-sourced in the display.
    for (level, layer) in loaded.iter().rev() {
        for filename in &layer.required_files {
            if seen_file.insert(filename.clone()) {
                resolved_required.push(ResolvedRequiredFile {
                    filename: filename.clone(),
                    source_level: level.clone(),
                    source_label: friendly_label(level, &layer.label),
                });
            }
        }
    }

    // Resolve naming (highest-priority level that defines naming wins).
    let (module_naming, artifact_naming) = resolve_naming(&loaded, &project_default);

    Ok(EffectivePolicy {
        module_naming,
        artifact_naming,
        overrides: resolved_overrides,
        forbidden_dependencies: resolved_forbidden,
        required_files: resolved_required,
        locked_rules,
        governance_roles: resolved_roles,
    })
}

fn is_allowed_by_locks(
    locked_rules: &[ResolvedLockedRule],
    rule_id: &str,
    requesting_level: &OverrideLevel,
) -> bool {
    !locked_rules.iter().any(|locked| {
        locked.rule_id == rule_id
            && locked.source_level.precedence() > requesting_level.precedence()
    })
}

fn resolve_naming(
    loaded: &[(OverrideLevel, PolicyLayer)],
    default: &PolicyProfileConfig,
) -> (
    (NamingRule, OverrideLevel, String),
    (NamingRule, OverrideLevel, String),
) {
    for (level, layer) in loaded {
        if let Some(naming) = &layer.naming {
            let label = friendly_label(level, &layer.label);
            return (
                (naming.module.clone(), level.clone(), label.clone()),
                (naming.artifact.clone(), level.clone(), label),
            );
        }
    }
    (
        (
            default.naming.module.clone(),
            OverrideLevel::Default,
            "default".to_string(),
        ),
        (
            default.naming.artifact.clone(),
            OverrideLevel::Default,
            "default".to_string(),
        ),
    )
}

fn friendly_label(level: &OverrideLevel, label: &str) -> String {
    if label.trim().is_empty() {
        level.as_str().to_string()
    } else {
        format!("{} ({})", level.as_str(), label)
    }
}

// ---------------------------------------------------------------------------
// Default policy path loader
// ---------------------------------------------------------------------------

/// Load and resolve the effective policy using the standard lookup paths.
pub fn load_effective_policy() -> Result<EffectivePolicy, String> {
    resolve(
        Some(Path::new(ORG_POLICY_PATH)),
        Some(Path::new(TEAM_POLICY_PATH)),
        Some(Path::new(PROJECT_POLICY_PATH)),
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write(dir: &Path, name: &str, content: &str) {
        if let Some(p) = dir.join(name).parent() {
            fs::create_dir_all(p).unwrap();
        }
        fs::write(dir.join(name), content).unwrap();
    }

    fn org_layer_yaml(locked_rules: &[&str], overrides_yaml: &str) -> String {
        let locked = if locked_rules.is_empty() {
            "[]".to_string()
        } else {
            locked_rules
                .iter()
                .map(|r| format!("\n  - {}", r))
                .collect::<String>()
        };
        format!(
            "version: 1\nlabel: acme-org\nlocked_rules:{}\n{}\n",
            locked, overrides_yaml
        )
    }

    #[test]
    fn resolve_empty_layers_returns_defaults() {
        let tmp = tempdir().unwrap();
        let ep = resolve(None, None, Some(tmp.path().join("missing.yaml").as_path())).unwrap();
        // Should have default required files.
        assert!(!ep.required_files.is_empty());
        assert!(ep.overrides.is_empty());
        assert!(ep.locked_rules.is_empty());
    }

    #[test]
    fn project_override_is_applied() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "project.yaml",
            r#"
version: 1
overrides:
  - rule_id: required-root-file
    targets:
      - some-optional.yaml
    reason: this file is managed externally
"#,
        );

        let ep = resolve(None, None, Some(&tmp.path().join("project.yaml"))).unwrap();
        assert!(ep.is_overridden("required-root-file", "some-optional.yaml"));
    }

    #[test]
    fn org_override_takes_precedence_over_project() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "org.yaml",
            r#"
version: 1
label: acme-org
overrides:
  - rule_id: module-name-policy
    targets:
      - module:LegacyModule
    reason: legacy module kept for migration
"#,
        );
        write(
            tmp.path(),
            "project.yaml",
            r#"
version: 1
overrides:
  - rule_id: module-name-policy
    targets:
      - module:LegacyModule
    reason: project-level duplicate (should be shadowed by org entry)
"#,
        );

        let ep = resolve(
            Some(&tmp.path().join("org.yaml")),
            None,
            Some(&tmp.path().join("project.yaml")),
        )
        .unwrap();

        let matching: Vec<&ResolvedOverride> = ep
            .overrides
            .iter()
            .filter(|o| o.rule_id == "module-name-policy")
            .collect();
        // There should be exactly one resolved entry for this target (not a duplicate).
        assert_eq!(matching.len(), 1);
        assert_eq!(matching[0].source_level, OverrideLevel::Org);
    }

    #[test]
    fn locked_rule_prevents_project_override() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "org.yaml",
            &org_layer_yaml(&["module-name-policy"], "overrides: []"),
        );
        write(
            tmp.path(),
            "project.yaml",
            r#"
version: 1
overrides:
  - rule_id: module-name-policy
    targets:
      - module:SomeModule
    reason: project wants to suppress naming rule
"#,
        );

        let ep = resolve(
            Some(&tmp.path().join("org.yaml")),
            None,
            Some(&tmp.path().join("project.yaml")),
        )
        .unwrap();

        // Org locked the rule; project override should not be applied.
        assert!(!ep.is_overridden("module-name-policy", "module:SomeModule"));
        assert!(!ep.locked_rules.is_empty());
        assert_eq!(ep.locked_rules[0].rule_id, "module-name-policy");
        assert_eq!(ep.locked_rules[0].source_level, OverrideLevel::Org);
    }

    #[test]
    fn team_override_blocked_by_org_lock() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "org.yaml",
            &org_layer_yaml(&["artifact-name-policy"], "overrides: []"),
        );
        write(
            tmp.path(),
            "team.yaml",
            r#"
version: 1
label: backend-team
overrides:
  - rule_id: artifact-name-policy
    targets:
      - artifact:MyArtifact
    reason: team exception
"#,
        );

        let ep = resolve(
            Some(&tmp.path().join("org.yaml")),
            Some(&tmp.path().join("team.yaml")),
            None,
        )
        .unwrap();

        // Org locked the rule; team override should also be blocked.
        assert!(!ep.is_overridden("artifact-name-policy", "artifact:MyArtifact"));
    }

    #[test]
    fn required_files_are_union_of_all_levels() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "org.yaml",
            r#"
version: 1
required_files:
  - org-required.yaml
"#,
        );
        write(
            tmp.path(),
            "project.yaml",
            r#"
version: 1
required_files:
  - project-required.yaml
"#,
        );

        let ep = resolve(
            Some(&tmp.path().join("org.yaml")),
            None,
            Some(&tmp.path().join("project.yaml")),
        )
        .unwrap();

        let filenames: Vec<&str> = ep.required_files.iter().map(|r| r.filename.as_str()).collect();
        assert!(filenames.contains(&"org-required.yaml"), "expected org-required.yaml in {:?}", filenames);
        assert!(filenames.contains(&"project-required.yaml"), "expected project-required.yaml in {:?}", filenames);
        // Default files should also be present.
        assert!(filenames.contains(&"project.arch.yaml"), "expected project.arch.yaml in {:?}", filenames);
    }

    #[test]
    fn naming_resolved_from_highest_level_with_naming() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "org.yaml",
            r#"
version: 1
naming:
  module: kebab-case
  artifact: kebab-case
"#,
        );
        write(
            tmp.path(),
            "project.yaml",
            r#"
version: 1
naming:
  module: lowercase-identifier
  artifact: lowercase-identifier
"#,
        );

        let ep = resolve(
            Some(&tmp.path().join("org.yaml")),
            None,
            Some(&tmp.path().join("project.yaml")),
        )
        .unwrap();

        // Org is highest priority — its naming should win.
        assert!(
            matches!(ep.module_naming.0, NamingRule::KebabCase),
            "Expected org naming rule (kebab-case)"
        );
        assert_eq!(ep.module_naming.1, OverrideLevel::Org);
    }

    #[test]
    fn forbidden_dependencies_highest_level_wins_per_role() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "org.yaml",
            r#"
version: 1
forbidden_dependencies:
  - role: service
    forbidden_entries:
      - infrastructure_module
"#,
        );
        write(
            tmp.path(),
            "project.yaml",
            r#"
version: 1
forbidden_dependencies:
  - role: service
    forbidden_entries:
      - project_level_entry
"#,
        );

        let ep = resolve(
            Some(&tmp.path().join("org.yaml")),
            None,
            Some(&tmp.path().join("project.yaml")),
        )
        .unwrap();

        let deps = ep.forbidden_entries_for_role("service").unwrap();
        // Org wins; project-level entry should not be present.
        assert!(deps.contains(&"infrastructure_module".to_string()));
        assert!(!deps.contains(&"project_level_entry".to_string()));
    }

    #[test]
    fn governance_roles_highest_level_wins_per_role() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "org.yaml",
            r#"
version: 1
governance_roles:
  - role: auditor
    members:
      - "@org-auditor"
"#,
        );
        write(
            tmp.path(),
            "project.yaml",
            r#"
version: 1
governance_roles:
  - role: auditor
    members:
      - "@project-auditor"
"#,
        );

        let ep = resolve(
            Some(&tmp.path().join("org.yaml")),
            None,
            Some(&tmp.path().join("project.yaml")),
        )
        .unwrap();

        let roles: Vec<&ResolvedGovernanceRoleBinding> = ep
            .governance_roles
            .iter()
            .filter(|r| r.role == "auditor")
            .collect();
        assert_eq!(roles.len(), 1);
        assert_eq!(roles[0].members, vec!["@org-auditor".to_string()]);
        assert_eq!(roles[0].source_level, OverrideLevel::Org);
    }

    #[test]
    fn override_is_ignored_when_expired() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "project.yaml",
            r#"
version: 1
overrides:
  - rule_id: expiring-rule
    targets: ["some.yml"]
    reason: "testing"
    expires_at: "2010-01-01"
"#,
        );
        let ep = resolve(None, None, Some(&tmp.path().join("project.yaml"))).unwrap();
        assert!(!ep.is_overridden("expiring-rule", "some.yml"), "Expired override should not be actively applied");
    }

    #[test]
    fn override_is_applied_when_not_expired() {
        let tmp = tempdir().unwrap();
        // Use a far future date
        write(
            tmp.path(),
            "project.yaml",
            r#"
version: 1
overrides:
  - rule_id: expiring-rule
    targets: ["some.yml"]
    reason: "testing"
    expires_at: "2099-01-01"
"#,
        );
        let ep = resolve(None, None, Some(&tmp.path().join("project.yaml"))).unwrap();
        assert!(ep.is_overridden("expiring-rule", "some.yml"), "Active override should be applied");
    }

    #[test]
    fn to_policy_profile_config_roundtrip() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "org.yaml",
            r#"
version: 1
overrides:
  - rule_id: required-root-file
    targets:
      - extras.yaml
    reason: managed externally
"#,
        );

        let ep = resolve(Some(&tmp.path().join("org.yaml")), None, None).unwrap();
        let ppc = ep.to_policy_profile_config();
        assert!(ppc.is_overridden("required-root-file", "extras.yaml"));
    }
}
