use rfd::{MessageButtons, MessageDialog, MessageLevel};
use std::io;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("An I/O error accured: {0}")]
    IOError(String),
    #[error("A Serde error accured: {0}")]
    SerdeError(String),
    #[error("A RustFmt error accured: {0}")]
    FormatError(String),
    #[error("The element tree contains no matching element")]
    NonExistentElement,
    #[error("The file dialog has been closed without selecting a valid option")]
    DialogClosed,
    #[error("{0}")]
    Other(String),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value.to_string())
    }
}

impl From<rust_format::Error> for Error {
    fn from(value: rust_format::Error) -> Self {
        Self::FormatError(value.to_string())
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
