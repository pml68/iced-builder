use std::collections::BTreeMap;
use std::str::FromStr;

use iced::Padding;
use iced::widget::{Button, Column, Container, Image, Row, Svg, Text};

pub trait ApplyOptions {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self;
}

impl<'a, Message> ApplyOptions for Button<'a, Message> {
    fn apply_options(self, options: BTreeMap<String, Option<String>>) -> Self {
        let mut button = self;

        if let Some(padding) = options.get("padding").expect("padding key") {
            let padding: Padding = padding
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
                .and_then(|s| {
                    Some(
                        s.split(',')
                            .map(|n| f32::from_str(n).unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .and_then(|s| {
                    if s.len() == 4 {
                        Some(Padding {
                            top: s[0],
                            right: s[1],
                            bottom: s[2],
                            left: s[3],
                        })
                    } else {
                        None
                    }
                })
                .unwrap();
            button = button.padding(padding);
        }

        button
    }
}
