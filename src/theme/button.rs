#![allow(dead_code)]
use iced::widget::button::{Catalog, Status, Style, StyleFn};
use iced::{Background, Border, Color, Shadow, Vector};

use super::OtherTheme;

impl Catalog for OtherTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

fn default(theme: &OtherTheme, status: Status) -> Style {
    filled(theme, status)
}

fn button(
    foreground: Color,
    background: Color,
    background_hover: Color,
    disabled: Color,
    shadow_color: Color,
    shadow_elevation: u8,
    status: Status,
) -> Style {
    let border = Border {
        radius: 400.0.into(),
        ..Default::default()
    };

    let elevation_to_offset = |elevation: u8| {
        (match elevation {
            0 => 0.0,
            1 => 1.0,
            2 => 3.0,
            3 => 6.0,
            4 => 8.0,
            _ => 12.0,
        } as f32)
    };

    match status {
        Status::Active | Status::Pressed => Style {
            background: Some(Background::Color(background)),
            text_color: foreground,
            border,
            shadow: Shadow {
                color: shadow_color,
                offset: Vector {
                    x: 0.0,
                    y: elevation_to_offset(shadow_elevation),
                },
                blur_radius: elevation_to_offset(shadow_elevation)
                    * (1.0
                        + 0.4_f32.powf(elevation_to_offset(shadow_elevation))),
            },
        },
        Status::Hovered => Style {
            background: Some(Background::Color(background_hover)),
            text_color: foreground,
            border,
            shadow: Shadow {
                color: shadow_color,
                offset: Vector {
                    x: 0.0,
                    y: elevation_to_offset(shadow_elevation + 1),
                },
                blur_radius: (elevation_to_offset(shadow_elevation + 1))
                    * (1.0
                        + 0.4_f32
                            .powf(elevation_to_offset(shadow_elevation + 1))),
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

pub fn elevated(theme: &OtherTheme, status: Status) -> Style {
    let surface_colors = theme.colorscheme.surface;

    let foreground = theme.colorscheme.primary.color;
    let background = surface_colors.surface_container.low;
    let disabled = surface_colors.on_surface;

    let shadow_color = theme.colorscheme.shadow;

    button(
        foreground,
        background,
        background,
        disabled,
        shadow_color,
        1,
        status,
    )
}

pub fn filled(theme: &OtherTheme, status: Status) -> Style {
    let primary_colors = theme.colorscheme.primary;

    let foreground = primary_colors.on_primary;
    let background = primary_colors.color;
    let disabled = theme.colorscheme.surface.on_surface;

    let shadow_color = theme.colorscheme.shadow;

    button(
        foreground,
        background,
        background,
        disabled,
        shadow_color,
        0,
        status,
    )
}

pub fn filled_tonal(theme: &OtherTheme, status: Status) -> Style {
    let secondary_colors = theme.colorscheme.secondary;

    let foreground = secondary_colors.on_secondary_container;
    let background = secondary_colors.secondary_container;
    let disabled = theme.colorscheme.surface.on_surface;

    let shadow_color = theme.colorscheme.shadow;

    button(
        foreground,
        background,
        background,
        disabled,
        shadow_color,
        0,
        status,
    )
}

pub fn outlined(theme: &OtherTheme, status: Status) -> Style {
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
        background,
        disabled,
        Color::TRANSPARENT,
        0,
        status,
    );

    Style { border, ..style }
}
