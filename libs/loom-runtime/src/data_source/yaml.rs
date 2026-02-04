use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use saphyr::{Yaml, YamlEmitter};

use crate::MediaType;
use crate::path::{FieldPath, Path};
use crate::value::Value;

use super::{DataSource, Document, Entity, Id, ReadError, WriteError};

#[derive(Debug, Clone)]
pub struct YamlFileSourceOptions {
    pub path: PathBuf,
}

impl Default for YamlFileSourceOptions {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
        }
    }
}

impl YamlFileSourceOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = path.into();
        self
    }
}

pub struct YamlFileSource {
    options: YamlFileSourceOptions,
    cache: RwLock<HashMap<Id, Document>>,
}

impl YamlFileSource {
    pub fn new() -> Self {
        Self::with_options(YamlFileSourceOptions::default())
    }

    pub fn with_options(options: YamlFileSourceOptions) -> Self {
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
                "YamlFileSource only supports File paths".to_string(),
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

impl Default for YamlFileSource {
    fn default() -> Self {
        Self::new()
    }
}

impl DataSource for YamlFileSource {
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

        let content = if media_type == MediaType::TextYaml {
            let docs = Yaml::load_from_str(&content_str)
                .map_err(|e| ReadError::Custom(format!("YAML parse error: {}", e)))?;
            let yaml = docs.into_iter().next().unwrap_or(Yaml::Null);
            Value::from(yaml)
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

        let output = if document.media_type == MediaType::TextYaml {
            let yaml = saphyr::Yaml::from(&content.content);
            let mut out_str = String::new();
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter
                .dump(&yaml)
                .map_err(|e| WriteError::Custom(format!("YAML serialize error: {}", e)))?;
            out_str
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
    use crate::{path::FilePath, value::Object};
    use std::env::temp_dir;

    fn test_dir() -> PathBuf {
        temp_dir().join("loom_yaml_source_test")
    }

    fn test_options() -> YamlFileSourceOptions {
        YamlFileSourceOptions::new().with_path(test_dir())
    }

    #[test]
    fn test_read_yaml_file() {
        let dir = test_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("test.yaml");
        std::fs::write(&file_path, "name: test\nvalue: 42").unwrap();

        let ds = YamlFileSource::with_options(test_options());
        let path = Path::File(FilePath::parse(file_path.to_str().unwrap()));

        let doc = ds.read(&path).unwrap();

        assert_eq!(doc.media_type, MediaType::TextYaml);
        assert!(doc.content[0].content.is_object());
        assert_eq!(doc.content[0].content["name"].as_str(), Some("test"));
        assert_eq!(doc.content[0].content["value"].as_int(), Some(42));
        assert_eq!(doc.content[0].otype, "application/yaml");

        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_read_text_file() {
        let dir = test_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("test.txt");
        std::fs::write(&file_path, "Hello, World!").unwrap();

        let ds = YamlFileSource::with_options(test_options());
        let path = Path::File(FilePath::parse(file_path.to_str().unwrap()));

        let doc = ds.read(&path).unwrap();

        assert_eq!(doc.media_type, MediaType::TextPlain);
        assert_eq!(doc.content[0].content.as_str(), Some("Hello, World!"));
        assert_eq!(doc.content[0].otype, "text/plain");

        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_write_yaml_file() {
        let ds = YamlFileSource::with_options(test_options());
        let file_path = test_dir().join("write_test.yaml");
        let path = Path::File(FilePath::parse(file_path.to_str().unwrap()));

        let mut obj = Object::new();
        obj.insert("key".to_string(), Value::String("value".to_string()));
        let content = Value::Object(obj);

        let entity = Entity::new(FieldPath::parse("root").unwrap(), "text/yaml", content);
        let doc = Document::new(path.clone(), MediaType::TextYaml, vec![entity]);

        ds.write(doc).unwrap();

        let written = std::fs::read_to_string(&file_path).unwrap();
        assert!(written.contains("key"));
        assert!(written.contains("value"));

        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_roundtrip() {
        let ds = YamlFileSource::with_options(test_options());
        let file_path = test_dir().join("roundtrip.yaml");
        let path = Path::File(FilePath::parse(file_path.to_str().unwrap()));

        let mut obj = Object::new();
        obj.insert("test".to_string(), Value::from(123));
        let content = Value::Object(obj);

        let entity = Entity::new(
            FieldPath::parse("root").unwrap(),
            "text/yaml",
            content.clone(),
        );
        let doc = Document::new(path.clone(), MediaType::TextYaml, vec![entity]);

        ds.write(doc).unwrap();
        ds.clear().unwrap();

        let read_doc = ds.read(&path).unwrap();
        assert_eq!(read_doc.content[0].content["test"].as_int(), Some(123));

        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_read_not_found() {
        let ds = YamlFileSource::with_options(test_options());
        let path = Path::File(FilePath::parse("/nonexistent/file.yaml"));

        let result = ds.read(&path);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_io());
    }

    #[test]
    fn test_options_builder() {
        let options = YamlFileSourceOptions::new().with_path("/custom/path");

        assert_eq!(options.path, PathBuf::from("/custom/path"));
    }
}
