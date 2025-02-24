use std::str::FromStr;

use iced::Padding;

use super::ValueFromStr;

#[derive(Debug)]
pub enum PaddingError {
    Nah,
}

impl ValueFromStr for Padding {
    type Err = PaddingError;

    fn value_from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .map(|s| {
                s.split(',')
                    .map(|n| f32::from_str(n).unwrap())
                    .collect::<Vec<_>>()
            })
            .and_then(|s| {
                if s.len() == 4 {
                    Some(Padding {
                        top: s[0],
                        right: s[1],
                        bottom: s[2],
                        left: s[3],
                    })
                } else {
                    None
                }
            })
            .ok_or(PaddingError::Nah)
    }
}
