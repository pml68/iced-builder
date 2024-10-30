pub mod error;
pub mod types;
pub mod views;

pub type Result<T> = std::result::Result<T, error::Error>;
