use std::num::ParseFloatError;
use std::str::FromStr;

use iced::Pixels;

use super::Value;

impl Value for Pixels {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pixels(f32::from_str(s.trim())?))
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
