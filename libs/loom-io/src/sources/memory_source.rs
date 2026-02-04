use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;

use crate::path::Path;

use crate::{DataSource, Id, ReadError, Record, WriteError};

#[derive(Debug, Clone)]
pub struct MemorySourceConfig {
    name: String,
}

impl MemorySourceConfig {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct MemorySourceBuilder {
    name: Option<String>,
    initial_records: Vec<Record>,
}

impl MemorySourceBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            initial_records: Vec::new(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_records(mut self, records: impl IntoIterator<Item = Record>) -> Self {
        self.initial_records.extend(records);
        self
    }

    pub fn with_record(mut self, record: Record) -> Self {
        self.initial_records.push(record);
        self
    }

    pub fn build(self) -> MemorySource {
        let mut records_map = HashMap::new();
        for record in self.initial_records {
            records_map.insert(record.id, record);
        }

        MemorySource {
            config: MemorySourceConfig {
                name: self.name.unwrap_or_else(|| "memory".to_string()),
            },
            records: RwLock::new(records_map),
        }
    }
}

impl Default for MemorySourceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MemorySource {
    config: MemorySourceConfig,
    records: RwLock<HashMap<Id, Record>>,
}

impl MemorySource {
    pub fn builder() -> MemorySourceBuilder {
        MemorySourceBuilder::new()
    }

    pub fn config(&self) -> &MemorySourceConfig {
        &self.config
    }
}

impl Default for MemorySource {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[async_trait]
impl DataSource for MemorySource {
    fn name(&self) -> &str {
        &self.config.name
    }

    async fn exists(&self, path: &Path) -> Result<bool, ReadError> {
        let id = Id::new(path.to_string().as_str());
        let records = self
            .records
            .read()
            .map_err(|e| ReadError::Panic(e.to_string()))?;
        Ok(records.contains_key(&id))
    }

    async fn count(&self, path: &Path) -> Result<usize, ReadError> {
        let path_str = path.to_string();
        let records = self
            .records
            .read()
            .map_err(|e| ReadError::Panic(e.to_string()))?;

        let count = records
            .values()
            .filter(|r| r.path.to_string().starts_with(&path_str))
            .count();
        Ok(count)
    }

    async fn find_one(&self, path: &Path) -> Result<Record, ReadError> {
        let id = Id::new(path.to_string().as_str());
        let records = self
            .records
            .read()
            .map_err(|e| ReadError::Panic(e.to_string()))?;

        records
            .get(&id)
            .cloned()
            .ok_or_else(|| ReadError::Custom(format!("record not found: {}", path)))
    }

    async fn find(&self, path: &Path) -> Result<Vec<Record>, ReadError> {
        let path_str = path.to_string();
        let records = self
            .records
            .read()
            .map_err(|e| ReadError::Panic(e.to_string()))?;

        let results: Vec<Record> = records
            .values()
            .filter(|r| r.path.to_string().starts_with(&path_str))
            .cloned()
            .collect();
        Ok(results)
    }

    async fn create(&self, record: Record) -> Result<(), WriteError> {
        let mut records = self
            .records
            .write()
            .map_err(|e| WriteError::Panic(e.to_string()))?;

        if records.contains_key(&record.id) {
            return Err(WriteError::Custom(format!(
                "record already exists: {}",
                record.path
            )));
        }

        records.insert(record.id, record);
        Ok(())
    }

    async fn update(&self, record: Record) -> Result<(), WriteError> {
        let mut records = self
            .records
            .write()
            .map_err(|e| WriteError::Panic(e.to_string()))?;

        if !records.contains_key(&record.id) {
            return Err(WriteError::Custom(format!(
                "record not found: {}",
                record.path
            )));
        }

        records.insert(record.id, record);
        Ok(())
    }

    async fn upsert(&self, record: Record) -> Result<(), WriteError> {
        let mut records = self
            .records
            .write()
            .map_err(|e| WriteError::Panic(e.to_string()))?;
        records.insert(record.id, record);
        Ok(())
    }

    async fn delete(&self, path: &Path) -> Result<(), WriteError> {
        let id = Id::new(path.to_string().as_str());
        let mut records = self
            .records
            .write()
            .map_err(|e| WriteError::Panic(e.to_string()))?;

        if records.remove(&id).is_none() {
            return Err(WriteError::Custom(format!("record not found: {}", path)));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MediaType, path::FilePath};

    fn make_record(path: &Path) -> Record {
        Record::from_str(path.clone(), MediaType::TextPlain, "hello")
    }

    #[tokio::test]
    async fn test_create_and_find_one() {
        let ds = MemorySource::builder().build();
        let path = Path::File(FilePath::parse("/test/file.txt"));
        let record = make_record(&path);

        ds.create(record.clone()).await.unwrap();
        let read_record = ds.find_one(&path).await.unwrap();

        assert_eq!(read_record, record);
    }

    #[tokio::test]
    async fn test_exists() {
        let ds = MemorySource::builder().build();
        let path = Path::File(FilePath::parse("/test/file.txt"));
        let record = make_record(&path);

        assert!(!ds.exists(&path).await.unwrap());
        ds.create(record).await.unwrap();
        assert!(ds.exists(&path).await.unwrap());
    }

    #[tokio::test]
    async fn test_count() {
        let ds = MemorySource::builder().build();
        let path1 = Path::File(FilePath::parse("/test/file1.txt"));
        let path2 = Path::File(FilePath::parse("/test/file2.txt"));
        let path3 = Path::File(FilePath::parse("/other/file.txt"));

        ds.create(make_record(&path1)).await.unwrap();
        ds.create(make_record(&path2)).await.unwrap();
        ds.create(make_record(&path3)).await.unwrap();

        let test_path = Path::File(FilePath::parse("/test"));
        assert_eq!(ds.count(&test_path).await.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_find() {
        let ds = MemorySource::builder().build();
        let path1 = Path::File(FilePath::parse("/test/file1.txt"));
        let path2 = Path::File(FilePath::parse("/test/file2.txt"));
        let path3 = Path::File(FilePath::parse("/other/file.txt"));

        ds.create(make_record(&path1)).await.unwrap();
        ds.create(make_record(&path2)).await.unwrap();
        ds.create(make_record(&path3)).await.unwrap();

        let test_path = Path::File(FilePath::parse("/test"));
        let results = ds.find(&test_path).await.unwrap();
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_create_duplicate_fails() {
        let ds = MemorySource::builder().build();
        let path = Path::File(FilePath::parse("/test/file.txt"));
        let record = make_record(&path);

        ds.create(record.clone()).await.unwrap();
        let result = ds.create(record).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update() {
        let ds = MemorySource::builder().build();
        let path = Path::File(FilePath::parse("/test/file.txt"));
        let record = make_record(&path);

        ds.create(record.clone()).await.unwrap();
        ds.update(record).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_not_found() {
        let ds = MemorySource::builder().build();
        let path = Path::File(FilePath::parse("/test/file.txt"));
        let record = make_record(&path);

        let result = ds.update(record).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_upsert() {
        let ds = MemorySource::builder().build();
        let path = Path::File(FilePath::parse("/test/file.txt"));
        let record = make_record(&path);

        ds.upsert(record.clone()).await.unwrap();
        assert!(ds.exists(&path).await.unwrap());

        ds.upsert(record).await.unwrap();
        assert!(ds.exists(&path).await.unwrap());
    }

    #[tokio::test]
    async fn test_delete() {
        let ds = MemorySource::builder().build();
        let path = Path::File(FilePath::parse("/test/file.txt"));
        let record = make_record(&path);

        ds.create(record).await.unwrap();
        assert!(ds.exists(&path).await.unwrap());

        ds.delete(&path).await.unwrap();
        assert!(!ds.exists(&path).await.unwrap());
    }

    #[tokio::test]
    async fn test_delete_not_found() {
        let ds = MemorySource::builder().build();
        let path = Path::File(FilePath::parse("/nonexistent"));
        let result = ds.delete(&path).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_one_not_found() {
        let ds = MemorySource::builder().build();
        let path = Path::File(FilePath::parse("/nonexistent"));
        let result = ds.find_one(&path).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is_custom());
    }

    #[test]
    fn test_builder() {
        let ds = MemorySource::builder().name("custom_memory").build();

        assert_eq!(ds.config().name(), "custom_memory");
    }

    #[test]
    fn test_builder_defaults() {
        let ds = MemorySource::builder().build();
        assert_eq!(ds.config().name(), "memory");
    }

    #[tokio::test]
    async fn test_builder_with_records() {
        let path = Path::File(FilePath::parse("/test/file.txt"));
        let record = make_record(&path);

        let ds = MemorySource::builder().with_record(record).build();

        assert!(ds.exists(&path).await.unwrap());
    }
}
