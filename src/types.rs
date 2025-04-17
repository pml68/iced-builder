pub mod element_name;
pub mod project;
pub mod rendered_element;

use std::path::PathBuf;

pub use element_name::ElementName;
use iced::advanced::widget::Id;
use iced::widget::{pane_grid, text_editor};
use iced_anim::Event;
pub use project::Project;
pub use rendered_element::*;

use crate::Error;
use crate::config::Config;

#[derive(Debug, Clone)]
pub enum Message {
    ConfigLoad(Result<Config, Error>),
    SwitchTheme(Event<iced::Theme>),
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
    OpenDialog(&'static str, String, DialogButtons, DialogAction),
    CloseDialog,
    DialogOk,
    DialogCancel,
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Project), Error>),
    SaveFile,
    SaveFileAs,
    FileSaved(Result<PathBuf, Error>),
}

#[derive(Debug, Clone, Copy, Default)]
pub enum DialogButtons {
    #[default]
    None,
    Ok,
    OkCancel,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum DialogAction {
    #[default]
    None,
    NewFile,
    OpenFile,
}

#[derive(Debug, Clone, Copy)]
pub enum DesignerPane {
    DesignerView,
    CodeView,
}
