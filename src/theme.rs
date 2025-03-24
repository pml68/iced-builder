use std::sync::{Arc, LazyLock};

use iced::Color;
use iced::theme::Base;
use iced::theme::palette::Extended;
use serde::Deserialize;

use crate::config::Config;

const DARK_THEME_CONTENT: &str = include_str!("../assets/themes/dark.toml");
const LIGHT_THEME_CONTENT: &str = include_str!("../assets/themes/light.toml");

pub fn theme_index(theme_name: &str, slice: &[iced::Theme]) -> Option<usize> {
    slice
        .iter()
        .position(|theme| theme.to_string() == theme_name)
}

pub fn theme_from_str(
    config: Option<&Config>,
    theme_name: &str,
) -> iced::Theme {
    match theme_name {
        "Light" => iced::Theme::Light,
        "Dark" => iced::Theme::Dark,
        "Dracula" => iced::Theme::Dracula,
        "Nord" => iced::Theme::Nord,
        "Solarized Light" => iced::Theme::SolarizedLight,
        "Solarized Dark" => iced::Theme::SolarizedDark,
        "Gruvbox Light" => iced::Theme::GruvboxLight,
        "Gruvbox Dark" => iced::Theme::GruvboxDark,
        "Catppuccin Latte" => iced::Theme::CatppuccinLatte,
        "Catppuccin Frappé" => iced::Theme::CatppuccinFrappe,
        "Catppuccin Macchiato" => iced::Theme::CatppuccinMacchiato,
        "Catppuccin Mocha" => iced::Theme::CatppuccinMocha,
        "Tokyo Night" => iced::Theme::TokyoNight,
        "Tokyo Night Storm" => iced::Theme::TokyoNightStorm,
        "Tokyo Night Light" => iced::Theme::TokyoNightLight,
        "Kanagawa Wave" => iced::Theme::KanagawaWave,
        "Kanagawa Dragon" => iced::Theme::KanagawaDragon,
        "Kanagawa Lotus" => iced::Theme::KanagawaLotus,
        "Moonfly" => iced::Theme::Moonfly,
        "Nightfly" => iced::Theme::Nightfly,
        "Oxocarbon" => iced::Theme::Oxocarbon,
        "Ferra" => iced::Theme::Ferra,
        _ => {
            if let Some(config) = config {
                if theme_name == config.theme.selected.to_string() {
                    config.theme.selected.clone()
                } else if let Some(index) =
                    theme_index(theme_name, &config.theme.all)
                {
                    config.theme.all[index].clone()
                } else {
                    iced::Theme::default()
                }
            } else {
                iced::Theme::default()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Appearance {
    pub selected: iced::Theme,
    pub all: Arc<[iced::Theme]>,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            selected: Theme::default().into(),
            all: {
                let mut themes = iced::Theme::ALL.to_owned();
                themes.push(Theme::default().into());
                themes.into()
            },
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct OtherTheme {
    name: String,
    #[serde(flatten)]
    colorscheme: ColorScheme,
}

impl Clone for OtherTheme {
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

impl Default for OtherTheme {
    fn default() -> Self {
        match dark_light::detect().unwrap_or(dark_light::Mode::Unspecified) {
            dark_light::Mode::Dark | dark_light::Mode::Unspecified => {
                DARK.clone()
            }
            dark_light::Mode::Light => LIGHT.clone(),
        }
    }
}

impl Base for OtherTheme {
    fn base(&self) -> iced::theme::Style {
        iced::theme::Style {
            background_color: self.colorscheme.surface.surface,
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
}

pub static DARK: LazyLock<OtherTheme> = LazyLock::new(|| {
    toml::from_str(DARK_THEME_CONTENT).expect("parse dark theme")
});

pub static LIGHT: LazyLock<OtherTheme> = LazyLock::new(|| {
    toml::from_str(LIGHT_THEME_CONTENT).expect("parse light theme")
});

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Primary {
    #[serde(with = "color_serde")]
    pub primary: Color,
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
    pub secondary: Color,
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
    pub tertiary: Color,
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
    pub error: Color,
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
    pub surface: Color,
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
    pub outline: Color,
    #[serde(with = "color_serde")]
    pub outline_variant: Color,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    name: String,
    palette: ThemePalette,
    dark: Option<bool>,
    #[serde(flatten)]
    extended: Option<ExtendedThemePalette>,
}

impl From<Theme> for iced::Theme {
    fn from(value: Theme) -> Self {
        iced::Theme::custom_with_fn(
            value.name.clone(),
            value.palette.into(),
            |_| value.into(),
        )
    }
}

impl Default for Theme {
    fn default() -> Self {
        toml::from_str(include_str!("../assets/themes/rose_pine.toml"))
            .expect("parse default theme")
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ThemePalette {
    #[serde(with = "color_serde")]
    background: Color,
    #[serde(with = "color_serde")]
    text: Color,
    #[serde(with = "color_serde")]
    primary: Color,
    #[serde(with = "color_serde")]
    success: Color,
    #[serde(with = "color_serde")]
    danger: Color,
    #[serde(with = "color_serde")]
    warning: Color,
}

impl Default for ThemePalette {
    fn default() -> Self {
        let palette = iced::Theme::default().palette();
        Self {
            background: palette.background,
            text: palette.text,
            primary: palette.primary,
            success: palette.success,
            danger: palette.danger,
            warning: palette.warning,
        }
    }
}

impl From<ThemePalette> for iced::theme::Palette {
    fn from(palette: ThemePalette) -> Self {
        iced::theme::Palette {
            background: palette.background,
            text: palette.text,
            primary: palette.primary,
            success: palette.success,
            danger: palette.danger,
            warning: palette.warning,
        }
    }
}

impl From<Theme> for Extended {
    fn from(theme: Theme) -> Self {
        let mut extended = Extended::generate(theme.palette.into());

        if let Some(is_dark) = theme.dark {
            extended.is_dark = is_dark;
        }

        if let Some(extended_palette) = theme.extended {
            if let Some(background) = extended_palette.background {
                if let Some(base) = background.base {
                    extended.background.base = base.into();
                }
                if let Some(weak) = background.weak {
                    extended.background.weak = weak.into();
                }
                if let Some(strong) = background.strong {
                    extended.background.strong = strong.into();
                }
            }

            if let Some(primary) = extended_palette.primary {
                if let Some(base) = primary.base {
                    extended.primary.base = base.into();
                }
                if let Some(weak) = primary.weak {
                    extended.primary.weak = weak.into();
                }
                if let Some(strong) = primary.strong {
                    extended.primary.strong = strong.into();
                }
            }

            if let Some(secondary) = extended_palette.secondary {
                if let Some(base) = secondary.base {
                    extended.secondary.base = base.into();
                }
                if let Some(weak) = secondary.weak {
                    extended.secondary.weak = weak.into();
                }
                if let Some(strong) = secondary.strong {
                    extended.secondary.strong = strong.into();
                }
            }

            if let Some(success) = extended_palette.success {
                if let Some(base) = success.base {
                    extended.success.base = base.into();
                }
                if let Some(weak) = success.weak {
                    extended.success.weak = weak.into();
                }
                if let Some(strong) = success.strong {
                    extended.success.strong = strong.into();
                }
            }

            if let Some(danger) = extended_palette.danger {
                if let Some(base) = danger.base {
                    extended.danger.base = base.into();
                }
                if let Some(weak) = danger.weak {
                    extended.danger.weak = weak.into();
                }
                if let Some(strong) = danger.strong {
                    extended.danger.strong = strong.into();
                }
            }

            if let Some(warning) = extended_palette.warning {
                if let Some(base) = warning.base {
                    extended.warning.base = base.into();
                }
                if let Some(weak) = warning.weak {
                    extended.warning.weak = weak.into();
                }
                if let Some(strong) = warning.strong {
                    extended.warning.strong = strong.into();
                }
            }
        }

        extended
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ExtendedThemePalette {
    background: Option<ThemeBackground>,
    primary: Option<ThemePrimary>,
    secondary: Option<ThemeSecondary>,
    success: Option<ThemeSuccess>,
    danger: Option<ThemeDanger>,
    warning: Option<ThemeWarning>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ThemeBackground {
    base: Option<ThemePair>,
    weak: Option<ThemePair>,
    strong: Option<ThemePair>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ThemePrimary {
    base: Option<ThemePair>,
    weak: Option<ThemePair>,
    strong: Option<ThemePair>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ThemeSecondary {
    base: Option<ThemePair>,
    weak: Option<ThemePair>,
    strong: Option<ThemePair>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ThemeSuccess {
    base: Option<ThemePair>,
    weak: Option<ThemePair>,
    strong: Option<ThemePair>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ThemeDanger {
    base: Option<ThemePair>,
    weak: Option<ThemePair>,
    strong: Option<ThemePair>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ThemeWarning {
    base: Option<ThemePair>,
    weak: Option<ThemePair>,
    strong: Option<ThemePair>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ThemePair {
    #[serde(with = "color_serde")]
    color: Color,
    #[serde(with = "color_serde")]
    text: Color,
}

impl From<ThemePair> for iced::theme::palette::Pair {
    fn from(pair: ThemePair) -> Self {
        Self {
            color: pair.color,
            text: pair.text,
        }
    }
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
    use iced::Color;
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
