use serde::de::DeserializeOwned;

use loom_core::path::{IdentPath, IdentSegment};
use loom_core::value::Value;

use super::ConfigError;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ConfigSection {
    value: Value,
    path: IdentPath,
}

impl ConfigSection {
    pub(crate) fn new(value: Value, path: IdentPath) -> Self {
        Self { value, path }
    }

    pub(crate) fn root(value: Value) -> Self {
        Self {
            value,
            path: IdentPath::parse("root").expect("valid path"),
        }
    }

    pub fn path(&self) -> &IdentPath {
        &self.path
    }

    pub fn exists(&self) -> bool {
        !self.value.is_null()
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn is_object(&self) -> bool {
        self.value.is_object()
    }

    pub fn is_array(&self) -> bool {
        self.value.is_array()
    }

    pub fn get(&self, path: &IdentPath) -> Option<&Value> {
        self.value.get_by_path(path)
    }

    pub fn get_section(&self, key: &str) -> ConfigSection {
        let child_value = match &self.value {
            Value::Object(obj) => obj.get(key).unwrap_or(&Value::Null).clone(),
            _ => Value::Null,
        };

        // Build child path
        let mut segments: Vec<IdentSegment> = self.path.segments().to_vec();
        segments.push(IdentSegment::Key(key.to_string()));

        // Create a simple path string and parse it
        let child_path_str = if self.path.to_string() == "root" {
            key.to_string()
        } else {
            format!("{}.{}", self.path, key)
        };

        let child_path = IdentPath::parse(&child_path_str).unwrap_or(self.path.clone());

        ConfigSection::new(child_value, child_path)
    }

    pub fn get_index(&self, index: usize) -> ConfigSection {
        let child_value = match &self.value {
            Value::Array(arr) => arr.get(index).cloned().unwrap_or(Value::Null),
            _ => Value::Null,
        };

        let child_path_str = if self.path.to_string() == "root" {
            format!("[{}]", index)
        } else {
            format!("{}[{}]", self.path, index)
        };

        let child_path = IdentPath::parse(&child_path_str).unwrap_or(self.path.clone());

        ConfigSection::new(child_value, child_path)
    }

    pub fn bind<T: DeserializeOwned>(&self) -> Result<T, ConfigError> {
        if self.value.is_null() {
            return Err(ConfigError::not_found(self.path.to_string()));
        }

        let json: serde_json::Value = (&self.value).into();
        serde_json::from_value(json).map_err(ConfigError::deserialize)
    }

    pub fn keys(&self) -> Option<impl Iterator<Item = &str>> {
        match &self.value {
            Value::Object(obj) => Some(obj.keys().map(|s| s.as_str())),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn children(&self) -> Vec<ConfigSection> {
        match &self.value {
            Value::Object(obj) => obj
                .iter()
                .map(|(k, v)| {
                    let child_path_str = if self.path.to_string() == "root" {
                        k.clone()
                    } else {
                        format!("{}.{}", self.path, k)
                    };
                    let child_path = IdentPath::parse(&child_path_str).unwrap_or(self.path.clone());
                    ConfigSection::new(v.clone(), child_path)
                })
                .collect(),
            Value::Array(arr) => arr
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let child_path_str = if self.path.to_string() == "root" {
                        format!("[{}]", i)
                    } else {
                        format!("{}[{}]", self.path, i)
                    };
                    let child_path = IdentPath::parse(&child_path_str).unwrap_or(self.path.clone());
                    ConfigSection::new(v.clone(), child_path)
                })
                .collect(),
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use loom_core::value::Object;

    fn create_test_config() -> Value {
        use loom_core::value::Number;

        let mut db = Object::new();
        db.insert("host".to_string(), Value::String("localhost".to_string()));
        db.insert("port".to_string(), Value::Number(Number::Int(5432)));

        let servers = vec![
            {
                let mut s = Object::new();
                s.insert("name".to_string(), Value::String("primary".to_string()));
                Value::Object(s)
            },
            {
                let mut s = Object::new();
                s.insert("name".to_string(), Value::String("secondary".to_string()));
                Value::Object(s)
            },
        ];

        let mut root = Object::new();
        root.insert("database".to_string(), Value::Object(db));
        root.insert("servers".to_string(), Value::Array(servers.into()));

        Value::Object(root)
    }

    #[test]
    fn test_section_exists() {
        let config = create_test_config();
        let section = ConfigSection::root(config);

        assert!(section.exists());
        assert!(section.get_section("database").exists());
        assert!(!section.get_section("nonexistent").exists());
    }

    #[test]
    fn test_section_is_object() {
        let config = create_test_config();
        let section = ConfigSection::root(config);

        assert!(section.is_object());
        assert!(section.get_section("database").is_object());
        assert!(!section.get_section("servers").is_object());
    }

    #[test]
    fn test_section_is_array() {
        let config = create_test_config();
        let section = ConfigSection::root(config);

        assert!(!section.is_array());
        assert!(section.get_section("servers").is_array());
    }

    #[test]
    fn test_section_get_index() {
        let config = create_test_config();
        let section = ConfigSection::root(config);
        let servers = section.get_section("servers");

        let first = servers.get_index(0);
        assert!(first.exists());
        assert!(first.is_object());

        let path = IdentPath::parse("name").unwrap();
        assert_eq!(first.get(&path).unwrap().as_str(), Some("primary"));
    }

    #[test]
    fn test_section_keys() {
        let config = create_test_config();
        let section = ConfigSection::root(config);
        let db = section.get_section("database");

        let keys: Vec<_> = db.keys().unwrap().collect();
        assert!(keys.contains(&"host"));
        assert!(keys.contains(&"port"));
    }

    #[test]
    fn test_section_len() {
        let config = create_test_config();
        let section = ConfigSection::root(config);

        assert_eq!(section.get_section("servers").len(), 2);
        assert_eq!(section.get_section("database").len(), 2);
    }

    #[test]
    fn test_section_children() {
        let config = create_test_config();
        let section = ConfigSection::root(config);
        let servers = section.get_section("servers");

        let children = servers.children();
        assert_eq!(children.len(), 2);
    }
}
