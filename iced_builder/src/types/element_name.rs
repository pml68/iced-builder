use serde::{Deserialize, Serialize};

use crate::Error;

use super::rendered_element::{
    button, column, container, image, row, svg, text, ActionKind, RenderedElement,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElementName {
    Text(String),
    Button(String),
    SVG(String),
    Image(String),
    Container,
    Row,
    Column,
}

impl ElementName {
    pub const ALL: [Self; 7] = [
        Self::Text(String::new()),
        Self::Button(String::new()),
        Self::SVG(String::new()),
        Self::Image(String::new()),
        Self::Container,
        Self::Row,
        Self::Column,
    ];

    pub fn handle_action(
        &self,
        element_tree: Option<&mut RenderedElement>,
        action: ActionKind,
    ) -> Result<Option<RenderedElement>, Error> {
        let element = match self {
            Self::Text(_) => text(""),
            Self::Button(_) => button(""),
            Self::SVG(_) => svg(""),
            Self::Image(_) => image(""),
            Self::Container => container(None),
            Self::Row => row(None),
            Self::Column => column(None),
        };
        match action {
            ActionKind::Stop => Ok(None),
            ActionKind::AddNew => Ok(Some(element)),
            ActionKind::PushFront(id) => {
                element_tree
                    .ok_or("The action was of kind `PushFront`, but no element tree was provided.")?
                    .find_by_id(id)
                    .ok_or(Error::NonExistentElement)?
                    .push_front(&element);
                Ok(None)
            }
            ActionKind::InsertAfter(parent_id, child_id) => {
                element_tree
                    .ok_or(
                        "The action was of kind `InsertAfter`, but no element tree was provided.",
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
                Self::SVG(_) => "SVG",
                Self::Image(_) => "Image",
                Self::Container => "Container",
                Self::Row => "Row",
                Self::Column => "Column",
            }
        )
    }
}
