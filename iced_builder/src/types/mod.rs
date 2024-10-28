pub mod element_name;
pub mod project;
pub mod rendered_element;

pub use element_name::ElementName;
pub use project::Project;
pub use rendered_element::*;

#[derive(Debug, Clone)]
pub enum DesignerPage {
    Designer,
    CodeView,
}
