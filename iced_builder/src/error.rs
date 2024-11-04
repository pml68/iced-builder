use rfd::{MessageButtons, MessageDialog, MessageLevel};
use std::io;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error(transparent)]
    IOError(Arc<io::Error>),
    #[error(transparent)]
    SerdeError(Arc<serde_json::Error>),
    #[error(transparent)]
    FormatError(Arc<rust_format::Error>),
    #[error("The element tree contains no matching element")]
    NonExistentElement,
    #[error("The file dialog has been closed without selecting a valid option")]
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
        Self::SerdeError(Arc::new(value))
    }
}

impl From<rust_format::Error> for Error {
    fn from(value: rust_format::Error) -> Self {
        Self::FormatError(Arc::new(value))
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::Other(value.to_owned())
    }
}

pub fn error_dialog(description: impl Into<String>) {
    MessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_buttons(MessageButtons::Ok)
        .set_title("Oops! Something went wrong.")
        .set_description(description)
        .show();
}

pub fn warning_dialog(title: impl Into<String>, description: impl Into<String>) {
    MessageDialog::new()
        .set_level(MessageLevel::Warning)
        .set_buttons(MessageButtons::Ok)
        .set_title(title)
        .set_description(description)
        .show();
}
