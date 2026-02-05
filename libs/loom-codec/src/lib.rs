mod error;
mod registry;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "yaml")]
mod yaml;

#[cfg(feature = "toml")]
mod toml;

mod text;

pub use error::*;
pub use registry::*;

#[cfg(feature = "json")]
pub use json::*;

#[cfg(feature = "yaml")]
pub use yaml::*;

#[cfg(feature = "toml")]
pub use toml::*;

pub use text::*;

// Re-export types from dependencies
pub use loom_core::{Format, MediaType, path, value};
pub use loom_io::{Document, Entity, Record};

pub trait Codec: Send + Sync {
    fn format(&self) -> Format;
    fn decode(&self, record: Record) -> Result<Document, CodecError>;
    fn encode(&self, document: Document) -> Result<Record, CodecError>;
}

#[macro_export]
macro_rules! encode {
    ($value:expr) => {{
        if cfg!(feature = "json") {
            serde_json::to_string($value).expect("json")
        } else if cfg!(feature = "yaml") {
            serde_saphyr::to_string($value).expect("yaml")
        } else if cfg!(feature = "toml") {
            toml::to_string($value).expect("toml")
        } else {
            panic!("no encoder found");
        }
    }};
    ($value:expr; json) => {
        serde_json::to_string($value).expect("json")
    };
    ($value:expr; yaml) => {
        serde_saphyr::to_string($value).expect("yaml")
    };
    ($value:expr; toml) => {
        toml::to_string($value).expect("toml")
    };
}

#[macro_export]
macro_rules! decode {
    ($value:expr) => {{
        if cfg!(feature = "json") {
            serde_json::from_str($value).expect("json")
        } else if cfg!(feature = "yaml") {
            serde_saphyr::from_str($value).expect("yaml")
        } else if cfg!(feature = "toml") {
            toml::from_str($value).expect("toml")
        } else {
            panic!("no encoder found");
        }
    }};
    ($value:expr; json) => {
        serde_json::from_str($value).expect("json")
    };
    ($value:expr; yaml) => {
        serde_saphyr::from_str($value).expect("yaml")
    };
    ($value:expr; toml) => {
        toml::from_str($value).expect("toml")
    };
}

#[cfg(test)]
mod tests {
    #[derive(serde::Serialize, serde::Deserialize)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_encode_json() {
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        debug_assert_eq!(encode!(&data; json), "{\"name\":\"test\",\"value\":42}");
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_encode_yaml() {
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        debug_assert_eq!(encode!(&data; yaml), "name: test\nvalue: 42\n");
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_encode_toml() {
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        debug_assert_eq!(encode!(&data; toml), "name = \"test\"\nvalue = 42\n");
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_decode_json() {
        let json = r#"{"name":"test","value":42}"#;
        let data: TestData = decode!(json; json);

        debug_assert_eq!(data.name, "test");
        debug_assert_eq!(data.value, 42);
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_decode_yaml() {
        let yaml = "name: test\nvalue: 42\n";
        let data: TestData = decode!(yaml; yaml);

        debug_assert_eq!(data.name, "test");
        debug_assert_eq!(data.value, 42);
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_decode_toml() {
        let toml = "name = \"test\"\nvalue = 42\n";
        let data: TestData = decode!(toml; toml);

        debug_assert_eq!(data.name, "test");
        debug_assert_eq!(data.value, 42);
    }
}
