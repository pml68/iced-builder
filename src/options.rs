use std::collections::BTreeMap;

use iced::Padding;
#[allow(unused_imports)]
use iced::widget::{Button, Column, Container, Image, Row, Svg, Text};

use crate::values::ValueFromStr;

pub trait ApplyOptions {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self;
}

impl<Message> ApplyOptions for Button<'_, Message> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut button = self;

        if let Some(padding) = options.get("padding").expect("padding key") {
            let padding: Padding = Padding::value_from_str(padding).unwrap();
            button = button.padding(padding);
        }

        button
    }
}
