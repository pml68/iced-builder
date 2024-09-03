use crate::types::{props::Props, rendered_element::RenderedElement, ElementName};

impl RenderedElement {
    pub fn codegen(&self) -> (String, String) {
        let mut imports = String::new();
        let mut view = String::new();
        let mut props = String::new();

        let mut elements = String::new();

        for element in &self.child_elements {
            let mut children = String::new();

            (imports, children) = element.codegen();
            elements = format!("{elements}{},", children);
        }

        match &self.name {
            ElementName::Container => {
                imports = format!("{imports}\nuse iced::widget::container;");
                view = if self.child_elements.len() < 2 {
                    format!("{view}\ncontainer({elements}){props}")
                } else {
                    format!("{view}\ncontainer(){props}")
                };
            }
            ElementName::Row => {
                imports = format!("{imports}\nuse iced::widget::row;");
                view = format!("{view}\nrow![{elements}]{props}");
            }
            ElementName::Text(string) => {
                imports = format!("{imports}\nuse iced::widget::text;");
                view = format!("{view}\ntext(\"{string}\"){props}");
            }
            _ => {}
        }

        (imports, view)
    }
}
