use iced::widget::{self, container, text, tooltip};
use iced_material::Theme;

use crate::types::Element;

pub mod button;
pub mod tip {
    pub use super::tooltip::Position;
}

pub fn tip<'a, Message: 'a>(
    target: impl Into<Element<'a, Message>>,
    tip: &'a str,
    position: tip::Position,
) -> Element<'a, Message> {
    tooltip(
        target,
        container(text(tip).size(14)).padding(5).style(|theme| {
            let base = iced_material::container::surface_container_low(theme);
            container::Style {
                border: iced::border::rounded(4),
                ..base
            }
        }),
        position,
    )
    .into()
}

pub type Text<'a> = widget::Text<'a, Theme>;
