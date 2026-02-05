use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

/// Log level for configuration.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }

    pub fn is_trace(&self) -> bool {
        matches!(self, Self::Trace)
    }

    pub fn is_debug(&self) -> bool {
        matches!(self, Self::Debug)
    }

    pub fn is_info(&self) -> bool {
        matches!(self, Self::Info)
    }

    pub fn is_warn(&self) -> bool {
        matches!(self, Self::Warn)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Configuration for a single logging namespace.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct LogConfig {
    #[serde(default)]
    pub level: Option<LogLevel>, // only show this level

    #[serde(default)]
    pub format: Option<String>, // my::app:::*::hi"

    #[serde(default)]
    pub output: Option<String>, // stdout, stderror, path/to/file.txt
}

/// Logging configuration as a key/value store.
/// Keys are namespace strings, values are LogConfig objects.
pub type LoggingConfig = HashMap<String, LogConfig>;
