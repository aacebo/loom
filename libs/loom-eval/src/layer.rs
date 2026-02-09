use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use loom_config::Config;
use loom_core::{Map, ident_path, value::Value};
use loom_cortex::CortexModel;
use loom_error::{Error, ErrorCode};
use loom_pipe::LayerContext;
use loom_runtime::RunContext;

use crate::config::EvalConfig;
use crate::output::{CategoryOutput, EvalOutput, LabelOutput};

pub struct EvalLayer {
    model: Arc<Mutex<CortexModel>>,
    config: EvalConfig,
}

impl EvalLayer {
    /// Build an EvalLayer from a raw `loom_config::Config`.
    ///
    /// Reads the `layers.eval` section, deserializes it to `EvalConfig`,
    /// validates, builds the model, and wraps it in `Arc<Mutex<>>`.
    pub fn from_config(config: &Config) -> loom_error::Result<Self> {
        let eval_path = ident_path!("layers.eval");
        let eval_section = config.get_section(&eval_path);
        let eval_config: EvalConfig = eval_section.bind().map_err(|e| {
            Error::builder()
                .code(ErrorCode::BadArguments)
                .message(&format!("Failed to bind EvalConfig: {}", e))
                .build()
        })?;

        eval_config.validate_full()?;

        let model = eval_config.model.clone().build()?;
        Ok(Self {
            model: Arc::new(Mutex::new(model)),
            config: eval_config,
        })
    }

    /// Get the configuration for this layer.
    pub fn config(&self) -> &EvalConfig {
        &self.config
    }

    /// Get all valid category names from the config.
    pub fn valid_categories(&self) -> Vec<String> {
        self.config.categories.keys().cloned().collect()
    }

    /// Get all valid label names from the config.
    pub fn valid_labels(&self) -> Vec<String> {
        self.config
            .categories
            .values()
            .flat_map(|c| c.labels.keys().cloned())
            .collect()
    }

    /// Score a single text and return the eval output.
    pub fn score(&self, text: &str) -> loom_error::Result<EvalOutput> {
        let model = self.model.lock().expect("model lock poisoned");

        // Extract the zero-shot model
        let zs_model = match &*model {
            CortexModel::ZeroShotClassification { model, .. } => model,
            _ => {
                return Err(Error::builder()
                    .code(ErrorCode::BadArguments)
                    .message("EvalLayer requires a ZeroShotClassification model")
                    .build());
            }
        };

        // Get all label names from config
        let label_names: Vec<&str> = self
            .config
            .categories
            .values()
            .flat_map(|c| c.labels.keys().map(|s| s.as_str()))
            .collect();

        // Build a static hypothesis map for the closure
        let hypothesis_map: HashMap<String, String> = self
            .config
            .categories
            .values()
            .flat_map(|c| {
                c.labels
                    .iter()
                    .map(|(name, l)| (name.clone(), l.hypothesis.clone()))
            })
            .collect();

        // Create hypothesis function using the cloned map
        let hypothesis_fn = Box::new(move |label: &str| {
            hypothesis_map
                .get(label)
                .cloned()
                .unwrap_or_else(|| format!("This example is {}.", label))
        });

        // Run zero-shot classification
        let predictions =
            zs_model.predict_multilabel(&[text], &label_names, Some(hypothesis_fn), 128)?;

        // Build a lookup map for predictions by label name
        let mut prediction_map: HashMap<&str, f32> = HashMap::new();

        for sentence_predictions in &predictions {
            for pred in sentence_predictions {
                prediction_map.insert(
                    label_names
                        .iter()
                        .find(|&&n| n == pred.text)
                        .copied()
                        .unwrap_or(&pred.text),
                    pred.score as f32,
                );
            }
        }

        // Build CategoryOutput for each category in config
        let mut categories = BTreeMap::new();

        for (cat_name, cat_config) in &self.config.categories {
            let mut labels = BTreeMap::new();

            for (label_name, label_config) in &cat_config.labels {
                let raw_score = prediction_map
                    .get(label_name.as_str())
                    .copied()
                    .unwrap_or(0.0);

                let label_output = LabelOutput::new(raw_score, 0, label_config);
                labels.insert(label_name.clone(), label_output);
            }

            let top_k = cat_config.top_k;
            categories.insert(cat_name.clone(), CategoryOutput::topk(labels, top_k));
        }

        Ok(EvalOutput::new(categories))
    }
}

impl loom_pipe::Layer for EvalLayer {
    type Input = RunContext;

    fn process(&self, ctx: &RunContext) -> loom_error::Result<Value> {
        let text = ctx.input().as_str().unwrap_or_default();
        let eval_output = self.score(text)?;

        let mut attrs = Map::new();
        attrs.set("score", Value::from(eval_output.score as f64));
        ctx.emit("eval.scored", &attrs);

        Ok(eval_output.into())
    }

    fn name(&self) -> &'static str {
        "eval"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === EvalConfig Threshold Tests ===

    #[test]
    fn threshold_short_text_lowers_threshold() {
        let config = EvalConfig::default();
        let result = config.threshold_of(10);
        assert!(
            (result - 0.70).abs() < f32::EPSILON,
            "Expected 0.70, got {}",
            result
        );
    }

    #[test]
    fn threshold_medium_text_unchanged() {
        let config = EvalConfig::default();
        let result = config.threshold_of(100);
        assert!(
            (result - 0.75).abs() < f32::EPSILON,
            "Expected 0.75, got {}",
            result
        );
    }

    #[test]
    fn threshold_long_text_raises_threshold() {
        let config = EvalConfig::default();
        let result = config.threshold_of(250);
        assert!(
            (result - 0.80).abs() < f32::EPSILON,
            "Expected 0.80, got {}",
            result
        );
    }

    #[test]
    fn threshold_boundary_20_chars() {
        let config = EvalConfig::default();
        let result = config.threshold_of(20);
        assert!(
            (result - 0.70).abs() < f32::EPSILON,
            "20 chars should be short, expected 0.70, got {}",
            result
        );
    }

    #[test]
    fn threshold_boundary_21_chars() {
        let config = EvalConfig::default();
        let result = config.threshold_of(21);
        assert!(
            (result - 0.75).abs() < f32::EPSILON,
            "21 chars should be medium, expected 0.75, got {}",
            result
        );
    }

    #[test]
    fn threshold_boundary_200_chars() {
        let config = EvalConfig::default();
        let result = config.threshold_of(200);
        assert!(
            (result - 0.75).abs() < f32::EPSILON,
            "200 chars should be medium, expected 0.75, got {}",
            result
        );
    }

    #[test]
    fn threshold_boundary_201_chars() {
        let config = EvalConfig::default();
        let result = config.threshold_of(201);
        assert!(
            (result - 0.80).abs() < f32::EPSILON,
            "201 chars should be long, expected 0.80, got {}",
            result
        );
    }

    #[test]
    fn threshold_empty_text() {
        let config = EvalConfig::default();
        let result = config.threshold_of(0);
        assert!(
            (result - 0.70).abs() < f32::EPSILON,
            "Empty text should be short, expected 0.70, got {}",
            result
        );
    }
}
