use std::sync::Arc;

use iced::Color;
use iced::theme::palette::Extended;
use serde::Deserialize;

use crate::config::Config;

const DEFAULT_THEME_CONTENT: &str =
    include_str!("../assets/themes/rose_pine.toml");

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
        toml::from_str(DEFAULT_THEME_CONTENT).expect("parse default theme")
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
