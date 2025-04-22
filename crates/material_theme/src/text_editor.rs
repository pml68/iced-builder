use iced_widget::core::{Background, Border, Color, border};
use iced_widget::text_editor::{Catalog, Status, Style, StyleFn};

use super::Theme;
use crate::utils::DISABLED_TEXT_OPACITY;

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
        background: Background::Color(surface.surface_container.highest),
        border: Border {
            color: theme.colorscheme.outline.color,
            width: 1.0,
            radius: 4.into(),
        },
        icon: surface.on_surface_variant,
        placeholder: surface.on_surface_variant,
        value: surface.on_surface,
        selection: Color {
            a: DISABLED_TEXT_OPACITY,
            ..primary.color
        },
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            border: Border {
                color: surface.on_surface,
                ..active.border
            },
            ..active
        },
        Status::Focused { .. } => Style {
            border: Border {
                color: primary.color,
                width: 2.0,
                ..active.border
            },
            placeholder: primary.color,
            ..active
        },
        Status::Disabled => Style {
            background: Color::TRANSPARENT.into(),
            border: Border {
                color: Color {
                    a: DISABLED_TEXT_OPACITY,
                    ..surface.on_surface
                },
                width: 1.0,
                radius: border::radius(4),
            },
            icon: Color {
                a: DISABLED_TEXT_OPACITY,
                ..surface.on_surface
            },
            placeholder: Color {
                a: DISABLED_TEXT_OPACITY,
                ..surface.on_surface
            },
            selection: Color {
                a: DISABLED_TEXT_OPACITY,
                ..surface.on_surface
            },
            value: Color {
                a: DISABLED_TEXT_OPACITY,
                ..surface.on_surface
            },
        },
    }
}
