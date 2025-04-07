use std::sync::LazyLock;

use iced_widget::core::Color;
use iced_widget::core::theme::{Base, Style};
use serde::Deserialize;

pub mod button;
pub mod container;
pub mod text;
pub mod utils;

const DARK_THEME_CONTENT: &str = include_str!("../assets/themes/dark.toml");
const LIGHT_THEME_CONTENT: &str = include_str!("../assets/themes/light.toml");

#[derive(Debug, PartialEq, Deserialize)]
pub struct Theme {
    pub name: String,
    #[serde(flatten)]
    pub colorscheme: ColorScheme,
}

impl Theme {
    pub fn new(name: impl Into<String>, colorscheme: ColorScheme) -> Self {
        Self {
            name: name.into(),
            colorscheme,
        }
    }
}

impl Clone for Theme {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            colorscheme: self.colorscheme,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.name = source.name.clone();
        self.colorscheme = source.colorscheme;
    }
}

impl Default for Theme {
    fn default() -> Self {
        match dark_light::detect().unwrap_or(dark_light::Mode::Unspecified) {
            dark_light::Mode::Dark | dark_light::Mode::Unspecified => {
                DARK.clone()
            }
            dark_light::Mode::Light => LIGHT.clone(),
        }
    }
}

impl Base for Theme {
    fn base(&self) -> Style {
        Style {
            background_color: self.colorscheme.surface.color,
            text_color: self.colorscheme.surface.on_surface,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ColorScheme {
    pub primary: Primary,
    pub secondary: Secondary,
    pub tertiary: Tertiary,
    pub error: Error,
    pub surface: Surface,
    pub inverse: Inverse,
    pub outline: Outline,
    #[serde(with = "color_serde")]
    pub shadow: Color,
}

pub static DARK: LazyLock<Theme> = LazyLock::new(|| {
    toml::from_str(DARK_THEME_CONTENT).expect("parse dark theme")
});

pub static LIGHT: LazyLock<Theme> = LazyLock::new(|| {
    toml::from_str(LIGHT_THEME_CONTENT).expect("parse light theme")
});

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Primary {
    #[serde(with = "color_serde")]
    pub color: Color,
    #[serde(with = "color_serde")]
    pub on_primary: Color,
    #[serde(with = "color_serde")]
    pub primary_container: Color,
    #[serde(with = "color_serde")]
    pub on_primary_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Secondary {
    #[serde(with = "color_serde")]
    pub color: Color,
    #[serde(with = "color_serde")]
    pub on_secondary: Color,
    #[serde(with = "color_serde")]
    pub secondary_container: Color,
    #[serde(with = "color_serde")]
    pub on_secondary_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Tertiary {
    #[serde(with = "color_serde")]
    pub color: Color,
    #[serde(with = "color_serde")]
    pub on_tertiary: Color,
    #[serde(with = "color_serde")]
    pub tertiary_container: Color,
    #[serde(with = "color_serde")]
    pub on_tertiary_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Error {
    #[serde(with = "color_serde")]
    pub color: Color,
    #[serde(with = "color_serde")]
    pub on_error: Color,
    #[serde(with = "color_serde")]
    pub error_container: Color,
    #[serde(with = "color_serde")]
    pub on_error_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Surface {
    #[serde(with = "color_serde")]
    pub color: Color,
    #[serde(with = "color_serde")]
    pub on_surface: Color,
    #[serde(with = "color_serde")]
    pub on_surface_variant: Color,
    pub surface_container: SurfaceContainer,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct SurfaceContainer {
    #[serde(with = "color_serde")]
    pub lowest: Color,
    #[serde(with = "color_serde")]
    pub low: Color,
    #[serde(with = "color_serde")]
    pub base: Color,
    #[serde(with = "color_serde")]
    pub high: Color,
    #[serde(with = "color_serde")]
    pub highest: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Inverse {
    #[serde(with = "color_serde")]
    pub inverse_surface: Color,
    #[serde(with = "color_serde")]
    pub inverse_on_surface: Color,
    #[serde(with = "color_serde")]
    pub inverse_primary: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Outline {
    #[serde(with = "color_serde")]
    pub color: Color,
    #[serde(with = "color_serde")]
    pub variant: Color,
}

pub fn parse_argb(s: &str) -> Option<Color> {
    let hex = s.strip_prefix('#').unwrap_or(s);

    let parse_channel = |from: usize, to: usize| {
        let num =
            usize::from_str_radix(&hex[from..=to], 16).ok()? as f32 / 255.0;

        // If we only got half a byte (one letter), expand it into a full byte (two letters)
        Some(if from == to { num + num * 16.0 } else { num })
    };

    Some(match hex.len() {
        3 => Color::from_rgb(
            parse_channel(0, 0)?,
            parse_channel(1, 1)?,
            parse_channel(2, 2)?,
        ),
        4 => Color::from_rgba(
            parse_channel(1, 1)?,
            parse_channel(2, 2)?,
            parse_channel(3, 3)?,
            parse_channel(0, 0)?,
        ),
        6 => Color::from_rgb(
            parse_channel(0, 1)?,
            parse_channel(2, 3)?,
            parse_channel(4, 5)?,
        ),
        8 => Color::from_rgba(
            parse_channel(2, 3)?,
            parse_channel(4, 5)?,
            parse_channel(6, 7)?,
            parse_channel(0, 1)?,
        ),
        _ => None?,
    })
}

mod color_serde {
    use iced_widget::core::Color;
    use serde::{Deserialize, Deserializer};

    use super::parse_argb;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)
            .map(|hex| parse_argb(&hex))?
            .unwrap_or(Color::TRANSPARENT))
    }
}
