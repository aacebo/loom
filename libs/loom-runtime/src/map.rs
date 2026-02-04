use std::collections::BTreeMap;

use crate::value::Value;

#[derive(Default, Clone)]
pub struct Map(BTreeMap<String, Value>);

impl Map {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exists(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn set(&mut self, key: &str, value: Value) -> &mut Self {
        self.0.insert(key.to_string(), value);
        self
    }

    pub fn merge(&mut self, other: Self) -> &mut Self {
        for (key, value) in other.0 {
            self.0.insert(key, value);
        }

        self
    }
}

impl std::ops::Deref for Map {
    type Target = BTreeMap<String, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_map();

        for (key, value) in &self.0 {
            s.entry(key, value);
        }

        s.finish()
    }
}
