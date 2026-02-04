use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::MediaType;
use crate::path::{FieldPath, Path};
use crate::value::Value;

use crate::data_source::{DataSource, Document, Entity, Id, ReadError, WriteError};

#[derive(Debug, Clone)]
pub struct JsonFileSourceOptions {
    pub path: PathBuf,
    pub pretty_print: bool,
}

impl Default for JsonFileSourceOptions {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
            pretty_print: false,
        }
    }
}

impl JsonFileSourceOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = path.into();
        self
    }

    pub fn with_pretty_print(mut self, pretty: bool) -> Self {
        self.pretty_print = pretty;
        self
    }
}

pub struct JsonFileSource {
    options: JsonFileSourceOptions,
    cache: RwLock<HashMap<Id, Document>>,
}

impl JsonFileSource {
    pub fn new() -> Self {
        Self::with_options(JsonFileSourceOptions::default())
    }

    pub fn with_options(options: JsonFileSourceOptions) -> Self {
        Self {
            options,
            cache: RwLock::new(HashMap::new()),
        }
    }

    fn full_path(&self, path: &Path) -> Result<PathBuf, ReadError> {
        match path {
            Path::File(file_path) => {
                let path_buf: &std::path::Path = file_path;
                if path_buf.is_absolute() {
                    Ok(path_buf.to_path_buf())
                } else {
                    Ok(self.options.path.join(path_buf))
                }
            }
            _ => Err(ReadError::Custom(
                "JsonFileSource only supports File paths".to_string(),
            )),
        }
    }

    pub fn clear(&self) -> Result<(), ReadError> {
        let mut cache = self
            .cache
            .write()
            .map_err(|e| ReadError::Panic(e.to_string()))?;
        cache.clear();
        Ok(())
    }
}

impl Default for JsonFileSource {
    fn default() -> Self {
        Self::new()
    }
}

impl DataSource for JsonFileSource {
    fn read(&self, path: &Path) -> Result<Document, ReadError> {
        let id = Id::new(path.to_string().as_str());

        {
            let cache = self
                .cache
                .read()
                .map_err(|e| ReadError::Panic(e.to_string()))?;
            if let Some(doc) = cache.get(&id) {
                return Ok(doc.clone());
            }
        }

        let full_path = self.full_path(path)?;
        let content_str = std::fs::read_to_string(&full_path)?;
        let media_type = MediaType::from_path(&full_path);
        let content = if media_type == MediaType::TextJson {
            let json: serde_json::Value = serde_json::from_str(&content_str)
                .map_err(|e| ReadError::Custom(format!("JSON parse error: {}", e)))?;
            json.into()
        } else if media_type.is_textlike() {
            Value::String(content_str)
        } else {
            return Err(ReadError::Custom(format!(
                "Unsupported media type: {}",
                media_type
            )));
        };

        let entity = Entity::new(
            FieldPath::parse("root").expect("valid field path"),
            media_type.as_mime_str(),
            content,
        );

        let document = Document::new(path.clone(), media_type, vec![entity]);

        {
            let mut cache = self
                .cache
                .write()
                .map_err(|e| ReadError::Panic(e.to_string()))?;
            cache.insert(id, document.clone());
        }

        Ok(document)
    }

    fn write(&self, document: Document) -> Result<(), WriteError> {
        let full_path = self.full_path(&document.path).map_err(|e| match e {
            ReadError::Custom(msg) => WriteError::Custom(msg),
            ReadError::IO(io) => WriteError::IO(io),
            ReadError::Panic(msg) => WriteError::Panic(msg),
        })?;

        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = document
            .content
            .first()
            .ok_or_else(|| WriteError::Custom("Document has no content".to_string()))?;

        let output = if document.media_type == MediaType::TextJson {
            let json: serde_json::Value = (&content.content).into();
            if self.options.pretty_print {
                serde_json::to_string_pretty(&json)
            } else {
                serde_json::to_string(&json)
            }
            .map_err(|e| WriteError::Custom(format!("JSON serialize error: {}", e)))?
        } else if document.media_type.is_textlike() {
            content
                .content
                .as_str()
                .ok_or_else(|| {
                    WriteError::Custom("Text content must be a string Value".to_string())
                })?
                .to_string()
        } else {
            return Err(WriteError::Custom(format!(
                "Unsupported media type: {}",
                document.media_type
            )));
        };

        std::fs::write(&full_path, &output)?;

        let id = document.id;
        {
            let mut cache = self
                .cache
                .write()
                .map_err(|e| WriteError::Panic(e.to_string()))?;
            cache.insert(id, document);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::FilePath;
    use std::env::temp_dir;

    fn test_dir() -> PathBuf {
        temp_dir().join("loom_file_source_test")
    }

    fn test_options() -> JsonFileSourceOptions {
        JsonFileSourceOptions::new()
            .with_path(test_dir())
            .with_pretty_print(true)
    }

    #[test]
    fn test_read_json_file() {
        let dir = test_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("test.json");
        std::fs::write(&file_path, r#"{"name": "test", "value": 42}"#).unwrap();

        let ds = JsonFileSource::with_options(test_options());
        let path = Path::File(FilePath::parse(file_path.to_str().unwrap()));

        let doc = ds.read(&path).unwrap();

        assert_eq!(doc.media_type, MediaType::TextJson);
        assert!(doc.content[0].content.is_object());
        assert_eq!(doc.content[0].content["name"].as_str(), Some("test"));
        assert_eq!(doc.content[0].content["value"].as_int(), Some(42));
        assert_eq!(doc.content[0].otype, "application/json");

        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_read_text_file() {
        let dir = test_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("test.txt");
        std::fs::write(&file_path, "Hello, World!").unwrap();

        let ds = JsonFileSource::with_options(test_options());
        let path = Path::File(FilePath::parse(file_path.to_str().unwrap()));

        let doc = ds.read(&path).unwrap();

        assert_eq!(doc.media_type, MediaType::TextPlain);
        assert_eq!(doc.content[0].content.as_str(), Some("Hello, World!"));
        assert_eq!(doc.content[0].otype, "text/plain");

        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_write_json_file() {
        let ds = JsonFileSource::with_options(test_options());
        let file_path = test_dir().join("write_test.json");
        let path = Path::File(FilePath::parse(file_path.to_str().unwrap()));

        let mut obj = crate::value::Object::new();
        obj.insert("key".to_string(), Value::String("value".to_string()));
        let content = Value::Object(obj);

        let entity = Entity::new(
            FieldPath::parse("root").unwrap(),
            "application/json",
            content,
        );
        let doc = Document::new(path.clone(), MediaType::TextJson, vec![entity]);

        ds.write(doc).unwrap();

        let written = std::fs::read_to_string(&file_path).unwrap();
        assert!(written.contains("\"key\""));
        assert!(written.contains("\"value\""));

        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_write_text_file() {
        let ds = JsonFileSource::with_options(test_options());
        let file_path = test_dir().join("write_test.txt");
        let path = Path::File(FilePath::parse(file_path.to_str().unwrap()));

        let entity = Entity::new(
            FieldPath::parse("root").unwrap(),
            "text/plain",
            Value::String("Hello from write!".to_string()),
        );
        let doc = Document::new(path.clone(), MediaType::TextPlain, vec![entity]);

        ds.write(doc).unwrap();

        let written = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(written, "Hello from write!");

        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_roundtrip() {
        let ds = JsonFileSource::with_options(test_options());
        let file_path = test_dir().join("roundtrip.json");
        let path = Path::File(FilePath::parse(file_path.to_str().unwrap()));

        let mut obj = crate::value::Object::new();
        obj.insert("test".to_string(), Value::from(123));
        let content = Value::Object(obj);

        let entity = Entity::new(
            FieldPath::parse("root").unwrap(),
            "application/json",
            content.clone(),
        );
        let doc = Document::new(path.clone(), MediaType::TextJson, vec![entity]);

        ds.write(doc).unwrap();
        ds.clear().unwrap();

        let read_doc = ds.read(&path).unwrap();
        assert_eq!(read_doc.content[0].content["test"].as_int(), Some(123));

        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_read_not_found() {
        let ds = JsonFileSource::with_options(test_options());
        let path = Path::File(FilePath::parse("/nonexistent/file.txt"));

        let result = ds.read(&path);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_io());
    }

    #[test]
    fn test_options_builder() {
        let options = JsonFileSourceOptions::new()
            .with_path("/custom/path")
            .with_pretty_print(true);

        assert_eq!(options.path, PathBuf::from("/custom/path"));
        assert!(options.pretty_print);
    }
}
