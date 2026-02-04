mod document;
mod entity;
mod error;
mod etag;
mod id;
#[cfg(feature = "json")]
mod json;
mod memory;
#[cfg(feature = "yaml")]
mod yaml;

pub use document::*;
pub use entity::*;
pub use error::*;
pub use etag::*;
pub use id::*;
#[cfg(feature = "json")]
pub use json::*;
pub use memory::*;
#[cfg(feature = "yaml")]
pub use yaml::*;

use crate::path::Path;

pub trait DataSource {
    fn read(&self, path: &Path) -> Result<Document, ReadError>;
    fn write(&self, document: Document) -> Result<(), WriteError>;
}
