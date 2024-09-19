use rust_format::{Config, Edition, Formatter, RustFmt};

use crate::types::{rendered_element::RenderedElement, ElementName};

impl RenderedElement {
    fn props_codegen(&self) -> String {
        let mut props_string = String::new();

        for (k, v) in self.props.clone() {
            props_string = format!("{props_string}.{k}({v})");
        }

        props_string
    }

    fn codegen(&self) -> (String, String) {
        let mut imports = String::new();
        let mut view = String::new();
        let props = self.props_codegen();

        let mut elements = String::new();

        for element in &self.child_elements {
            let mut children = String::new();

            (imports, children) = element.codegen();
            elements = format!("{elements}{},", children);
        }

        match &self.name {
            ElementName::Container => {
                imports = format!("{imports}container,");
                view = if self.child_elements.len() < 2 {
                    format!("{view}\ncontainer({elements}){props}")
                } else {
                    format!("{view}\ncontainer(){props}")
                };
            }
            ElementName::Row => {
                imports = format!("{imports}row,");
                view = format!("{view}\nrow![{elements}]{props}");
            }
            ElementName::Text(string) => {
                imports = format!("{imports}text,");
                view = format!(
                    "{view}\ntext(\"{}\"){props}",
                    if *string == String::new() {
                        "New Text"
                    } else {
                        string
                    }
                );
            }
            _ => {}
        }

        (imports, view)
    }

    pub fn app_code(&self, title: &str) -> Result<String, Box<dyn std::error::Error>> {
        let (imports, view) = self.codegen();
        let mut app_code = format!("use iced::{{widget::{{{imports}}},Sandbox,Settings,Element}};");

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

    pub fn test() -> String {
        let mut text1 = RenderedElement::new(ElementName::Text("wow"));
        text1.set_property("padding", "[10, 20]");
        text1.set_property("spacing", "20.0");
        text1.set_property("max_height", "120.5");

        let element = RenderedElement::new(ElementName::Container).push(RenderedElement::from_vec(
            ElementName::Row,
            vec![text1, RenderedElement::new(ElementName::Text("heh"))],
        ));

        element.app_code("new app").unwrap()
    }
}
