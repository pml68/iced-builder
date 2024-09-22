mod codegen;
mod types;

use iced::{
    clipboard, executor,
    highlighter::{self, Highlighter},
    theme,
    widget::{
        button, column, container,
        pane_grid::{self, Pane, PaneGrid},
        row, text, text_editor, tooltip, Column, Space,
    },
    Alignment, Application, Color, Command, Element, Font, Length, Settings,
};
use types::{rendered_element::RenderedElement, DesignerPage, DesignerState};

fn main() -> iced::Result {
    App::run(Settings {
        fonts: vec![include_bytes!("../fonts/icons.ttf").as_slice().into()],
        ..Settings::default()
    })
}

struct App {
    is_saved: bool,
    current_project: Option<String>,
    dark_theme: bool,
    pane_state: pane_grid::State<Panes>,
    focus: Option<Pane>,
    designer_state: DesignerState,
    element_list: Vec<String>,
    editor_content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleTheme,
    CopyCode,
    Resized(pane_grid::ResizeEvent),
    Clicked(pane_grid::Pane),
}

#[derive(Clone, Debug)]
enum Panes {
    Designer,
    ElementList,
}

impl Application for App {
    type Message = Message;
    type Theme = theme::Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let state = pane_grid::State::with_configuration(pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.8,
            a: Box::new(pane_grid::Configuration::Pane(Panes::Designer)),
            b: Box::new(pane_grid::Configuration::Pane(Panes::ElementList)),
        });
        (
            Self {
                is_saved: true,
                current_project: None,
                dark_theme: true,
                pane_state: state,
                focus: None,
                designer_state: DesignerState {
                    designer_content: vec![],
                    designer_page: DesignerPage::CodeView,
                },
                element_list: vec!["Column", "Row", "PickList", "PaneGrid", "Button", "Text"]
                    .into_iter()
                    .map(|c| c.to_owned())
                    .collect(),
                editor_content: text_editor::Content::with_text(&RenderedElement::test()),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let saved_state = if self.is_saved { "" } else { " *" };

        let project_name = match &self.current_project {
            Some(n) => format!(" - {n}"),
            None => "".to_owned(),
        };

        format!("iced Builder{project_name}{saved_state}")
    }

    fn theme(&self) -> iced::Theme {
        if self.dark_theme {
            theme::Theme::CatppuccinMocha
        } else {
            theme::Theme::CatppuccinLatte
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleTheme => self.dark_theme = !self.dark_theme,
            Message::CopyCode => return clipboard::write(self.editor_content.text()),
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.pane_state.resize(split, ratio);
            }
            Message::Clicked(pane) => {
                self.focus = Some(pane);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let header = row![button("Toggle Theme")
            .on_press(Message::ToggleTheme)
            .padding(5)]
        .width(200);
        let pane_grid = PaneGrid::new(&self.pane_state, |id, pane, _is_maximized| {
            let is_focused = Some(id) == self.focus;
            match pane {
                Panes::Designer => match self.designer_state.designer_page {
                    DesignerPage::Designer => {
                        let content = column![text("Designer"),]
                            .align_items(Alignment::Center)
                            .height(Length::Fill)
                            .width(Length::Fill);
                        let title = text("Designer").style(if is_focused {
                            PANE_ID_COLOR_FOCUSED
                        } else {
                            PANE_ID_COLOR_UNFOCUSED
                        });
                        let title_bar = pane_grid::TitleBar::new(title)
                            .padding(10)
                            .style(style::title_bar);
                        pane_grid::Content::new(content)
                            .title_bar(title_bar)
                            .style(if is_focused {
                                style::pane_focused
                            } else {
                                style::pane_active
                            })
                    }
                    DesignerPage::CodeView => {
                        let title = row![
                            text("Generated Code").style(if is_focused {
                                PANE_ID_COLOR_FOCUSED
                            } else {
                                PANE_ID_COLOR_UNFOCUSED
                            }),
                            Space::with_width(Length::Fill),
                            tooltip(
                                button(
                                    container(
                                        text('\u{0e801}').font(Font::with_name("editor-icons"))
                                    )
                                    .width(30)
                                    .center_x()
                                )
                                .on_press(Message::CopyCode),
                                "Copy code to clipboard",
                                tooltip::Position::Left
                            )
                        ];
                        let title_bar = pane_grid::TitleBar::new(title)
                            .padding(10)
                            .style(style::title_bar);
                        pane_grid::Content::new(
                            text_editor(&self.editor_content)
                                .highlight::<Highlighter>(
                                    highlighter::Settings {
                                        theme: highlighter::Theme::Base16Mocha,
                                        extension: "rs".to_string(),
                                    },
                                    |highlight, _theme| highlight.to_format(),
                                )
                                .height(Length::Fill)
                                .padding(20),
                        )
                        .title_bar(title_bar)
                        .style(if is_focused {
                            style::pane_focused
                        } else {
                            style::pane_active
                        })
                    }
                },
                Panes::ElementList => {
                    let items_list = items_list_view(&self.element_list);
                    let content = column![items_list]
                        .align_items(Alignment::Center)
                        .height(Length::Fill)
                        .width(Length::Fill);
                    let title = text("Element List").style(if is_focused {
                        PANE_ID_COLOR_FOCUSED
                    } else {
                        PANE_ID_COLOR_UNFOCUSED
                    });
                    let title_bar = pane_grid::TitleBar::new(title)
                        .padding(10)
                        .style(style::title_bar);
                    pane_grid::Content::new(content)
                        .title_bar(title_bar)
                        .style(if is_focused {
                            style::pane_focused
                        } else {
                            style::pane_active
                        })
                }
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .on_resize(10, Message::Resized)
        .on_click(Message::Clicked);

        let content = Column::new()
            .push(header)
            .push(pane_grid)
            .spacing(5)
            .align_items(Alignment::Center)
            .width(Length::Fill);

        container(content).height(Length::Fill).into()
    }
}

const fn from_grayscale(grayscale: f32) -> Color {
    Color {
        r: grayscale,
        g: grayscale,
        b: grayscale,
        a: 1.0,
    }
}

// #ffffff
const PANE_ID_COLOR_FOCUSED: Color = from_grayscale(1.0);

// #e8e8e8
const PANE_ID_COLOR_UNFOCUSED: Color = from_grayscale(0xE8 as f32 / 255.0);

fn items_list_view(items: &Vec<String>) -> Element<'static, Message> {
    let mut column = Column::new()
        .spacing(20)
        .align_items(Alignment::Center)
        .width(Length::Fill);

    for value in items {
        column = column.push(text(value));
    }

    container(column).height(250.0).width(300).into()
}

mod style {
    use iced::widget::container;
    use iced::{Border, Theme};

    pub fn title_bar(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(palette.background.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 1.0,
                color: palette.background.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 4.0,
                color: palette.background.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
