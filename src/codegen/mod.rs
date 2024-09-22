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

        match self.name {
            ElementName::Column | ElementName::Row | ElementName::Container => {
                for element in &self.child_elements {
                    let mut children = String::new();

                    (imports, children) = element.codegen();
                    elements = format!("{elements}{},", children);
                }
            }
            _ => {}
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
            ElementName::Column => {
                imports = format!("{imports}column,");
                view = format!("{view}\ncolumn![{elements}]{props}");
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
            ElementName::Button(string) => {
                imports = format!("{imports}button,");
                view = format!(
                    "{view}\nbutton(\"{}\"){props}",
                    if *string == String::new() {
                        "New Button"
                    } else {
                        string
                    }
                );
            }
            ElementName::Image(path) => {
                imports = format!("{imports}image,");
                view = format!("{view}\nimage({}){props}", path.display().to_string());
            }
            ElementName::SVG(path) => {
                imports = format!("{imports}svg,");
                view = format!("{view}\nsvg({}){props}", path.display().to_string());
            }
        }

        (imports, view)
    }

    pub fn app_code(
        &self,
        title: &str,
        theme: Option<iced::Theme>,
    ) -> Result<String, Box<dyn std::error::Error>> {
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

            fn theme(&self) -> iced::Theme {{
                iced::Theme::{}
            }}

            fn update(&mut self, message: Message) {{

            }}

            fn view(&self) -> Element<Message> {{
                {view}.into()
            }}
        }}"#,
            if let Some(c) = theme {
                c.to_string().replace(' ', "")
            } else {
                "default()".to_owned()
            }
        );

        Ok(RustFmt::default().format_str(app_code)?)
    }

    pub fn test() -> String {
        let mut text1 = RenderedElement::new(ElementName::Text("wow"));
        text1.set_property("padding", "[10, 20]");
        text1.set_property("spacing", "20.0");
        text1.set_property("max_height", "120.5");
        text1.set_property("max_width", "230");

        let element = RenderedElement::new(ElementName::Container).push(RenderedElement::from_vec(
            ElementName::Row,
            vec![text1, RenderedElement::new(ElementName::Text("heh"))],
        ));

        element.app_code("new app", None).unwrap()
    }
}
