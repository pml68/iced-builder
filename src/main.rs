#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod appearance;
mod config;
mod dialog;
mod environment;
mod error;
mod icon;
mod options;
mod panes;
mod types;
mod values;
mod widget;

use std::io;
use std::path::PathBuf;

use config::Config;
use dialog::{Dialog, UnsavedChanges};
use error::Error;
use iced::advanced::widget::Id;
use iced::widget::{Column, container, pane_grid, pick_list, row, text_editor};
use iced::{
    Alignment, Length, Subscription, Task, clipboard, keyboard, window,
};
use iced_anim::transition::Easing;
use iced_anim::{Animated, Animation};
use iced_material::Theme;
use panes::{code_view, designer_view, element_list};
use types::{
    Action, ConfigChangeType, DesignerPane, Element, Message, Panes, Project,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut project_path = None;

    if let Some(arg) = std::env::args().nth(1) {
        if arg == "-V" || arg == "--version" {
            println!("iced-builder {}", environment::formatted_version());
            println!("{}", env!("CARGO_PKG_REPOSITORY"));

            return Ok(());
        } else {
            let path = PathBuf::from(&arg);

            if path.try_exists()? && !path.is_file() {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::IsADirectory,
                    "Expected a file, directory given.",
                )));
            } else {
                project_path = Some(path);
            }
        }
    }

    iced::application(
        move || IcedBuilder::boot(project_path.clone()),
        IcedBuilder::update,
        IcedBuilder::view,
    )
    .title(IcedBuilder::title)
    .theme(IcedBuilder::theme)
    .subscription(IcedBuilder::subscription)
    .exit_on_close_request(false)
    .font(icon::FONT)
    .antialiasing(true)
    .centered()
    .run()?;

    Ok(())
}

struct IcedBuilder {
    is_dirty: bool,
    is_loading: bool,
    project_path: Option<PathBuf>,
    project: Project,
    config: Config,
    theme: Animated<Theme>,
    pane_state: pane_grid::State<Panes>,
    focus: Option<pane_grid::Pane>,
    designer_page: DesignerPane,
    dialog: Dialog,
    editor_content: text_editor::Content,
}

impl IcedBuilder {
    fn boot(project_path: Option<PathBuf>) -> (Self, Task<Message>) {
        let state = pane_grid::State::with_configuration(
            pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Vertical,
                ratio: 0.8,
                a: Box::new(pane_grid::Configuration::Pane(Panes::Designer)),
                b: Box::new(pane_grid::Configuration::Pane(Panes::ElementList)),
            },
        );

        let config = Config::default();
        let theme = config.selected_theme();

        let mut tasks =
            vec![Task::perform(Config::load(), Message::ConfigLoad)];
        if let Some(path) = project_path.as_deref()
            && path.exists()
            && path.is_file()
        {
            tasks.push(Task::perform(
                Project::from_path(path.to_path_buf()),
                Message::FileOpened,
            ));
        }

        (
            Self {
                is_dirty: false,
                is_loading: false,
                project_path,
                project: Project::new(),
                config,
                theme: Animated::new(theme, Easing::EASE_IN),
                pane_state: state,
                focus: None,
                designer_page: DesignerPane::DesignerView,
                dialog: Dialog::default(),
                editor_content: text_editor::Content::new(),
            },
            Task::batch(tasks),
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

    fn theme(&self) -> Theme {
        self.theme.value().clone()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ConfigLoad(result) => match result {
                Ok(config) => {
                    self.config = config;
                    self.theme.settle_at(self.config.selected_theme());

                    if let Some(path) = self.config.last_project()
                        && self.project_path.is_none()
                    {
                        if path.exists() && path.is_file() {
                            return Task::perform(
                                Project::from_path(path.to_owned()),
                                Message::FileOpened,
                            );
                        } else {
                            self.dialog = Dialog::warning(format!(
                                "The file {} does not exist, or isn't a file.",
                                path.to_string_lossy()
                            ));
                        };
                    };
                }
                Err(error) => self.dialog = Dialog::error(error),
            },
            Message::ConfigWrite(result) => {
                if let Err(error) = result {
                    self.dialog = Dialog::error(error);
                }
            }
            Message::SaveConfigChanges(change) => {
                match change {
                    ConfigChangeType::LastProject => {
                        self.config.last_project = self.project_path.clone();
                    }
                    ConfigChangeType::SelectedTheme => {
                        self.config.appearance.selected =
                            self.theme.target().clone();
                    }
                }

                return Task::perform(
                    self.config.clone().save(),
                    Message::ConfigWrite,
                );
            }
            Message::SwitchTheme(event) => {
                self.theme.update(event);

                return self.update(ConfigChangeType::SelectedTheme.into());
            }
            Message::CopyCode => {
                return clipboard::write(self.editor_content.text());
            }
            Message::SwitchPane(pane) => self.designer_page = pane,
            Message::EditorAction(action) => {
                if matches!(action, text_editor::Action::Scroll { .. }) {
                    self.editor_content.perform(action);
                }
            }
            Message::RefreshEditorContent => match self.project.app_code() {
                Ok(code) => {
                    self.editor_content =
                        text_editor::Content::with_text(&code);
                }
                Err(error) => self.dialog = Dialog::error(error),
            },
            Message::DropNewElement(name, point, _) => {
                return iced_drop::zones_on_point(
                    move |zones| Message::HandleNew(name.clone(), zones),
                    point,
                    None,
                    None,
                );
            }
            Message::HandleNew(name, zones) => {
                let ids: Vec<Id> = zones.into_iter().map(|z| z.0).collect();
                if !ids.is_empty() {
                    let action = Action::new(
                        &ids,
                        self.project.element_tree.as_ref(),
                        None,
                    );
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
                            self.dialog = Dialog::error(error);
                        }
                        _ => {}
                    }
                    return self.update(Message::RefreshEditorContent);
                }
            }
            Message::MoveElement(element, point, _) => {
                return iced_drop::zones_on_point(
                    move |zones| Message::HandleMove(element.clone(), zones),
                    point,
                    None,
                    None,
                );
            }
            Message::HandleMove(element, zones) => {
                let ids: Vec<Id> = zones.into_iter().map(|z| z.0).collect();
                if !ids.is_empty() {
                    let action = Action::new(
                        &ids,
                        self.project.element_tree.as_ref(),
                        Some(element.id()),
                    );
                    let result = element.handle_action(
                        self.project.element_tree.as_mut(),
                        action,
                    );
                    if let Err(error) = result {
                        self.dialog = Dialog::error(error);
                    }

                    self.is_dirty = true;
                    return self.update(Message::RefreshEditorContent);
                }
            }
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.pane_state.resize(split, ratio);
            }
            Message::PaneClicked(pane) => self.focus = Some(pane),
            Message::PaneDragged(pane_grid::DragEvent::Dropped {
                pane,
                target,
            }) => self.pane_state.drop(pane, target),
            Message::PaneDragged(_) => {}
            Message::CloseDialog => self.dialog.close(),
            Message::DialogYes => {
                return if matches!(
                    self.dialog.action(),
                    dialog::Action::UnsavedChanges(_)
                ) {
                    self.is_loading = true;
                    Task::perform(
                        self.project
                            .clone()
                            .write_to_file(self.project_path.clone()),
                        Message::FileSaved,
                    )
                    .chain(Task::done(Message::DialogNo))
                } else {
                    self.update(Message::CloseDialog)
                };
            }
            Message::DialogNo => {
                let mut task = Task::done(Message::CloseDialog);

                if let dialog::Action::UnsavedChanges(unsaved_changes) =
                    self.dialog.action()
                {
                    match unsaved_changes {
                        UnsavedChanges::New => {
                            self.is_dirty = false;
                            self.project = Project::new();
                            self.project_path = None;
                            self.editor_content = text_editor::Content::new();
                        }
                        UnsavedChanges::Open => {
                            self.is_dirty = false;
                            self.is_loading = true;
                            task = Task::perform(
                                Project::from_file(),
                                Message::FileOpened,
                            )
                            .chain(task);
                        }
                        UnsavedChanges::Exit => {
                            return self.update(Message::CloseApp);
                        }
                    }
                }

                return task;
            }
            Message::DialogCancel => return Task::done(Message::CloseDialog),
            Message::NewFile => {
                if !self.is_loading {
                    if !self.is_dirty {
                        self.project = Project::new();
                        self.project_path = None;
                        self.editor_content = text_editor::Content::new();
                    } else {
                        self.dialog = Dialog::unsaved_changes(
                            "You have unsaved changes. Do you want to save them before creating a new project?",
                            UnsavedChanges::New,
                        );
                    }
                }
            }
            Message::OpenFile => {
                if !self.is_loading {
                    if !self.is_dirty {
                        self.is_loading = true;

                        return Task::perform(
                            Project::from_file(),
                            Message::FileOpened,
                        );
                    } else {
                        self.dialog = Dialog::unsaved_changes(
                            "You have unsaved changes. Do you want to save them before opening another project?",
                            UnsavedChanges::Open,
                        );
                    }
                }
            }
            Message::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                match result {
                    Ok((path, project)) => {
                        self.project = project;
                        self.project_path =
                            Some(path.canonicalize().unwrap_or(path));

                        return Task::done(
                            ConfigChangeType::LastProject.into(),
                        )
                        .chain(Task::done(Message::RefreshEditorContent));
                    }
                    Err(error) => self.dialog = Dialog::error(error),
                };
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

                match result {
                    Ok(path) => {
                        self.project_path = Some(path);
                        self.is_dirty = false;

                        return self
                            .update(ConfigChangeType::LastProject.into());
                    }
                    Err(error) => self.dialog = Dialog::error(error),
                }
            }
            Message::CloseApp => {
                return window::get_latest().and_then(window::close);
            }
            Message::WindowEvent(window::Event::CloseRequested) => {
                if self.is_dirty {
                    self.dialog = Dialog::unsaved_changes(
                        "You have unsaved changes. Do you want to save them before closing iced Builder?",
                        UnsavedChanges::Exit,
                    );
                } else {
                    return self.update(Message::CloseApp);
                }
            }
            Message::WindowEvent(_) => {}
        }

        Task::none()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        let keyboard = keyboard::on_key_press(|key, modifiers| {
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
        });

        let window_events =
            window::events().map(|(_id, event)| Message::WindowEvent(event));

        Subscription::batch([keyboard, window_events])
    }

    fn view(&self) -> Element<'_, Message> {
        let header = row![pick_list(
            self.config.themes(),
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
                            self.project.get_theme(),
                            is_focused,
                        ),
                        DesignerPane::CodeView => {
                            code_view::view(&self.editor_content, is_focused)
                        }
                    },
                    Panes::ElementList => element_list::view(is_focused),
                }
            },
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .on_resize(10, Message::PaneResized)
        .on_click(Message::PaneClicked)
        .on_drag(Message::PaneDragged);

        let base = Column::new()
            .push(header)
            .push(pane_grid)
            .spacing(5)
            .align_x(Alignment::Center)
            .width(Length::Fill);

        let content = self
            .dialog
            .as_iced_dialog(container(base).height(Length::Fill));

        Animation::new(&self.theme, content)
            .on_update(Message::SwitchTheme)
            .into()
    }
}
