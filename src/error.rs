use std::borrow::Cow;
use std::io;
use std::sync::Arc;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error(transparent)]
pub enum Error {
    Io(Arc<io::Error>),
    #[error("Config file does not exist, so a default one was created")]
    ConfigMissing,
    #[error("JSON parsing error: {0}")]
    Json(Arc<serde_json::Error>),
    #[error("TOML parsing error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
    RustFmt(Arc<rust_format::Error>),
    #[error("The element tree contains no matching element")]
    NonExistentElement,
    #[error("The file dialog has been closed without selecting a valid option")]
    DialogClosed,
    #[error("{0}")]
    Other(String),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(Arc::new(value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(Arc::new(value))
    }
}

impl From<rust_format::Error> for Error {
    fn from(value: rust_format::Error) -> Self {
        Self::RustFmt(Arc::new(value))
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_owned())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        value.to_string()
    }
}

impl From<Error> for Cow<'static, str> {
    fn from(value: Error) -> Self {
        value.to_string().into()
    }
}
