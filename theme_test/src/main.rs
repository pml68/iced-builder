use iced::widget::{
    button, center, checkbox, column, container, horizontal_rule, pane_grid,
    pick_list, progress_bar, radio, row, slider, text_editor, text_input,
    toggler,
};
use iced::{Element, Length};
use iced_anim::{Animated, Animation, Event};
use iced_dialog::dialog;
use material_theme::Theme;
use material_theme::button::{elevated, filled_tonal, outlined, text};
use material_theme::container::{
    error, error_container, inverse_surface, primary, primary_container,
    secondary, secondary_container, surface, surface_container,
    surface_container_high, surface_container_highest, surface_container_low,
    surface_container_lowest, tertiary, tertiary_container,
};
use material_theme::text::surface_variant;

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
    Input(String),
    Bool(bool),
    Radio(Choice),
    Slider(f32),
    Edit(text_editor::Action),
    Resize(pane_grid::ResizeEvent),
    SwitchTheme(Event<Theme>),
}

#[derive(Debug)]
pub struct State {
    theme: Animated<Theme>,
    show_dialog: bool,
    content: String,
    is_checked: bool,
    selection: Option<Choice>,
    value: f32,
    editor_content: text_editor::Content,
    panes: pane_grid::State<Pane>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            show_dialog: Default::default(),
            content: Default::default(),
            is_checked: Default::default(),
            selection: Default::default(),
            value: Default::default(),
            editor_content: text_editor::Content::new(),
            panes: pane_grid::State::with_configuration(
                pane_grid::Configuration::Split {
                    axis: pane_grid::Axis::Vertical,
                    ratio: 0.5,
                    a: Box::new(pane_grid::Configuration::Pane(Pane::Left)),
                    b: Box::new(pane_grid::Configuration::Pane(Pane::Right)),
                },
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Left,
    Right,
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
            Message::Input(content) => self.content = content,
            Message::Bool(is_checked) => self.is_checked = is_checked,
            Message::Radio(choice) => self.selection = Some(choice),
            Message::Slider(value) => self.value = value,
            Message::Edit(action) => self.editor_content.perform(action),
            Message::Resize(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::SwitchTheme(event) => {
                self.theme.update(event);
            }
        }
    }
    fn view(&self) -> Element<'_, Message, Theme> {
        let base: pane_grid::PaneGrid<'_, Message, Theme> =
            pane_grid(&self.panes, |_pane, state, _is_maximized| {
                pane_grid::Content::new(match state {
                    Pane::Left => container(
                        row![
                            column![
                                button("Disabled"),
                                button("Filled").on_press(Message::Noop),
                                button("Filled Tonal")
                                    .on_press(Message::Noop)
                                    .style(filled_tonal),
                                button("Elevated")
                                    .on_press(Message::Noop)
                                    .style(elevated),
                                button("Outlined")
                                    .on_press(Message::Noop)
                                    .style(outlined),
                                button("Text")
                                    .on_press(Message::Noop)
                                    .style(text),
                                button("Text Disabled").style(text),
                            ]
                            .spacing(10),
                            column![
                                container("None").padding(8),
                                container("Primary").padding(8).style(primary),
                                container("Primary Container")
                                    .padding(8)
                                    .style(primary_container),
                                container("Secondary")
                                    .padding(8)
                                    .style(secondary),
                                container("Secondary Container")
                                    .padding(8)
                                    .style(secondary_container),
                                container("Tertiary")
                                    .padding(8)
                                    .style(tertiary),
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
                        ]
                        .spacing(10),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(12),

                    Pane::Right => container(
                        column![
                            // Pick List
                            pick_list(
                                Theme::ALL,
                                Some(self.theme.target()),
                                |theme| Message::SwitchTheme(theme.into())
                            )
                            .placeholder("Select a theme..."),
                            horizontal_rule(1),
                            // Dialog
                            button("Open Dialog").on_press(Message::OpenDialog),
                            horizontal_rule(1),
                            // Text Input
                            text_input("Type something here...", &self.content)
                                .on_input(Message::Input),
                            text_input("Disabled", "Disabled"),
                            horizontal_rule(1),
                            // Checkbox
                            checkbox("Normal", self.is_checked)
                                .on_toggle(Message::Bool),
                            checkbox("Error", self.is_checked)
                                .on_toggle(Message::Bool)
                                .style(material_theme::checkbox::error),
                            checkbox("Disabled", self.is_checked),
                            horizontal_rule(1),
                            // Radio
                            radio(
                                "A",
                                Choice::A,
                                self.selection,
                                Message::Radio,
                            ),
                            radio(
                                "B",
                                Choice::B,
                                self.selection,
                                Message::Radio,
                            ),
                            radio(
                                "C",
                                Choice::C,
                                self.selection,
                                Message::Radio,
                            ),
                            horizontal_rule(1),
                            // Slider
                            center(iced::widget::text!("{:.1}", self.value))
                                .width(Length::Fill)
                                .height(Length::Shrink),
                            slider(0.0..=100.0, self.value, Message::Slider)
                                .step(0.1),
                            progress_bar(0.0..=100.0, self.value),
                            horizontal_rule(1),
                            // Toggler
                            toggler(self.is_checked)
                                .on_toggle(Message::Bool)
                                .size(24.0),
                            toggler(self.is_checked).size(24.0),
                            horizontal_rule(1),
                            // Text Editor
                            text_editor(&self.editor_content)
                                .on_action(Message::Edit),
                            text_editor(&self.editor_content)
                                .placeholder("Disabled")
                        ]
                        .spacing(10),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(12),
                })
            })
            .on_resize(10, Message::Resize);

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
