use std::collections::BTreeMap;
use std::str::FromStr;

use iced::widget::text::LineHeight;
#[allow(unused_imports)]
use iced::widget::{Button, Column, Container, Image, Row, Svg, Text};
use iced::{Alignment, ContentFit, Length, Padding, Pixels, Rotation};

use crate::values::Value;

pub trait ApplyOptions {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self;
}

impl<Message> ApplyOptions for Button<'_, Message> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut button = self;

        if let Some(width) = options.get("width").expect("width key") {
            let width = Length::from_str(width).unwrap();
            button = button.width(width);
        }

        if let Some(height) = options.get("height").expect("height key") {
            let height = Length::from_str(height).unwrap();
            button = button.height(height);
        }

        if let Some(padding) = options.get("padding").expect("padding key") {
            let padding = Padding::from_str(padding).unwrap();
            button = button.padding(padding);
        }

        if let Some(clip) = options.get("clip").expect("clip key") {
            let clip = bool::from_str(clip).unwrap();
            button = button.clip(clip);
        }

        button
    }
}

impl ApplyOptions for Text<'_> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut text = self;

        if let Some(size) = options.get("size").expect("size key") {
            let size = Pixels::from_str(size).unwrap();
            text = text.size(size);
        }

        if let Some(line_height) =
            options.get("line_height").expect("line_height key")
        {
            let line_height = LineHeight::from_str(line_height).unwrap();
            text = text.line_height(line_height);
        }

        if let Some(width) = options.get("width").expect("width key") {
            let width = Length::from_str(width).unwrap();
            text = text.width(width);
        }

        if let Some(height) = options.get("height").expect("height key") {
            let height = Length::from_str(height).unwrap();
            text = text.height(height);
        }

        if let Some(align_x) = options.get("align_x").expect("align_x key") {
            let align_x = Alignment::from_str(align_x).unwrap();
            text = text.align_x(align_x);
        }

        if let Some(align_y) = options.get("align_y").expect("align_y key") {
            let align_y = Alignment::from_str(align_y).unwrap();
            text = text.align_y(align_y);
        }

        text
    }
}

impl<Message> ApplyOptions for Container<'_, Message> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut container = self;

        if let Some(padding) = options.get("padding").expect("padding key") {
            let padding = Padding::from_str(padding).unwrap();
            container = container.padding(padding);
        }

        if let Some(width) = options.get("width").expect("width key") {
            let width = Length::from_str(width).unwrap();
            container = container.width(width);
        }

        if let Some(height) = options.get("height").expect("height key") {
            let height = Length::from_str(height).unwrap();
            container = container.height(height);
        }

        if let Some(max_width) =
            options.get("max_width").expect("max_width key")
        {
            let max_width = Pixels::from_str(max_width).unwrap();
            container = container.max_width(max_width);
        }

        if let Some(max_height) =
            options.get("max_height").expect("max_height key")
        {
            let max_height = Pixels::from_str(max_height).unwrap();
            container = container.max_height(max_height);
        }

        if let Some(center_x) = options.get("center_x").expect("center_x key") {
            let center_x = Length::from_str(center_x).unwrap();
            container = container.center_x(center_x);
        }

        if let Some(center_y) = options.get("center_y").expect("center_y key") {
            let center_y = Length::from_str(center_y).unwrap();
            container = container.center_y(center_y);
        }

        if let Some(center) = options.get("center").expect("center key") {
            let center = Length::from_str(center).unwrap();
            container = container.center(center);
        }

        if let Some(align_left) =
            options.get("align_left").expect("align_left key")
        {
            let align_left = Length::from_str(align_left).unwrap();
            container = container.align_left(align_left);
        }

        if let Some(align_right) =
            options.get("align_right").expect("align_right key")
        {
            let align_right = Length::from_str(align_right).unwrap();
            container = container.align_right(align_right);
        }

        if let Some(align_top) =
            options.get("align_top").expect("align_top key")
        {
            let align_top = Length::from_str(align_top).unwrap();
            container = container.align_top(align_top);
        }

        if let Some(align_bottom) =
            options.get("align_bottom").expect("align_bottom key")
        {
            let align_bottom = Length::from_str(align_bottom).unwrap();
            container = container.align_bottom(align_bottom);
        }

        if let Some(align_x) = options.get("align_x").expect("align_x key") {
            let align_x = Alignment::from_str(align_x).unwrap();
            container = container.align_x(align_x);
        }

        if let Some(align_y) = options.get("align_y").expect("align_y key") {
            let align_y = Alignment::from_str(align_y).unwrap();
            container = container.align_y(align_y);
        }

        if let Some(clip) = options.get("clip").expect("clip key") {
            let clip = bool::from_str(clip).unwrap();
            container = container.clip(clip);
        }

        container
    }
}

impl<Message> ApplyOptions for Column<'_, Message> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut column = self;

        if let Some(spacing) = options.get("spacing").expect("spacing key") {
            let spacing = Pixels::from_str(spacing).unwrap();
            column = column.spacing(spacing);
        }

        if let Some(padding) = options.get("padding").expect("padding key") {
            let padding = Padding::from_str(padding).unwrap();
            column = column.padding(padding);
        }

        if let Some(width) = options.get("width").expect("width key") {
            let width = Length::from_str(width).unwrap();
            column = column.width(width);
        }

        if let Some(height) = options.get("height").expect("height key") {
            let height = Length::from_str(height).unwrap();
            column = column.height(height);
        }

        if let Some(max_width) =
            options.get("max_width").expect("max_width key")
        {
            let max_width = Pixels::from_str(max_width).unwrap();
            column = column.max_width(max_width);
        }

        if let Some(align_x) = options.get("align_x").expect("align_x key") {
            let align_x = Alignment::from_str(align_x).unwrap();
            column = column.align_x(align_x);
        }

        if let Some(clip) = options.get("clip").expect("clip key") {
            let clip = bool::from_str(clip).unwrap();
            column = column.clip(clip);
        }

        column
    }
}

impl<Message> ApplyOptions for Row<'_, Message> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut row = self;

        if let Some(spacing) = options.get("spacing").expect("spacing key") {
            let spacing = Pixels::from_str(spacing).unwrap();
            row = row.spacing(spacing);
        }

        if let Some(padding) = options.get("padding").expect("padding key") {
            let padding = Padding::from_str(padding).unwrap();
            row = row.padding(padding);
        }

        if let Some(width) = options.get("width").expect("width key") {
            let width = Length::from_str(width).unwrap();
            row = row.width(width);
        }

        if let Some(height) = options.get("height").expect("height key") {
            let height = Length::from_str(height).unwrap();
            row = row.height(height);
        }

        if let Some(align_y) = options.get("align_y").expect("align_y key") {
            let align_y = Alignment::from_str(align_y).unwrap();
            row = row.align_y(align_y);
        }

        if let Some(clip) = options.get("clip").expect("clip key") {
            let clip = bool::from_str(clip).unwrap();
            row = row.clip(clip);
        }

        row
    }
}

impl<Handle> ApplyOptions for Image<Handle> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut image = self;

        if let Some(width) = options.get("width").expect("width key") {
            let width = Length::from_str(width).unwrap();
            image = image.width(width);
        }

        if let Some(height) = options.get("height").expect("height key") {
            let height = Length::from_str(height).unwrap();
            image = image.height(height);
        }

        if let Some(content_fit) =
            options.get("content_fit").expect("content_fit key")
        {
            let content_fit = ContentFit::from_str(content_fit).unwrap();
            image = image.content_fit(content_fit);
        }

        if let Some(rotation) = options.get("rotation").expect("rotation key") {
            let rotation = Rotation::from_str(rotation).unwrap();
            image = image.rotation(rotation);
        }

        if let Some(opacity) = options.get("opacity").expect("opacity key") {
            let opacity = f32::from_str(opacity).unwrap();
            image = image.opacity(opacity);
        }

        if let Some(scale) = options.get("scale").expect("scale key") {
            let scale = f32::from_str(scale).unwrap();
            image = image.scale(scale);
        }

        image
    }
}

impl ApplyOptions for Svg<'_> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut svg = self;

        if let Some(width) = options.get("width").expect("width key") {
            let width = Length::from_str(width).unwrap();
            svg = svg.width(width);
        }

        if let Some(height) = options.get("height").expect("height key") {
            let height = Length::from_str(height).unwrap();
            svg = svg.height(height);
        }

        if let Some(content_fit) =
            options.get("content_fit").expect("content_fit key")
        {
            let content_fit = ContentFit::from_str(content_fit).unwrap();
            svg = svg.content_fit(content_fit);
        }

        if let Some(rotation) = options.get("rotation").expect("rotation key") {
            let rotation = Rotation::from_str(rotation).unwrap();
            svg = svg.rotation(rotation);
        }

        if let Some(opacity) = options.get("opacity").expect("opacity key") {
            let opacity = f32::from_str(opacity).unwrap();
            svg = svg.opacity(opacity);
        }

        svg
    }
}
