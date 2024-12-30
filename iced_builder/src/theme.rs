use std::sync::Arc;

use iced::Color;

use crate::config::Config;

pub fn theme_index(theme_name: String, slice: &[iced::Theme]) -> Option<usize> {
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
                    theme_index(theme_name.into(), &config.theme.all)
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

#[derive(Debug)]
pub struct Theme {
    pub selected: iced::Theme,
    pub all: Arc<[iced::Theme]>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            selected: iced::Theme::default(),
            all: iced::Theme::ALL.into(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
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
        }
    }
}

mod color_serde {
    use iced::Color;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)
            .map(|hex| Color::parse(&hex))?
            .unwrap_or(Color::TRANSPARENT))
    }
}
