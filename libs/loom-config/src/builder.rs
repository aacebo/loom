use loom_core::Format;
use loom_core::path::Path;
use loom_core::value::{Object, Value};

use super::providers::Provider;
use super::{Config, ConfigError, Env};

#[derive(Default)]
pub struct ConfigBuilder {
    providers: Vec<Box<dyn Provider>>,
    env: Option<Env>,
    path: Option<Path>,
    format: Option<Format>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_provider<P: Provider + 'static>(mut self, provider: P) -> Self {
        self.providers.push(Box::new(provider));
        self
    }

    pub fn with_env(mut self, env: Env) -> Self {
        self.env = Some(env);
        self
    }

    pub fn with_path(mut self, path: Path) -> Self {
        self.path = Some(path);
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn build(self) -> Result<Config, ConfigError> {
        use super::ConfigSource;

        let env = self.env.unwrap_or_else(Env::from_env);
        let mut merged = Value::Object(Object::new());
        let mut sources = Vec::new();

        for provider in &self.providers {
            match provider.load() {
                Ok(Some(value)) => {
                    merged.merge(value);
                    sources.push(ConfigSource {
                        name: provider.name().to_string(),
                        path: provider.path(),
                        format: provider.format(),
                    });
                }
                Ok(None) => {
                    if !provider.optional() {
                        return Err(ConfigError::not_found(provider.name()));
                    }
                }
                Err(e) => {
                    if !provider.optional() {
                        return Err(e);
                    }
                }
            }
        }

        Ok(Config {
            env,
            path: self.path,
            format: self.format,
            data: merged,
            sources,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::providers::MemoryProvider;
    use super::*;
    use loom_core::path::{FilePath, IdentPath};

    #[test]
    fn test_builder_with_provider() {
        let config = Config::new()
            .with_provider(MemoryProvider::from_pairs([
                ("database.host", "localhost"),
                ("database.port", "5432"),
            ]))
            .build()
            .unwrap();

        let path = IdentPath::parse("database.host").unwrap();
        assert_eq!(config.get_str(&path), Some("localhost"));
    }

    #[test]
    fn test_builder_merge_order() {
        let config = Config::new()
            .with_provider(MemoryProvider::from_pairs([("database.host", "first")]))
            .with_provider(MemoryProvider::from_pairs([("database.host", "second")]))
            .build()
            .unwrap();

        let path = IdentPath::parse("database.host").unwrap();
        assert_eq!(config.get_str(&path), Some("second"));
    }

    #[test]
    fn test_builder_merge() {
        let config = Config::new()
            .with_provider(MemoryProvider::from_pairs([
                ("database.host", "localhost"),
                ("database.port", "5432"),
            ]))
            .with_provider(MemoryProvider::from_pairs([
                ("database.host", "remotehost"),
                ("logging.level", "debug"),
            ]))
            .build()
            .unwrap();

        let path = IdentPath::parse("database.host").unwrap();
        assert_eq!(config.get_str(&path), Some("remotehost"));

        let path = IdentPath::parse("database.port").unwrap();
        assert_eq!(config.get_str(&path), Some("5432"));

        let path = IdentPath::parse("logging.level").unwrap();
        assert_eq!(config.get_str(&path), Some("debug"));
    }

    #[test]
    fn test_builder_empty() {
        let config = Config::new().build().unwrap();
        assert!(config.as_value().is_empty());
    }

    #[test]
    fn test_builder_with_env() {
        let config = Config::new().with_env(Env::Dev).build().unwrap();

        assert!(config.env().is_dev());
    }

    #[test]
    fn test_builder_with_path_and_format() {
        let config = Config::new()
            .with_provider(MemoryProvider::from_pairs([("key", "value")]))
            .with_path(Path::File(FilePath::parse("config.json")))
            .with_format(Format::Json)
            .build()
            .unwrap();

        assert!(config.path().is_some());
        assert_eq!(config.format(), Some(Format::Json));
    }
}
