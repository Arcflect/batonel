use crate::config::placement::PlacementRulesConfig;
use crate::model::artifact::Artifact;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolutionError {
    #[error("Role '{0}' not found in placement rules")]
    RoleNotFound(String),
}

/// Resolves the concrete scaffold path for a given artifact based on placement rules.
pub fn resolve_artifact_path(
    artifact: &Artifact,
    placement_rules: &PlacementRulesConfig,
) -> Result<PathBuf, ResolutionError> {
    // 1. Explicit path override takes ultimate precedence
    if let Some(explicit_path) = &artifact.path {
        return Ok(PathBuf::from(explicit_path));
    }

    // 2. Lookup placement rules by the artifact's assigned role
    let role_config = placement_rules
        .roles
        .get(&artifact.role)
        .ok_or_else(|| ResolutionError::RoleNotFound(artifact.role.clone()))?;

    let mut path = PathBuf::from(&role_config.path);

    // 3. Construct the actual file name
    let file_name = match &role_config.file_extension {
        Some(ext) => format!("{}.{}", artifact.name, ext),
        None => artifact.name.clone(),
    };

    path.push(file_name);

    Ok(path)
}

/// Resolves the sibling paths for sidecar elements (contracts, prompts) based on the artifact path.
pub fn resolve_sidecar_path(
    artifact: &Artifact,
    artifact_path: &std::path::Path,
    override_dir: Option<&str>,
    extension: &str,
) -> PathBuf {
    let file_name = format!("{}.{}", artifact.name, extension);

    if let Some(dir) = override_dir {
        let mut path = PathBuf::from(dir);
        path.push(file_name);
        path
    } else {
        // Default: adjacent to the artifact
        if let Some(parent) = artifact_path.parent() {
            parent.join(file_name)
        } else {
            PathBuf::from(file_name)
        }
    }
}
