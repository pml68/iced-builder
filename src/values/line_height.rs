use std::num::ParseFloatError;
use std::str::FromStr;

use iced::Pixels;
use iced::advanced::text::LineHeight;

use super::Value;

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum ParseLineHeightError {
    #[error("float parsing error: {0}")]
    ParseFloatError(ParseFloatError),
    #[error("missing prefix")]
    MissingPrefix,
    #[error("invalid prefix")]
    InvalidPrefix,
    #[error("cannot parse line height from empty string")]
    Empty,
}

impl From<ParseFloatError> for ParseLineHeightError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl Value for LineHeight {
    type Err = ParseLineHeightError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err(ParseLineHeightError::Empty);
        }

        if s.starts_with(|c: char| !c.is_ascii_digit()) {
            let (prefix, value) = s.split_at(1);
            match prefix.to_lowercase().as_str() {
                "r" => Ok(Self::Relative(f32::from_str(value)?)),
                "a" => Ok(Self::Absolute(Pixels::from_str(value)?)),
                _ => Err(ParseLineHeightError::InvalidPrefix),
            }
        } else {
            Err(ParseLineHeightError::MissingPrefix)
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Relative(value) => format!("r{}", value),
            Self::Absolute(value) => format!("a{}", value.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_with_r_prefix() {
        assert_eq!(
            LineHeight::from_str("  r3.2"),
            Ok(LineHeight::Relative(3.2))
        );

        assert_eq!(
            LineHeight::from_str(" R6.5 "),
            Ok(LineHeight::Relative(6.5))
        )
    }

    #[test]
    fn can_parse_with_a_prefix() {
        assert_eq!(
            LineHeight::from_str("a9.4 "),
            Ok(LineHeight::Absolute(Pixels(9.4)))
        );

        assert_eq!(
            LineHeight::from_str("A1.3"),
            Ok(LineHeight::Absolute(Pixels(1.3)))
        )
    }

    #[test]
    fn cant_parse_with_missing_prefix() {
        assert_eq!(
            LineHeight::from_str("5.1"),
            Err(ParseLineHeightError::MissingPrefix)
        )
    }

    #[test]
    fn cant_parse_invalid_prefix() {
        assert_eq!(
            LineHeight::from_str("g21"),
            Err(ParseLineHeightError::InvalidPrefix)
        )
    }

    #[test]
    fn cant_parse_invalid_float() {
        assert_eq!(
            LineHeight::from_str("a2f"),
            Err(ParseLineHeightError::ParseFloatError(
                f32::from_str("2f").expect_err("float parse should fail")
            ))
        )
    }

    #[test]
    fn cant_parse_empty_string() {
        assert_eq!(LineHeight::from_str(" "), Err(ParseLineHeightError::Empty))
    }
}
