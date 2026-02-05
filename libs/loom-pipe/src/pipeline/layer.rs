use loom_error::Result;

use super::{LayerContext, LayerResult};

/// A processing layer in a pipeline.
/// Layers transform input context into output wrapped in LayerResult.
pub trait Layer: Send {
    type Input: LayerContext;
    type Output: Send + 'static;

    /// Process input and produce output.
    fn process(&self, input: Self::Input) -> Result<LayerResult<Self::Output>>;

    /// Optional: name for debugging/tracing
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}
