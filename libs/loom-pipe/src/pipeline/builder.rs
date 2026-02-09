use super::{Layer, LayerContext, Pipeline};

/// Builder for constructing pipelines.
pub struct PipelineBuilder<C: LayerContext> {
    layers: Vec<Box<dyn Layer<Input = C>>>,
}

impl<C: LayerContext> PipelineBuilder<C> {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// Add a layer to the pipeline.
    pub fn then<L: Layer<Input = C> + 'static>(mut self, layer: L) -> Self {
        self.layers.push(Box::new(layer));
        self
    }

    /// Build the final pipeline.
    pub fn build(self) -> Pipeline<C> {
        Pipeline::new(self.layers)
    }
}

impl<C: LayerContext> Default for PipelineBuilder<C> {
    fn default() -> Self {
        Self::new()
    }
}
