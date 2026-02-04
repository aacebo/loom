use std::collections::HashMap;

use super::DataSource;

pub struct DataSourceRegistry {
    sources: HashMap<String, Box<dyn DataSource>>,
}

#[derive(Default)]
pub struct DataSourceRegistryBuilder {
    sources: HashMap<String, Box<dyn DataSource>>,
}

impl DataSourceRegistryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn source<T: DataSource + 'static>(mut self, source: T) -> Self {
        self.sources
            .insert(source.name().to_string(), Box::new(source));
        self
    }

    pub fn build(self) -> DataSourceRegistry {
        DataSourceRegistry {
            sources: self.sources,
        }
    }
}

impl DataSourceRegistry {
    pub fn builder() -> DataSourceRegistryBuilder {
        DataSourceRegistryBuilder::new()
    }

    pub fn get(&self, name: &str) -> Option<&dyn DataSource> {
        self.sources.get(name).map(|s| s.as_ref())
    }

    pub fn names(&self) -> impl Iterator<Item = &str> + '_ {
        self.sources.keys().map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.sources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }
}
