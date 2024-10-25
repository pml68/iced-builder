pub mod types;
pub mod views;

use std::path::PathBuf;

use iced::widget::{pane_grid, text_editor};
use types::{
    element_name::ElementName, project::Project, rendered_element::RenderedElement, DesignerPage,
};

#[derive(Debug, Clone)]
pub enum Error {
    IOError(std::io::ErrorKind),
    SerdeError(String),
    FormatError(String),
    NonExistentElement,
    DialogClosed,
    String(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerdeError(string) | Self::FormatError(string) | Self::String(string) => {
                write!(f, "{}", string)
            }
            Self::IOError(kind) => {
                write!(f, "{}", kind)
            }
            Self::NonExistentElement => {
                write!(f, "The element tree contains no matching element.")
            }
            Self::DialogClosed => {
                write!(
                    f,
                    "The file dialog has been closed without selecting a valid option."
                )
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value.kind())
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
    FileSaved(Result<PathBuf, Error>),
}
