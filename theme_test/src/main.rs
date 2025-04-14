use iced::Element;
use iced::Length::Fill;
use iced::widget::{button, column, container, pick_list, row};
use iced_anim::{Animated, Animation, Event};
use iced_dialog::dialog;
use material_theme::button::{elevated, filled_tonal, outlined, text};
use material_theme::container::{
    error, error_container, inverse_surface, primary, primary_container,
    secondary, secondary_container, surface, surface_container,
    surface_container_high, surface_container_highest, surface_container_low,
    surface_container_lowest, tertiary, tertiary_container,
};
use material_theme::text::surface_variant;
use material_theme::{DARK, LIGHT, Theme};

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .theme(|state| state.theme.value().clone())
        .run()
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message {
    Noop,
    OpenDialog,
    CloseDialog,
    SwitchTheme(Event<Theme>),
}

#[derive(Debug, Default)]
pub struct State {
    show_dialog: bool,
    theme: Animated<Theme>,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Noop => {}
            Message::OpenDialog => {
                self.show_dialog = true;
            }
            Message::CloseDialog => {
                self.show_dialog = false;
            }
            Message::SwitchTheme(event) => {
                self.theme.update(event);
            }
        }
    }
    fn view(&self) -> Element<'_, Message, Theme> {
        let base = container(
            row![
                column![
                    button("Disabled"),
                    button("Filled").on_press(Message::Noop),
                    button("Filled Tonal")
                        .on_press(Message::Noop)
                        .style(filled_tonal),
                    button("Elevated").on_press(Message::Noop).style(elevated),
                    button("Outlined").on_press(Message::Noop).style(outlined),
                    button("Text").on_press(Message::Noop).style(text),
                    button("Text Disabled").style(text),
                ]
                .spacing(10),
                column![
                    container("None").padding(8),
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
                    container(
                        iced::widget::text("Surface Variant")
                            .style(surface_variant)
                    )
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
                .spacing(10),
                pick_list(
                    [LIGHT.clone(), DARK.clone()],
                    Some(self.theme.target()),
                    |theme| Message::SwitchTheme(theme.into())
                )
                .placeholder("Select a theme..."),
                button("Open Dialog").on_press(Message::OpenDialog)
            ]
            .spacing(20),
        )
        .width(Fill)
        .height(Fill)
        .padding(12);

        let dialog =
            dialog(self.show_dialog, base, iced::widget::text("Say Hi!"))
                .title("This is a Dialog.")
                .push_button(iced_dialog::button("Hi!", Message::CloseDialog))
                .width(280)
                .height(186);

        Animation::new(&self.theme, dialog)
            .on_update(Message::SwitchTheme)
            .into()
    }
}
