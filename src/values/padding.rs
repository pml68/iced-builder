use std::num::ParseFloatError;
use std::str::FromStr;

use iced::Padding;

use super::Value;

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum ParsePaddingError {
    #[error("wrong number of values: {0}, expected 1-4")]
    WrongNumberOfValues(usize),
    #[error("float parsing error: {0}")]
    ParseFloatError(ParseFloatError),
    #[error("missing bracket")]
    MissingBracket,
    #[error("cannot parse padding from empty string")]
    Empty,
}

impl From<ParseFloatError> for ParsePaddingError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl Value for Padding {
    type Err = ParsePaddingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err(ParsePaddingError::Empty);
        }

        if !s.contains(['[', ',', ']']) {
            let value = f32::from_str(s)?;
            Ok(Padding {
                top: value,
                right: value,
                bottom: value,
                left: value,
            })
        } else {
            let values = s
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
                .ok_or(ParsePaddingError::MissingBracket)?
                .split(',')
                .map(str::trim)
                .map(f32::from_str)
                .collect::<Result<Vec<_>, _>>()?;

            match values.len() {
                1 => Ok(Padding {
                    top: values[0],
                    right: values[0],
                    bottom: values[0],
                    left: values[0],
                }),
                2 => Ok(Padding {
                    top: values[0],
                    right: values[1],
                    bottom: values[0],
                    left: values[1],
                }),
                3 => Ok(Padding {
                    top: values[0],
                    right: values[1],
                    bottom: values[2],
                    left: values[1],
                }),
                4 => Ok(Padding {
                    top: values[0],
                    right: values[1],
                    bottom: values[2],
                    left: values[3],
                }),
                other => Err(ParsePaddingError::WrongNumberOfValues(other)),
            }
        }
    }

    fn to_string(&self) -> String {
        format!(
            "[{}, {}, {}, {}]",
            self.top, self.right, self.bottom, self.left
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_single_value() {
        assert_eq!(
            Padding::from_str("[1.5]"),
            Ok(Padding {
                top: 1.5,
                right: 1.5,
                bottom: 1.5,
                left: 1.5,
            }),
        )
    }

    #[test]
    fn can_parse_single_value_without_brackets() {
        assert_eq!(
            Padding::from_str("1.5"),
            Ok(Padding {
                top: 1.5,
                right: 1.5,
                bottom: 1.5,
                left: 1.5,
            }),
        )
    }

    #[test]
    fn can_parse_two_values() {
        assert_eq!(
            Padding::from_str("[3.2, 6.7]"),
            Ok(Padding {
                top: 3.2,
                right: 6.7,
                bottom: 3.2,
                left: 6.7,
            }),
        )
    }

    #[test]
    fn can_parse_three_values() {
        assert_eq!(
            Padding::from_str("[4.8, 8.1,5.9]"),
            Ok(Padding {
                top: 4.8,
                right: 8.1,
                bottom: 5.9,
                left: 8.1,
            }),
        )
    }

    #[test]
    fn can_parse_four_values() {
        assert_eq!(
            Padding::from_str("[35.4,74.6 ,53.1, 25.0]"),
            Ok(Padding {
                top: 35.4,
                right: 74.6,
                bottom: 53.1,
                left: 25.0,
            }),
        )
    }

    #[test]
    fn cant_parse_five_values() {
        assert_eq!(
            Padding::from_str("[1,2,3,4,5]"),
            Err(ParsePaddingError::WrongNumberOfValues(5)),
        )
    }

    #[test]
    fn cant_parse_invalid_floats() {
        assert_eq!(
            Padding::from_str("[1f,2,3,4]"),
            Err(ParsePaddingError::ParseFloatError(
                f32::from_str("1f").expect_err("float parse should fail")
            ))
        )
    }

    #[test]
    fn cant_parse_with_missing_bracket() {
        assert_eq!(
            Padding::from_str("1,2,3,4,5]"),
            Err(ParsePaddingError::MissingBracket)
        );

        assert_eq!(
            Padding::from_str("[1,2,3,4,5"),
            Err(ParsePaddingError::MissingBracket)
        );

        assert_eq!(
            Padding::from_str("1,2,3,4,5"),
            Err(ParsePaddingError::MissingBracket)
        )
    }

    #[test]
    fn cant_parse_empty_string() {
        assert_eq!(Padding::from_str(" "), Err(ParsePaddingError::Empty))
    }
}
