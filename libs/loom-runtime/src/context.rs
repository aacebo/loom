use loom_core::Map;
use loom_pipe::LayerContext;

#[derive(Debug, Default)]
pub struct Context<Input> {
    pub meta: Map,
    pub step: usize,
    pub text: String,
    pub input: Input,
}

impl<Input> Context<Input> {
    pub fn new(text: &str, input: Input) -> Self {
        Self {
            meta: Map::default(),
            step: 0,
            text: text.to_string(),
            input,
        }
    }
}

impl<Input: Send + 'static> LayerContext for Context<Input> {
    fn text(&self) -> &str {
        &self.text
    }

    fn step(&self) -> usize {
        self.step
    }

    fn meta(&self) -> &Map {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut Map {
        &mut self.meta
    }
}
