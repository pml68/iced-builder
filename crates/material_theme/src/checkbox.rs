use iced_widget::checkbox::{Catalog, Status, Style, StyleFn};
use iced_widget::core::{Background, Border, Color, border};

use super::Theme;
use crate::utils::{DISABLED_CONTAINER_OPACITY, HOVERED_LAYER_OPACITY, mix};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn styled(
    background_color: Color,
    background_hover: Option<Color>,
    icon_color: Color,
    border_color: Color,
    text_color: Option<Color>,
    is_checked: bool,
) -> Style {
    Style {
        background: Background::Color(if is_checked {
            background_color
        } else {
            background_hover.unwrap_or(Color::TRANSPARENT)
        }),
        icon_color,
        border: if is_checked {
            border::rounded(2)
        } else {
            Border {
                color: border_color,
                width: 2.0,
                radius: border::radius(2),
            }
        },
        text_color,
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let surface = theme.colorscheme.surface;
    let primary = theme.colorscheme.primary;

    match status {
        Status::Active { is_checked } => styled(
            primary.color,
            None,
            primary.on_primary,
            surface.on_surface_variant,
            Some(surface.on_surface),
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            mix(primary.color, surface.on_surface, HOVERED_LAYER_OPACITY),
            Some(Color {
                a: HOVERED_LAYER_OPACITY,
                ..surface.on_surface
            }),
            primary.on_primary,
            surface.on_surface_variant,
            Some(surface.on_surface),
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            Color {
                a: DISABLED_CONTAINER_OPACITY,
                ..surface.on_surface
            },
            None,
            surface.color,
            Color {
                a: DISABLED_CONTAINER_OPACITY,
                ..surface.on_surface
            },
            Some(surface.on_surface),
            is_checked,
        ),
    }
}

pub fn error(theme: &Theme, status: Status) -> Style {
    let surface = theme.colorscheme.surface;
    let error = theme.colorscheme.error;

    match status {
        Status::Active { is_checked } => styled(
            error.color,
            None,
            error.on_error,
            error.color,
            Some(error.color),
            is_checked,
        ),
        Status::Hovered { is_checked } => styled(
            mix(error.color, surface.on_surface, HOVERED_LAYER_OPACITY),
            Some(Color {
                a: HOVERED_LAYER_OPACITY,
                ..error.color
            }),
            error.on_error,
            error.color,
            Some(error.color),
            is_checked,
        ),
        Status::Disabled { is_checked } => styled(
            Color {
                a: DISABLED_CONTAINER_OPACITY,
                ..surface.on_surface
            },
            None,
            surface.color,
            Color {
                a: DISABLED_CONTAINER_OPACITY,
                ..surface.on_surface
            },
            Some(surface.on_surface),
            is_checked,
        ),
    }
}
