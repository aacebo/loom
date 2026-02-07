//! Evaluation builder for dataset processing.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use loom_signal::{Level, Signal, Type};

use super::{
    CategoryResult, EvalConfig, EvalResult, Evaluable, LabelResult, Progress, Sample,
    SampleDataset, SampleResult,
};
use crate::Runtime;

/// Builder for evaluating datasets using an Evaluable layer.
///
/// # Example
///
/// ```ignore
/// let scorer = Arc::new(Mutex::new(score_config.build()?));
/// let result = runtime
///     .eval(scorer)
///     .batch_size(16)
///     .on_progress(|p| println!("{}/{}", p.current, p.total))
///     .run(&dataset)
///     .await;
/// ```
pub struct EvalBuilder<'a, E: Evaluable> {
    runtime: &'a Runtime,
    evaluable: Arc<Mutex<E>>,
    config: EvalConfig,
    progress_callback: Option<Box<dyn Fn(Progress) + Send + Sync>>,
}

impl<'a, E: Evaluable + 'static> EvalBuilder<'a, E> {
    /// Create a new evaluation builder.
    pub fn new(runtime: &'a Runtime, evaluable: Arc<Mutex<E>>) -> Self {
        Self {
            runtime,
            evaluable,
            config: EvalConfig::default(),
            progress_callback: None,
        }
    }

    /// Set the batch size for inference.
    pub fn batch_size(mut self, size: usize) -> Self {
        self.config.batch_size = size;
        self
    }

    /// Set the concurrency level (reserved for future use).
    pub fn concurrency(mut self, n: usize) -> Self {
        self.config.concurrency = n;
        self
    }

    /// Set a progress callback.
    pub fn on_progress<F>(mut self, callback: F) -> Self
    where
        F: Fn(Progress) + Send + Sync + 'static,
    {
        self.progress_callback = Some(Box::new(callback));
        self
    }

    /// Execute the evaluation against a dataset.
    pub async fn run(self, dataset: &SampleDataset) -> EvalResult {
        let eval_start = std::time::Instant::now();
        let total = dataset.samples.len();

        // Emit start signal
        self.runtime.emit(
            Signal::new()
                .otype(Type::Event)
                .name("eval.start")
                .attr("total", total as i64)
                .build(),
        );

        let mut all_results: Vec<(Sample, SampleResult)> = Vec::with_capacity(total);
        let mut processed = 0;

        // Process in batches
        for chunk in dataset.samples.chunks(self.config.batch_size) {
            let batch_samples: Vec<Sample> = chunk.to_vec();
            let evaluable = self.evaluable.clone();

            // Execute batch in spawn_blocking for CPU-bound work
            let outputs = tokio::task::spawn_blocking(move || {
                let evaluable = evaluable.lock().expect("evaluable lock poisoned");
                let refs: Vec<&Sample> = batch_samples.iter().collect();
                evaluable.eval_batch(&refs)
            })
            .await
            .expect("spawn_blocking failed");

            match outputs {
                Ok(outputs) => {
                    for (sample, output) in chunk.iter().zip(outputs) {
                        let evaluable = self.evaluable.lock().expect("evaluable lock poisoned");
                        let sample_result = evaluable.to_result(sample, output);
                        drop(evaluable);

                        processed += 1;

                        // Emit progress signal
                        self.runtime.emit(
                            Signal::new()
                                .otype(Type::Event)
                                .name("eval.progress")
                                .attr("current", processed as i64)
                                .attr("total", total as i64)
                                .attr("sample_id", sample.id.clone())
                                .attr("correct", sample_result.correct)
                                .build(),
                        );

                        // Call user callback if provided
                        if let Some(ref cb) = self.progress_callback {
                            cb(Progress {
                                current: processed,
                                total,
                                sample_id: sample.id.clone(),
                                correct: sample_result.correct,
                            });
                        }

                        all_results.push((sample.clone(), sample_result));
                    }
                }
                Err(e) => {
                    // Emit error signal
                    self.runtime.emit(
                        Signal::new()
                            .otype(Type::Event)
                            .level(Level::Error)
                            .name("eval.batch_error")
                            .attr("error", e.to_string())
                            .build(),
                    );

                    // Mark all samples in batch as failed
                    for sample in chunk {
                        let sample_result = SampleResult {
                            id: sample.id.clone(),
                            expected_decision: sample.expected_decision,
                            actual_decision: super::Decision::Reject,
                            correct: sample.expected_decision == super::Decision::Reject,
                            score: 0.0,
                            expected_labels: sample.expected_labels.clone(),
                            detected_labels: vec![],
                            elapsed_ms: None,
                        };

                        processed += 1;

                        if let Some(ref cb) = self.progress_callback {
                            cb(Progress {
                                current: processed,
                                total,
                                sample_id: sample.id.clone(),
                                correct: sample_result.correct,
                            });
                        }

                        all_results.push((sample.clone(), sample_result));
                    }
                }
            }
        }

        // Calculate timing metrics
        let elapsed = eval_start.elapsed();
        let elapsed_ms = elapsed.as_millis() as i64;
        let throughput = if elapsed.as_secs_f32() > 0.0 {
            total as f32 / elapsed.as_secs_f32()
        } else {
            0.0
        };

        // Emit completion signal
        self.runtime.emit(
            Signal::new()
                .otype(Type::Event)
                .name("eval.complete")
                .attr("elapsed_ms", elapsed_ms)
                .attr("throughput", throughput as f64)
                .attr("total", total as i64)
                .attr(
                    "correct",
                    all_results.iter().filter(|(_, r)| r.correct).count() as i64,
                )
                .build(),
        );

        Self::build_result(all_results, elapsed_ms, throughput)
    }

    /// Build an EvalResult from sample results.
    fn build_result(
        samples_and_results: Vec<(Sample, SampleResult)>,
        elapsed_ms: i64,
        throughput: f32,
    ) -> EvalResult {
        let mut result = EvalResult::new();
        result.total = samples_and_results.len();
        result.elapsed_ms = elapsed_ms;
        result.throughput = throughput;

        for (sample, sample_result) in samples_and_results {
            if sample_result.correct {
                result.correct += 1;
            }

            // Update per-category results
            let cat_result = result
                .per_category
                .entry(sample.primary_category.clone())
                .or_insert_with(CategoryResult::default);
            cat_result.total += 1;
            if sample_result.correct {
                cat_result.correct += 1;
            }

            // Update per-label results
            Self::update_label_metrics(&mut result.per_label, &sample, &sample_result);

            result.sample_results.push(sample_result);
        }

        result
    }

    /// Update per-label metrics based on sample results.
    fn update_label_metrics(
        per_label: &mut HashMap<String, LabelResult>,
        sample: &Sample,
        sample_result: &SampleResult,
    ) {
        let expected_set: HashSet<_> = sample.expected_labels.iter().collect();
        let detected_set: HashSet<_> = sample_result.detected_labels.iter().collect();

        // Count expected labels
        for label in &sample.expected_labels {
            let entry = per_label.entry(label.clone()).or_default();
            entry.expected_count += 1;
        }

        // Count detected labels and compute TP/FP
        for label in &sample_result.detected_labels {
            let entry = per_label.entry(label.clone()).or_default();
            entry.detected_count += 1;

            if expected_set.contains(label) {
                entry.true_positives += 1;
            } else {
                entry.false_positives += 1;
            }
        }

        // Count false negatives (expected but not detected)
        for label in &sample.expected_labels {
            if !detected_set.contains(label) {
                let entry = per_label.entry(label.clone()).or_default();
                entry.false_negatives += 1;
            }
        }
    }
}
