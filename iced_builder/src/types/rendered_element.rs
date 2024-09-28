use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use unique_id::{string::StringGenerator, Generator};

use super::ElementName;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderedElement {
    pub id: String,
    pub child_elements: Option<Vec<RenderedElement>>,
    pub name: ElementName,
    pub props: HashMap<String, Option<String>>,
}

impl RenderedElement {
    fn new(name: ElementName) -> Self {
        let gen = StringGenerator::default();
        Self {
            id: gen.next_id(),
            child_elements: None,
            name,
            props: HashMap::new(),
        }
    }

    fn from_vec(name: ElementName, child_elements: Vec<RenderedElement>) -> Self {
        let gen = StringGenerator::default();
        Self {
            id: gen.next_id(),
            child_elements: Some(child_elements),
            name,
            props: HashMap::new(),
        }
    }

    pub fn push(mut self, element: RenderedElement) -> Self {
        if let Some(els) = self.child_elements.as_mut() {
            els.push(element);
        } else {
            self.child_elements = Some(vec![element]);
        }
        self
    }

    pub fn option(mut self, prop: &'static str, value: &'static str) -> Self {
        let prop_ref = self
            .props
            .entry(prop.to_owned())
            .or_insert(Some(value.to_owned()));
        *prop_ref = Some(value.to_owned());
        self
    }
}

pub fn text(text: &str) -> RenderedElement {
    RenderedElement::new(ElementName::Text(text.to_owned()))
}

pub fn button(text: &str) -> RenderedElement {
    RenderedElement::new(ElementName::Button(text.to_owned()))
}

pub fn svg(path: &str) -> RenderedElement {
    RenderedElement::new(ElementName::SVG(path.to_owned()))
}

pub fn image(path: &str) -> RenderedElement {
    RenderedElement::new(ElementName::Image(path.to_owned()))
}

pub fn container(content: RenderedElement) -> RenderedElement {
    RenderedElement::from_vec(ElementName::Container, vec![content])
}

pub fn row(child_elements: Vec<RenderedElement>) -> RenderedElement {
    RenderedElement::from_vec(ElementName::Row, child_elements)
}
