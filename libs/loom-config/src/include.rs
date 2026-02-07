use std::collections::HashSet;
use std::path::{Path, PathBuf};

use loom_core::Format;
use loom_core::value::{Object, Value};

use crate::ConfigError;

const INCLUDE_KEY: &str = "$include";

fn infer_format(path: &Path) -> Format {
    match path.extension().and_then(|e| e.to_str()) {
        Some("json") => Format::Json,
        Some("yaml") | Some("yml") => Format::Yaml,
        Some("toml") => Format::Toml,
        _ => Format::Json,
    }
}

fn parse_content(content: &str, format: Format) -> Result<Value, ConfigError> {
    #[cfg(feature = "json")]
    if format == Format::Json {
        let json: serde_json::Value = serde_json::from_str(content).map_err(ConfigError::parse)?;
        return Ok(json.into());
    }

    #[cfg(feature = "yaml")]
    if format == Format::Yaml {
        let docs = saphyr::Yaml::load_from_str(content).map_err(ConfigError::parse)?;
        if let Some(doc) = docs.into_iter().next() {
            return Ok(doc.into());
        } else {
            return Ok(Value::Null);
        }
    }

    #[cfg(feature = "toml")]
    if format == Format::Toml {
        let toml_value: toml::Value = toml::from_str(content).map_err(ConfigError::parse)?;
        return Ok(toml_value.into());
    }

    Err(ConfigError::provider(format!(
        "unsupported format: {:?}",
        format
    )))
}

/// Resolves `$include` directives in configuration values.
///
/// The resolver processes include directives recursively, merging
/// included files in order. Later includes override earlier ones.
pub struct IncludeResolver {
    visited: HashSet<PathBuf>,
    include_chain: Vec<PathBuf>,
}

impl Default for IncludeResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl IncludeResolver {
    /// Create a new include resolver.
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            include_chain: Vec::new(),
        }
    }

    /// Resolve all `$include` directives in the given value.
    ///
    /// The `source_file` is the path to the file containing this value,
    /// used for resolving relative include paths.
    pub fn resolve(&mut self, value: Value, source_file: &Path) -> Result<Value, ConfigError> {
        let canonical = source_file
            .canonicalize()
            .unwrap_or_else(|_| source_file.to_path_buf());

        // Check for circular reference
        if self.visited.contains(&canonical) {
            let chain: Vec<String> = self
                .include_chain
                .iter()
                .map(|p| p.display().to_string())
                .collect();
            return Err(ConfigError::circular_include(
                canonical.display().to_string(),
                chain,
            ));
        }

        // Track this file
        self.visited.insert(canonical.clone());
        self.include_chain.push(canonical.clone());

        let result = self.resolve_inner(value, source_file);
        self.include_chain.pop();

        result
    }

    fn resolve_inner(&mut self, value: Value, source_file: &Path) -> Result<Value, ConfigError> {
        let mut value = value;

        // Extract and process $include if present
        if let Some(include_paths) = self.extract_includes(&value) {
            // Remove $include key from value
            if let Value::Object(ref mut obj) = value {
                obj.remove(INCLUDE_KEY);
            }

            let base_dir = source_file.parent().unwrap_or(Path::new("."));

            // Start with empty object, merge includes in order
            let mut merged = Value::Object(Object::new());
            for include_path in include_paths {
                let resolved_path = if include_path.is_absolute() {
                    include_path
                } else {
                    base_dir.join(&include_path)
                };

                let included_value = self.load_file(&resolved_path, source_file)?;
                merged.merge(included_value);
            }

            // Finally merge the current file's content on top
            merged.merge(value);
            return Ok(merged);
        }

        Ok(value)
    }

    /// Extract include paths from a value's `$include` key.
    fn extract_includes(&self, value: &Value) -> Option<Vec<PathBuf>> {
        let obj = value.as_object()?;
        let include_value = obj.get(INCLUDE_KEY)?;

        match include_value {
            // Single include: $include: "./file.yaml"
            Value::String(s) => Some(vec![PathBuf::from(s)]),
            // Multiple includes: $include: ["./a.yaml", "./b.yaml"]
            Value::Array(arr) => {
                let paths: Vec<PathBuf> = arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(PathBuf::from)
                    .collect();
                if paths.is_empty() { None } else { Some(paths) }
            }
            _ => None,
        }
    }

    /// Load and parse a file, then recursively resolve its includes.
    fn load_file(&mut self, path: &Path, source_file: &Path) -> Result<Value, ConfigError> {
        if !path.exists() {
            return Err(ConfigError::include_not_found(
                path.display().to_string(),
                source_file.display().to_string(),
            ));
        }

        let content = std::fs::read_to_string(path)?;
        let format = infer_format(path);
        let value = parse_content(&content, format)?;

        // Recursively resolve includes in the loaded file
        self.resolve(value, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_file(dir: &Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, content).unwrap();
        path
    }

    fn get_key<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
        value.as_object().and_then(|obj| obj.get(key))
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_basic_include() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        create_test_file(dir, "base.yaml", "concurrency: 4\nbatch_size: 32");
        let main_path = create_test_file(
            dir,
            "main.yaml",
            "$include: ./base.yaml\nlayers:\n  score:\n    threshold: 0.7",
        );

        let content = fs::read_to_string(&main_path).unwrap();
        let value = parse_content(&content, Format::Yaml).unwrap();

        let mut resolver = IncludeResolver::new();
        let result = resolver.resolve(value, &main_path).unwrap();

        assert_eq!(
            get_key(&result, "concurrency").and_then(|v| v.as_int()),
            Some(4)
        );
        assert_eq!(
            get_key(&result, "batch_size").and_then(|v| v.as_int()),
            Some(32)
        );
        assert!(get_key(&result, "layers").is_some());
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_multiple_includes() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        create_test_file(dir, "a.yaml", "key_a: 1");
        create_test_file(dir, "b.yaml", "key_b: 2");
        let main_path = create_test_file(
            dir,
            "main.yaml",
            "$include:\n  - ./a.yaml\n  - ./b.yaml\nkey_main: 3",
        );

        let content = fs::read_to_string(&main_path).unwrap();
        let value = parse_content(&content, Format::Yaml).unwrap();

        let mut resolver = IncludeResolver::new();
        let result = resolver.resolve(value, &main_path).unwrap();

        assert_eq!(get_key(&result, "key_a").and_then(|v| v.as_int()), Some(1));
        assert_eq!(get_key(&result, "key_b").and_then(|v| v.as_int()), Some(2));
        assert_eq!(
            get_key(&result, "key_main").and_then(|v| v.as_int()),
            Some(3)
        );
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_nested_includes() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        create_test_file(dir, "c.yaml", "key_c: 3");
        create_test_file(dir, "b.yaml", "$include: ./c.yaml\nkey_b: 2");
        create_test_file(dir, "a.yaml", "$include: ./b.yaml\nkey_a: 1");
        let main_path = create_test_file(dir, "main.yaml", "$include: ./a.yaml\nkey_main: 0");

        let content = fs::read_to_string(&main_path).unwrap();
        let value = parse_content(&content, Format::Yaml).unwrap();

        let mut resolver = IncludeResolver::new();
        let result = resolver.resolve(value, &main_path).unwrap();

        assert_eq!(get_key(&result, "key_c").and_then(|v| v.as_int()), Some(3));
        assert_eq!(get_key(&result, "key_b").and_then(|v| v.as_int()), Some(2));
        assert_eq!(get_key(&result, "key_a").and_then(|v| v.as_int()), Some(1));
        assert_eq!(
            get_key(&result, "key_main").and_then(|v| v.as_int()),
            Some(0)
        );
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_circular_include_direct() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        let main_path = create_test_file(dir, "main.yaml", "$include: ./main.yaml\nkey: 1");

        let content = fs::read_to_string(&main_path).unwrap();
        let value = parse_content(&content, Format::Yaml).unwrap();

        let mut resolver = IncludeResolver::new();
        let result = resolver.resolve(value, &main_path);

        assert!(result.is_err());
        assert!(result.unwrap_err().is_circular_include());
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_circular_include_indirect() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        create_test_file(dir, "a.yaml", "$include: ./b.yaml\nkey_a: 1");
        create_test_file(dir, "b.yaml", "$include: ./a.yaml\nkey_b: 2");
        let main_path = create_test_file(dir, "main.yaml", "$include: ./a.yaml\nkey_main: 0");

        let content = fs::read_to_string(&main_path).unwrap();
        let value = parse_content(&content, Format::Yaml).unwrap();

        let mut resolver = IncludeResolver::new();
        let result = resolver.resolve(value, &main_path);

        assert!(result.is_err());
        assert!(result.unwrap_err().is_circular_include());
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_include_not_found() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        let main_path = create_test_file(dir, "main.yaml", "$include: ./missing.yaml\nkey: 1");

        let content = fs::read_to_string(&main_path).unwrap();
        let value = parse_content(&content, Format::Yaml).unwrap();

        let mut resolver = IncludeResolver::new();
        let result = resolver.resolve(value, &main_path);

        assert!(result.is_err());
        assert!(result.unwrap_err().is_include_not_found());
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_override_behavior() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        create_test_file(dir, "base.yaml", "key: base\nbase_only: true");
        let main_path = create_test_file(dir, "main.yaml", "$include: ./base.yaml\nkey: override");

        let content = fs::read_to_string(&main_path).unwrap();
        let value = parse_content(&content, Format::Yaml).unwrap();

        let mut resolver = IncludeResolver::new();
        let result = resolver.resolve(value, &main_path).unwrap();

        // Main file overrides base
        assert_eq!(
            get_key(&result, "key").and_then(|v| v.as_str()),
            Some("override")
        );
        // Base-only key is preserved
        assert_eq!(
            get_key(&result, "base_only").and_then(|v| v.as_bool()),
            Some(true)
        );
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_no_include() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        let main_path = create_test_file(dir, "main.yaml", "key: value\nother: 42");

        let content = fs::read_to_string(&main_path).unwrap();
        let value = parse_content(&content, Format::Yaml).unwrap();

        let mut resolver = IncludeResolver::new();
        let result = resolver.resolve(value.clone(), &main_path).unwrap();

        // Value unchanged when no $include
        assert_eq!(
            get_key(&result, "key").and_then(|v| v.as_str()),
            Some("value")
        );
        assert_eq!(get_key(&result, "other").and_then(|v| v.as_int()), Some(42));
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_deep_merge() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path();

        create_test_file(
            dir,
            "base.yaml",
            "database:\n  host: localhost\n  port: 5432",
        );
        let main_path = create_test_file(
            dir,
            "main.yaml",
            "$include: ./base.yaml\ndatabase:\n  host: production",
        );

        let content = fs::read_to_string(&main_path).unwrap();
        let value = parse_content(&content, Format::Yaml).unwrap();

        let mut resolver = IncludeResolver::new();
        let result = resolver.resolve(value, &main_path).unwrap();

        let db = get_key(&result, "database").unwrap();
        // Host overridden
        assert_eq!(
            get_key(db, "host").and_then(|v| v.as_str()),
            Some("production")
        );
        // Port preserved from base
        assert_eq!(get_key(db, "port").and_then(|v| v.as_int()), Some(5432));
    }
}
