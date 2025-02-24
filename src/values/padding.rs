use std::num::ParseFloatError;
use std::str::FromStr;

use iced::Padding;

use super::ValueFromStr;

#[derive(Debug, thiserror::Error)]
pub enum PaddingError {
    #[error("wrong number of values")]
    WrongNumberOfValues,
    #[error("float parsing error: {0}")]
    ParseFloatError(ParseFloatError),
    #[error("missing bracket")]
    MissingBracket,
    #[error("empty string given")]
    Empty,
}

impl From<ParseFloatError> for PaddingError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl ValueFromStr for Padding {
    type Err = PaddingError;

    fn value_from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(PaddingError::Empty);
        }
        let values = s
            .strip_prefix('[')
            .ok_or(PaddingError::MissingBracket)?
            .strip_suffix(']')
            .ok_or(PaddingError::MissingBracket)?
            .split(',')
            .map(|n| f32::from_str(n))
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
            _ => Err(PaddingError::WrongNumberOfValues),
        }
    }
}
