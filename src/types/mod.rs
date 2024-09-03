pub mod props;
pub mod rendered_element;

use iced::widget::text_editor;
use rendered_element::RenderedElement;
use std::path::PathBuf;

pub struct DesignerState {
    pub designer_content: Vec<RenderedElement>,
    pub designer_page: DesignerPage,
}

pub enum ElementName {
    App(String, iced::Theme),
    Text(String),
    Button(String),
    TextEditor(text_editor::Content),
    SVG(PathBuf),
    Image(PathBuf),
    Container,
    Row,
    Column,
}

pub enum DesignerPage {
    Designer,
    CodeView,
}
