use crate::types::{ElementName, RenderedElement};

impl RenderedElement {
    pub fn codegen(&self) -> Result<(String, String), &str> {
        let mut imports = String::new();
        let mut view = String::new();

        match self.name {
            ElementName::Row => {
                imports = format!("{imports}\nuse iced::widget::row");
                view = format!("{view}\nrow![]");
            }
        }
    }
}
