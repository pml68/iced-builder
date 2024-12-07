// Generated automatically by iced_fontello at build time.
// Do not edit manually.
// 02c7558d187cdc056fdd0e6a638ef805fa10f5955f834575e51d75acd35bc70e
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

fn icon<'a>(codepoint: &'a str) -> Text<'a> {
    text(codepoint).font(Font::with_name("icons"))
}
