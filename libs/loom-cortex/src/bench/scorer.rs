use async_trait::async_trait;

use super::Decision;

/// A scorer that can evaluate text and produce a scoring output.
pub trait Scorer {
    type Output: ScorerOutput;
    type Error;

    /// Score the given text.
    fn score(&self, text: &str) -> Result<Self::Output, Self::Error>;
}

/// An async scorer that can evaluate text asynchronously.
/// Use this for parallel processing of samples.
#[async_trait]
pub trait AsyncScorer: Send + Sync {
    type Output: ScorerOutput + Send;
    type Error: Send;

    /// Score the given text asynchronously.
    async fn score_async(&self, text: &str) -> Result<Self::Output, Self::Error>;
}

/// A scorer that can evaluate multiple texts in a single batch.
/// This is more efficient than scoring texts one at a time.
///
/// Note: Unlike `AsyncScorer`, this trait does not require `Send + Sync` because
/// rust-bert models contain `tch::Tensor` with raw pointers that aren't thread-safe.
/// Use with `Arc<Mutex<S>>` for thread-safe access in async contexts.
pub trait BatchScorer {
    type Output: ScorerOutput;
    type Error;

    /// Score multiple texts in a single batch.
    fn score_batch(&self, texts: &[&str]) -> Result<Vec<Self::Output>, Self::Error>;

    /// Recommended batch size for optimal throughput.
    fn batch_size(&self) -> usize {
        8 // sensible default
    }
}

/// Output from a scorer containing decision, score, and label information.
pub trait ScorerOutput {
    /// The decision (Accept/Reject) for this scoring.
    fn decision(&self) -> Decision;

    /// The overall score value.
    fn score(&self) -> f32;

    /// Labels with their raw (uncalibrated) scores.
    /// Returns tuples of (label_name, raw_score).
    fn labels(&self) -> Vec<(String, f32)>;

    /// Labels that were detected (score > 0).
    fn detected_labels(&self) -> Vec<String> {
        self.labels()
            .into_iter()
            .filter(|(_, score)| *score > 0.0)
            .map(|(name, _)| name)
            .collect()
    }
}
