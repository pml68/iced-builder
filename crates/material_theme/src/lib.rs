use std::sync::LazyLock;

use iced_widget::core::Color;
use iced_widget::core::theme::{Base, Style};
use serde::Deserialize;

pub mod button;
pub mod checkbox;
pub mod combo_box;
pub mod container;
#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "markdown")]
pub mod markdown;
pub mod menu;
pub mod pick_list;
pub mod progress_bar;
#[cfg(feature = "qr_code")]
pub mod qr_code;
pub mod scrollable;
#[cfg(feature = "svg")]
pub mod svg;
pub mod text;
pub mod text_input;
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
        static DEFAULT: LazyLock<Theme> = LazyLock::new(|| {
            match dark_light::detect().unwrap_or(dark_light::Mode::Unspecified)
            {
                dark_light::Mode::Dark | dark_light::Mode::Unspecified => {
                    DARK.clone()
                }
                dark_light::Mode::Light => LIGHT.clone(),
            }
        });

        DEFAULT.clone()
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Base for Theme {
    fn base(&self) -> Style {
        Style {
            background_color: self.colorscheme.surface.color,
            text_color: self.colorscheme.surface.on_surface,
        }
    }

    fn palette(&self) -> Option<iced_widget::theme::Palette> {
        // TODO: create a Palette
        None
    }
}

#[cfg(feature = "animate")]
impl iced_anim::Animate for Theme {
    fn components() -> usize {
        ColorScheme::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        let mut colors = self.colorscheme;
        colors.update(components);

        *self = Theme::new("Animating Theme", colors);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        self.colorscheme.distance_to(&end.colorscheme)
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        let mut colors = self.colorscheme;
        colors.lerp(&start.colorscheme, &end.colorscheme, progress);

        *self = Theme::new("Animating Theme", colors);
    }
}

pub static DARK: LazyLock<Theme> = LazyLock::new(|| {
    toml::from_str(DARK_THEME_CONTENT).expect("parse dark theme")
});

pub static LIGHT: LazyLock<Theme> = LazyLock::new(|| {
    toml::from_str(LIGHT_THEME_CONTENT).expect("parse light theme")
});

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
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
    #[serde(with = "color_serde")]
    pub scrim: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
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
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
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
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
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
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
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
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
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
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
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
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
pub struct Inverse {
    #[serde(with = "color_serde")]
    pub inverse_surface: Color,
    #[serde(with = "color_serde")]
    pub inverse_on_surface: Color,
    #[serde(with = "color_serde")]
    pub inverse_primary: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
pub struct Outline {
    #[serde(with = "color_serde")]
    pub color: Color,
    #[serde(with = "color_serde")]
    pub variant: Color,
}

mod color_serde {
    use iced_widget::core::Color;
    use serde::{Deserialize, Deserializer};

    use super::utils::parse_argb;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)
            .map(|hex| parse_argb(&hex))?
            .unwrap_or(Color::TRANSPARENT))
    }
}
