use std::collections::BTreeMap;

#[allow(unused_imports)]
use iced::widget::{Button, Column, Container, Image, Row, Svg, Text};
use iced::{Padding, Rotation};

use crate::values::ValueFromStr;

pub trait ApplyOptions: Sized {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self;
}

impl<Message> ApplyOptions for Button<'_, Message> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut button = self;

        if let Some(padding) = options.get("padding").expect("padding key") {
            let padding = Padding::value_from_str(padding).unwrap();
            button = button.padding(padding);
        }

        button
    }
}

impl<Message> ApplyOptions for Column<'_, Message> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut column = self;

        if let Some(padding) = options.get("padding").expect("padding key") {
            let padding = Padding::value_from_str(padding).unwrap();
            column = column.padding(padding);
        }

        column
    }
}

impl<Message> ApplyOptions for Row<'_, Message> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut row = self;

        if let Some(padding) = options.get("padding").expect("padding key") {
            let padding = Padding::value_from_str(padding).unwrap();
            row = row.padding(padding);
        }

        row
    }
}

impl<Handle> ApplyOptions for Image<Handle> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut image = self;

        if let Some(rotation) = options.get("rotation").expect("rotation key") {
            let rotation = Rotation::value_from_str(rotation).unwrap();
            image = image.rotation(rotation);
        }

        image
    }
}

impl ApplyOptions for Svg<'_> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut svg = self;

        if let Some(rotation) = options.get("rotation").expect("rotation key") {
            let rotation = Rotation::value_from_str(rotation).unwrap();
            svg = svg.rotation(rotation);
        }

        svg
    }
}
