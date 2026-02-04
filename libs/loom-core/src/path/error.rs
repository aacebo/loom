#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentPathError {
    Empty,
    UnmatchedBracket,
    EmptyBracket,
    EmptySegment,
    InvalidIndex,
}

impl std::fmt::Display for IdentPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "empty ident path"),
            Self::UnmatchedBracket => write!(f, "unmatched bracket"),
            Self::EmptyBracket => write!(f, "empty bracket"),
            Self::EmptySegment => write!(f, "empty segment"),
            Self::InvalidIndex => write!(f, "invalid index"),
        }
    }
}

impl std::error::Error for IdentPathError {}
