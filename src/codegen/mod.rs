use rust_format::{Config, Edition, Formatter, RustFmt};

use crate::types::{props::Props, rendered_element::RenderedElement, ElementName};

impl Props {
    pub fn codegen(&self) -> String {
        let mut props_string = String::new();

        match self.spacing {
            Some(s) => {
                props_string = format!("{props_string}.spacing({s})");
            }
            None => {}
        }

        match self.max_height {
            Some(h) => {
                props_string = format!("{props_string}.max_height({h})");
            }
            None => {}
        }

        props_string
    }
}

impl RenderedElement {
    fn codegen(&self) -> (String, String) {
        let mut imports = String::new();
        let mut view = String::new();
        let props = self.props.codegen();

        let mut elements = String::new();

        for element in &self.child_elements {
            let mut children = String::new();

            (imports, children) = element.codegen();
            elements = format!("{elements}{},", children);
        }

        match &self.name {
            ElementName::Container => {
                imports = format!("{imports}widget::container,");
                view = if self.child_elements.len() < 2 {
                    format!("{view}\ncontainer({elements}){props}")
                } else {
                    format!("{view}\ncontainer(){props}")
                };
            }
            ElementName::Row => {
                imports = format!("{imports}widget::row,");
                view = format!("{view}\nrow![{elements}]{props}");
            }
            ElementName::Text(string) => {
                imports = format!("{imports}widget::text,");
                view = format!("{view}\ntext(\"{string}\"){props}");
            }
            _ => {}
        }

        (imports, view)
    }

    pub fn app_code(&self, title: String) -> Result<String, Box<dyn std::error::Error>> {
        let (imports, view) = self.codegen();
        let mut app_code = format!("use iced::{{{imports}Sandbox,Settings,Element}};");

        app_code = format!(
            r#"{app_code}

        fn main() -> iced::Result {{
            App::run(Settings::default())
        }}

        struct App;

        impl Sandbox for App {{
            type Message = ();

            fn new() -> Self {{
                Self {{}}
            }}

            fn title(&self) -> String {{
                "{title}".into()
            }}

            fn update(&mut self, message: Message) {{

            }}

            fn view(&self) -> Element<Message> {{
                {view}.into()
            }}
        }}"#
        );

        Ok(RustFmt::default().format_str(app_code)?)
    }
}
