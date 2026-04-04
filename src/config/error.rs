use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file not found: {0}")]
    NotFound(PathBuf),

    #[error("Failed to read configuration file at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse configuration file at {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("Invalid configuration at {path}: {message}")]
    Validation { path: PathBuf, message: String },
}
