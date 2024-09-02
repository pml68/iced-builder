use std::path::PathBuf;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{text::Shaping, text_editor},
    Alignment, ContentFit, Font, Length,
};

pub struct DesignerState {
    pub designer_content: Vec<RenderedElement>,
    pub designer_page: DesignerPage,
}

pub struct RenderedElement {
    pub id: String,
    pub children: Option<Vec<RenderedElement>>,
    pub name: ElementName,
    pub props: Props,
}

pub enum ElementName {
    Text(String),
    Button(String),
    TextEditor(text_editor::Content),
    SVG(PathBuf),
    Image(PathBuf),
    Container,
    Row,
    Column,
}

pub struct Props {
    align_items: Option<Alignment>,
    align_x: Option<Horizontal>,
    align_y: Option<Vertical>,
    horizontal_alignment: Option<Horizontal>,
    vertical_alignment: Option<Vertical>,
    height: Option<Length>,
    width: Option<Length>,
    font: Option<Font>,
    padding: Option<i32>,
    spacing: Option<i32>,
    content_fit: Option<ContentFit>,
    shaping: Option<Shaping>,
}

pub enum DesignerPage {
    Designer,
    CodeView,
}
