mod config;
mod dialogs;
mod environment;
mod error;
#[allow(clippy::all, dead_code)]
mod icon;
mod options;
mod panes;
mod theme;
mod types;
mod values;
mod widget;

use std::path::PathBuf;
use std::sync::Arc;

use config::Config;
use dialogs::{error_dialog, unsaved_changes_dialog, warning_dialog};
use error::Error;
use iced::advanced::widget::Id;
use iced::widget::{Column, container, pane_grid, pick_list, row, text_editor};
use iced::{Alignment, Element, Length, Task, Theme, clipboard, keyboard};
use iced_anim::transition::Easing;
use iced_anim::{Animated, Animation};
use panes::{code_view, designer_view, element_list};
use tokio::runtime;
use types::{Action, DesignerPane, ElementName, Message, Project};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = std::env::args()
        .nth(1)
        .is_some_and(|s| s == "--version" || s == "-V");

    if version {
        println!("iced-builder {}", environment::formatted_version());
        println!("{}", env!("CARGO_PKG_REPOSITORY"));

        return Ok(());
    }

    let config_load = {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        rt.block_on(Config::load())
    };

    iced::application(App::title, App::update, App::view)
        .font(icon::FONT)
        .theme(|state| state.theme.value().clone())
        .subscription(App::subscription)
        .run_with(move || App::new(config_load))?;

    Ok(())
}

struct App {
    is_dirty: bool,
    is_loading: bool,
    project_path: Option<PathBuf>,
    project: Project,
    config: Arc<Config>,
    theme: Animated<Theme>,
    pane_state: pane_grid::State<Panes>,
    focus: Option<pane_grid::Pane>,
    designer_page: DesignerPane,
    element_list: &'static [ElementName],
    editor_content: text_editor::Content,
}

#[derive(Clone, Copy, Debug)]
enum Panes {
    Designer,
    ElementList,
}

impl App {
    fn new(config_load: Result<Config, Error>) -> (Self, Task<Message>) {
        let state = pane_grid::State::with_configuration(
            pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Vertical,
                ratio: 0.8,
                a: Box::new(pane_grid::Configuration::Pane(Panes::Designer)),
                b: Box::new(pane_grid::Configuration::Pane(Panes::ElementList)),
            },
        );

        let config = Arc::new(config_load.unwrap_or_default());
        let theme = config.selected_theme();

        let task = if let Some(path) = config.last_project.clone() {
            if path.exists() && path.is_file() {
                Task::perform(
                    Project::from_path(path, config.clone()),
                    Message::FileOpened,
                )
            } else {
                Task::future(warning_dialog(format!(
                    "The file {} does not exist, or isn't a file.",
                    path.to_string_lossy()
                )))
                .discard()
            }
        } else {
            Task::none()
        };

        (
            Self {
                is_dirty: false,
                is_loading: false,
                project_path: None,
                project: Project::new(),
                config,
                theme: Animated::new(theme, Easing::EASE_IN),
                pane_state: state,
                focus: None,
                designer_page: DesignerPane::DesignerView,
                element_list: ElementName::ALL,
                editor_content: text_editor::Content::new(),
            },
            task,
        )
    }

    fn title(&self) -> String {
        let saved_state = if self.is_dirty { " *" } else { "" };

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
            None => String::new(),
        };

        format!("iced Builder{project_name}{saved_state}")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SwitchTheme(event) => {
                self.theme.update(event);

                Task::none()
            }
            Message::CopyCode => clipboard::write(self.editor_content.text()),
            Message::SwitchPage(page) => {
                self.designer_page = page;
                Task::none()
            }
            Message::EditorAction(action) => {
                if let text_editor::Action::Scroll { lines: _ } = action {
                    self.editor_content.perform(action);
                }
                Task::none()
            }
            Message::RefreshEditorContent => {
                match self.project.app_code(&self.config) {
                    Ok(code) => {
                        self.editor_content =
                            text_editor::Content::with_text(&code);
                        Task::none()
                    }
                    Err(error) => Task::future(error_dialog(error)).discard(),
                }
            }
            Message::DropNewElement(name, point, _) => {
                iced_drop::zones_on_point(
                    move |zones| Message::HandleNew(name.clone(), zones),
                    point,
                    None,
                    None,
                )
            }
            Message::HandleNew(name, zones) => {
                let ids: Vec<Id> = zones.into_iter().map(|z| z.0).collect();
                if !ids.is_empty() {
                    let eltree_clone = self.project.element_tree.clone();
                    let action = Action::new(&ids, eltree_clone.as_ref(), None);
                    let result = name.handle_action(
                        self.project.element_tree.as_mut(),
                        action,
                    );
                    self.is_dirty = true;
                    match result {
                        Ok(Some(ref element)) => {
                            self.project.element_tree = Some(element.clone());
                        }
                        Err(error) => {
                            return Task::future(error_dialog(error))
                                .map(|_| Message::RefreshEditorContent);
                        }
                        _ => {}
                    }
                    Task::done(Message::RefreshEditorContent)
                } else {
                    Task::none()
                }
            }
            Message::MoveElement(element, point, _) => {
                iced_drop::zones_on_point(
                    move |zones| Message::HandleMove(element.clone(), zones),
                    point,
                    None,
                    None,
                )
            }
            Message::HandleMove(element, zones) => {
                let ids: Vec<Id> = zones.into_iter().map(|z| z.0).collect();
                if !ids.is_empty() {
                    let eltree_clone = self.project.element_tree.clone();
                    let action = Action::new(
                        &ids,
                        eltree_clone.as_ref(),
                        Some(element.id()),
                    );
                    let result = element.handle_action(
                        self.project.element_tree.as_mut(),
                        action,
                    );
                    if let Err(error) = result {
                        return Task::future(error_dialog(error)).discard();
                    }

                    self.is_dirty = true;
                    Task::done(Message::RefreshEditorContent)
                } else {
                    Task::none()
                }
            }
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.pane_state.resize(split, ratio);
                Task::none()
            }
            Message::PaneClicked(pane) => {
                self.focus = Some(pane);
                Task::none()
            }
            Message::PaneDragged(pane_grid::DragEvent::Dropped {
                pane,
                target,
            }) => {
                self.pane_state.drop(pane, target);
                Task::none()
            }
            Message::PaneDragged(_) => Task::none(),
            Message::NewFile => {
                if !self.is_loading {
                    if !self.is_dirty {
                        self.project = Project::new();
                        self.project_path = None;
                        self.editor_content = text_editor::Content::new();
                    } else if unsaved_changes_dialog(
                        "You have unsaved changes. Do you wish to discard these and create a new project?",
                    ) {
                        self.is_dirty = false;
                        self.project = Project::new();
                        self.project_path = None;
                        self.editor_content = text_editor::Content::new();
                    }
                }

                Task::none()
            }
            Message::OpenFile => {
                if !self.is_loading {
                    if !self.is_dirty {
                        self.is_loading = true;

                        Task::perform(
                            Project::from_file(self.config.clone()),
                            Message::FileOpened,
                        )
                    } else if unsaved_changes_dialog(
                        "You have unsaved changes. Do you wish to discard these and open another project?",
                    ) {
                        self.is_dirty = false;
                        self.is_loading = true;
                        Task::perform(
                            Project::from_file(self.config.clone()),
                            Message::FileOpened,
                        )
                    } else {
                        Task::none()
                    }
                } else {
                    Task::none()
                }
            }
            Message::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                match result {
                    Ok((path, project)) => {
                        self.project = project;
                        self.project_path = Some(path);
                        Task::done(Message::RefreshEditorContent)
                    }
                    Err(error) => Task::future(error_dialog(error)).discard(),
                }
            }
            Message::SaveFile => {
                if !self.is_loading {
                    self.is_loading = true;

                    Task::perform(
                        self.project
                            .clone()
                            .write_to_file(self.project_path.clone()),
                        Message::FileSaved,
                    )
                } else {
                    Task::none()
                }
            }
            Message::SaveFileAs => {
                if !self.is_loading {
                    self.is_loading = true;

                    Task::perform(
                        self.project.clone().write_to_file(None),
                        Message::FileSaved,
                    )
                } else {
                    Task::none()
                }
            }
            Message::FileSaved(result) => {
                self.is_loading = false;

                match result {
                    Ok(path) => {
                        self.project_path = Some(path);
                        self.is_dirty = false;
                        Task::none()
                    }
                    Err(error) => Task::future(error_dialog(error)).discard(),
                }
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| {
            if modifiers.command() {
                match key.as_ref() {
                    keyboard::Key::Character("o") => Some(Message::OpenFile),
                    keyboard::Key::Character("s") => {
                        Some(if modifiers.shift() {
                            Message::SaveFileAs
                        } else {
                            Message::SaveFile
                        })
                    }
                    keyboard::Key::Character("n") => Some(Message::NewFile),
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    fn view(&self) -> Element<'_, Message> {
        let header = row![pick_list(
            self.config.theme.all.clone(),
            Some(self.theme.target()),
            |theme| Message::SwitchTheme(theme.into())
        )]
        .width(200);
        let pane_grid = pane_grid::PaneGrid::new(
            &self.pane_state,
            |id, pane, _is_maximized| {
                let is_focused = Some(id) == self.focus;
                match pane {
                    Panes::Designer => match &self.designer_page {
                        DesignerPane::DesignerView => designer_view::view(
                            self.project.element_tree.as_ref(),
                            self.project.get_theme(&self.config),
                            is_focused,
                        ),
                        DesignerPane::CodeView => {
                            code_view::view(&self.editor_content, is_focused)
                        }
                    },
                    Panes::ElementList => {
                        element_list::view(self.element_list, is_focused)
                    }
                }
            },
        )
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

        Animation::new(&self.theme, container(content).height(Length::Fill))
            .on_update(Message::SwitchTheme)
            .into()
    }
}
