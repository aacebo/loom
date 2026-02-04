mod array;
mod number;
mod object;

pub use array::*;
pub use number::*;
pub use object::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Array),
    Object(Object),
}

impl Value {
    pub fn kind(&self) -> &str {
        match self {
            Self::Null => "null",
            Self::Bool(_) => "bool",
            Self::Number(_) => "number",
            Self::String(_) => "string",
            Self::Array(_) => "array",
            Self::Object(_) => "object",
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Array(v) => write!(f, "{}", v),
            Self::Object(v) => write!(f, "{}", v),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Self::Number(Number::Int(value as i64))
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Self::Number(Number::Int(value as i64))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Number(Number::Int(value as i64))
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Number(Number::Int(value))
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::Number(Number::Int(value as i64))
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::Number(Number::Int(value as i64))
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::Number(Number::Int(value as i64))
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::Number(Number::Int(value as i64))
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Self::Number(Number::Int(value as i64))
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self::Number(Number::Int(value as i64))
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Number(Number::Float(value as f64))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(Number::Float(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Array> for Value {
    fn from(value: Array) -> Self {
        Self::Array(value)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(value: Vec<T>) -> Self {
        Self::Array(Array::from(value))
    }
}

impl<T: Into<Value>, const N: usize> From<[T; N]> for Value {
    fn from(value: [T; N]) -> Self {
        Self::Array(Array::from(value))
    }
}
