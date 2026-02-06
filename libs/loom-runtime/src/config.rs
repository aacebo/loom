use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::score::ScoreConfig;

/// Top-level configuration for Loom.
///
/// This configuration supports CLI parameter overrides at the top level,
/// with layer-specific configurations nested under `layers`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoomConfig {
    /// Output path for results
    #[serde(default)]
    pub output: Option<PathBuf>,

    /// Fail on unknown categories/labels instead of skipping
    #[serde(default)]
    pub strict: bool,

    /// Number of parallel inference workers
    #[serde(default = "LoomConfig::default_concurrency")]
    pub concurrency: usize,

    /// Batch size for ML inference
    #[serde(default = "LoomConfig::default_batch_size")]
    pub batch_size: usize,

    /// Layer configurations
    #[serde(default)]
    pub layers: LayersConfig,
}

impl LoomConfig {
    fn default_concurrency() -> usize {
        4
    }

    fn default_batch_size() -> usize {
        8
    }
}

impl Default for LoomConfig {
    fn default() -> Self {
        Self {
            output: None,
            strict: false,
            concurrency: Self::default_concurrency(),
            batch_size: Self::default_batch_size(),
            layers: LayersConfig::default(),
        }
    }
}

/// Layer configurations by name.
///
/// Each layer type has its own configuration struct.
/// Layers default to their respective `Default` implementations if not specified.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayersConfig {
    /// Score layer configuration
    #[serde(default)]
    pub score: ScoreConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_expected_values() {
        let config = LoomConfig::default();
        assert_eq!(config.concurrency, 4);
        assert_eq!(config.batch_size, 8);
        assert!(!config.strict);
        assert!(config.output.is_none());
    }

    #[test]
    fn config_deserializes_with_defaults() {
        let json = r#"{}"#;
        let config: LoomConfig = serde_json::from_str(json).unwrap();

        assert_eq!(config.concurrency, 4);
        assert_eq!(config.batch_size, 8);
        assert!(!config.strict);
        assert!(config.output.is_none());
    }

    #[test]
    fn config_deserializes_with_overrides() {
        let json = r#"{
            "output": "results.json",
            "strict": true,
            "concurrency": 8,
            "batch_size": 16
        }"#;
        let config: LoomConfig = serde_json::from_str(json).unwrap();

        assert_eq!(config.concurrency, 8);
        assert_eq!(config.batch_size, 16);
        assert!(config.strict);
        assert_eq!(config.output, Some(PathBuf::from("results.json")));
    }

    #[test]
    fn config_deserializes_layers() {
        let json = r#"{
            "layers": {
                "score": {
                    "threshold": 0.8,
                    "top_k": 3,
                    "categories": {
                        "test": {
                            "labels": {
                                "label1": {"hypothesis": "Test"}
                            }
                        }
                    }
                }
            }
        }"#;
        let config: LoomConfig = serde_json::from_str(json).unwrap();

        assert_eq!(config.layers.score.threshold, 0.8);
        assert_eq!(config.layers.score.top_k, 3);
        assert_eq!(config.layers.score.categories.len(), 1);
    }
}
