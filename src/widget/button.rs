use iced::widget::button::{Status, Style};
use iced_material::Theme;
use iced_material::button::styled;

pub fn danger(theme: &Theme, status: Status) -> Style {
    let primary = theme.colors().error;

    let foreground = primary.on_error;
    let background = primary.color;
    let disabled = theme.colors().surface.on_surface;

    let shadow_color = theme.colors().shadow;

    styled(background, foreground, disabled, shadow_color, 0, status)
}
