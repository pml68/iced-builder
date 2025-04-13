use iced_widget::container::{Catalog, Style, StyleFn};
use iced_widget::core::{Background, border};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn transparent(_theme: &Theme) -> Style {
    Style {
        border: border::rounded(4),
        ..Style::default()
    }
}

pub fn primary(theme: &Theme) -> Style {
    let colors = theme.colorscheme.primary;
    Style {
        background: Some(Background::Color(colors.color)),
        text_color: Some(colors.on_primary),
        border: border::rounded(4),
        ..Style::default()
    }
}

pub fn primary_container(theme: &Theme) -> Style {
    let colors = theme.colorscheme.primary;
    Style {
        background: Some(Background::Color(colors.primary_container)),
        text_color: Some(colors.on_primary_container),
        border: border::rounded(8),
        ..Style::default()
    }
}

pub fn secondary(theme: &Theme) -> Style {
    let colors = theme.colorscheme.secondary;
    Style {
        background: Some(Background::Color(colors.color)),
        text_color: Some(colors.on_secondary),
        border: border::rounded(4),
        ..Style::default()
    }
}

pub fn secondary_container(theme: &Theme) -> Style {
    let colors = theme.colorscheme.secondary;
    Style {
        background: Some(Background::Color(colors.secondary_container)),
        text_color: Some(colors.on_secondary_container),
        border: border::rounded(8),
        ..Style::default()
    }
}

pub fn tertiary(theme: &Theme) -> Style {
    let colors = theme.colorscheme.tertiary;
    Style {
        background: Some(Background::Color(colors.color)),
        text_color: Some(colors.on_tertiary),
        border: border::rounded(4),
        ..Style::default()
    }
}

pub fn tertiary_container(theme: &Theme) -> Style {
    let colors = theme.colorscheme.tertiary;
    Style {
        background: Some(Background::Color(colors.tertiary_container)),
        text_color: Some(colors.on_tertiary_container),
        border: border::rounded(8),
        ..Style::default()
    }
}

pub fn error(theme: &Theme) -> Style {
    let colors = theme.colorscheme.error;
    Style {
        background: Some(Background::Color(colors.color)),
        text_color: Some(colors.on_error),
        border: border::rounded(4),
        ..Style::default()
    }
}

pub fn error_container(theme: &Theme) -> Style {
    let colors = theme.colorscheme.error;
    Style {
        background: Some(Background::Color(colors.error_container)),
        text_color: Some(colors.on_error_container),
        border: border::rounded(8),
        ..Style::default()
    }
}

pub fn surface(theme: &Theme) -> Style {
    let colors = theme.colorscheme.surface;
    Style {
        background: Some(Background::Color(colors.color)),
        text_color: Some(colors.on_surface),
        border: border::rounded(4),
        ..Style::default()
    }
}

pub fn surface_container_lowest(theme: &Theme) -> Style {
    let colors = theme.colorscheme.surface;
    Style {
        background: Some(Background::Color(colors.surface_container.lowest)),
        text_color: Some(colors.on_surface),
        border: border::rounded(8),
        ..Style::default()
    }
}

pub fn surface_container_low(theme: &Theme) -> Style {
    let colors = theme.colorscheme.surface;
    Style {
        background: Some(Background::Color(colors.surface_container.low)),
        text_color: Some(colors.on_surface),
        border: border::rounded(8),
        ..Style::default()
    }
}

pub fn surface_container(theme: &Theme) -> Style {
    let colors = theme.colorscheme.surface;
    Style {
        background: Some(Background::Color(colors.surface_container.base)),
        text_color: Some(colors.on_surface),
        border: border::rounded(8),
        ..Style::default()
    }
}

pub fn surface_container_high(theme: &Theme) -> Style {
    let colors = theme.colorscheme.surface;
    Style {
        background: Some(Background::Color(colors.surface_container.high)),
        text_color: Some(colors.on_surface),
        border: border::rounded(8),
        ..Style::default()
    }
}

pub fn surface_container_highest(theme: &Theme) -> Style {
    let colors = theme.colorscheme.surface;
    Style {
        background: Some(Background::Color(colors.surface_container.highest)),
        text_color: Some(colors.on_surface),
        border: border::rounded(8),
        ..Style::default()
    }
}

pub fn inverse_surface(theme: &Theme) -> Style {
    let colors = theme.colorscheme.inverse;
    Style {
        background: Some(Background::Color(colors.inverse_surface)),
        text_color: Some(colors.inverse_on_surface),
        border: border::rounded(4),
        ..Style::default()
    }
}
