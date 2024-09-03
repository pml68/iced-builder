use crate::types::{ElementName, RenderedElement};

impl RenderedElement {
    pub fn codegen(&self) -> Result<(String, String), &str> {
        let mut imports = String::new();
        let mut view = String::new();
        let mut props = String::new();

        let mut elements = String::new();

        match self.children {
            Some(els) => {
                for el in els {
                    let mut children = String::new();

                    match el.codegen() {
                        Ok(e) => (children, imports) = e,
                        Err(err) => return Err(err),
                    }
                    elements = format!("{elements},{}", children);
                }
            }
            None => {}
        }

        match self.name {
            ElementName::Row => {
                imports = format!("{imports}\nuse iced::widget::row;");
                view = format!("{view}\nrow![{elements}]{props};");
            }
            ElementName::Container => {
                imports = format!("{imports}\nuse iced::widget::container;");
                view = format!("{view}\ncontainer({elements}){props};");
            }
            _ => {}
        }
    }
}
