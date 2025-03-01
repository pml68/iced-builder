use iced::Alignment;

use super::Value;

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum ParseAlignmentError {
    #[error("cannot parse rotation from empty string")]
    Empty,
    #[error("invalid variant")]
    InvalidVariant,
}

impl Value for Alignment {
    type Err = ParseAlignmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err(ParseAlignmentError::Empty);
        }

        match s {
            "start" => Ok(Self::Start),
            "center" => Ok(Self::Center),
            "end" => Ok(Self::End),
            _ => Err(ParseAlignmentError::InvalidVariant),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Start => String::from("start"),
            Self::Center => String::from("center"),
            Self::End => String::from("end"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_with_spaces() {
        assert_eq!(Alignment::from_str("  start"), Ok(Alignment::Start));

        assert_eq!(Alignment::from_str("   center   "), Ok(Alignment::Center));

        assert_eq!(Alignment::from_str("end "), Ok(Alignment::End))
    }

    #[test]
    fn cant_parse_invalid_variant() {
        assert_eq!(
            Alignment::from_str("middle"),
            Err(ParseAlignmentError::InvalidVariant)
        )
    }

    #[test]
    fn cant_parse_empty_string() {
        assert_eq!(Alignment::from_str(" "), Err(ParseAlignmentError::Empty))
    }
}
