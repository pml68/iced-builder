use iced::Element;
use iced::widget::{button, column, container, row, text};
use material_theme::Theme;
use material_theme::button::{
    elevated, filled_tonal, outlined, text as text_style,
};
use material_theme::container::{
    error, error_container, inverse_surface, primary, primary_container,
    secondary, secondary_container, surface, surface_container,
    surface_container_high, surface_container_highest, surface_container_low,
    surface_container_lowest, tertiary, tertiary_container,
};
use material_theme::text::surface_variant;

fn main() {
    iced::application("Theme Test", (), view)
        .theme(|_| material_theme::DARK.clone())
        .run()
        .unwrap();
}

#[derive(Debug, Clone)]
enum Message {
    Noop,
}

fn view(_: &()) -> Element<'_, Message, Theme> {
    container(
        row![
            column![
                button("Disabled"),
                button("Filled").on_press(Message::Noop),
                button("Filled Tonal")
                    .on_press(Message::Noop)
                    .style(filled_tonal),
                button("Elevated").on_press(Message::Noop).style(elevated),
                button("Outlined").on_press(Message::Noop).style(outlined),
                button("Text").on_press(Message::Noop).style(text_style),
                button("Text Disabled").style(text_style),
            ]
            .spacing(10),
            column![
                text("None"),
                container("Primary").padding(8).style(primary),
                container("Primary Container")
                    .padding(8)
                    .style(primary_container),
                container("Secondary").padding(8).style(secondary),
                container("Secondary Container")
                    .padding(8)
                    .style(secondary_container),
                container("Tertiary").padding(8).style(tertiary),
                container("Tertiary Container")
                    .padding(8)
                    .style(tertiary_container),
                container("Error").padding(8).style(error),
                container("Error Container")
                    .padding(8)
                    .style(error_container),
                container("Surface").padding(8).style(surface),
                container(text("Surface Variant").style(surface_variant))
                    .padding(8)
                    .style(surface),
                container("Inverse Surface")
                    .padding(8)
                    .style(inverse_surface),
                container("Surface Container Lowest")
                    .padding(8)
                    .style(surface_container_lowest),
                container("Surface Container Low")
                    .padding(8)
                    .style(surface_container_low),
                container("Surface Container")
                    .padding(8)
                    .style(surface_container),
                container("Surface Container High")
                    .padding(8)
                    .style(surface_container_high),
                container("Surface Container Highest")
                    .padding(8)
                    .style(surface_container_highest),
            ]
            .spacing(10)
        ]
        .spacing(20),
    )
    .padding(12)
    .into()
}
