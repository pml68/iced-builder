// Generated automatically by iced_fontello at build time.
// Do not edit manually. Source: ../fonts/icons.toml
// 0a164ed48e8a0ef9ffb68cfe442a0cabc6c251beb644b51d01da8e5b7fdbd34e
use iced::Font;
use iced::widget::text;

use crate::widget::Text;

pub const FONT: &[u8] = include_bytes!("../fonts/icons.ttf");

pub fn copy<'a>() -> Text<'a> {
    icon("\u{F1C9}")
}

pub fn switch<'a>() -> Text<'a> {
    icon("\u{21C6}")
}

fn icon(codepoint: &str) -> Text<'_> {
    text(codepoint).font(Font::with_name("icons"))
}
