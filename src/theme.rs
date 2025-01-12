use std::sync::Arc;

use iced::theme::palette::Extended;
use iced::Color;

use crate::config::Config;

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

fn palette_to_string(palette: &iced::theme::Palette) -> String {
    format!(
        r"Palette {{
            background: color!(0x{}),
            text: color!(0x{}),
            primary: color!(0x{}),
            success: color!(0x{}),
            danger: color!(0x{}),
        }}",
        color_to_hex(palette.background),
        color_to_hex(palette.text),
        color_to_hex(palette.primary),
        color_to_hex(palette.success),
        color_to_hex(palette.danger),
    )
}

fn extended_to_string(extended: &Extended) -> String {
    format!(
        r"
Extended{{background:Background{{base:Pair{{color:color!(0x{}),text:color!(0x{}),}},weak:Pair{{color:color!(0x{}),text:color!(0x{}),}},strong:Pair{{color:color!(0x{}),text:color!(0x{}),}},}},primary:Primary{{base:Pair{{color:color!(0x{}),text:color!(0x{}),}},weak:Pair{{color:color!(0x{}),text:color!(0x{}),}},strong:Pair{{color:color!(0x{}),text:color!(0x{}),}},}},secondary:Secondary{{base:Pair{{color:color!(0x{}),text:color!(0x{}),}},weak:Pair{{color:color!(0x{}),text:color!(0x{}),}},strong:Pair{{color:color!(0x{}),text:color!(0x{}),}},}},success:Success{{base:Pair{{color:color!(0x{}),text:color!(0x{}),}},weak:Pair{{color:color!(0x{}),text:color!(0x{}),}},strong:Pair{{color:color!(0x{}),text:color!(0x{}),}},}},danger:Danger{{base:Pair{{color:color!(0x{}),text:color!(0x{}),}},weak:Pair{{color:color!(0x{}),text:color!(0x{}),}},strong:Pair{{color:color!(0x{}),text:color!(0x{}),}},}},is_dark:true,}}",
        color_to_hex(extended.background.base.color),
        color_to_hex(extended.background.base.text),
        color_to_hex(extended.background.weak.color),
        color_to_hex(extended.background.weak.text),
        color_to_hex(extended.background.strong.color),
        color_to_hex(extended.background.strong.text),
        color_to_hex(extended.primary.base.color),
        color_to_hex(extended.primary.base.text),
        color_to_hex(extended.primary.weak.color),
        color_to_hex(extended.primary.weak.text),
        color_to_hex(extended.primary.strong.color),
        color_to_hex(extended.primary.strong.text),
        color_to_hex(extended.secondary.base.color),
        color_to_hex(extended.secondary.base.text),
        color_to_hex(extended.secondary.weak.color),
        color_to_hex(extended.secondary.weak.text),
        color_to_hex(extended.secondary.strong.color),
        color_to_hex(extended.secondary.strong.text),
        color_to_hex(extended.success.base.color),
        color_to_hex(extended.success.base.text),
        color_to_hex(extended.success.weak.color),
        color_to_hex(extended.success.weak.text),
        color_to_hex(extended.success.strong.color),
        color_to_hex(extended.success.strong.text),
        color_to_hex(extended.danger.base.color),
        color_to_hex(extended.danger.base.text),
        color_to_hex(extended.danger.weak.color),
        color_to_hex(extended.danger.weak.text),
        color_to_hex(extended.danger.strong.color),
        color_to_hex(extended.danger.strong.text),
    )
}

pub fn theme_to_string(theme: &iced::Theme) -> String {
    let palette = theme.palette();
    let extended = theme.extended_palette();

    let generated_extended = Extended::generate(palette);

    if &generated_extended == extended {
        format!(
            r#"custom(
                "{}".to_string(),
                {}
            )"#,
            theme,
            palette_to_string(&palette)
        )
    } else {
        format!(
            r#"custom_with_fn(
                "{}".to_string(),
                {},
                |_| {}
            )"#,
            theme,
            palette_to_string(&palette),
            extended_to_string(extended)
        )
    }
}

fn color_to_hex(color: Color) -> String {
    use std::fmt::Write;

    let mut hex = String::with_capacity(12);

    let [r, g, b, a] = color.into_rgba8();

    let _ = write!(&mut hex, "{:02X}", r);
    let _ = write!(&mut hex, "{:02X}", g);
    let _ = write!(&mut hex, "{:02X}", b);

    if a < u8::MAX {
        let _ = write!(&mut hex, ", {:.2}", a as f32 / 255.0);
    }

    hex
}

#[derive(Debug, Clone)]
pub struct Appearance {
    pub selected: iced::Theme,
    pub all: Arc<[iced::Theme]>,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            selected: iced::Theme::default(),
            all: iced::Theme::ALL.into(),
        }
    }
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Theme {
    palette: ThemePalette,
    is_dark: Option<bool>,
    #[serde(flatten)]
    extended: Option<ExtendedThemePalette>,
}

#[derive(Debug, Clone, serde::Deserialize)]
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

impl Theme {
    pub fn into_iced_theme(self, name: String) -> iced::Theme {
        iced::Theme::custom_with_fn(name, self.palette.clone().into(), |_| {
            self.into()
        })
    }
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

impl From<Theme> for Extended {
    fn from(theme: Theme) -> Self {
        let mut extended = Extended::generate(theme.palette.into());

        if let Some(is_dark) = theme.is_dark {
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

            // Handle primary
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

            // Handle secondary
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

            // Handle success
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

            // Handle danger
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
        }

        extended
    }
}

#[derive(Debug, Default, serde::Deserialize)]
struct ExtendedThemePalette {
    background: Option<ThemeBackground>,
    primary: Option<ThemePrimary>,
    secondary: Option<ThemeSecondary>,
    success: Option<ThemeSuccess>,
    danger: Option<ThemeDanger>,
}

#[derive(Debug, Default, serde::Deserialize)]
struct ThemeBackground {
    base: Option<ThemePair>,
    weak: Option<ThemePair>,
    strong: Option<ThemePair>,
}

#[derive(Debug, Default, serde::Deserialize)]
struct ThemePrimary {
    base: Option<ThemePair>,
    weak: Option<ThemePair>,
    strong: Option<ThemePair>,
}

#[derive(Debug, Default, serde::Deserialize)]
struct ThemeSecondary {
    base: Option<ThemePair>,
    weak: Option<ThemePair>,
    strong: Option<ThemePair>,
}

#[derive(Debug, Default, serde::Deserialize)]
struct ThemeSuccess {
    base: Option<ThemePair>,
    weak: Option<ThemePair>,
    strong: Option<ThemePair>,
}

#[derive(Debug, Default, serde::Deserialize)]
struct ThemeDanger {
    base: Option<ThemePair>,
    weak: Option<ThemePair>,
    strong: Option<ThemePair>,
}

#[derive(Debug, Default, serde::Deserialize)]
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
