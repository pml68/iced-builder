pub mod rendered_element;

use rendered_element::RenderedElement;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct DesignerState {
    pub designer_content: Option<RenderedElement>,
    pub designer_page: DesignerPage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementName {
    Text(String),
    Button(String),
    SVG(String),
    Image(String),
    Container,
    Row,
    Column,
}

impl ElementName {
    pub const ALL: [Self; 7] = [
        Self::Text(String::new()),
        Self::Button(String::new()),
        Self::SVG(String::new()),
        Self::Image(String::new()),
        Self::Container,
        Self::Row,
        Self::Column,
    ];
}

impl std::fmt::Display for ElementName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Text(_) => "Text",
                Self::Button(_) => "Button",
                Self::SVG(_) => "SVG",
                Self::Image(_) => "Image",
                Self::Container => "Container",
                Self::Row => "Row",
                Self::Column => "Column",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum DesignerPage {
    Designer,
    CodeView,
}
