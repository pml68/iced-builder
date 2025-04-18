use iced_widget::core::border::Radius;
use iced_widget::rule::{Catalog, FillMode, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(inset)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn inset(theme: &Theme) -> Style {
    Style {
        color: theme.colorscheme.outline.variant,
        fill_mode: FillMode::Padded(8),
        width: 1,
        radius: Radius::default(),
    }
}
pub fn full_width(theme: &Theme) -> Style {
    Style {
        color: theme.colorscheme.outline.variant,
        fill_mode: FillMode::Full,
        width: 1,
        radius: Radius::default(),
    }
}
