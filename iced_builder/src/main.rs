use std::path::PathBuf;

use iced::{
    advanced::widget::Id,
    clipboard, keyboard,
    widget::{
        button, container,
        pane_grid::{self, Pane, PaneGrid},
        row, text_editor, Column,
    },
    Alignment, Element, Length, Settings, Task, Theme,
};
use iced_builder::{
    types::{element_name::ElementName, project::Project, rendered_element::Action, DesignerPage},
    views::{code_view, designer_view, element_list},
    Message,
};

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .settings(Settings {
            fonts: vec![include_bytes!("../fonts/icons.ttf").as_slice().into()],
            ..Settings::default()
        })
        .theme(App::theme)
        .subscription(App::subscription)
        .run_with(App::new)
}

struct App {
    is_dirty: bool,
    is_loading: bool,
    project_path: Option<PathBuf>,
    project: Project,
    dark_theme: bool,
    pane_state: pane_grid::State<Panes>,
    focus: Option<Pane>,
    designer_page: DesignerPage,
    element_list: Vec<ElementName>,
    editor_content: text_editor::Content,
}

#[derive(Clone, Copy, Debug)]
enum Panes {
    Designer,
    ElementList,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let state = pane_grid::State::with_configuration(pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.8,
            a: Box::new(pane_grid::Configuration::Pane(Panes::Designer)),
            b: Box::new(pane_grid::Configuration::Pane(Panes::ElementList)),
        });
        (
            Self {
                is_dirty: false,
                is_loading: false,
                project_path: None,
                project: Project::new(),
                dark_theme: true,
                pane_state: state,
                focus: None,
                designer_page: DesignerPage::Designer,
                element_list: ElementName::ALL.to_vec(),
                editor_content: text_editor::Content::new(),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        let saved_state = if !self.is_dirty { "" } else { " *" };

        let project_name = match &self.project.title {
            Some(n) => {
                format!(
                    " - {}",
                    if n.len() > 60 {
                        format!("...{}", &n[n.len() - 40..])
                    } else {
                        n.to_owned()
                    }
                )
            }
            None => "".to_owned(),
        };

        format!("iced Builder{project_name}{saved_state}")
    }

    fn theme(&self) -> iced::Theme {
        if self.dark_theme {
            Theme::SolarizedDark
        } else {
            Theme::SolarizedLight
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleTheme => self.dark_theme = !self.dark_theme,
            Message::CopyCode => return clipboard::write(self.editor_content.text()),
            Message::SwitchPage(page) => self.designer_page = page,
            Message::EditorAction(action) => {
                if let text_editor::Action::Scroll { lines: _ } = action {
                    self.editor_content.perform(action);
                }
            }
            Message::RefreshEditorContent => {
                let code = self
                    .project
                    .clone()
                    .app_code()
                    .unwrap_or_else(|err| err.to_string());
                self.editor_content = text_editor::Content::with_text(&code);
            }
            Message::DropNewElement(name, point, _) => {
                return iced_drop::zones_on_point(
                    move |zones| Message::HandleNew(name.clone(), zones),
                    point,
                    None,
                    None,
                )
                .into()
            }
            Message::HandleNew(name, zones) => {
                let ids: Vec<Id> = zones.into_iter().map(|z| z.0).collect();
                if ids.len() > 0 {
                    let action = Action::new(ids, &mut self.project.element_tree.clone(), None);
                    let result = name.handle_action(self.project.element_tree.as_mut(), action);
                    if let Ok(Some(ref element)) = result {
                        self.project.element_tree = Some(element.clone());
                    }
                }

                return Task::done(Message::RefreshEditorContent);
            }
            Message::MoveElement(element, point, _) => {
                return iced_drop::zones_on_point(
                    move |zones| Message::HandleMove(element.clone(), zones),
                    point,
                    None,
                    None,
                )
                .into()
            }
            Message::HandleMove(element, zones) => {
                let ids: Vec<Id> = zones.into_iter().map(|z| z.0).collect();
                if ids.len() > 0 {
                    let action = Action::new(
                        ids,
                        &mut self.project.element_tree.clone(),
                        Some(element.get_id()),
                    );
                    let _ = element.handle_action(self.project.element_tree.as_mut(), action);
                }

                return Task::done(Message::RefreshEditorContent);
            }
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.pane_state.resize(split, ratio);
            }
            Message::PaneClicked(pane) => {
                self.focus = Some(pane);
            }
            Message::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.pane_state.drop(pane, target);
            }
            Message::PaneDragged(_) => {}
            Message::NewFile => {
                if !self.is_loading {
                    self.project = Project::new();
                    self.project_path = None;
                    self.editor_content = text_editor::Content::new();
                }
            }
            Message::OpenFile => {
                if !self.is_loading {
                    self.is_loading = true;

                    return Task::perform(Project::from_file(), Message::FileOpened);
                }
            }
            Message::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                if let Ok((path, project)) = result {
                    self.project = project.clone();
                    self.project_path = Some(path);
                    self.editor_content = text_editor::Content::with_text(
                        &project.app_code().unwrap_or_else(|err| err.to_string()),
                    );
                }
            }
            Message::SaveFile => {
                if !self.is_loading {
                    self.is_loading = true;

                    return Task::perform(
                        self.project
                            .clone()
                            .write_to_file(self.project_path.clone()),
                        Message::FileSaved,
                    );
                }
            }
            Message::SaveFileAs => {
                if !self.is_loading {
                    self.is_loading = true;

                    return Task::perform(
                        self.project.clone().write_to_file(None),
                        Message::FileSaved,
                    );
                }
            }
            Message::FileSaved(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.project_path = Some(path);
                    self.is_dirty = false;
                }
            }
        }

        Task::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("o") if modifiers.command() => Some(Message::OpenFile),
            keyboard::Key::Character("s") if modifiers.command() => {
                if modifiers.shift() {
                    Some(Message::SaveFileAs)
                } else {
                    Some(Message::SaveFile)
                }
            }
            keyboard::Key::Character("n") if modifiers.command() => Some(Message::NewFile),
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {
        let header = row![button("Toggle Theme")
            .on_press(Message::ToggleTheme)
            .padding(5)]
        .width(200);
        let pane_grid = PaneGrid::new(&self.pane_state, |id, pane, _is_maximized| {
            let is_focused = Some(id) == self.focus;
            match pane {
                Panes::Designer => match &self.designer_page {
                    DesignerPage::Designer => designer_view::view(
                        &self.project.element_tree,
                        self.project.get_theme(),
                        is_focused,
                    ),
                    DesignerPage::CodeView => {
                        code_view::view(&self.editor_content, self.dark_theme, is_focused)
                    }
                },
                Panes::ElementList => element_list::view(&self.element_list, is_focused),
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .on_resize(10, Message::PaneResized)
        .on_click(Message::PaneClicked)
        .on_drag(Message::PaneDragged);

        let content = Column::new()
            .push(header)
            .push(pane_grid)
            .spacing(5)
            .align_x(Alignment::Center)
            .width(Length::Fill);

        container(content).height(Length::Fill).into()
    }
}
