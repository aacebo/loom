pub use super::error::IdentPathError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct IdentPath(Vec<IdentSegment>);

impl IdentPath {
    pub fn parse(input: &str) -> Result<Self, IdentPathError> {
        let s = input.trim();

        if s.is_empty() {
            return Err(IdentPathError::Empty);
        }

        let mut segments = Vec::new();
        let mut chars = s.chars().peekable();
        let mut first = true;

        while let Some(segment) = IdentSegment::parse_next(&mut chars, !first)? {
            segments.push(segment);
            first = false;
        }

        if segments.is_empty() {
            return Err(IdentPathError::Empty);
        }

        Ok(Self(segments))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn segments(&self) -> &[IdentSegment] {
        &self.0
    }
}

impl std::fmt::Display for IdentPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, segment) in self.0.iter().enumerate() {
            match segment {
                IdentSegment::Key(v) if i == 0 => write!(f, "{}", v)?,
                IdentSegment::Key(v) => write!(f, ".{}", v)?,
                IdentSegment::Index(v) => write!(f, "[{}]", v)?,
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum IdentSegment {
    Key(String),
    Index(usize),
}

impl IdentSegment {
    fn parse_next(
        chars: &mut std::iter::Peekable<std::str::Chars>,
        expect_separator: bool,
    ) -> Result<Option<Self>, IdentPathError> {
        if expect_separator {
            match chars.peek() {
                None => return Ok(None),
                Some(&'.') => {
                    chars.next();
                    if chars.peek().is_none() {
                        return Err(IdentPathError::EmptySegment);
                    }
                }
                Some(&'[') => {}
                Some(&']') => return Err(IdentPathError::UnmatchedBracket),
                Some(_) => return Err(IdentPathError::EmptySegment),
            }
        }

        match chars.peek() {
            None => Ok(None),
            Some(&'.') => Err(IdentPathError::EmptySegment),
            Some(&'[') => Self::parse_index(chars).map(Some),
            Some(&']') => Err(IdentPathError::UnmatchedBracket),
            Some(_) => Self::parse_key(chars).map(Some),
        }
    }

    fn parse_key(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Self, IdentPathError> {
        let mut key = String::new();

        while let Some(&c) = chars.peek() {
            match c {
                '.' | '[' => break,
                ']' => return Err(IdentPathError::UnmatchedBracket),
                _ => {
                    key.push(c);
                    chars.next();
                }
            }
        }

        if key.is_empty() {
            return Err(IdentPathError::EmptySegment);
        }

        Ok(Self::Key(key))
    }

    fn parse_index(
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<Self, IdentPathError> {
        chars.next(); // consume '['

        let mut index = String::new();

        loop {
            match chars.next() {
                Some(']') => break,
                Some(c) => index.push(c),
                None => return Err(IdentPathError::UnmatchedBracket),
            }
        }

        if index.is_empty() {
            return Err(IdentPathError::EmptyBracket);
        }

        let value = index.parse().map_err(|_| IdentPathError::InvalidIndex)?;
        Ok(Self::Index(value))
    }
}

impl std::fmt::Display for IdentSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(v) => write!(f, ".{}", v),
            Self::Index(v) => write!(f, "[{}]", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_key() {
        let path = IdentPath::parse("object").unwrap();
        assert_eq!(path.to_string(), "object");
    }

    #[test]
    fn test_parse_dotted_path() {
        let path = IdentPath::parse("object.field").unwrap();
        assert_eq!(path.to_string(), "object.field");
    }

    #[test]
    fn test_parse_index() {
        let path = IdentPath::parse("arr[0]").unwrap();
        assert_eq!(path.to_string(), "arr[0]");
    }

    #[test]
    fn test_parse_complex() {
        let path = IdentPath::parse("object.field[2].test").unwrap();
        assert_eq!(path.to_string(), "object.field[2].test");
    }

    #[test]
    fn test_parse_consecutive_indices() {
        let path = IdentPath::parse("arr[0][1]").unwrap();
        assert_eq!(path.to_string(), "arr[0][1]");
    }

    #[test]
    fn test_parse_index_after_dot() {
        let path = IdentPath::parse("a[0].b").unwrap();
        assert_eq!(path.to_string(), "a[0].b");
    }

    #[test]
    fn test_parse_empty_error() {
        let err = IdentPath::parse("").unwrap_err();
        assert_eq!(err, IdentPathError::Empty);
    }

    #[test]
    fn test_parse_empty_segment_error() {
        let err = IdentPath::parse("a..b").unwrap_err();
        assert_eq!(err, IdentPathError::EmptySegment);
    }

    #[test]
    fn test_parse_trailing_dot_error() {
        let err = IdentPath::parse("a.").unwrap_err();
        assert_eq!(err, IdentPathError::EmptySegment);
    }

    #[test]
    fn test_parse_leading_dot_error() {
        let err = IdentPath::parse(".a").unwrap_err();
        assert_eq!(err, IdentPathError::EmptySegment);
    }

    #[test]
    fn test_parse_unmatched_open_bracket_error() {
        let err = IdentPath::parse("a[0").unwrap_err();
        assert_eq!(err, IdentPathError::UnmatchedBracket);
    }

    #[test]
    fn test_parse_unmatched_close_bracket_error() {
        let err = IdentPath::parse("a]0").unwrap_err();
        assert_eq!(err, IdentPathError::UnmatchedBracket);
    }

    #[test]
    fn test_parse_empty_bracket_error() {
        let err = IdentPath::parse("a[]").unwrap_err();
        assert_eq!(err, IdentPathError::EmptyBracket);
    }

    #[test]
    fn test_parse_invalid_index_error() {
        let err = IdentPath::parse("a[abc]").unwrap_err();
        assert_eq!(err, IdentPathError::InvalidIndex);
    }

    #[test]
    fn test_display_roundtrip() {
        let inputs = [
            "object",
            "object.field",
            "arr[0]",
            "object.field[2].test",
            "arr[0][1]",
            "a[0].b",
        ];

        for input in inputs {
            let path = IdentPath::parse(input).unwrap();
            assert_eq!(path.to_string(), input);
        }
    }
}
