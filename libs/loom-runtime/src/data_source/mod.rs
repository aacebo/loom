mod document;
mod entity;
mod error;
mod etag;
mod id;

pub use document::*;
pub use entity::*;
pub use error::*;
pub use etag::*;
pub use id::*;

pub trait DataSource {
    fn read(&self) -> Result<Document, ReadError>;
    fn write(&self, document: Document) -> Result<(), WriteError>;
}
