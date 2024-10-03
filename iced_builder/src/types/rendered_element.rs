use std::collections::HashMap;

use iced::advanced::widget::Id;
use iced::{widget, Element, Length};
use serde::{Deserialize, Serialize};
use unique_id::{string::StringGenerator, Generator};

use crate::Message;

use super::ElementName;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    pub fn find_by_id(&mut self, id: Id) -> Option<&mut Self> {
        if Id::new(self.id.clone()) == id.clone() {
            println!("");
            return Some(self);
        } else if let Some(child_elements) = self.child_elements.as_mut() {
            for element in child_elements {
                let element = element.find_by_id(id.clone());
                if element.is_some() {
                    return element;
                }
            }
            return None;
        } else {
            return None;
        }
    }

    pub fn find_parent(&mut self, child_element: &RenderedElement) -> Option<&mut Self> {
        if child_element == self {
            return Some(self);
        } else if self.child_elements.is_some() {
            if self.child_elements.clone()?.contains(child_element) {
                return Some(self);
            } else {
                if let Some(child_elements) = self.child_elements.as_mut() {
                    for element in child_elements {
                        let element: Option<&mut Self> = element.find_parent(child_element);
                        if element.is_some() {
                            return element;
                        }
                    }
                }
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn remove(&mut self, element: &RenderedElement) {
        let parent = self.find_parent(element);
        if let Some(child_elements) = parent.unwrap().child_elements.as_mut() {
            if let Some(index) = child_elements.iter().position(|x| x == element) {
                child_elements.remove(index);
            }
        }
    }

    pub fn push(&mut self, element: RenderedElement) {
        if let Some(child_elements) = self.child_elements.as_mut() {
            child_elements.push(element);
        }
    }

    pub fn insert_after(&mut self, id: Id, element: RenderedElement) {
        if let Some(child_elements) = self.child_elements.as_mut() {
            if let Some(index) = child_elements
                .iter()
                .position(|x| Id::new(x.id.clone()) == id)
            {
                child_elements.insert(index, element);
            } else {
                child_elements.push(element);
            }
        }
    }

    fn preset_options(mut self, options: Vec<&str>) -> Self {
        for opt in options {
            self.props.insert(opt.to_owned(), None);
        }
        self
    }

    pub fn option(&mut self, option: &'static str, value: &'static str) {
        self.props
            .entry(option.to_owned())
            .and_modify(|opt| *opt = Some(value.to_owned()));
    }

    pub fn as_element(self) -> Element<'static, Message> {
        let mut children = widget::column![];

        if let Some(els) = self.child_elements.clone() {
            for el in els {
                children = children.push(el.clone().as_element());
            }
        }
        iced_drop::droppable(
            widget::container(
                widget::column![widget::text(self.name.clone().to_string()), children]
                    .width(Length::Fill),
            )
            .style(widget::container::bordered_box),
        )
        .id(Id::new(self.id.clone()))
        .on_drop(move |point, rect| Message::MoveElement(self.clone(), point, rect))
        .into()
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

pub fn container(content: Option<RenderedElement>) -> RenderedElement {
    match content {
        Some(el) => RenderedElement::from_vec(ElementName::Container, vec![el]),
        None => RenderedElement::from_vec(ElementName::Container, vec![]),
    }
}

pub fn row(child_elements: Option<Vec<RenderedElement>>) -> RenderedElement {
    match child_elements {
        Some(els) => RenderedElement::from_vec(ElementName::Row, els),
        None => RenderedElement::from_vec(ElementName::Row, vec![]),
    }
}

pub fn column(child_elements: Option<Vec<RenderedElement>>) -> RenderedElement {
    match child_elements {
        Some(els) => RenderedElement::from_vec(ElementName::Column, els),
        None => RenderedElement::from_vec(ElementName::Column, vec![]),
    }
}
