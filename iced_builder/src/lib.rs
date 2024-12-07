pub mod dialogs;
pub mod error;
pub mod icon;
pub mod types;
pub mod views;

pub use error::Error;
pub type Result<T> = core::result::Result<T, Error>;
