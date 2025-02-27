use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use iced::Length;

use super::Value;

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum ParseLengthError {
    #[error("float parsing error: {0}")]
    ParseFloatError(ParseFloatError),
    #[error("int parsing error: {0}")]
    ParseIntError(ParseIntError),
    #[error("invalid type")]
    InvalidType,
    #[error("invalid prefix")]
    InvalidPrefix,
    #[error("missing prefix")]
    MissingPrefix,
    #[error("cannot parse length from empty string")]
    Empty,
}

impl From<ParseFloatError> for ParseLengthError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl From<ParseIntError> for ParseLengthError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Value for Length {
    type Err = ParseLengthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err(ParseLengthError::Empty);
        }

        if !s.contains(|c: char| c.is_ascii_digit()) {
            match s {
                "fill" => Ok(Self::Fill),
                "shrink" => Ok(Self::Shrink),
                _ => Err(ParseLengthError::InvalidType),
            }
        } else {
            if s.starts_with(|c: char| !c.is_alphabetic()) {
                return Err(ParseLengthError::MissingPrefix);
            }

            let (prefix, value) = s.split_at(2);
            match prefix.to_lowercase().as_str() {
                "fx" => Ok(Self::Fixed(f32::from_str(value)?)),
                "fp" => Ok(Self::FillPortion(u16::from_str(value)?)),
                _ => Err(ParseLengthError::InvalidPrefix),
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Fill => String::from("fill"),
            Self::Shrink => String::from("shrink"),
            Self::Fixed(value) => format!("fx{}", value),
            Self::FillPortion(value) => format!("fp{}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_fill() {
        assert_eq!(Length::from_str("fill"), Ok(Length::Fill))
    }

    #[test]
    fn can_parse_shrink_with_space() {
        assert_eq!(Length::from_str("shrink "), Ok(Length::Shrink))
    }

    #[test]
    fn can_parse_fill_portion() {
        assert_eq!(Length::from_str("fp15"), Ok(Length::FillPortion(15)))
    }

    #[test]
    fn can_parse_fixed_with_spaces() {
        assert_eq!(Length::from_str(" fx3.1 "), Ok(Length::Fixed(3.1)))
    }

    #[test]
    fn cant_parse_invalid_type() {
        assert_eq!(
            Length::from_str("fillportion"),
            Err(ParseLengthError::InvalidType)
        )
    }

    #[test]
    fn cant_parse_invalid_prefix() {
        assert_eq!(
            Length::from_str("f2.0"),
            Err(ParseLengthError::InvalidPrefix),
        )
    }

    #[test]
    fn cant_parse_invalid_float() {
        assert_eq!(
            Length::from_str(" fx2.a"),
            Err(ParseLengthError::ParseFloatError(
                f32::from_str("2.a").expect_err("float parse should fail")
            ))
        )
    }

    #[test]
    fn cant_parse_invalid_integer() {
        assert_eq!(
            Length::from_str("fp1a "),
            Err(ParseLengthError::ParseIntError(
                u16::from_str("1a").expect_err("integer parse should fail")
            ))
        )
    }

    #[test]
    fn cant_parse_with_missing_prefix() {
        assert_eq!(Length::from_str("24"), Err(ParseLengthError::MissingPrefix))
    }

    #[test]
    fn cant_parse_empty_string() {
        assert_eq!(Length::from_str(" "), Err(ParseLengthError::Empty))
    }
}
