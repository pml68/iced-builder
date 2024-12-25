pub mod dialogs;
pub mod error;
pub mod icon;
pub mod panes;
pub mod types;

pub use error::Error;
pub type Result<T> = core::result::Result<T, Error>;
