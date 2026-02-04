mod builder;
mod config;
mod env;
mod error;
pub mod providers;
mod section;

pub use builder::*;
pub use config::*;
pub use env::*;
pub use error::*;
pub use providers::{EnvProvider, FileProvider, MemoryProvider, Provider};
pub use section::*;

#[macro_export]
macro_rules! get {
    ($config:expr, $path:expr) => {{
        ::loom_core::path::IdentPath::parse($path)
            .ok()
            .and_then(|p| $config.get_str(&p))
    }};
    ($config:expr, $path:expr, int) => {{
        ::loom_core::path::IdentPath::parse($path)
            .ok()
            .and_then(|p| $config.get_int(&p))
    }};
    ($config:expr, $path:expr, float) => {{
        ::loom_core::path::IdentPath::parse($path)
            .ok()
            .and_then(|p| $config.get_float(&p))
    }};
    ($config:expr, $path:expr, bool) => {{
        ::loom_core::path::IdentPath::parse($path)
            .ok()
            .and_then(|p| $config.get_bool(&p))
    }};
    ($config:expr, $path:expr, value) => {{
        ::loom_core::path::IdentPath::parse($path)
            .ok()
            .and_then(|p| $config.get(&p))
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_str() {
        let config = Config::new()
            .with_provider(MemoryProvider::from_pairs([("database.host", "localhost")]))
            .build()
            .unwrap();

        assert_eq!(crate::get!(config, "database.host"), Some("localhost"));
    }

    #[test]
    fn test_get_int() {
        let config = Config::new()
            .with_provider(MemoryProvider::from_pairs([("database.port", 5432i64)]))
            .build()
            .unwrap();

        assert_eq!(crate::get!(config, "database.port", int), Some(5432));
    }

    #[test]
    fn test_get_bool() {
        let config = Config::new()
            .with_provider(MemoryProvider::from_pairs([("debug", true)]))
            .build()
            .unwrap();

        assert_eq!(crate::get!(config, "debug", bool), Some(true));
    }

    #[test]
    fn test_get_float() {
        let config = Config::new()
            .with_provider(MemoryProvider::from_pairs([("rate", 3.14f64)]))
            .build()
            .unwrap();

        assert_eq!(crate::get!(config, "rate", float), Some(3.14));
    }

    #[test]
    fn test_get_value() {
        let config = Config::new()
            .with_provider(MemoryProvider::from_pairs([("data", "test")]))
            .build()
            .unwrap();

        assert!(crate::get!(config, "data", value).is_some());
    }

    #[test]
    fn test_get_nonexistent() {
        let config = Config::new().build().unwrap();
        assert_eq!(crate::get!(config, "nonexistent"), None);
    }
}
