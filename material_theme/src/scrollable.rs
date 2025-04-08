use iced_widget::core::{Border, border};
use iced_widget::scrollable::{
    Catalog, Rail, Scroller, Status, Style, StyleFn,
};

use super::Theme;
use super::container::surface_container;
use super::utils::mix;
use crate::utils::{HOVERED_LAYER_OPACITY, PRESSED_LAYER_OPACITY};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let colors = theme.colorscheme.surface;

    let rail = Rail {
        background: None,
        scroller: Scroller {
            color: colors.on_surface,
            border: border::rounded(400),
        },
        border: Border::default(),
    };

    let style = Style {
        container: surface_container(theme),
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: None,
    };

    match status {
        Status::Active { .. } => style,
        Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            ..
        } => {
            let hovered_rail = Rail {
                scroller: Scroller {
                    color: mix(
                        colors.on_surface,
                        colors.color,
                        HOVERED_LAYER_OPACITY,
                    ),
                    border: border::rounded(400),
                },
                ..rail
            };

            Style {
                horizontal_rail: if is_horizontal_scrollbar_hovered {
                    hovered_rail
                } else {
                    rail
                },
                vertical_rail: if is_vertical_scrollbar_hovered {
                    hovered_rail
                } else {
                    rail
                },
                ..style
            }
        }
        Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            ..
        } => {
            let dragged_rail = Rail {
                scroller: Scroller {
                    color: mix(
                        colors.on_surface,
                        colors.color,
                        PRESSED_LAYER_OPACITY,
                    ),
                    border: border::rounded(400),
                },
                ..rail
            };

            Style {
                horizontal_rail: if is_horizontal_scrollbar_dragged {
                    dragged_rail
                } else {
                    rail
                },
                vertical_rail: if is_vertical_scrollbar_dragged {
                    dragged_rail
                } else {
                    rail
                },
                ..style
            }
        }
    }
}
