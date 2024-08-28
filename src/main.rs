use iced::{
    executor, theme,
    widget::{
        button, column, container,
        pane_grid::{self, Pane, PaneGrid},
        row, text, Column,
    },
    Alignment, Application, Color, Command, Element, Length, Settings,
};

fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    is_saved: bool,
    current_project: Option<String>,
    theme: theme::Theme,
    panes: pane_grid::State<Panes>,
    focus: Option<Pane>,
    element_list: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleTheme(theme::Theme),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
}

#[derive(Clone, Debug)]
enum Panes {
    Preview,
    ElementList,
}

impl Application for App {
    type Message = Message;
    type Theme = theme::Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (mut panes, pane) = pane_grid::State::new(Panes::Preview);
        panes.split(pane_grid::Axis::Vertical, pane, Panes::ElementList);
        (
            Self {
                is_saved: true,
                current_project: None,
                theme: theme::Theme::TokyoNight,
                panes,
                focus: None,
                element_list: vec!["Column", "Row", "PickList", "PaneGrid", "Button", "Text"]
                    .into_iter()
                    .map(|c| c.to_owned())
                    .collect(),
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
        self.theme.clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleTheme(theme) => self.theme = theme,
            Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            Message::Dragged(_) => {}
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let header: Element<_> = row![button("Toggle Theme")
            .on_press(Message::ToggleTheme)
            .padding(5)]
        .width(200)
        .into();
        let pane_grid = PaneGrid::new(&self.panes, |id, pane, _is_maximized| {
            let is_focused = Some(id) == self.focus;
            match pane {
                Panes::Preview => {
                    let content = column![text("Preview")]
                        .align_items(Alignment::Center)
                        .height(Length::Fill)
                        .width(Length::Fill);
                    let title = text("App Preview").style(if is_focused {
                        PANE_ID_COLOR_FOCUSED
                    } else {
                        PANE_ID_COLOR_UNFOCUSED
                    });
                    let title_bar =
                        pane_grid::TitleBar::new(title)
                            .padding(10)
                            .style(if is_focused {
                                style::title_bar_focused
                            } else {
                                style::title_bar_active
                            });
                    pane_grid::Content::new(content)
                        .title_bar(title_bar)
                        .style(if is_focused {
                            style::pane_focused
                        } else {
                            style::pane_active
                        })
                }
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
                    let title_bar =
                        pane_grid::TitleBar::new(title)
                            .padding(10)
                            .style(if is_focused {
                                style::title_bar_focused
                            } else {
                                style::title_bar_active
                            });
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
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);
        //
        //let content = Column::new()
        //    .push(header)
        //    .push(pane_grid)
        //    .spacing(5)
        //    .align_items(Alignment::Center)
        //    .width(Length::Fill);

        container(pane_grid).height(Length::Fill).into()
    }
}

// #fefefe
const PANE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
    0xFE as f32 / 255.0,
    0xFE as f32 / 255.0,
    0xFE as f32 / 255.0,
);

// #bbbbbb
const PANE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
    0xBB as f32 / 255.0,
    0xBB as f32 / 255.0,
    0xBB as f32 / 255.0,
);

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

    pub fn title_bar_active(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(palette.background.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.primary.strong.text),
            background: Some(palette.primary.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
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
                width: 2.0,
                color: palette.primary.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
