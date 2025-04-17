use iced_widget::core::{Background, Color};
use iced_widget::radio::{Catalog, Status, Style, StyleFn};

use super::Theme;
use crate::utils::{DISABLED_TEXT_OPACITY, HOVERED_LAYER_OPACITY, mix};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let surface = theme.colorscheme.surface;
    let primary = theme.colorscheme.primary;

    let active = Style {
        background: Color::TRANSPARENT.into(),
        dot_color: primary.color,
        border_width: 1.0,
        border_color: primary.color,
        text_color: None,
    };

    match status {
        Status::Active { is_selected } => Style {
            border_color: if is_selected {
                active.border_color
            } else {
                surface.on_surface
            },
            ..active
        },
        Status::Hovered { is_selected } => Style {
            dot_color: mix(
                primary.color,
                surface.on_surface,
                HOVERED_LAYER_OPACITY,
            ),
            border_color: if is_selected {
                mix(primary.color, surface.on_surface, HOVERED_LAYER_OPACITY)
            } else {
                Color {
                    a: DISABLED_TEXT_OPACITY,
                    ..surface.on_surface
                }
            },
            background: Background::Color(if is_selected {
                Color {
                    a: HOVERED_LAYER_OPACITY,
                    ..surface.on_surface
                }
            } else {
                Color::TRANSPARENT
            }),
            ..active
        },
    }
}
