use std::io;
use std::sync::Arc;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error(transparent)]
pub enum Error {
    IO(Arc<io::Error>),
    #[error("config does not exist")]
    ConfigMissing,
    #[error("JSON parsing error: {0}")]
    SerdeJSON(Arc<serde_json::Error>),
    #[error("TOML parsing error: {0}")]
    SerdeTOML(#[from] toml::de::Error),
    RustFmt(Arc<rust_format::Error>),
    #[error("the element tree contains no matching element")]
    NonExistentElement,
    #[error("the file dialog has been closed without selecting a valid option")]
    DialogClosed,
    #[error("{0}")]
    Other(String),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(Arc::new(value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJSON(Arc::new(value))
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
