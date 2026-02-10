use std::sync::Arc;

use loom_core::{Map, value::Value};
use loom_io::DataSourceRegistry;
use loom_signal::{Emitter, Signal};

/// Runtime execution context providing emitter and data source access to layers.
pub struct RunContext {
    input: Value,
    meta: Map,
    emitter: Arc<dyn Emitter + Send + Sync>,
    sources: Arc<DataSourceRegistry>,
}

impl RunContext {
    pub fn new(
        input: impl Into<Value>,
        emitter: Arc<dyn Emitter + Send + Sync>,
        sources: Arc<DataSourceRegistry>,
    ) -> Self {
        Self {
            input: input.into(),
            meta: Map::new(),
            emitter,
            sources,
        }
    }

    /// Create a new context for the next layer with updated input.
    pub fn next(&self, input: Value) -> Self {
        Self {
            input,
            meta: self.meta.clone(),
            emitter: self.emitter.clone(),
            sources: self.sources.clone(),
        }
    }

    pub fn sources(&self) -> &DataSourceRegistry {
        &self.sources
    }
}

impl RunContext {
    pub fn input(&self) -> &Value {
        &self.input
    }

    pub fn meta(&self) -> &Map {
        &self.meta
    }

    pub fn emit(&self, name: &str, attrs: &Map) {
        let mut builder = Signal::new().name(name);
        for (k, v) in attrs.iter() {
            builder = builder.attr(k.clone(), v.clone());
        }
        self.emitter.emit(builder.build());
    }
}
