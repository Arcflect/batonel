use std::path::Path;

use super::error::ConfigError;

/// Load raw text from disk. This stage performs only I/O, no parsing.
pub fn load_text<P: AsRef<Path>>(path: P) -> Result<String, ConfigError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(ConfigError::NotFound(path.to_path_buf()));
    }

    std::fs::read_to_string(path).map_err(|e| ConfigError::Io {
        path: path.to_path_buf(),
        source: e,
    })
}
