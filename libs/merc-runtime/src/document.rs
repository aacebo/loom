use crate::MediaType;

#[derive(Debug, Clone)]
pub struct Document {
    pub path: String,
    pub etag: String,
    pub mime_type: MediaType,
    pub bytes: Box<[u8]>,
}

impl Eq for Document {}
impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.etag.eq(&other.etag)
    }
}
