pub mod element_name;
pub mod project;
pub mod rendered_element;

pub use element_name::ElementName;
pub use project::Project;
pub use rendered_element::*;

use std::path::PathBuf;

use crate::error::Error;
use iced::widget::{pane_grid, text_editor};

#[derive(Debug, Clone)]
pub enum Message {
    ToggleDarkMode,
    CopyCode,
    SwitchPage(DesignerPage),
    EditorAction(text_editor::Action),
    RefreshEditorContent,
    DropNewElement(ElementName, iced::Point, iced::Rectangle),
    HandleNew(
        ElementName,
        Vec<(iced::advanced::widget::Id, iced::Rectangle)>,
    ),
    MoveElement(RenderedElement, iced::Point, iced::Rectangle),
    HandleMove(
        RenderedElement,
        Vec<(iced::advanced::widget::Id, iced::Rectangle)>,
    ),
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
pub enum DesignerPage {
    Designer,
    CodeView,
}
