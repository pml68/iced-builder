use serde::{Deserialize, Serialize};

use super::rendered_element::{
    Action, RenderedElement, button, column, container, image, row, svg, text,
};
use crate::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElementName {
    Text(String),
    Button(String),
    Svg(String),
    Image(String),
    Container,
    Row,
    Column,
}

impl ElementName {
    pub const ALL: &'static [Self; 7] = &[
        Self::Text(String::new()),
        Self::Button(String::new()),
        Self::Svg(String::new()),
        Self::Image(String::new()),
        Self::Container,
        Self::Row,
        Self::Column,
    ];

    pub fn handle_action(
        &self,
        element_tree: Option<&mut RenderedElement>,
        action: Action,
    ) -> Result<Option<RenderedElement>, Error> {
        let element = match self {
            Self::Text(_) => text(""),
            Self::Button(_) => button(""),
            Self::Svg(_) => svg(""),
            Self::Image(_) => image(""),
            Self::Container => container(None),
            Self::Row => row(vec![]),
            Self::Column => column(vec![]),
        };
        match action {
            Action::Stop | Action::Drop => Ok(None),
            Action::AddNew => Ok(Some(element)),
            Action::PushFront(id) => {
                element_tree
                    .ok_or("the action was of kind `PushFront`, but no element tree was provided.")?
                    .find_by_id(id)
                    .ok_or(Error::NonExistentElement)?
                    .push_front(&element);
                Ok(None)
            }
            Action::InsertAfter(parent_id, child_id) => {
                element_tree
                    .ok_or(
                        "the action was of kind `InsertAfter`, but no element tree was provided.",
                    )?
                    .find_by_id(parent_id)
                    .ok_or(Error::NonExistentElement)?
                    .insert_after(child_id, &element);
                Ok(None)
            }
        }
    }
}

impl std::fmt::Display for ElementName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Text(_) => "Text",
                Self::Button(_) => "Button",
                Self::Svg(_) => "SVG",
                Self::Image(_) => "Image",
                Self::Container => "Container",
                Self::Row => "Row",
                Self::Column => "Column",
            }
        )
    }
}
