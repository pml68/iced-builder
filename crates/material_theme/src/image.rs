use iced_widget::core::{Background, Border, Color, border};
use iced_widget::image::{Catalog, Style, StyleFn};

use super::Theme;
use crate::utils::{elevation, shadow_from_elevation};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(_theme: &Theme) -> Style {
    Style::default()
}
