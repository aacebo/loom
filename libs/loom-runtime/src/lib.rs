mod config;
mod context;

pub use config::*;
pub use context::*;

use std::sync::Arc;

use loom_codec::CodecRegistryBuilder;
use loom_core::{Format, MediaType, decode, encode, value::Value};
use loom_error::Result;
use loom_io::{DataSourceRegistry, DataSourceRegistryBuilder, path::Path};
use loom_pipe::{Layer, Pipeline};
use serde::{Serialize, de::DeserializeOwned};

// Re-export config types
pub use loom_config::{Config as RConfig, ConfigError};

// Re-export codec types
#[cfg(feature = "toml")]
pub use loom_codec::TomlCodec;
#[cfg(feature = "yaml")]
pub use loom_codec::YamlCodec;
pub use loom_codec::{JsonCodec, TextCodec};

// Re-export IO types
pub use loom_io::Record;
pub use loom_io::sources::FileSystemSource;

// Re-export signal types
pub use loom_signal::{
    Emitter, Level, NoopEmitter, Signal, SignalBroadcaster, Span, Type as SignalType,
    consumers::{FileEmitter, MemoryEmitter, StdoutEmitter},
};

pub struct Runtime {
    sources: Arc<DataSourceRegistry>,
    pipeline: Pipeline<RunContext>,
    signals: Arc<dyn Emitter + Send + Sync>,
}

impl Runtime {
    pub fn new() -> Builder {
        Builder::new()
    }

    /// Execute the pipeline on a given input value.
    ///
    /// Creates a `RunContext` with the runtime's emitter and data sources,
    /// then threads the value through each layer.
    pub fn execute(&self, input: impl Into<Value>) -> Result<Value> {
        let mut ctx = RunContext::new(input, self.signals.clone(), self.sources.clone());

        for layer in self.pipeline.layers() {
            let output = layer.process(&ctx)?;
            ctx = ctx.next(output);
        }

        Ok(ctx.input().clone())
    }

    /// Load and deserialize data from a DataSource.
    pub async fn load<T: DeserializeOwned>(&self, source: &str, path: &Path) -> Result<T> {
        let source = self.sources.get(source).ok_or_else(|| {
            loom_error::Error::builder()
                .code(loom_error::ErrorCode::NotFound)
                .message(format!("DataSource '{}' not found", source))
                .build()
        })?;

        let record = source.find_one(path).await.map_err(|e| {
            loom_error::Error::builder()
                .code(loom_error::ErrorCode::Unknown)
                .message(format!("Failed to load from path '{}': {}", path, e))
                .build()
        })?;

        let content = record.content_str().map_err(|e| {
            loom_error::Error::builder()
                .code(loom_error::ErrorCode::Unknown)
                .message(format!("Invalid UTF-8 content: {}", e))
                .build()
        })?;

        decode!(content, record.media_type.format()).map_err(|e| {
            loom_error::Error::builder()
                .code(loom_error::ErrorCode::Unknown)
                .message(format!("Deserialization failed: {}", e))
                .build()
        })
    }

    /// Save and serialize data to a DataSource.
    pub async fn save<T: Serialize>(
        &self,
        source: &str,
        path: &Path,
        data: &T,
        format: Format,
    ) -> Result<()> {
        let source = self.sources.get(source).ok_or_else(|| {
            loom_error::Error::builder()
                .code(loom_error::ErrorCode::NotFound)
                .message(format!("DataSource '{}' not found", source))
                .build()
        })?;

        let content = encode!(data, format).map_err(|e| {
            loom_error::Error::builder()
                .code(loom_error::ErrorCode::Unknown)
                .message(format!("Serialization failed: {}", e))
                .build()
        })?;

        let media_type = match format {
            Format::Json => MediaType::TextJson,
            Format::Yaml => MediaType::TextYaml,
            Format::Toml => MediaType::TextToml,
            _ => MediaType::TextPlain,
        };

        let record = loom_io::Record::from_str(path.clone(), media_type, &content);

        source.upsert(record).await.map_err(|e| {
            loom_error::Error::builder()
                .code(loom_error::ErrorCode::Unknown)
                .message(format!("Failed to save to path '{}': {}", path, e))
                .build()
        })?;

        Ok(())
    }
}

pub struct Builder {
    codecs: CodecRegistryBuilder,
    sources: DataSourceRegistryBuilder,
    signals: SignalBroadcaster,
    layers: Vec<Box<dyn Layer<Input = RunContext>>>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            codecs: CodecRegistryBuilder::default(),
            sources: DataSourceRegistryBuilder::default(),
            signals: SignalBroadcaster::default(),
            layers: Vec::new(),
        }
    }
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn codec<T: loom_codec::Codec + 'static>(mut self, codec: T) -> Self {
        self.codecs = self.codecs.codec(codec);
        self
    }

    pub fn source<T: loom_io::DataSource + 'static>(mut self, source: T) -> Self {
        self.sources = self.sources.source(source);
        self
    }

    /// Add a processing layer to the runtime pipeline.
    pub fn layer<L: Layer<Input = RunContext> + 'static>(mut self, layer: L) -> Self {
        self.layers.push(Box::new(layer));
        self
    }

    /// Add a signal emitter to the runtime.
    pub fn emitter<E: Emitter + Send + Sync + 'static>(mut self, emitter: E) -> Self {
        self.signals = self.signals.add(emitter);
        self
    }

    pub fn build(self) -> Runtime {
        let signals: Arc<dyn Emitter + Send + Sync> = if self.signals.is_empty() {
            Arc::new(NoopEmitter)
        } else {
            Arc::new(self.signals)
        };

        let pipeline = Pipeline::new(self.layers);
        let sources = Arc::new(self.sources.build());

        Runtime {
            sources,
            pipeline,
            signals,
        }
    }
}
