use std::io;
use std::sync::Arc;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error(transparent)]
pub enum Error {
    IOError(Arc<io::Error>),
    #[error("config does not exist")]
    ConfigMissing,
    #[error("JSON parsing error: {0}")]
    SerdeJSONError(Arc<serde_json::Error>),
    #[error("TOML parsing error: {0}")]
    SerdeTOMLError(#[from] toml::de::Error),
    FormatError(Arc<rust_format::Error>),
    #[error("the element tree contains no matching element")]
    NonExistentElement,
    #[error(
        "the file dialog has been closed without selecting a valid option"
    )]
    DialogClosed,
    #[error("{0}")]
    Other(String),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(Arc::new(value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJSONError(Arc::new(value))
    }
}

impl From<rust_format::Error> for Error {
    fn from(value: rust_format::Error) -> Self {
        Self::FormatError(Arc::new(value))
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
