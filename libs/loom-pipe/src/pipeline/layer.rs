use loom_core::value::Value;
use loom_error::Result;

use super::LayerContext;

/// A processing layer in a pipeline.
///
/// Each layer specifies its context type via the `Input` associated type
/// and returns a `Value` result from `process()`.
pub trait Layer: Send + Sync {
    type Input: LayerContext;

    fn process(&self, ctx: &Self::Input) -> Result<Value>;

    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}
