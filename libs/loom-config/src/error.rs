use std::io;

use loom_core::path::IdentPathError;

/// Errors that can occur during configuration operations
#[derive(Debug)]
pub enum ConfigError {
    /// Configuration file not found
    NotFound(String),

    /// IO error reading configuration
    IO(io::Error),

    /// Error parsing configuration content
    Parse(String),

    /// Error deserializing to target type
    Deserialize(String),

    /// Invalid field path
    InvalidPath(IdentPathError),

    /// Provider-specific error
    Provider(String),

    /// Circular include detected
    CircularInclude { file: String, chain: Vec<String> },

    /// Include file not found
    IncludeNotFound { path: String, source_file: String },
}

impl ConfigError {
    pub fn not_found<S: Into<String>>(path: S) -> Self {
        Self::NotFound(path.into())
    }

    pub fn parse<E: std::error::Error>(err: E) -> Self {
        Self::Parse(err.to_string())
    }

    pub fn deserialize<E: std::error::Error>(err: E) -> Self {
        Self::Deserialize(err.to_string())
    }

    pub fn provider<S: Into<String>>(msg: S) -> Self {
        Self::Provider(msg.into())
    }

    pub fn circular_include<S: Into<String>>(file: S, chain: Vec<String>) -> Self {
        Self::CircularInclude {
            file: file.into(),
            chain,
        }
    }

    pub fn include_not_found<S: Into<String>>(path: S, source_file: S) -> Self {
        Self::IncludeNotFound {
            path: path.into(),
            source_file: source_file.into(),
        }
    }

    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound(_))
    }

    pub fn is_io(&self) -> bool {
        matches!(self, Self::IO(_))
    }

    pub fn is_parse(&self) -> bool {
        matches!(self, Self::Parse(_))
    }

    pub fn is_deserialize(&self) -> bool {
        matches!(self, Self::Deserialize(_))
    }

    pub fn is_invalid_path(&self) -> bool {
        matches!(self, Self::InvalidPath(_))
    }

    pub fn is_provider(&self) -> bool {
        matches!(self, Self::Provider(_))
    }

    pub fn is_circular_include(&self) -> bool {
        matches!(self, Self::CircularInclude { .. })
    }

    pub fn is_include_not_found(&self) -> bool {
        matches!(self, Self::IncludeNotFound { .. })
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(path) => write!(f, "configuration not found: {}", path),
            Self::IO(e) => write!(f, "io error: {}", e),
            Self::Parse(msg) => write!(f, "parse error: {}", msg),
            Self::Deserialize(msg) => write!(f, "deserialize error: {}", msg),
            Self::InvalidPath(e) => write!(f, "invalid path: {}", e),
            Self::Provider(msg) => write!(f, "provider error: {}", msg),
            Self::CircularInclude { file, chain } => {
                write!(
                    f,
                    "circular include detected: {} (chain: {})",
                    file,
                    chain.join(" -> ")
                )
            }
            Self::IncludeNotFound { path, source_file } => {
                write!(
                    f,
                    "include not found: {} (referenced from {})",
                    path, source_file
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IO(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<IdentPathError> for ConfigError {
    fn from(err: IdentPathError) -> Self {
        Self::InvalidPath(err)
    }
}
