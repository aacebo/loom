mod document;
mod entity;
mod error;
mod etag;
mod id;
pub mod sources;

pub use document::*;
pub use entity::*;
pub use error::*;
pub use etag::*;
pub use id::*;

use crate::path::Path;

pub trait DataSource {
    fn read(&self, path: &Path) -> Result<Document, ReadError>;
    fn write(&self, document: Document) -> Result<(), WriteError>;
}
