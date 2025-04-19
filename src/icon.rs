// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/icons.toml
// 915ea6b0646871c0f04350f201f27f28881b61f3bd6ef292a415d67a211739c1
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../fonts/icons.ttf");

pub fn copy<'a>() -> Text<'a> {
    icon("\u{F1C9}")
}

pub fn open<'a>() -> Text<'a> {
    icon("\u{F115}")
}

pub fn save<'a>() -> Text<'a> {
    icon("\u{1F4BE}")
}

pub fn switch<'a>() -> Text<'a> {
    icon("\u{21C6}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("icons"))
}
