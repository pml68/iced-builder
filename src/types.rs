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

#[derive(Debug, Clone)]
pub enum Message {
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
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Project), Error>),
    SaveFile,
    SaveFileAs,
    FileSaved(Result<PathBuf, Error>),
}

#[derive(Debug, Clone)]
pub enum DesignerPane {
    DesignerView,
    CodeView,
}
