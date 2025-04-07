#![allow(dead_code)]
use iced_widget::button::{Catalog, Status, Style, StyleFn};
use iced_widget::core::{Background, Border, Color, Shadow, Vector};

use crate::Theme;
use crate::utils::{elevation, mix};

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
    shadow_elevation: u8,
    status: Status,
) -> Style {
    let border = Border {
        radius: 400.into(),
        ..Default::default()
    };

    let active = Style {
        background: Some(Background::Color(background)),
        text_color: foreground,
        border,
        shadow: Shadow {
            color: shadow_color,
            offset: Vector {
                x: 0.0,
                y: elevation(shadow_elevation),
            },
            blur_radius: elevation(shadow_elevation)
                * (1.0 + 0.4_f32.powf(elevation(shadow_elevation))),
        },
    };

    match status {
        Status::Active => active,
        Status::Pressed => Style {
            background: Some(Background::Color(mix(
                background,
                tone_overlay,
                0.08,
            ))),
            ..active
        },
        Status::Hovered => Style {
            background: Some(Background::Color(mix(
                background,
                tone_overlay,
                0.1,
            ))),
            text_color: foreground,
            border,
            shadow: Shadow {
                color: shadow_color,
                offset: Vector {
                    x: 0.0,
                    y: elevation(shadow_elevation + 1),
                },
                blur_radius: (elevation(shadow_elevation + 1))
                    * (1.0 + 0.4_f32.powf(elevation(shadow_elevation + 1))),
            },
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color {
                a: 0.12,
                ..disabled
            })),
            text_color: Color {
                a: 0.38,
                ..disabled
            },
            border,
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
                a: 0.12,
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
