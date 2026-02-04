use crate::{Id, path::IdentPath, value::Value};

#[derive(Debug, Clone, Hash, serde::Deserialize, serde::Serialize)]
pub struct Entity {
    pub id: Id,
    pub path: IdentPath,
    pub otype: String,
    pub content: Value,
}

impl Entity {
    pub fn new(path: IdentPath, otype: &str, content: Value) -> Self {
        Self {
            id: Id::new(path.to_string().as_str()),
            path,
            otype: otype.to_string(),
            content,
        }
    }
}

impl Eq for Entity {}
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

#[cfg(feature = "json")]
impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).expect("should serialize")
        )
    }
}
