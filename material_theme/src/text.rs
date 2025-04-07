#![allow(dead_code)]
use iced_widget::text::{Catalog, Style, StyleFn};

use crate::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(none)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn none(_: &Theme) -> Style {
    Style { color: None }
}

pub fn primary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colorscheme.primary.on_primary),
    }
}

pub fn primary_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colorscheme.primary.on_primary_container),
    }
}

pub fn secondary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colorscheme.secondary.on_secondary),
    }
}

pub fn secondary_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colorscheme.secondary.on_secondary_container),
    }
}

pub fn tertiary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colorscheme.tertiary.on_tertiary),
    }
}

pub fn tertiary_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colorscheme.tertiary.on_tertiary_container),
    }
}

pub fn error(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colorscheme.error.on_error),
    }
}

pub fn error_container(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colorscheme.error.on_error_container),
    }
}

pub fn surface(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colorscheme.surface.on_surface),
    }
}

pub fn surface_variant(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colorscheme.surface.on_surface_variant),
    }
}

pub fn inverse_surface(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colorscheme.inverse.inverse_on_surface),
    }
}
