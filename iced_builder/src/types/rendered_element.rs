use std::collections::HashMap;

use unique_id::{string::StringGenerator, Generator};

use iced::advanced::widget::Id;

use super::ElementName;

#[derive(Debug)]
pub struct RenderedElement {
    pub id: Id,
    pub child_elements: Vec<RenderedElement>,
    pub name: ElementName,
    pub props: HashMap<&'static str, &'static str>,
}

impl RenderedElement {
    pub fn new(name: ElementName) -> Self {
        let gen = StringGenerator::default();
        Self {
            id: Id::new(gen.next_id()),
            child_elements: vec![],
            name,
            props: HashMap::new(),
        }
    }

    pub fn from_vec(name: ElementName, child_elements: Vec<RenderedElement>) -> Self {
        let gen = StringGenerator::default();
        Self {
            id: Id::new(gen.next_id()),
            child_elements,
            name,
            props: HashMap::new(),
        }
    }

    pub fn push(mut self, element: RenderedElement) -> Self {
        self.child_elements.push(element);
        self
    }

    pub fn set_property(&mut self, prop: &'static str, value: &'static str) {
        let prop_ref = self.props.entry(prop).or_insert(value);
        *prop_ref = value;
    }
}
