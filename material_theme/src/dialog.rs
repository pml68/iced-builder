use iced_widget::container::Style;
use iced_widget::core::{Background, border};

use super::{Theme, text};

impl iced_dialog::dialog::Catalog for Theme {
    fn default_container<'a>()
    -> <Self as iced_widget::container::Catalog>::Class<'a> {
        Box::new(default_container)
    }

    fn default_title<'a>() -> <Self as iced_widget::text::Catalog>::Class<'a> {
        Box::new(text::surface)
    }
}

pub fn default_container(theme: &Theme) -> Style {
    let colors = theme.colorscheme.surface;
    Style {
        background: Some(Background::Color(colors.surface_container.high)),
        text_color: Some(colors.on_surface_variant),
        border: border::rounded(28),
        ..Style::default()
    }
}
