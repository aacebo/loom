use std::collections::BTreeMap;

use loom_core::value::Value;

#[derive(Default, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct Attributes(BTreeMap<String, Value>);

impl Attributes {
    pub fn new() -> AttributesBuilder {
        AttributesBuilder::new()
    }
}

#[derive(Default)]
pub struct AttributesBuilder {
    inner: BTreeMap<String, Value>,
}

impl AttributesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attr(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.inner.insert(key.into(), value.into());
        self
    }

    pub fn merge(mut self, other: Attributes) -> Self {
        for (key, value) in other.0 {
            self.inner.insert(key, value);
        }
        self
    }

    pub fn build(self) -> Attributes {
        Attributes(self.inner)
    }
}

impl Attributes {
    pub fn exists(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }
}

impl std::ops::Deref for Attributes {
    type Target = BTreeMap<String, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_map();

        for (key, value) in &self.0 {
            s.entry(key, value);
        }

        s.finish()
    }
}

#[cfg(feature = "json")]
impl std::fmt::Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).expect("should serialize")
        )
    }
}
