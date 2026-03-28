use crate::config::artifact::Artifact;
use crate::config::placement::PlacementRulesConfig;
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
