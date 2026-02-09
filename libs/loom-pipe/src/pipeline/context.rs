use std::any::Any;

use loom_core::{Map, value::Value};

/// Context passed between pipeline layers.
///
/// Provides access to the current input value, metadata, data sources,
/// and signal emission. Concrete implementations (e.g. `RunContext` in
/// loom-runtime) add runtime-specific services.
pub trait LayerContext: Send + Sync {
    /// The current input value for this layer.
    fn input(&self) -> &Value;

    /// Arbitrary metadata carried through the pipeline.
    fn meta(&self) -> &Map;

    /// Look up a named data source. Returns `None` by default.
    fn data_source(&self, _name: &str) -> Option<&dyn Any> {
        None
    }

    /// Emit a named signal with attributes. No-op by default.
    fn emit(&self, _name: &str, _attrs: &Map) {}
}
