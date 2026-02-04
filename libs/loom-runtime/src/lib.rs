pub mod bench;
mod context;
mod layer;
mod options;
pub mod score;

pub use context::*;
pub use layer::*;
pub use options::*;

pub struct Runtime {
    #[allow(unused)]
    codecs: Vec<Box<dyn loom_codec::Codec>>,

    #[allow(unused)]
    sources: Vec<Box<dyn loom_io::DataSource>>,
}

#[derive(Default)]
pub struct Builder {
    codecs: Vec<Box<dyn loom_codec::Codec>>,
    sources: Vec<Box<dyn loom_io::DataSource>>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_codec<T: loom_codec::Codec + 'static>(mut self, codec: T) -> Self {
        self.codecs.push(Box::new(codec));
        self
    }

    pub fn with_source<T: loom_io::DataSource + 'static>(mut self, source: T) -> Self {
        self.sources.push(Box::new(source));
        self
    }

    pub fn build(self) -> Runtime {
        Runtime {
            codecs: self.codecs,
            sources: self.sources,
        }
    }
}
