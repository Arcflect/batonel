use crate::config::artifact::Artifact;
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

pub fn generate_artifact_file(artifact: &Artifact, path: &Path) -> Result<(), ScaffoldError> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| ScaffoldError::DirCreation {
                path: parent.to_string_lossy().to_string(),
                source: e,
            })?;
        }
    }

    // Only generate if not exists so we don't destructively overwrite existing code.
    if !path.exists() {
        let content = format!("// Archflow placeholder: {} ({})\n", artifact.name, artifact.role);
        fs::write(path, content).map_err(|e| ScaffoldError::FileWrite {
            path: path.to_string_lossy().to_string(),
            source: e,
        })?;
    }

    Ok(())
}
