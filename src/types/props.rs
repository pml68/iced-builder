use core::f32;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::text::Shaping,
    Alignment, ContentFit, Font, Length, Padding,
};
pub struct Props {
    pub align_items: Option<Alignment>,
    pub align_x: Option<Horizontal>,
    pub align_y: Option<Vertical>,
    pub horizontal_alignment: Option<Horizontal>,
    pub vertical_alignment: Option<Vertical>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub font: Option<Font>,
    pub padding: Option<Padding>,
    pub spacing: Option<f32>,
    pub content_fit: Option<ContentFit>,
    pub shaping: Option<Shaping>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            align_items: None,
            align_x: None,
            align_y: None,
            horizontal_alignment: None,
            vertical_alignment: None,
            width: None,
            height: None,
            max_width: None,
            max_height: None,
            font: None,
            padding: None,
            spacing: None,
            content_fit: None,
            shaping: None,
        }
    }
}
