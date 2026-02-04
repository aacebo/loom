use crate::MediaType;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct ETag([u8; 32]);

impl ETag {
    pub fn new(media_type: MediaType, content: &str) -> Self {
        let tag = format!("{}::{}", media_type, content);
        Self(*blake3::hash(tag.as_bytes()).as_bytes())
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl std::fmt::Display for ETag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}
