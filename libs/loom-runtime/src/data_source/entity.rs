use crate::{Id, value::Value};

#[derive(Debug, Clone, Hash, serde::Deserialize, serde::Serialize)]
pub struct Entity {
    pub id: Id,
    pub content: Value,
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
