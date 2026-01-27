use crate::Map;

#[derive(Debug, Default)]
pub struct Context {
    meta: Map,
    text: String,
}

impl Context {
    pub fn new(text: &str) -> Self {
        Self {
            meta: Map::default(),
            text: text.to_string(),
        }
    }

    pub fn meta(&self) -> &Map {
        &self.meta
    }

    pub fn meta_mut(&mut self) -> &mut Map {
        &mut self.meta
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
