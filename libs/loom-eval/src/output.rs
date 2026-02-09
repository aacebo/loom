use std::collections::BTreeMap;

use loom_core::value::Value;
use serde::{Deserialize, Serialize};

use super::config::LabelConfig;
use crate::result::{EvalResult, SampleResult};
use crate::{Decision, Sample};

/// Apply Platt scaling to calibrate raw model scores.
/// P(y|x) = 1 / (1 + exp(-Ax - B))
/// With identity params (a=1.0, b=0.0), returns raw score unchanged.
#[inline]
fn calibrate(raw: f32, a: f32, b: f32) -> f32 {
    // Identity: skip calibration
    if (a - 1.0).abs() < f32::EPSILON && b.abs() < f32::EPSILON {
        return raw;
    }
    1.0 / (1.0 + (-a * raw - b).exp())
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EvalOutput {
    /// Overall score (max of category scores)
    pub score: f32,
    /// Categories keyed by name (mirrors config structure)
    pub categories: BTreeMap<String, CategoryOutput>,
}

impl EvalOutput {
    pub fn new(categories: BTreeMap<String, CategoryOutput>) -> Self {
        let score = categories.values().map(|c| c.score).fold(0.0f32, f32::max);
        Self { score, categories }
    }

    pub fn category(&self, name: &str) -> Option<&CategoryOutput> {
        self.categories.get(name)
    }

    pub fn label(&self, name: &str) -> Option<&LabelOutput> {
        self.categories
            .values()
            .flat_map(|c| c.labels.get(name))
            .next()
    }

    pub fn label_score(&self, name: &str) -> f32 {
        self.label(name).map(|l| l.score).unwrap_or_default()
    }

    /// Returns labels whose calibrated score is above their threshold.
    pub fn detected_labels(&self) -> Vec<String> {
        self.categories
            .values()
            .flat_map(|c| c.labels.iter())
            .filter(|(_, l)| l.score > 0.0)
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Decide Accept/Reject based on the given threshold.
    pub fn decide(&self, threshold: f32) -> Decision {
        if self.score >= threshold {
            Decision::Accept
        } else {
            Decision::Reject
        }
    }

    /// Convert this output into an EvalResult for a single sample.
    pub fn to_result(self, sample: &Sample, threshold: f32) -> EvalResult {
        let detected_labels = self.detected_labels();
        let actual_decision = self.decide(threshold);
        let correct = actual_decision == sample.expected_decision;

        let sample_result = SampleResult {
            id: sample.id.clone(),
            expected_decision: sample.expected_decision,
            actual_decision,
            correct,
            score: self.score,
            expected_labels: sample.expected_labels.clone(),
            detected_labels: detected_labels.clone(),
            elapsed_ms: None,
        };

        let mut result = EvalResult::new();
        result.total = 1;
        result.accumulate(sample, &sample_result);
        result.sample_results.push(sample_result);
        result
    }

    /// Returns (label_name, raw_score) pairs for external use.
    pub fn raw_scores(&self) -> Vec<(String, f32)> {
        self.categories
            .iter()
            .flat_map(|(_, cat)| {
                cat.labels
                    .iter()
                    .map(|(name, label)| (name.clone(), label.raw_score))
            })
            .collect()
    }
}

#[cfg(feature = "json")]
impl From<EvalOutput> for Value {
    fn from(result: EvalOutput) -> Self {
        let json = serde_json::to_value(&result).expect("EvalOutput is serializable");
        Value::from(json)
    }
}

#[cfg(feature = "json")]
impl TryFrom<Value> for EvalOutput {
    type Error = loom_error::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let json: serde_json::Value = (&value).into();
        serde_json::from_value(json).map_err(|e| {
            loom_error::Error::builder()
                .code(loom_error::ErrorCode::BadArguments)
                .message(&format!("Failed to deserialize EvalOutput: {}", e))
                .build()
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryOutput {
    /// Category score (avg of top-k labels)
    pub score: f32,
    /// Labels keyed by name (mirrors config structure)
    pub labels: BTreeMap<String, LabelOutput>,
}

impl CategoryOutput {
    pub fn new(labels: BTreeMap<String, LabelOutput>) -> Self {
        let score = if labels.is_empty() {
            0.0
        } else {
            labels.values().map(|l| l.score).sum::<f32>() / labels.len() as f32
        };
        Self { score, labels }
    }

    pub fn topk(labels: BTreeMap<String, LabelOutput>, k: usize) -> Self {
        let take = k.min(labels.len()).max(1);

        // Sort by score for top-k calculation
        let mut sorted: Vec<_> = labels.values().collect();
        sorted.sort_by(|a, b| b.score.total_cmp(&a.score));

        let score = if sorted.is_empty() {
            0.0
        } else {
            sorted.iter().take(take).map(|l| l.score).sum::<f32>() / take as f32
        };

        Self { score, labels }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LabelOutput {
    /// Calibrated score (raw * weight, if above threshold)
    pub score: f32,
    /// Raw model output before calibration
    pub raw_score: f32,
    /// Sentence index (for multi-sentence inputs)
    pub sentence: usize,
}

impl LabelOutput {
    pub fn new(raw_score: f32, sentence: usize, config: &LabelConfig) -> Self {
        let calibrated = calibrate(raw_score, config.platt_a, config.platt_b);
        let score = if calibrated >= config.threshold {
            calibrated * config.weight
        } else {
            0.0
        };
        Self {
            score,
            raw_score,
            sentence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Platt Calibration Tests ===

    #[test]
    fn calibrate_identity_params_returns_raw() {
        let raw_scores = [0.0, 0.25, 0.5, 0.75, 1.0];
        for raw in raw_scores {
            let result = calibrate(raw, 1.0, 0.0);
            assert!(
                (result - raw).abs() < f32::EPSILON,
                "Identity calibration failed: expected {}, got {}",
                raw,
                result
            );
        }
    }

    #[test]
    fn calibrate_identity_near_epsilon() {
        let raw = 0.75;
        let near_one = 1.0 + f32::EPSILON * 0.5;
        let near_zero = f32::EPSILON * 0.5;
        let result = calibrate(raw, near_one, near_zero);
        assert!(
            (result - raw).abs() < 0.001,
            "Near-identity should return raw: expected {}, got {}",
            raw,
            result
        );
    }

    #[test]
    fn calibrate_sigmoid_midpoint() {
        let result = calibrate(0.5, 2.0, -1.0);
        assert!(
            (result - 0.5).abs() < 0.001,
            "Sigmoid midpoint should be 0.5, got {}",
            result
        );
    }

    #[test]
    fn calibrate_high_raw_produces_high_output() {
        let result = calibrate(0.95, 2.0, 0.0);
        assert!(
            result > 0.8,
            "High raw should produce high output, got {}",
            result
        );
    }

    #[test]
    fn calibrate_low_raw_produces_low_output() {
        let result = calibrate(0.1, 2.0, 0.0);
        assert!(
            result < 0.6,
            "Low raw should produce lower output, got {}",
            result
        );
    }

    #[test]
    fn calibrate_negative_b_shifts_down() {
        let raw = 0.7;
        let with_zero_b = calibrate(raw, 1.5, 0.0);
        let with_neg_b = calibrate(raw, 1.5, -0.5);
        assert!(
            with_neg_b < with_zero_b,
            "Negative B should shift down: {} should be < {}",
            with_neg_b,
            with_zero_b
        );
    }

    #[test]
    fn calibrate_positive_b_shifts_up() {
        let raw = 0.3;
        let with_zero_b = calibrate(raw, 1.5, 0.0);
        let with_pos_b = calibrate(raw, 1.5, 0.5);
        assert!(
            with_pos_b > with_zero_b,
            "Positive B should shift up: {} should be > {}",
            with_pos_b,
            with_zero_b
        );
    }

    #[test]
    fn calibrate_output_bounded_0_to_1() {
        let extreme_cases = [
            (0.0, 5.0, -10.0),
            (1.0, 5.0, 10.0),
            (0.5, 0.1, 0.0),
            (0.5, 10.0, 0.0),
        ];
        for (raw, a, b) in extreme_cases {
            let result = calibrate(raw, a, b);
            assert!(
                result >= 0.0 && result <= 1.0,
                "Calibrated score must be in [0,1], got {} for ({}, {}, {})",
                result,
                raw,
                a,
                b
            );
        }
    }

    #[test]
    fn calibrate_formula_verification() {
        let raw: f32 = 0.6;
        let a: f32 = 1.5;
        let b: f32 = -0.3;
        let expected: f32 = 1.0 / (1.0 + (-a * raw - b).exp());
        let result = calibrate(raw, a, b);
        assert!(
            (result - expected).abs() < f32::EPSILON,
            "Formula mismatch: expected {}, got {}",
            expected,
            result
        );
    }

    // === LabelOutput Tests ===

    #[test]
    fn label_output_applies_calibration() {
        let config = LabelConfig {
            hypothesis: "test".to_string(),
            weight: 0.30,
            threshold: 0.70,
            platt_a: 1.0,
            platt_b: 0.0,
        };
        let label_output = LabelOutput::new(0.8, 0, &config);
        let expected = 0.8 * config.weight;
        assert!(
            (label_output.score - expected).abs() < 0.001,
            "Expected {}, got {}",
            expected,
            label_output.score
        );
    }

    #[test]
    fn label_output_below_threshold_zeroes_score() {
        let config = LabelConfig {
            hypothesis: "test".to_string(),
            weight: 0.30,
            threshold: 0.70,
            platt_a: 1.0,
            platt_b: 0.0,
        };
        let label_output = LabelOutput::new(0.5, 0, &config);
        assert!(
            (label_output.score - 0.0).abs() < f32::EPSILON,
            "Score below threshold should be 0, got {}",
            label_output.score
        );
    }

    #[test]
    fn label_output_at_threshold_passes() {
        let config = LabelConfig {
            hypothesis: "test".to_string(),
            weight: 1.00,
            threshold: 0.65,
            platt_a: 1.0,
            platt_b: 0.0,
        };
        let label_output = LabelOutput::new(0.65, 0, &config);
        let expected = 0.65 * config.weight;
        assert!(
            (label_output.score - expected).abs() < 0.001,
            "Score at threshold should pass: expected {}, got {}",
            expected,
            label_output.score
        );
    }

    // === CategoryOutput Tests ===

    #[test]
    fn category_output_topk() {
        let config = LabelConfig {
            hypothesis: "test".to_string(),
            weight: 1.0,
            threshold: 0.0,
            platt_a: 1.0,
            platt_b: 0.0,
        };

        let mut labels = BTreeMap::new();
        labels.insert("a".to_string(), LabelOutput::new(0.9, 0, &config));
        labels.insert("b".to_string(), LabelOutput::new(0.7, 0, &config));
        labels.insert("c".to_string(), LabelOutput::new(0.5, 0, &config));

        let category = CategoryOutput::topk(labels, 2);
        assert!(
            (category.score - 0.8).abs() < 0.001,
            "Expected 0.8, got {}",
            category.score
        );
    }

    // === EvalOutput Tests ===

    #[test]
    fn eval_output_category_lookup() {
        let mut categories = BTreeMap::new();
        categories.insert(
            "sentiment".to_string(),
            CategoryOutput {
                score: 0.5,
                labels: BTreeMap::new(),
            },
        );

        let result = EvalOutput::new(categories);
        assert!(result.category("sentiment").is_some());
        assert!(result.category("nonexistent").is_none());
    }

    #[test]
    fn eval_output_label_lookup() {
        let config = LabelConfig {
            hypothesis: "test".to_string(),
            weight: 1.0,
            threshold: 0.0,
            platt_a: 1.0,
            platt_b: 0.0,
        };

        let mut labels = BTreeMap::new();
        labels.insert("positive".to_string(), LabelOutput::new(0.8, 0, &config));

        let mut categories = BTreeMap::new();
        categories.insert("sentiment".to_string(), CategoryOutput::new(labels));

        let result = EvalOutput::new(categories);
        assert!(result.label("positive").is_some());
        assert_eq!(result.label_score("positive"), 0.8);
        assert_eq!(result.label_score("nonexistent"), 0.0);
    }
}
