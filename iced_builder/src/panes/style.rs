use iced::widget::container::Style;
use iced::{Border, Theme};

pub fn title_bar(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        text_color: Some(palette.background.strong.text),
        background: Some(palette.background.strong.color.into()),
        ..Default::default()
    }
}

pub fn pane_active(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 1.0,
            color: palette.background.strong.color,
            ..Border::default()
        },
        ..Default::default()
    }
}

pub fn pane_focused(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 4.0,
            color: palette.background.strong.color,
            ..Border::default()
        },
        ..Default::default()
    }
}
