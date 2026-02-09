use super::{Layer, LayerContext};

/// A collection of layers to be executed in sequence.
///
/// Execution and value threading are driven by the caller
/// (e.g. the Runtime), which creates a new context for each layer.
pub struct Pipeline<C: LayerContext> {
    layers: Vec<Box<dyn Layer<Input = C>>>,
}

impl<C: LayerContext> Pipeline<C> {
    pub fn new(layers: Vec<Box<dyn Layer<Input = C>>>) -> Self {
        Self { layers }
    }

    pub fn layers(&self) -> &[Box<dyn Layer<Input = C>>] {
        &self.layers
    }

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }
}
