use iced::{Font, Length};

pub struct DesignerState {
    pub designer_content: Vec<RenderedElement>,
    pub designer_page: DesignerPage,
}

pub struct RenderedElement {
    pub id: String,
    pub children: Vec<RenderedElement>,
    pub name: ElementName,
    pub props: Vec<Prop>,
}

pub enum ElementName {}

pub enum Prop {
    String(String, String),
    Decimal(String, i32),
    Float(String, f32),
    Font(String, Font),
    Length(String, Length),
}

pub enum DesignerPage {
    Designer,
    CodeView,
}
