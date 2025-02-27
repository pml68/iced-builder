use std::num::ParseFloatError;
use std::str::FromStr;

use iced::{Radians, Rotation};

use super::Value;

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum ParseRotationError {
    #[error("float parsing error: {0}")]
    ParseFloatError(ParseFloatError),
    #[error("invalid prefix")]
    InvalidPrefix,
    #[error("cannot parse rotation from empty string")]
    Empty,
}

impl From<ParseFloatError> for ParseRotationError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl Value for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err(ParseRotationError::Empty);
        }

        if s.starts_with(|c: char| !c.is_ascii_digit()) {
            let (prefix, value) = s.split_at(1);
            match prefix.to_lowercase().as_str() {
                "s" => Ok(Rotation::Solid(Radians(f32::from_str(value)?))),
                "f" => Ok(Rotation::Floating(Radians(f32::from_str(value)?))),
                _ => Err(ParseRotationError::InvalidPrefix),
            }
        } else {
            Ok(Rotation::Floating(Radians(f32::from_str(s)?)))
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Floating(value) => format!("f{}", value),
            Self::Solid(value) => format!("s{}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_without_prefix() {
        assert_eq!(
            Rotation::from_str("10.5"),
            Ok(Rotation::Floating(Radians(10.5)))
        )
    }

    #[test]
    fn can_parse_with_s_prefix() {
        assert_eq!(
            Rotation::from_str("s12.3"),
            Ok(Rotation::Solid(Radians(12.3)))
        )
    }

    #[test]
    fn can_parse_with_f_prefix() {
        assert_eq!(
            Rotation::from_str("f16.9"),
            Ok(Rotation::Floating(Radians(16.9)))
        )
    }

    #[test]
    fn can_parse_with_uppercase_prefix() {
        assert_eq!(
            Rotation::from_str("S9.4"),
            Ok(Rotation::Solid(Radians(9.4)))
        )
    }

    #[test]
    fn cant_parse_invalid_prefix() {
        assert_eq!(
            Rotation::from_str("a6.0"),
            Err(ParseRotationError::InvalidPrefix)
        )
    }

    #[test]
    fn cant_parse_invalid_float() {
        assert_eq!(
            Rotation::from_str("3.a"),
            Err(ParseRotationError::ParseFloatError(
                f32::from_str("3.a").expect_err("float parse should fail")
            ))
        )
    }

    #[test]
    fn cant_parse_empty_string() {
        assert_eq!(Rotation::from_str(" "), Err(ParseRotationError::Empty))
    }
}
