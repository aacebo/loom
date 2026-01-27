use merc_error::Result;

use crate::Map;

pub trait Layer {
    type In;
    type Out: std::fmt::Debug;

    fn invoke(&self, input: &mut Self::In) -> Result<LayerResult<Self::Out>>;
}

#[derive(Debug, Default)]
pub struct LayerResult<Data: std::fmt::Debug> {
    pub meta: Map,
    pub data: Data,
}

impl<Data: std::fmt::Debug> LayerResult<Data> {
    pub fn new(data: Data) -> Self {
        Self {
            meta: Map::default(),
            data,
        }
    }
}
