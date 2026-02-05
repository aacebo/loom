use std::path::PathBuf;

use loom_core::path::FilePath;
use loom_core::value::Value;
use loom_core::{Format, path::Path};

use super::{ConfigError, Provider};

fn infer_format(path: &std::path::Path) -> Format {
    match path.extension().and_then(|e| e.to_str()) {
        Some("json") => Format::Json,
        Some("yaml") | Some("yml") => Format::Yaml,
        Some("toml") => Format::Toml,
        _ => Format::Json,
    }
}

#[derive(Debug, Clone)]
pub struct FileProviderBuilder {
    path: PathBuf,
    format: Option<Format>,
    optional: bool,
}

impl FileProviderBuilder {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            format: None,
            optional: false,
        }
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);
        self
    }

    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }

    pub fn build(self) -> FileProvider {
        let format = self.format.unwrap_or_else(|| infer_format(&self.path));
        FileProvider {
            path: self.path,
            format,
            is_optional: self.optional,
        }
    }
}

pub struct FileProvider {
    path: PathBuf,
    format: Format,
    is_optional: bool,
}

impl FileProvider {
    pub fn builder(path: impl Into<PathBuf>) -> FileProviderBuilder {
        FileProviderBuilder::new(path)
    }

    fn parse_content(&self, content: &str) -> Result<Value, ConfigError> {
        #[cfg(feature = "json")]
        if self.format == Format::Json {
            let json: serde_json::Value =
                serde_json::from_str(content).map_err(ConfigError::parse)?;
            return Ok(json.into());
        }

        #[cfg(feature = "yaml")]
        if self.format == Format::Yaml {
            let docs = saphyr::Yaml::load_from_str(content).map_err(ConfigError::parse)?;
            if let Some(doc) = docs.into_iter().next() {
                return Ok(doc.into());
            } else {
                return Ok(Value::Null);
            }
        }

        #[cfg(feature = "toml")]
        if self.format == Format::Toml {
            let toml_value: toml::Value = toml::from_str(content).map_err(ConfigError::parse)?;
            return Ok(toml_value.into());
        }

        Err(ConfigError::provider(format!(
            "unsupported format: {:?}",
            self.format
        )))
    }
}

impl Provider for FileProvider {
    fn name(&self) -> &str {
        self.path.to_str().unwrap_or("file")
    }

    fn path(&self) -> Path {
        FilePath::from(self.path.clone()).into()
    }

    fn optional(&self) -> bool {
        self.is_optional
    }

    fn format(&self) -> Format {
        self.format
    }

    fn load(&self) -> Result<Option<Value>, ConfigError> {
        if !self.path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&self.path)?;
        let value = self.parse_content(&content)?;

        Ok(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_format_json() {
        assert_eq!(
            infer_format(std::path::Path::new("config.json")),
            Format::Json
        );
    }

    #[test]
    fn test_infer_format_yaml() {
        assert_eq!(
            infer_format(std::path::Path::new("config.yaml")),
            Format::Yaml
        );
        assert_eq!(
            infer_format(std::path::Path::new("config.yml")),
            Format::Yaml
        );
    }

    #[test]
    fn test_infer_format_toml() {
        assert_eq!(
            infer_format(std::path::Path::new("config.toml")),
            Format::Toml
        );
    }

    #[test]
    fn test_infer_format_default() {
        assert_eq!(infer_format(std::path::Path::new("config")), Format::Json);
        assert_eq!(
            infer_format(std::path::Path::new("config.txt")),
            Format::Json
        );
    }

    #[test]
    fn test_builder_infers_format() {
        let provider = FileProvider::builder("config.yaml").build();
        assert_eq!(provider.format, Format::Yaml);
    }

    #[test]
    fn test_builder_override_format() {
        let provider = FileProvider::builder("config.yaml")
            .format(Format::Json)
            .build();
        assert_eq!(provider.format, Format::Json);
    }

    #[test]
    fn test_builder_optional() {
        let provider = FileProvider::builder("config.json").optional(true).build();
        assert!(provider.is_optional);
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_file_provider_not_found_optional() {
        let provider = FileProvider::builder("/nonexistent/path.json")
            .optional(true)
            .build();
        let result = provider.load().unwrap();
        assert!(result.is_none());
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_file_provider_not_found_required() {
        let provider = FileProvider::builder("/nonexistent/path.json").build();
        let result = provider.load().unwrap();
        assert!(result.is_none());
    }
}
