//! Evaluation runner types and traits.

use loom_error::Result;

use super::{Sample, SampleResult};

/// Configuration for evaluation execution.
#[derive(Debug, Clone)]
pub struct EvalConfig {
    /// Number of samples to process in each batch.
    pub batch_size: usize,
    /// Number of parallel workers (reserved for future use).
    pub concurrency: usize,
}

impl Default for EvalConfig {
    fn default() -> Self {
        Self {
            batch_size: 8,
            concurrency: 1,
        }
    }
}

/// Trait for layers that can evaluate datasets.
///
/// This trait enables batch evaluation of samples with result aggregation.
/// It is implemented by `ScoreLayer` and can be implemented by other
/// evaluable layers.
///
/// Note: This trait only requires `Send`, not `Sync`. The EvalBuilder
/// handles synchronization via `Mutex` for types that aren't thread-safe
/// (e.g., types containing PyTorch tensors).
pub trait Evaluable: Send {
    /// The output type produced for each sample.
    type Output: Send;

    /// Evaluate a batch of samples.
    ///
    /// # Arguments
    /// * `samples` - Slice of sample references to evaluate
    ///
    /// # Returns
    /// A vector of outputs, one per sample in the same order.
    fn eval_batch(&self, samples: &[&Sample]) -> Result<Vec<Self::Output>>;

    /// Convert an output to a sample result for aggregation.
    ///
    /// # Arguments
    /// * `sample` - The original sample
    /// * `output` - The output from evaluation
    ///
    /// # Returns
    /// A `SampleResult` suitable for aggregation into `EvalResult`.
    fn to_result(&self, sample: &Sample, output: Self::Output) -> SampleResult;
}
