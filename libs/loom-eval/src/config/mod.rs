mod category;
mod label;
mod modifier;

pub use category::*;
pub use label::*;
pub use modifier::*;

use std::collections::BTreeMap;

use loom_cortex::config::{CortexModelConfig, CortexZeroShotConfig};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

/// Root configuration for the eval engine.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EvalConfig {
    /// Model configuration for zero-shot classification
    #[serde(default)]
    pub model: CortexModelConfig,

    /// Baseline threshold for overall score acceptance
    #[serde(default = "EvalConfig::threshold")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub threshold: f32,

    /// Number of top labels to consider per category (default)
    #[serde(default = "EvalConfig::top_k")]
    #[validate(minimum = 1)]
    pub top_k: usize,

    /// Dynamic threshold adjustments based on text length
    #[serde(default)]
    #[validate]
    pub modifiers: ModifierConfig,

    /// Category definitions with their labels (keyed by category name)
    pub categories: BTreeMap<String, CategoryConfig>,
}

impl EvalConfig {
    fn threshold() -> f32 {
        0.75
    }

    fn top_k() -> usize {
        2
    }

    /// Compute effective threshold based on text length.
    pub fn threshold_of(&self, text_len: usize) -> f32 {
        match text_len {
            len if len <= self.modifiers.short_text_limit => {
                self.threshold - self.modifiers.short_text_delta
            }
            len if len > self.modifiers.long_text_limit => {
                self.threshold + self.modifiers.long_text_delta
            }
            _ => self.threshold,
        }
    }

    /// Get a category by name.
    pub fn category(&self, name: &str) -> Option<&CategoryConfig> {
        self.categories.get(name)
    }

    /// Get a label by name across all categories.
    pub fn label(&self, name: &str) -> Option<&LabelConfig> {
        self.categories
            .values()
            .flat_map(|c| c.labels.get(name))
            .next()
    }

    /// Get all labels across all categories (returns pairs of name and config).
    pub fn labels(&self) -> Vec<(String, LabelConfig)> {
        self.categories
            .values()
            .flat_map(|c| c.labels.iter().map(|(n, l)| (n.clone(), l.clone())))
            .collect()
    }

    /// Get hypothesis for a label by name.
    pub fn hypothesis(&self, label_name: &str) -> String {
        self.label(label_name)
            .map(|l| l.hypothesis.clone())
            .unwrap_or_else(|| format!("This example is {}.", label_name))
    }

    /// Validate the full config (including nested BTreeMap items).
    pub fn validate_full(&self) -> loom_error::Result<()> {
        self.validate()
            .map_err(|e| loom_error::Error::builder().message(&e.to_string()).build())?;

        for (cat_name, cat_config) in &self.categories {
            cat_config.validate().map_err(|e| {
                loom_error::Error::builder()
                    .message(&format!("Category '{}': {}", cat_name, e))
                    .build()
            })?;

            for (label_name, label_config) in &cat_config.labels {
                label_config.validate().map_err(|e| {
                    loom_error::Error::builder()
                        .message(&format!(
                            "Category '{}', Label '{}': {}",
                            cat_name, label_name, e
                        ))
                        .build()
                })?;
            }
        }

        if self.modifiers.short_text_limit >= self.modifiers.long_text_limit {
            return Err(loom_error::Error::builder()
                .message("short_text_limit must be less than long_text_limit")
                .build());
        }

        Ok(())
    }
}

impl Default for EvalConfig {
    fn default() -> Self {
        Self {
            model: CortexModelConfig::ZeroShotClassification(CortexZeroShotConfig::default()),
            threshold: Self::threshold(),
            top_k: Self::top_k(),
            modifiers: ModifierConfig::default(),
            categories: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> EvalConfig {
        let mut labels = BTreeMap::new();
        labels.insert(
            "label1".to_string(),
            LabelConfig {
                hypothesis: "Test hypothesis 1".to_string(),
                weight: 0.50,
                threshold: 0.70,
                platt_a: 1.0,
                platt_b: 0.0,
            },
        );
        labels.insert(
            "label2".to_string(),
            LabelConfig {
                hypothesis: "Test hypothesis 2".to_string(),
                weight: 0.80,
                threshold: 0.65,
                platt_a: 1.0,
                platt_b: 0.0,
            },
        );

        let mut categories = BTreeMap::new();
        categories.insert("test".to_string(), CategoryConfig { top_k: 2, labels });

        EvalConfig {
            model: CortexModelConfig::default(),
            threshold: 0.75,
            top_k: 2,
            modifiers: ModifierConfig::default(),
            categories,
        }
    }

    #[test]
    fn default_config_has_empty_categories() {
        let config = EvalConfig::default();
        assert!(config.categories.is_empty());
        assert_eq!(config.threshold, 0.75);
        assert_eq!(config.top_k, 2);
    }

    #[test]
    fn threshold_of_short_text() {
        let config = test_config();
        let threshold = config.threshold_of(10);
        assert!((threshold - 0.70).abs() < f32::EPSILON);
    }

    #[test]
    fn threshold_of_medium_text() {
        let config = test_config();
        let threshold = config.threshold_of(100);
        assert!((threshold - 0.75).abs() < f32::EPSILON);
    }

    #[test]
    fn threshold_of_long_text() {
        let config = test_config();
        let threshold = config.threshold_of(250);
        assert!((threshold - 0.80).abs() < f32::EPSILON);
    }

    #[test]
    fn label_lookup_works() {
        let config = test_config();
        let label = config.label("label2");
        assert!(label.is_some());
        assert_eq!(label.unwrap().weight, 0.80);
    }

    #[test]
    fn category_lookup_works() {
        let config = test_config();
        let category = config.category("test");
        assert!(category.is_some());
        assert_eq!(category.unwrap().labels.len(), 2);
    }

    #[test]
    fn invalid_threshold_fails_validation() {
        let mut config = test_config();
        config.threshold = 1.5;
        assert!(config.validate().is_err());
    }

    #[test]
    fn invalid_weight_fails_validation() {
        let mut label = LabelConfig::default();
        label.hypothesis = "Test".to_string();
        label.weight = -0.5;
        assert!(label.validate().is_err());
    }

    #[test]
    fn label_config_uses_defaults() {
        let json = r#"{"hypothesis": "Test hypothesis"}"#;
        let label: LabelConfig = serde_json::from_str(json).unwrap();

        assert_eq!(label.hypothesis, "Test hypothesis");
        assert_eq!(label.weight, 0.50);
        assert_eq!(label.threshold, 0.70);
        assert_eq!(label.platt_a, 1.0);
        assert_eq!(label.platt_b, 0.0);
    }

    #[test]
    fn eval_config_uses_defaults() {
        let json = r#"{
            "categories": {
                "test": {
                    "labels": {
                        "label1": {"hypothesis": "Test"}
                    }
                }
            }
        }"#;
        let config: EvalConfig = serde_json::from_str(json).unwrap();

        assert_eq!(config.threshold, 0.75);
        assert_eq!(config.top_k, 2);
        assert_eq!(config.modifiers.short_text_delta, 0.05);
        assert_eq!(config.modifiers.long_text_delta, 0.05);
        assert!(config.model.is_conversation());
        assert!(config.validate().is_ok());
    }
}
