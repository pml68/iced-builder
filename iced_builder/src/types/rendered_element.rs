use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use unique_id::{string::StringGenerator, Generator};

use super::ElementName;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    fn preset_options(mut self, options: Vec<&str>) -> Self {
        for opt in options {
            self.props.insert(opt.to_owned(), None);
        }
        self
    }

    pub fn push(mut self, element: RenderedElement) -> Self {
        if let Some(els) = self.child_elements.as_mut() {
            els.push(element);
        } else {
            self.child_elements = Some(vec![element]);
        }
        self
    }

    pub fn option(mut self, option: &'static str, value: &'static str) -> Self {
        self.props
            .entry(option.to_owned())
            .and_modify(|opt| *opt = Some(value.to_owned()));
        self
    }
}

impl std::fmt::Display for RenderedElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut has_props = false;
        f.pad("")?;
        f.write_fmt(format_args!("{:?}\n", self.name))?;
        f.pad("")?;
        f.write_str("Options: (")?;
        for (k, v) in &self.props {
            if let Some(value) = v {
                has_props = true;
                f.write_fmt(format_args!(
                    "\n{:width$.precision$}{}: {}",
                    "",
                    k,
                    value,
                    width = f.width().unwrap_or(0) + f.precision().unwrap_or(0),
                    precision = f.precision().unwrap_or(0)
                ))?;
            }
        }
        if has_props {
            f.write_str("\n")?;
            f.pad("")?;
        }
        f.write_str(")")?;
        if let Some(els) = &self.child_elements {
            f.write_str(" {\n")?;
            for el in els {
                f.write_fmt(format_args!(
                    "\n{:width$.precision$}\n",
                    el,
                    width = f.width().unwrap_or(0) + f.precision().unwrap_or(0),
                    precision = f.precision().unwrap_or(0)
                ))?;
            }
            f.pad("")?;
            f.write_str("}")?;
        }
        Ok(())
    }
}

pub fn text(text: &str) -> RenderedElement {
    RenderedElement::new(ElementName::Text(text.to_owned())).preset_options(vec![
        "size",
        "line_height",
        "width",
        "height",
    ])
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
