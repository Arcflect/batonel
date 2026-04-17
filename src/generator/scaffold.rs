use crate::model::artifact::Artifact;
use crate::model::placement::RolePlacement;
use std::fs;
use std::path::Path;
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
        &format!(
            "// Batonel placeholder: {} ({})\n",
            artifact.name, artifact.role
        ),
    )?;

    // 2. Generate sidecar files
    let contract_path = crate::generator::resolver::resolve_sidecar_path(
        artifact,
        artifact_path,
        role_config.and_then(|r| r.sidecar.as_ref().and_then(|s| s.contract_dir.as_deref())),
        "contract.yaml",
    );
    write_file_safely(
        &contract_path,
        &format!(
            "name: {}\nmodule: {}\nrole: {}\npath: {}\nstatus: planned\nresponsibilities:\n  - TODO\nmust_not:\n  - TODO\n",
            artifact.name, artifact.module, artifact.role, artifact_path.to_string_lossy()
        ),
    )?;

    let prompt_path = crate::generator::resolver::resolve_sidecar_path(
        artifact,
        artifact_path,
        role_config.and_then(|r| r.sidecar.as_ref().and_then(|s| s.prompt_dir.as_deref())),
        "prompt.md",
    );
    write_file_safely(
        &prompt_path,
        &format!(
            "<!-- Prompt override for: {} ({}) -->\n",
            artifact.name, artifact.role
        ),
    )?;

    Ok(())
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
