use unique_id::{string::StringGenerator, Generator};

use super::{props::Props, ElementName};
pub struct RenderedElement {
    pub id: String,
    pub child_elements: Vec<RenderedElement>,
    pub name: ElementName,
    pub props: Props,
}

impl RenderedElement {
    pub fn new(name: ElementName) -> Self {
        let gen = StringGenerator::default();
        Self {
            id: gen.next_id(),
            child_elements: vec![],
            name,
            props: Props::default(),
        }
    }

    pub fn from_vec(name: ElementName, child_elements: Vec<RenderedElement>) -> Self {
        let gen = StringGenerator::default();
        Self {
            id: gen.next_id(),
            child_elements,
            name,
            props: Props::default(),
        }
    }

    pub fn push(mut self, element: RenderedElement) -> Self {
        self.child_elements.push(element);
        self
    }
}
