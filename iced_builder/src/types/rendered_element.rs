use std::collections::BTreeMap;

use blob_uuid::random_blob;
use iced::advanced::widget::Id;
use iced::{widget, Element, Length};
use serde::{Deserialize, Serialize};

use super::ElementName;
use crate::types::Message;
use crate::Result;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderedElement {
    id: String,
    child_elements: Option<Vec<RenderedElement>>,
    name: ElementName,
    options: BTreeMap<String, Option<String>>,
}

impl RenderedElement {
    fn new(name: ElementName) -> Self {
        Self {
            id: random_blob(),
            child_elements: None,
            name,
            options: BTreeMap::new(),
        }
    }

    fn with(name: ElementName, child_elements: Vec<RenderedElement>) -> Self {
        Self {
            id: random_blob(),
            child_elements: Some(child_elements),
            name,
            options: BTreeMap::new(),
        }
    }

    pub fn get_id(&self) -> Id {
        Id::new(self.id.clone())
    }

    pub fn find_by_id(&mut self, id: Id) -> Option<&mut Self> {
        if self.get_id() == id.clone() {
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

    pub fn find_parent(
        &mut self,
        child_element: &RenderedElement,
    ) -> Option<&mut Self> {
        if child_element == self {
            return Some(self);
        } else if self.child_elements.is_some() {
            if self
                .child_elements
                .clone()
                .unwrap_or(vec![])
                .contains(child_element)
            {
                return Some(self);
            } else {
                if let Some(child_elements) = self.child_elements.as_mut() {
                    for element in child_elements {
                        let element = element.find_parent(child_element);
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

    pub fn is_parent(&self) -> bool {
        self.child_elements.is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.child_elements == Some(vec![])
    }

    pub fn remove(&mut self, element: &RenderedElement) {
        if let Some(child_elements) = self.child_elements.as_mut() {
            if let Some(index) =
                child_elements.iter().position(|x| x == element)
            {
                let _ = child_elements.remove(index);
            }
        }
    }

    pub fn push_front(&mut self, element: &RenderedElement) {
        if let Some(child_elements) = self.child_elements.as_mut() {
            child_elements.insert(0, element.clone());
        }
    }

    pub fn insert_after(&mut self, id: Id, element: &RenderedElement) {
        if let Some(child_elements) = self.child_elements.as_mut() {
            if let Some(index) =
                child_elements.iter().position(|x| x.get_id() == id)
            {
                child_elements.insert(index + 1, element.clone());
            } else {
                child_elements.push(element.clone());
            }
        }
    }

    pub fn handle_action(
        &self,
        element_tree: Option<&mut RenderedElement>,
        action: Action,
    ) -> Result<()> {
        let element_tree = element_tree.unwrap();

        match action {
            Action::Stop => Ok(()),
            Action::Drop => {
                let parent = element_tree.find_parent(self).unwrap();
                parent.remove(self);

                Ok(())
            }
            Action::AddNew => Err(
                "the action was of kind `AddNew`, but invoking it on an existing element tree is not possible".into(),
            ),
            Action::PushFront(id) => {
                let old_parent = element_tree.find_parent(self).unwrap();
                old_parent.remove(self);

                let new_parent = element_tree.find_by_id(id).unwrap();
                new_parent.push_front(self);

                Ok(())
            }
            Action::InsertAfter(parent_id, target_id) => {
                let old_parent = element_tree.find_parent(self).unwrap();
                old_parent.remove(self);

                let new_parent = element_tree.find_by_id(parent_id).unwrap();
                new_parent.insert_after(target_id, self);

                Ok(())
            }
        }
    }

    fn preset_options(mut self, options: Vec<&str>) -> Self {
        for opt in options {
            let _ = self.options.insert(opt.to_owned(), None);
        }
        self
    }

    pub fn option<'a>(mut self, option: &'a str, value: &'a str) -> Self {
        let _ = self
            .options
            .entry(option.to_owned())
            .and_modify(|opt| *opt = Some(value.to_owned()));
        self
    }

    pub fn as_element<'a>(self) -> Element<'a, Message> {
        let mut children = widget::column![];

        if let Some(els) = self.child_elements.clone() {
            for el in els {
                children = children.push(el.clone().as_element());
            }
        }
        iced_drop::droppable(
            widget::container(
                widget::column![
                    widget::text(self.name.clone().to_string()),
                    children
                ]
                .width(Length::Fill)
                .spacing(10),
            )
            .padding(10)
            .style(widget::container::bordered_box),
        )
        .id(self.get_id())
        .drag_hide(true)
        .on_drop(move |point, rect| {
            Message::MoveElement(self.clone(), point, rect)
        })
        .into()
    }

    pub fn codegen(&self) -> (String, String) {
        let mut imports = String::new();
        let mut view = String::new();
        let mut options = String::new();

        for (k, v) in self.options.clone() {
            if let Some(v) = v {
                options = format!("{options}.{k}({v})");
            }
        }

        let mut elements = String::new();

        if let Some(els) = &self.child_elements {
            for element in els {
                let (c_imports, children) = element.codegen();
                imports = format!("{imports}{c_imports}");
                elements = format!("{elements}{},", children);
            }
        }

        match &self.name {
            ElementName::Container => {
                imports = format!("{imports}container,");
                view = format!("{view}\ncontainer({elements}){options}");
            }
            ElementName::Row => {
                imports = format!("{imports}row,");
                view = format!("{view}\nrow![{elements}]{options}");
            }
            ElementName::Column => {
                imports = format!("{imports}column,");
                view = format!("{view}\ncolumn![{elements}]{options}");
            }
            ElementName::Text(string) => {
                imports = format!("{imports}text,");
                view = format!(
                    "{view}\ntext(\"{}\"){options}",
                    if *string == String::new() {
                        "New Text"
                    } else {
                        string
                    }
                );
            }
            ElementName::Button(string) => {
                imports = format!("{imports}button,");
                view = format!(
                    "{view}\nbutton(\"{}\"){options}",
                    if *string == String::new() {
                        "New Button"
                    } else {
                        string
                    }
                );
            }
            ElementName::Image(path) => {
                imports = format!("{imports}image,");
                view = format!("{view}\nimage(\"{path}\"){options}");
            }
            ElementName::SVG(path) => {
                imports = format!("{imports}svg,");
                view = format!("{view}\nsvg(\"{path}\"){options}");
            }
        }

        (imports, view)
    }
}

impl std::fmt::Display for RenderedElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut has_options = false;
        f.pad("")?;
        f.write_fmt(format_args!("{:?}\n", self.name))?;
        f.pad("")?;
        f.write_str("Options: (")?;
        for (k, v) in &self.options {
            if let Some(value) = v {
                has_options = true;
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
        if has_options {
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

impl<'a> From<RenderedElement> for Element<'a, Message> {
    fn from(value: RenderedElement) -> Self {
        let child_elements = match value.child_elements {
            Some(ref elements) => elements.clone(),
            None => vec![],
        };

        let content: Element<'a, Message> = match value.name.clone() {
            ElementName::Text(s) => {
                if s == String::new() {
                    widget::text("New Text").into()
                } else {
                    widget::text(s).into()
                }
            }
            ElementName::Button(s) => {
                if s == String::new() {
                    widget::button(widget::text("New Button")).into()
                } else {
                    widget::button(widget::text(s)).into()
                }
            }
            ElementName::SVG(p) => widget::svg(p).into(),
            ElementName::Image(p) => widget::image(p).into(),
            ElementName::Container => {
                widget::container(if child_elements.len() == 1 {
                    child_elements[0].clone().into()
                } else {
                    Element::from("")
                })
                .padding(20)
                .into()
            }
            ElementName::Row => widget::Row::from_iter(
                child_elements.into_iter().map(|el| el.into()),
            )
            .padding(20)
            .into(),
            ElementName::Column => widget::Column::from_iter(
                child_elements.into_iter().map(|el| el.into()),
            )
            .padding(20)
            .into(),
        };
        iced_drop::droppable(content)
            .id(value.get_id())
            .drag_hide(true)
            .on_drop(move |point, rect| {
                Message::MoveElement(value.clone(), point, rect)
            })
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    AddNew,
    PushFront(Id),
    InsertAfter(Id, Id),
    Drop,
    Stop,
}

impl Action {
    pub fn new(
        ids: Vec<Id>,
        element_tree: &mut Option<RenderedElement>,
        source_id: Option<Id>,
    ) -> Self {
        let mut action = Self::Stop;
        if ids.len() == 1 {
            if element_tree.is_none() {
                action = Self::AddNew;
            } else {
                action = Self::Drop;
            }
        } else {
            let id: Id = match source_id {
                Some(id) if ids.contains(&id) => {
                    let element_id =
                        ids[ids.iter().position(|x| *x == id).unwrap()].clone();
                    if ids.len() > 2 && ids[ids.clone().len() - 1] == element_id
                    {
                        return Self::Stop;
                    }
                    element_id
                }
                _ => ids.last().cloned().unwrap(),
            };
            let element = element_tree
                .as_mut()
                .unwrap()
                .find_by_id(id.clone())
                .unwrap();

            // Element IS a parent but ISN'T a non-empty container
            match element.is_parent()
                && !(element.name == ElementName::Container
                    && !element.is_empty())
            {
                true => {
                    action = Self::PushFront(id);
                }
                false if ids.len() > 2 => {
                    let parent = element_tree
                        .as_mut()
                        .unwrap()
                        .find_by_id(ids[&ids.len() - 2].clone())
                        .unwrap();

                    if parent.name == ElementName::Container
                        && parent.child_elements != Some(vec![])
                    {
                        action = Self::Stop;
                    } else {
                        action = Self::InsertAfter(
                            ids[&ids.len() - 2].clone(),
                            ids[&ids.len() - 1].clone(),
                        );
                    }
                }
                _ => {}
            }
        }
        action
    }
}

pub fn text(text: &str) -> RenderedElement {
    RenderedElement::new(ElementName::Text(text.to_owned()))
        .preset_options(vec!["size", "line_height", "width", "height"])
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
        Some(el) => RenderedElement::with(ElementName::Container, vec![el]),
        None => RenderedElement::with(ElementName::Container, vec![]),
    }
}

pub fn row(child_elements: Option<Vec<RenderedElement>>) -> RenderedElement {
    RenderedElement::with(ElementName::Row, child_elements.unwrap_or_default())
}

pub fn column(child_elements: Option<Vec<RenderedElement>>) -> RenderedElement {
    RenderedElement::with(
        ElementName::Column,
        child_elements.unwrap_or_default(),
    )
}
