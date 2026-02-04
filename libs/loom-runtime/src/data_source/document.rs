use crate::{ETag, Id, MediaType, value::Value};

#[derive(Debug, Clone, Hash, serde::Deserialize, serde::Serialize)]
pub struct Document {
    pub id: Id,
    pub etag: ETag,
    pub mime_type: MediaType,
    pub content: Value,
}

impl Eq for Document {}
impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.etag.eq(&other.etag)
    }
}

#[cfg(feature = "json")]
impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).expect("should serialize")
        )
    }
}
