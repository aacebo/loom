#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Type {
    Event,
}

impl Type {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Event => "event",
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
