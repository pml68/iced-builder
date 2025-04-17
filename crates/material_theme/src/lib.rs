use std::sync::LazyLock;

use iced_widget::core::theme::{Base, Style};
use iced_widget::core::{Color, color};

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
pub mod radio;
pub mod scrollable;
#[cfg(feature = "svg")]
pub mod svg;
pub mod text;
pub mod text_input;
pub mod utils;

pub static DARK: LazyLock<Theme> =
    LazyLock::new(|| Theme::new("Dark", ColorScheme::DARK));
pub static LIGHT: LazyLock<Theme> =
    LazyLock::new(|| Theme::new("Light", ColorScheme::LIGHT));

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Theme {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
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

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorScheme {
    pub primary: Primary,
    pub secondary: Secondary,
    pub tertiary: Tertiary,
    pub error: Error,
    pub surface: Surface,
    pub inverse: Inverse,
    pub outline: Outline,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub shadow: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub scrim: Color,
}

macro_rules! from_argb {
    ($hex:expr) => {{
        let hex = $hex as u32;

        let a = ((hex & 0xff000000) >> 24) as f32 / 255.0;
        let r = (hex & 0x00ff0000) >> 16;
        let g = (hex & 0x0000ff00) >> 8;
        let b = (hex & 0x000000ff);

        color!(r as u8, g as u8, b as u8, a)
    }};
}

impl ColorScheme {
    const DARK: Self = Self {
        primary: Primary {
            color: from_argb!(0xff9bd4a1),
            on_primary: from_argb!(0xff003916),
            primary_container: from_argb!(0xff1b5129),
            on_primary_container: from_argb!(0xffb6f1bb),
        },
        secondary: Secondary {
            color: from_argb!(0xffb8ccb6),
            on_secondary: from_argb!(0xff233425),
            secondary_container: from_argb!(0xff394b3a),
            on_secondary_container: from_argb!(0xffd3e8d1),
        },
        tertiary: Tertiary {
            color: from_argb!(0xffa1ced7),
            on_tertiary: from_argb!(0xff00363e),
            tertiary_container: from_argb!(0xff1f4d55),
            on_tertiary_container: from_argb!(0xffbdeaf4),
        },
        error: Error {
            color: from_argb!(0xffffb4ab),
            on_error: from_argb!(0xff690005),
            error_container: from_argb!(0xff93000a),
            on_error_container: from_argb!(0xffffdad6),
        },
        surface: Surface {
            color: from_argb!(0xff101510),
            on_surface: from_argb!(0xffe0e4dc),
            on_surface_variant: from_argb!(0xffc1c9be),
            surface_container: SurfaceContainer {
                lowest: from_argb!(0xff0b0f0b),
                low: from_argb!(0xff181d18),
                base: from_argb!(0xff1c211c),
                high: from_argb!(0xff262b26),
                highest: from_argb!(0xff313631),
            },
        },
        inverse: Inverse {
            inverse_surface: from_argb!(0xffe0e4dc),
            inverse_on_surface: from_argb!(0xff2d322c),
            inverse_primary: from_argb!(0xff34693f),
        },
        outline: Outline {
            color: from_argb!(0xff8b9389),
            variant: from_argb!(0xff414941),
        },
        shadow: from_argb!(0xff000000),
        scrim: from_argb!(0x4d000000),
    };

    const LIGHT: Self = Self {
        primary: Primary {
            color: from_argb!(0xff34693f),
            on_primary: from_argb!(0xffffffff),
            primary_container: from_argb!(0xffb6f1bb),
            on_primary_container: from_argb!(0xff1b5129),
        },
        secondary: Secondary {
            color: from_argb!(0xff516351),
            on_secondary: from_argb!(0xffffffff),
            secondary_container: from_argb!(0xffd3e8d1),
            on_secondary_container: from_argb!(0xff394b3a),
        },
        tertiary: Tertiary {
            color: from_argb!(0xff39656d),
            on_tertiary: from_argb!(0xffffffff),
            tertiary_container: from_argb!(0xffbdeaf4),
            on_tertiary_container: from_argb!(0xff1f4d55),
        },
        error: Error {
            color: from_argb!(0xffba1a1a),
            on_error: from_argb!(0xffffffff),
            error_container: from_argb!(0xffffdad6),
            on_error_container: from_argb!(0xff93000a),
        },
        surface: Surface {
            color: from_argb!(0xfff7fbf2),
            on_surface: from_argb!(0xff181d18),
            on_surface_variant: from_argb!(0xff414941),
            surface_container: SurfaceContainer {
                lowest: from_argb!(0xffffffff),
                low: from_argb!(0xfff1f5ed),
                base: from_argb!(0xffebefe7),
                high: from_argb!(0xffe5e9e1),
                highest: from_argb!(0xffe0e4dc),
            },
        },
        inverse: Inverse {
            inverse_surface: from_argb!(0xff2d322c),
            inverse_on_surface: from_argb!(0xffeef2ea),
            inverse_primary: from_argb!(0xff9bd4a1),
        },
        outline: Outline {
            color: from_argb!(0xff727970),
            variant: from_argb!(0xffc1c9be),
        },
        shadow: from_argb!(0xff000000),
        scrim: from_argb!(0x4d000000),
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Primary {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_primary: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub primary_container: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_primary_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Secondary {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_secondary: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub secondary_container: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_secondary_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tertiary {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_tertiary: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub tertiary_container: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_tertiary_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Error {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_error: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub error_container: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_error_container: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Surface {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_surface: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub on_surface_variant: Color,
    pub surface_container: SurfaceContainer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SurfaceContainer {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub lowest: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub low: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub base: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub high: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub highest: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Inverse {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub inverse_surface: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub inverse_on_surface: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub inverse_primary: Color,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "animate", derive(iced_anim::Animate))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Outline {
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub color: Color,
    #[cfg_attr(feature = "serde", serde(with = "color_serde"))]
    pub variant: Color,
}

#[cfg(feature = "serde")]
mod color_serde {
    use iced_widget::core::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::utils::{color_to_argb, parse_argb};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)
            .map(|hex| parse_argb(&hex))?
            .unwrap_or(Color::TRANSPARENT))
    }

    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        color_to_argb(*color).serialize(serializer)
    }
}
