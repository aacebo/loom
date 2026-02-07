mod error;
mod file;
mod ident;
mod uri;

pub use file::*;
pub use ident::*;
pub use uri::*;

/// Creates a `Path` from a string literal.
///
/// # Variants
///
/// - `path!(file => "path/to/file")` - Creates a `Path::File`
/// - `path!(uri => "https://example.com")` - Creates a `Path::Uri` (panics on invalid URI)
/// - `path!(ident => "object.field[0]")` - Creates a `Path::Ident` (panics on invalid field path)
///
/// # Examples
///
/// ```ignore
/// use loom_core::path;
///
/// let file = path!(file => "/home/user/file.txt");
/// let uri = path!(uri => "https://example.com/path");
/// let field = path!(ident => "data.items[0].name");
/// ```
#[macro_export]
macro_rules! path {
    (file => $path:expr) => {
        $crate::path::Path::from($crate::file_path!($path))
    };
    (uri => $path:expr) => {
        $crate::path::Path::from($crate::uri_path!($path))
    };
    (ident => $path:expr) => {
        $crate::path::Path::from($crate::ident_path!($path))
    };
}

#[macro_export]
macro_rules! file_path {
    ($path:expr) => {
        $crate::path::FilePath::parse($path)
    };
}

#[macro_export]
macro_rules! uri_path {
    ($path:expr) => {
        $crate::path::UriPath::parse($path).expect("invalid URI")
    };
}

#[macro_export]
macro_rules! ident_path {
    ($path:expr) => {
        $crate::path::IdentPath::parse($path).expect("invalid ident path")
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Path {
    Empty,
    File(FilePath),
    Uri(UriPath),
    Ident(IdentPath),
}

impl Path {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    pub fn is_uri(&self) -> bool {
        matches!(self, Self::Uri(_))
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, Self::Ident(_))
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::Empty
    }
}

impl From<FilePath> for Path {
    fn from(value: FilePath) -> Self {
        Self::File(value)
    }
}

impl From<UriPath> for Path {
    fn from(value: UriPath) -> Self {
        Self::Uri(value)
    }
}

impl From<IdentPath> for Path {
    fn from(value: IdentPath) -> Self {
        Self::Ident(value)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "empty"),
            Self::File(v) => write!(f, "{}", v),
            Self::Uri(v) => write!(f, "{}", v),
            Self::Ident(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_macro_file() {
        let path = path!(file => "/home/user/file.txt");
        assert!(path.is_file());
        assert_eq!(path.to_string(), "/home/user/file.txt");
    }

    #[test]
    fn test_path_macro_uri() {
        let path = path!(uri => "https://example.com/path");
        assert!(path.is_uri());
        assert_eq!(path.to_string(), "https://example.com/path");
    }

    #[test]
    fn test_path_macro_field() {
        let path = path!(ident => "object.field[0]");
        assert!(path.is_ident());
        assert_eq!(path.to_string(), "object.field[0]");
    }

    #[test]
    fn test_path_is_file() {
        let path = Path::File(FilePath::parse("/home/user/file.txt"));
        assert!(path.is_file());
        assert!(!path.is_uri());
        assert!(!path.is_ident());
    }

    #[test]
    fn test_path_is_uri() {
        let path = Path::Uri(UriPath::parse("https://example.com").unwrap());
        assert!(!path.is_file());
        assert!(path.is_uri());
        assert!(!path.is_ident());
    }

    #[test]
    fn test_path_is_ident() {
        let path = Path::Ident(IdentPath::parse("object.field").unwrap());
        assert!(!path.is_file());
        assert!(!path.is_uri());
        assert!(path.is_ident());
    }

    #[test]
    fn test_path_display_file() {
        let path = Path::File(FilePath::parse("/home/user/file.txt"));
        assert_eq!(path.to_string(), "/home/user/file.txt");
    }

    #[test]
    fn test_path_display_ident() {
        let path = Path::Ident(IdentPath::parse("object.field[0]").unwrap());
        assert_eq!(path.to_string(), "object.field[0]");
    }
}
