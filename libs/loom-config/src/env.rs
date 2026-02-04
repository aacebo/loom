use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Env {
    Dev,
    Stage,
    Prod,
    #[serde(untagged)]
    Custom(Cow<'static, str>),
}

impl Default for Env {
    fn default() -> Self {
        Env::Prod
    }
}

impl Env {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dev" | "development" => Env::Dev,
            "stage" | "staging" => Env::Stage,
            "prod" | "production" => Env::Prod,
            other => Env::Custom(Cow::Owned(other.to_string())),
        }
    }

    pub fn from_env() -> Self {
        std::env::var("ENV")
            .or_else(|_| std::env::var("ENVIRONMENT"))
            .map(|s| Self::from_str(&s))
            .unwrap_or_default()
    }

    pub fn is_dev(&self) -> bool {
        matches!(self, Env::Dev)
    }

    pub fn is_stage(&self) -> bool {
        matches!(self, Env::Stage)
    }

    pub fn is_prod(&self) -> bool {
        matches!(self, Env::Prod)
    }

    pub fn is_custom(&self) -> bool {
        matches!(self, Env::Custom(_))
    }
}

impl std::fmt::Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Env::Dev => write!(f, "dev"),
            Env::Stage => write!(f, "stage"),
            Env::Prod => write!(f, "prod"),
            Env::Custom(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_prod() {
        assert_eq!(Env::default(), Env::Prod);
    }

    #[test]
    fn test_from_str_dev() {
        assert_eq!(Env::from_str("dev"), Env::Dev);
        assert_eq!(Env::from_str("DEV"), Env::Dev);
        assert_eq!(Env::from_str("development"), Env::Dev);
        assert_eq!(Env::from_str("DEVELOPMENT"), Env::Dev);
    }

    #[test]
    fn test_from_str_stage() {
        assert_eq!(Env::from_str("stage"), Env::Stage);
        assert_eq!(Env::from_str("STAGE"), Env::Stage);
        assert_eq!(Env::from_str("staging"), Env::Stage);
        assert_eq!(Env::from_str("STAGING"), Env::Stage);
    }

    #[test]
    fn test_from_str_prod() {
        assert_eq!(Env::from_str("prod"), Env::Prod);
        assert_eq!(Env::from_str("PROD"), Env::Prod);
        assert_eq!(Env::from_str("production"), Env::Prod);
        assert_eq!(Env::from_str("PRODUCTION"), Env::Prod);
    }

    #[test]
    fn test_from_str_custom() {
        assert_eq!(
            Env::from_str("test"),
            Env::Custom(Cow::Owned("test".to_string()))
        );
        assert_eq!(
            Env::from_str("local"),
            Env::Custom(Cow::Owned("local".to_string()))
        );
    }

    #[test]
    fn test_is_methods() {
        assert!(Env::Dev.is_dev());
        assert!(!Env::Dev.is_stage());
        assert!(!Env::Dev.is_prod());
        assert!(!Env::Dev.is_custom());

        assert!(Env::Stage.is_stage());
        assert!(Env::Prod.is_prod());
        assert!(Env::Custom(Cow::Borrowed("test")).is_custom());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Env::Dev), "dev");
        assert_eq!(format!("{}", Env::Stage), "stage");
        assert_eq!(format!("{}", Env::Prod), "prod");
        assert_eq!(format!("{}", Env::Custom(Cow::Borrowed("test"))), "test");
    }
}
