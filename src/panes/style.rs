use iced::widget::container::Style;
use iced::{Background, Border};
use iced_material::Theme;

pub fn title_bar(theme: &Theme) -> Style {
    let surface = theme.colors().surface;

    Style {
        text_color: Some(surface.on_surface),
        background: Some(Background::Color(surface.surface_container.high)),
        ..Default::default()
    }
}

pub fn pane_active(theme: &Theme) -> Style {
    let surface = theme.colors().surface;

    Style {
        background: Some(Background::Color(surface.surface_container.low)),
        border: Border {
            width: 1.0,
            color: surface.surface_container.high,
            ..Border::default()
        },
        ..Default::default()
    }
}

pub fn pane_focused(theme: &Theme) -> Style {
    let surface = theme.colors().surface;

    Style {
        background: Some(Background::Color(surface.surface_container.low)),
        border: Border {
            width: 2.0,
            color: surface.surface_container.high,
            ..Border::default()
        },
        ..Default::default()
    }
}
