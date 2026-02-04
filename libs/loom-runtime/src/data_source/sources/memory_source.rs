use std::collections::HashMap;
use std::sync::RwLock;

use crate::path::Path;

use crate::data_source::{DataSource, Document, Id, ReadError, WriteError};

pub struct MemorySource {
    documents: RwLock<HashMap<Id, Document>>,
}

impl MemorySource {
    pub fn new() -> Self {
        Self {
            documents: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemorySource {
    fn default() -> Self {
        Self::new()
    }
}

impl DataSource for MemorySource {
    fn read(&self, path: &Path) -> Result<Document, ReadError> {
        let id = Id::new(path.to_string().as_str());
        let documents = self
            .documents
            .read()
            .map_err(|e| ReadError::Panic(e.to_string()))?;

        documents
            .get(&id)
            .cloned()
            .ok_or_else(|| ReadError::Custom(format!("document not found: {}", path)))
    }

    fn write(&self, document: Document) -> Result<(), WriteError> {
        let mut documents = self
            .documents
            .write()
            .map_err(|e| WriteError::Panic(e.to_string()))?;
        documents.insert(document.id, document);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Entity, MediaType, path::FieldPath, path::FilePath, value::Value};

    #[test]
    fn test_write_and_read() {
        let ds = MemorySource::new();
        let path = Path::File(FilePath::parse("/test/file.txt"));
        let entity = Entity::new(
            FieldPath::parse("root").unwrap(),
            "text",
            Value::String("hello".to_string()),
        );
        let doc = Document::new(path.clone(), MediaType::TextPlain, vec![entity]);

        ds.write(doc.clone()).unwrap();
        let read_doc = ds.read(&path).unwrap();

        assert_eq!(read_doc, doc);
    }

    #[test]
    fn test_read_not_found() {
        let ds = MemorySource::new();
        let path = Path::File(FilePath::parse("/nonexistent"));
        let result = ds.read(&path);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_custom());
    }
}
