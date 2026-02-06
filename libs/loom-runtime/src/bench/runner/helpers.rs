use std::collections::{HashMap, HashSet};

use loom_cortex::bench::{Scorer, ScorerOutput};

use crate::bench::{BenchResult, BenchSample, Decision, LabelResult, SampleResult};

/// Evaluate a single sample using the scorer.
pub fn evaluate_sample<S: Scorer>(sample: &BenchSample, scorer: &S) -> SampleResult {
    let (actual_decision, score, detected_labels) = match scorer.score(&sample.text) {
        Ok(output) => {
            let detected = output.detected_labels();
            (output.decision(), output.score(), detected)
        }
        Err(_) => (Decision::Reject, 0.0, vec![]),
    };

    SampleResult {
        id: sample.id.clone(),
        expected_decision: sample.expected_decision,
        actual_decision,
        correct: actual_decision == sample.expected_decision,
        score,
        expected_labels: sample.expected_labels.clone(),
        detected_labels,
    }
}

/// Evaluate a single sample and capture raw scores.
///
/// Returns both the SampleResult and a map of label -> raw_score.
pub fn evaluate_sample_with_scores<S: Scorer>(
    sample: &BenchSample,
    scorer: &S,
) -> (SampleResult, HashMap<String, f32>) {
    let (actual_decision, score, detected_labels, raw_scores) = match scorer.score(&sample.text) {
        Ok(output) => {
            let detected = output.detected_labels();
            let raw: HashMap<String, f32> = output.labels().into_iter().collect();
            (output.decision(), output.score(), detected, raw)
        }
        Err(_) => (Decision::Reject, 0.0, vec![], HashMap::new()),
    };

    let result = SampleResult {
        id: sample.id.clone(),
        expected_decision: sample.expected_decision,
        actual_decision,
        correct: actual_decision == sample.expected_decision,
        score,
        expected_labels: sample.expected_labels.clone(),
        detected_labels,
    };

    (result, raw_scores)
}

/// Evaluate a batch output for a sample.
pub fn evaluate_batch_output<O: ScorerOutput>(sample: &BenchSample, output: O) -> SampleResult {
    let detected_labels = output.detected_labels();
    let actual_decision = output.decision();
    let score = output.score();

    SampleResult {
        id: sample.id.clone(),
        expected_decision: sample.expected_decision,
        actual_decision,
        correct: actual_decision == sample.expected_decision,
        score,
        expected_labels: sample.expected_labels.clone(),
        detected_labels,
    }
}

/// Evaluate a batch output for a sample and capture raw scores.
///
/// Returns both the SampleResult and a map of label -> raw_score.
pub fn evaluate_batch_output_with_scores<O: ScorerOutput>(
    sample: &BenchSample,
    output: O,
) -> (SampleResult, HashMap<String, f32>) {
    let detected_labels = output.detected_labels();
    let actual_decision = output.decision();
    let score = output.score();
    let raw_scores: HashMap<String, f32> = output.labels().into_iter().collect();

    let result = SampleResult {
        id: sample.id.clone(),
        expected_decision: sample.expected_decision,
        actual_decision,
        correct: actual_decision == sample.expected_decision,
        score,
        expected_labels: sample.expected_labels.clone(),
        detected_labels,
    };

    (result, raw_scores)
}

/// Update per-label metrics based on sample results.
pub(crate) fn update_label_metrics(
    per_label: &mut HashMap<String, LabelResult>,
    sample: &BenchSample,
    sample_result: &SampleResult,
) {
    let expected_set: HashSet<_> = sample.expected_labels.iter().collect();
    let detected_set: HashSet<_> = sample_result.detected_labels.iter().collect();

    for label in &sample.expected_labels {
        let entry = per_label.entry(label.clone()).or_default();
        entry.expected_count += 1;
    }

    for label in &sample_result.detected_labels {
        let entry = per_label.entry(label.clone()).or_default();
        entry.detected_count += 1;

        if expected_set.contains(label) {
            entry.true_positives += 1;
        } else {
            entry.false_positives += 1;
        }
    }

    for label in &sample.expected_labels {
        if !detected_set.contains(label) {
            let entry = per_label.entry(label.clone()).or_default();
            entry.false_negatives += 1;
        }
    }
}

/// Build a BenchResult from sample results.
///
/// This consolidates the repeated result-building logic from all runner variants.
pub(crate) fn build_result(samples_and_results: Vec<(BenchSample, SampleResult)>) -> BenchResult {
    let mut result = BenchResult::new();
    result.total = samples_and_results.len();

    for (sample, sample_result) in samples_and_results {
        if sample_result.correct {
            result.correct += 1;
        }

        let cat_result = result
            .per_category
            .entry(sample.primary_category.clone())
            .or_default();
        cat_result.total += 1;
        if sample_result.correct {
            cat_result.correct += 1;
        }

        update_label_metrics(&mut result.per_label, &sample, &sample_result);
        result.sample_results.push(sample_result);
    }

    result
}

/// Build a BenchResult from sample results with raw scores.
///
/// Returns both the BenchResult and a map of sample_id -> label -> raw_score.
pub(crate) fn build_result_with_scores(
    samples_and_results: Vec<(BenchSample, SampleResult, HashMap<String, f32>)>,
) -> (BenchResult, HashMap<String, HashMap<String, f32>>) {
    let mut result = BenchResult::new();
    let mut raw_scores_map: HashMap<String, HashMap<String, f32>> = HashMap::new();
    result.total = samples_and_results.len();

    for (sample, sample_result, raw_scores) in samples_and_results {
        if sample_result.correct {
            result.correct += 1;
        }

        let cat_result = result
            .per_category
            .entry(sample.primary_category.clone())
            .or_default();
        cat_result.total += 1;
        if sample_result.correct {
            cat_result.correct += 1;
        }

        update_label_metrics(&mut result.per_label, &sample, &sample_result);
        raw_scores_map.insert(sample_result.id.clone(), raw_scores);
        result.sample_results.push(sample_result);
    }

    (result, raw_scores_map)
}
