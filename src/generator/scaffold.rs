use crate::config::artifact::Artifact;
use crate::config::placement::RolePlacement;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScaffoldError {
    #[error("Failed to create directory {path}: {source}")]
    DirCreation {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to write file {path}: {source}")]
    FileWrite {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

pub fn generate_artifact_with_sidecars(
    artifact: &Artifact,
    artifact_path: &Path,
    role_config: Option<&RolePlacement>,
) -> Result<(), ScaffoldError> {
    // 1. Generate the main artifact file
    write_file_safely(
        artifact_path,
        &format!("// Archflow placeholder: {} ({})\n", artifact.name, artifact.role),
    )?;

    // 2. Generate sidecar files
    let contract_path = resolve_sidecar_path(
        artifact,
        artifact_path,
        role_config.and_then(|r| r.sidecar.as_ref().and_then(|s| s.contract_dir.as_deref())),
        "contract.yaml",
    );
    write_file_safely(
        &contract_path,
        &format!("# Contract template for: {}\n# Role: {}\n", artifact.name, artifact.role),
    )?;

    let prompt_path = resolve_sidecar_path(
        artifact,
        artifact_path,
        role_config.and_then(|r| r.sidecar.as_ref().and_then(|s| s.prompt_dir.as_deref())),
        "prompt.md",
    );
    write_file_safely(
        &prompt_path,
        &format!("<!-- Prompt override for: {} ({}) -->\n", artifact.name, artifact.role),
    )?;

    Ok(())
}

fn resolve_sidecar_path(
    artifact: &Artifact,
    artifact_path: &Path,
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

fn write_file_safely(path: &Path, content: &str) -> Result<(), ScaffoldError> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| ScaffoldError::DirCreation {
                path: parent.to_string_lossy().to_string(),
                source: e,
            })?;
        }
    }

    if !path.exists() {
        fs::write(path, content).map_err(|e| ScaffoldError::FileWrite {
            path: path.to_string_lossy().to_string(),
            source: e,
        })?;
    }

    Ok(())
}
