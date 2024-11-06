pub mod element_name;
pub mod project;
pub mod rendered_element;

pub use element_name::ElementName;
pub use project::Project;
pub use rendered_element::*;

use std::path::PathBuf;

use crate::Result;
use iced::{
    widget::{pane_grid, text_editor},
    Theme,
};
use iced_anim::SpringEvent;

#[derive(Debug, Clone)]
pub enum Message {
    ToggleTheme(SpringEvent<Theme>),
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
    FileOpened(Result<(PathBuf, Project)>),
    SaveFile,
    SaveFileAs,
    FileSaved(Result<PathBuf>),
}

#[derive(Debug, Clone)]
pub enum DesignerPage {
    DesignerView,
    CodeView,
}
