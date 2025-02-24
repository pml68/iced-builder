use std::num::ParseFloatError;
use std::str::FromStr;

use iced::{Radians, Rotation};

use super::ValueFromStr;

#[derive(Debug, thiserror::Error)]
pub enum ParseRotationError {
    #[error("float parsing error: {0}")]
    ParseFloatError(ParseFloatError),
    #[error("invalid prefix")]
    InvalidPrefix,
}

impl From<ParseFloatError> for ParseRotationError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl ValueFromStr for Rotation {
    type Err = ParseRotationError;

    fn value_from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(|c: char| !c.is_digit(10)) {
            let (prefix, value) = s.split_at(1);
            match prefix {
                "s" => Ok(Rotation::Solid(Radians(f32::from_str(value)?))),
                "f" => Ok(Rotation::Floating(Radians(f32::from_str(value)?))),
                _ => Err(ParseRotationError::InvalidPrefix),
            }
        } else {
            Ok(Rotation::Floating(Radians(f32::from_str(s)?)))
        }
    }
}
