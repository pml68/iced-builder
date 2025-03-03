use iced::ContentFit;

use super::Value;

#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum ParseContentFitError {
    #[error("invalid variant")]
    InvalidVariant,
}

impl Value for ContentFit {
    type Err = ParseContentFitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            Ok(Self::default())
        } else {
            match s {
                "fill" => Ok(Self::Fill),
                "none" => Ok(Self::None),
                "cover" => Ok(Self::Cover),
                "contain" => Ok(Self::Contain),
                "scale_down" => Ok(Self::ScaleDown),
                _ => Err(ParseContentFitError::InvalidVariant),
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Fill => String::from("fill"),
            Self::None => String::from("none"),
            Self::Cover => String::from("cover"),
            Self::Contain => String::from("contain"),
            Self::ScaleDown => String::from("scale_down"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_with_spaces() {
        assert_eq!(ContentFit::from_str("    fill"), Ok(ContentFit::Fill));

        assert_eq!(ContentFit::from_str("  none  "), Ok(ContentFit::None));

        assert_eq!(ContentFit::from_str("cover   "), Ok(ContentFit::Cover));

        assert_eq!(ContentFit::from_str("contain"), Ok(ContentFit::Contain));

        assert_eq!(
            ContentFit::from_str("scale_down"),
            Ok(ContentFit::ScaleDown)
        )
    }

    #[test]
    fn cant_parse_invalid_variant() {
        assert_eq!(
            ContentFit::from_str("clip"),
            Err(ParseContentFitError::InvalidVariant)
        )
    }
}
