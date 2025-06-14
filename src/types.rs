pub mod element_name;
pub mod project;
pub mod rendered_element;

use std::path::PathBuf;

pub use element_name::ElementName;
use iced::advanced::widget::Id;
use iced::widget::{pane_grid, text_editor};
use iced::window;
use iced_anim::Event;
use material_theme::Theme;
pub use project::Project;
pub use rendered_element::*;

use crate::Error;
use crate::config::Config;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum Message {
    ConfigLoad(Result<Config, Error>),
    SwitchTheme(Event<Theme>),
    CopyCode,
    SwitchPage(DesignerPane),
    EditorAction(text_editor::Action),
    RefreshEditorContent,
    DropNewElement(ElementName, iced::Point, iced::Rectangle),
    HandleNew(ElementName, Vec<(Id, iced::Rectangle)>),
    MoveElement(RenderedElement, iced::Point, iced::Rectangle),
    HandleMove(RenderedElement, Vec<(Id, iced::Rectangle)>),
    PaneResized(pane_grid::ResizeEvent),
    PaneClicked(pane_grid::Pane),
    PaneDragged(pane_grid::DragEvent),
    CloseDialog,
    DialogYes,
    DialogNo,
    DialogCancel,
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Project), Error>),
    SaveFile,
    SaveFileAs,
    FileSaved(Result<PathBuf, Error>),
    CloseApp,
    WindowEvent(window::Event),
}

#[derive(Debug, Clone, Copy)]
pub enum Panes {
    Designer,
    ElementList,
}

#[derive(Debug, Clone, Copy)]
pub enum DesignerPane {
    DesignerView,
    CodeView,
}
