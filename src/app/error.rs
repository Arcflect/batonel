use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigLoadError {
    #[error("failed to load '{path}': {source}")]
    Load {
        path: String,
        #[source]
        source: crate::config::error::ConfigError,
    },
}

#[derive(Debug, Error)]
pub enum PresetResolutionError {
    #[error("preset id must not be empty")]
    EmptyPresetId,
    #[error("preset id must use lowercase kebab-case (got '{preset_id}')")]
    InvalidPresetId { preset_id: String },
}

#[derive(Debug, Error)]
pub enum PlanBuildError {
    #[error(transparent)]
    Config(#[from] ConfigLoadError),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error(transparent)]
    Config(#[from] ConfigLoadError),
}

#[derive(Debug, Error)]
pub enum GenerationError {
    #[error(transparent)]
    Config(#[from] ConfigLoadError),
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    ConfigLoad(#[from] ConfigLoadError),
    #[error(transparent)]
    PresetResolution(#[from] PresetResolutionError),
    #[error(transparent)]
    PlanBuild(#[from] PlanBuildError),
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    Generation(#[from] GenerationError),
    #[error("render error: {0}")]
    Render(String),
}
