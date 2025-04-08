use iced_widget::button::{Catalog, Status, Style, StyleFn};
use iced_widget::core::{Background, Border, Color, border};

use crate::Theme;
use crate::utils::{
    DISABLED_CONTAINER_OPACITY, DISABLED_TEXT_OPACITY, HOVERED_LAYER_OPACITY,
    PRESSED_LAYER_OPACITY, elevation, mix, shadow_from_elevation,
};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(filled)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

fn button(
    foreground: Color,
    background: Color,
    tone_overlay: Color,
    disabled: Color,
    shadow_color: Color,
    elevation_level: u8,
    status: Status,
) -> Style {
    let active = Style {
        background: Some(Background::Color(background)),
        text_color: foreground,
        border: border::rounded(400),
        shadow: shadow_from_elevation(elevation(elevation_level), shadow_color),
    };

    match status {
        Status::Active => active,
        Status::Pressed => Style {
            background: Some(Background::Color(mix(
                background,
                tone_overlay,
                HOVERED_LAYER_OPACITY,
            ))),
            ..active
        },
        Status::Hovered => Style {
            background: Some(Background::Color(mix(
                background,
                tone_overlay,
                PRESSED_LAYER_OPACITY,
            ))),
            text_color: foreground,
            border: border::rounded(400),
            shadow: shadow_from_elevation(
                elevation(elevation_level + 1),
                shadow_color,
            ),
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color {
                a: DISABLED_CONTAINER_OPACITY,
                ..disabled
            })),
            text_color: Color {
                a: DISABLED_TEXT_OPACITY,
                ..disabled
            },
            border: border::rounded(400),
            ..Default::default()
        },
    }
}

pub fn elevated(theme: &Theme, status: Status) -> Style {
    let surface_colors = theme.colorscheme.surface;

    let foreground = theme.colorscheme.primary.color;
    let background = surface_colors.surface_container.low;
    let disabled = surface_colors.on_surface;

    let shadow_color = theme.colorscheme.shadow;

    button(
        foreground,
        background,
        foreground,
        disabled,
        shadow_color,
        1,
        status,
    )
}

pub fn filled(theme: &Theme, status: Status) -> Style {
    let primary_colors = theme.colorscheme.primary;

    let foreground = primary_colors.on_primary;
    let background = primary_colors.color;
    let disabled = theme.colorscheme.surface.on_surface;

    let shadow_color = theme.colorscheme.shadow;

    button(
        foreground,
        background,
        foreground,
        disabled,
        shadow_color,
        0,
        status,
    )
}

pub fn filled_tonal(theme: &Theme, status: Status) -> Style {
    let secondary_colors = theme.colorscheme.secondary;

    let foreground = secondary_colors.on_secondary_container;
    let background = secondary_colors.secondary_container;
    let disabled = theme.colorscheme.surface.on_surface;
    let shadow_color = theme.colorscheme.shadow;

    button(
        foreground,
        background,
        foreground,
        disabled,
        shadow_color,
        0,
        status,
    )
}

pub fn outlined(theme: &Theme, status: Status) -> Style {
    let foreground = theme.colorscheme.primary.color;
    let background = Color::TRANSPARENT;
    let disabled = theme.colorscheme.surface.on_surface;

    let outline = theme.colorscheme.outline.color;

    let border = match status {
        Status::Active | Status::Pressed | Status::Hovered => Border {
            color: outline,
            width: 1.0,
            radius: 400.0.into(),
        },
        Status::Disabled => Border {
            color: Color {
                a: DISABLED_CONTAINER_OPACITY,
                ..disabled
            },
            width: 1.0,
            radius: 400.0.into(),
        },
    };

    let style = button(
        foreground,
        background,
        foreground,
        disabled,
        Color::TRANSPARENT,
        0,
        status,
    );

    Style { border, ..style }
}

pub fn text(theme: &Theme, status: Status) -> Style {
    let foreground = theme.colorscheme.primary.color;
    let background = Color::TRANSPARENT;
    let disabled = theme.colorscheme.surface.on_surface;

    let style = button(
        foreground,
        background,
        foreground,
        disabled,
        Color::TRANSPARENT,
        0,
        status,
    );

    match status {
        Status::Hovered | Status::Pressed => style,
        _ => Style {
            background: None,
            ..style
        },
    }
}
