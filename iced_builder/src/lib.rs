pub mod types;
pub mod views;

use std::path::PathBuf;

use iced::widget::{pane_grid, text_editor};
use types::{
    element_name::ElementName, project::Project, rendered_element::RenderedElement, DesignerPage,
};

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("an IO error accured: {0}")]
    IOError(String),
    #[error("a Serde error accured: {0}")]
    SerdeError(String),
    #[error("an RustFmt error accured: {0}")]
    FormatError(String),
    #[error("the element tree contains no matching element")]
    NonExistentElement,
    #[error("the file dialog has been closed without selecting a valid option")]
    DialogClosed,
    #[error("{0}")]
    String(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value.to_string())
    }
}

impl From<rust_format::Error> for Error {
    fn from(value: rust_format::Error) -> Self {
        Self::FormatError(value.to_string())
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::String(value.to_owned())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTheme,
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
