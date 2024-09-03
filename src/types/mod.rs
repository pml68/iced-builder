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
    pub align_items: Option<Alignment>,
    pub align_x: Option<Horizontal>,
    pub align_y: Option<Vertical>,
    pub horizontal_alignment: Option<Horizontal>,
    pub vertical_alignment: Option<Vertical>,
    pub height: Option<Length>,
    pub width: Option<Length>,
    pub font: Option<Font>,
    pub padding: Option<i32>,
    pub spacing: Option<i32>,
    pub content_fit: Option<ContentFit>,
    pub shaping: Option<Shaping>,
}

pub enum DesignerPage {
    Designer,
    CodeView,
}
