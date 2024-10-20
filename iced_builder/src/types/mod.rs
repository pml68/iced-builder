pub mod element_name;
pub mod project;
pub mod rendered_element;

#[derive(Debug, Clone)]
pub enum DesignerPage {
    Designer,
    CodeView,
}
