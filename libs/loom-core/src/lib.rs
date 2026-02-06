mod format;
mod id;
mod map;
mod media_type;
pub mod path;
pub mod value;

pub use format::*;
pub use id::*;
pub use map::*;
pub use media_type::*;

/// Encode a value to a string in the specified format.
///
/// # Usage
/// ```ignore
/// // Encode using explicit format
/// let json = encode!(&data; json)?;
/// let yaml = encode!(&data; yaml)?;
/// let toml = encode!(&data; toml)?;
///
/// // Encode using Format enum (runtime dispatch)
/// let s = encode!(&data, Format::Json)?;
/// ```
#[macro_export]
macro_rules! encode {
    // Explicit format variants (compile-time dispatch)
    ($value:expr; json) => {{
        #[cfg(feature = "json")]
        {
            ::serde_json::to_string_pretty($value)
        }
        #[cfg(not(feature = "json"))]
        {
            compile_error!("json feature not enabled")
        }
    }};
    ($value:expr; yaml) => {{
        #[cfg(feature = "yaml")]
        {
            ::serde_saphyr::to_string($value)
        }
        #[cfg(not(feature = "yaml"))]
        {
            compile_error!("yaml feature not enabled")
        }
    }};
    ($value:expr; toml) => {{
        #[cfg(feature = "toml")]
        {
            ::toml::to_string_pretty($value)
        }
        #[cfg(not(feature = "toml"))]
        {
            compile_error!("toml feature not enabled")
        }
    }};
    // Runtime format dispatch
    ($value:expr, $format:expr) => {{
        match $format {
            #[cfg(feature = "json")]
            $crate::Format::Json => {
                ::serde_json::to_string_pretty($value).map_err(|e| e.to_string())
            }
            #[cfg(feature = "yaml")]
            $crate::Format::Yaml => ::serde_saphyr::to_string($value).map_err(|e| e.to_string()),
            #[cfg(feature = "toml")]
            $crate::Format::Toml => ::toml::to_string_pretty($value).map_err(|e| e.to_string()),
            #[allow(unreachable_patterns)]
            _ => Err(format!("Unsupported format: {:?}", $format)),
        }
    }};
}

/// Decode a string to a value in the specified format.
///
/// # Usage
/// ```ignore
/// // Decode using explicit format
/// let data: MyType = decode!(content; json)?;
/// let data: MyType = decode!(content; yaml)?;
/// let data: MyType = decode!(content; toml)?;
///
/// // Decode using Format enum (runtime dispatch)
/// let data: MyType = decode!(content, Format::Json)?;
/// ```
#[macro_export]
macro_rules! decode {
    // Explicit format variants (compile-time dispatch)
    ($value:expr; json) => {{
        #[cfg(feature = "json")]
        {
            ::serde_json::from_str($value)
        }
        #[cfg(not(feature = "json"))]
        {
            compile_error!("json feature not enabled")
        }
    }};
    ($value:expr; yaml) => {{
        #[cfg(feature = "yaml")]
        {
            ::serde_saphyr::from_str($value)
        }
        #[cfg(not(feature = "yaml"))]
        {
            compile_error!("yaml feature not enabled")
        }
    }};
    ($value:expr; toml) => {{
        #[cfg(feature = "toml")]
        {
            ::toml::from_str($value)
        }
        #[cfg(not(feature = "toml"))]
        {
            compile_error!("toml feature not enabled")
        }
    }};
    // Runtime format dispatch
    ($value:expr, $format:expr) => {{
        match $format {
            #[cfg(feature = "json")]
            $crate::Format::Json => ::serde_json::from_str($value).map_err(|e| e.to_string()),
            #[cfg(feature = "yaml")]
            $crate::Format::Yaml => ::serde_saphyr::from_str($value).map_err(|e| e.to_string()),
            #[cfg(feature = "toml")]
            $crate::Format::Toml => ::toml::from_str($value).map_err(|e| e.to_string()),
            #[allow(unreachable_patterns)]
            _ => Err(format!("Unsupported format: {:?}", $format)),
        }
    }};
}
