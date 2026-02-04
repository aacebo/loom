use serde::de::DeserializeOwned;

use loom_core::Format;
use loom_core::path::{FieldPath, Path};
use loom_core::value::Value;

use super::{ConfigBuilder, ConfigError, ConfigSection, Env};

#[derive(Debug, Clone)]
pub struct ConfigSource {
    pub name: String,
    pub path: Option<Path>,
    pub format: Option<Format>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) env: Env,
    pub(crate) data: Value,
    pub(crate) path: Option<Path>,
    pub(crate) format: Option<Format>,
    pub(crate) sources: Vec<ConfigSource>,
}

impl Config {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn as_value(&self) -> &Value {
        &self.data
    }

    pub fn sources(&self) -> &[ConfigSource] {
        &self.sources
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref()
    }

    pub fn format(&self) -> Option<Format> {
        self.format
    }

    pub fn get(&self, path: &FieldPath) -> Option<&Value> {
        self.data.get_by_path(path)
    }

    pub fn get_str(&self, path: &FieldPath) -> Option<&str> {
        self.get(path).and_then(|v| v.as_str())
    }

    pub fn get_int(&self, path: &FieldPath) -> Option<i64> {
        self.get(path).and_then(|v| v.as_int())
    }

    pub fn get_float(&self, path: &FieldPath) -> Option<f64> {
        self.get(path).and_then(|v| v.as_float())
    }

    pub fn get_bool(&self, path: &FieldPath) -> Option<bool> {
        self.get(path).and_then(|v| v.as_bool())
    }

    pub fn get_section(&self, path: &FieldPath) -> ConfigSection<'_> {
        ConfigSection::new(self.get(path), path.clone())
    }

    pub fn root_section(&self) -> ConfigSection<'_> {
        ConfigSection::root(&self.data)
    }

    pub fn merge(self, other: Self) -> Self {
        let mut data = self.data;
        data.merge(other.data);

        let mut sources = self.sources;
        sources.extend(other.sources);

        Self {
            env: self.env,
            data,
            path: self.path.or(other.path),
            format: self.format.or(other.format),
            sources,
        }
    }

    pub fn write(&self) -> Result<(), ConfigError> {
        let path = self
            .path
            .as_ref()
            .ok_or_else(|| ConfigError::provider("No primary config file path set"))?;
        let format = self
            .format
            .ok_or_else(|| ConfigError::provider("No primary config file format set"))?;

        self.write_to(path.clone(), format)
    }

    pub fn write_to(&self, path: Path, format: Format) -> Result<(), ConfigError> {
        let file_path: &std::path::Path = match &path {
            Path::File(fp) => fp,
            _ => return Err(ConfigError::provider("Can only write to file paths")),
        };

        #[cfg(feature = "json")]
        if format == Format::Json {
            let json: serde_json::Value = (&self.data).into();
            let content = serde_json::to_string_pretty(&json).map_err(ConfigError::parse)?;
            std::fs::write(file_path, content)?;
            return Ok(());
        }

        #[cfg(feature = "yaml")]
        if format == Format::Yaml {
            let yaml: saphyr::Yaml = (&self.data).into();
            let mut out = String::new();
            let mut emitter = saphyr::YamlEmitter::new(&mut out);
            emitter.dump(&yaml).map_err(ConfigError::parse)?;
            std::fs::write(file_path, out)?;
            return Ok(());
        }

        #[cfg(feature = "toml")]
        if format == Format::Toml {
            let toml_value: toml::Value = (&self.data).into();
            let content = toml::to_string_pretty(&toml_value).map_err(ConfigError::parse)?;
            std::fs::write(file_path, content)?;
            return Ok(());
        }

        Err(ConfigError::provider(format!(
            "Unsupported format: {:?}",
            format
        )))
    }

    pub fn bind<T: DeserializeOwned>(&self) -> Result<T, ConfigError> {
        let json: serde_json::Value = (&self.data).into();
        serde_json::from_value(json).map_err(ConfigError::deserialize)
    }

    pub fn bind_section<T: DeserializeOwned>(&self, path: &FieldPath) -> Result<T, ConfigError> {
        let value = self
            .get(path)
            .ok_or_else(|| ConfigError::not_found(path.to_string()))?;
        let json: serde_json::Value = value.into();
        serde_json::from_value(json).map_err(ConfigError::deserialize)
    }
}

#[cfg(test)]
mod tests {
    use super::super::MemoryProvider;
    use super::*;

    fn create_test_config() -> Config {
        use loom_core::value::{Number, Object};

        let mut db = Object::new();
        db.insert("host".to_string(), Value::String("localhost".to_string()));
        db.insert("port".to_string(), Value::Number(Number::Int(5432)));

        let servers = vec![
            {
                let mut s = Object::new();
                s.insert("name".to_string(), Value::String("primary".to_string()));
                s.insert("port".to_string(), Value::Number(Number::Int(8080)));
                Value::Object(s)
            },
            {
                let mut s = Object::new();
                s.insert("name".to_string(), Value::String("secondary".to_string()));
                s.insert("port".to_string(), Value::Number(Number::Int(8081)));
                Value::Object(s)
            },
        ];

        let mut root = Object::new();
        root.insert("database".to_string(), Value::Object(db));
        root.insert("servers".to_string(), Value::Array(servers.into()));
        root.insert("debug".to_string(), Value::Bool(true));

        Config::new()
            .with_provider(MemoryProvider::from_value(Value::Object(root)))
            .build()
            .unwrap()
    }

    #[test]
    fn test_get_str() {
        let config = create_test_config();
        let path = FieldPath::parse("database.host").unwrap();
        assert_eq!(config.get_str(&path), Some("localhost"));
    }

    #[test]
    fn test_get_int() {
        let config = create_test_config();
        let path = FieldPath::parse("database.port").unwrap();
        assert_eq!(config.get_int(&path), Some(5432));
    }

    #[test]
    fn test_get_bool() {
        let config = create_test_config();
        let path = FieldPath::parse("debug").unwrap();
        assert_eq!(config.get_bool(&path), Some(true));
    }

    #[test]
    fn test_get_array_element() {
        let config = create_test_config();
        let path = FieldPath::parse("servers[0].name").unwrap();
        assert_eq!(config.get_str(&path), Some("primary"));

        let path = FieldPath::parse("servers[1].port").unwrap();
        assert_eq!(config.get_int(&path), Some(8081));
    }

    #[test]
    fn test_get_section() {
        let config = create_test_config();
        let path = FieldPath::parse("database").unwrap();
        let section = config.get_section(&path);

        assert!(section.exists());
        assert!(section.is_object());
    }

    #[test]
    fn test_get_nonexistent() {
        let config = create_test_config();
        let path = FieldPath::parse("nonexistent.path").unwrap();
        assert!(config.get(&path).is_none());
    }

    #[test]
    fn test_merge() {
        let config1 = Config::new()
            .with_provider(MemoryProvider::from_pairs([
                ("database.host", "localhost"),
                ("database.port", "5432"),
            ]))
            .build()
            .unwrap();

        let config2 = Config::new()
            .with_provider(MemoryProvider::from_pairs([
                ("database.host", "remotehost"),
                ("logging.level", "debug"),
            ]))
            .build()
            .unwrap();

        let merged = config1.merge(config2);

        let path = FieldPath::parse("database.host").unwrap();
        assert_eq!(merged.get_str(&path), Some("remotehost"));

        let path = FieldPath::parse("database.port").unwrap();
        assert_eq!(merged.get_str(&path), Some("5432"));

        let path = FieldPath::parse("logging.level").unwrap();
        assert_eq!(merged.get_str(&path), Some("debug"));
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_bind_section() {
        use serde::Deserialize;

        #[derive(Debug, Deserialize, PartialEq)]
        struct DatabaseConfig {
            host: String,
            port: i64,
        }

        let config = create_test_config();
        let path = FieldPath::parse("database").unwrap();
        let db: DatabaseConfig = config.bind_section(&path).unwrap();

        assert_eq!(
            db,
            DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
            }
        );
    }
}
