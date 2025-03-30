#![allow(dead_code)]
use iced::widget::text::{Catalog, Style, StyleFn};

use super::OtherTheme;

impl Catalog for OtherTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(none)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn none(_: &OtherTheme) -> Style {
    Style { color: None }
}

pub fn primary(theme: &OtherTheme) -> Style {
    Style {
        color: Some(theme.colorscheme.primary.on_primary),
    }
}

pub fn secondary(theme: &OtherTheme) -> Style {
    Style {
        color: Some(theme.colorscheme.secondary.on_secondary),
    }
}

pub fn tertiary(theme: &OtherTheme) -> Style {
    Style {
        color: Some(theme.colorscheme.tertiary.on_tertiary),
    }
}

pub fn error(theme: &OtherTheme) -> Style {
    Style {
        color: Some(theme.colorscheme.error.on_error),
    }
}

pub fn surface(theme: &OtherTheme) -> Style {
    Style {
        color: Some(theme.colorscheme.surface.on_surface),
    }
}
