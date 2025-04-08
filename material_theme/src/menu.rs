use iced_widget::core::{Background, border};
use iced_widget::overlay::menu::{Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(theme: &Theme) -> Style {
    let surface = theme.colorscheme.surface;
    let secondary = theme.colorscheme.secondary;

    Style {
        border: border::rounded(4),
        background: Background::Color(surface.surface_container.base),
        text_color: surface.on_surface,
        selected_background: Background::Color(secondary.secondary_container),
        selected_text_color: secondary.on_secondary_container,
    }
}
