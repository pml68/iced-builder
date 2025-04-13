use iced_widget::core::{Border, Color, border};
use iced_widget::scrollable::{
    Catalog, Rail, Scroller, Status, Style, StyleFn,
};

use super::Theme;
use super::container::surface_container;
use super::utils::mix;
use crate::utils::{
    DISABLED_CONTAINER_OPACITY, DISABLED_TEXT_OPACITY, HOVERED_LAYER_OPACITY,
    PRESSED_LAYER_OPACITY,
};

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

    let active = Rail {
        background: None,
        scroller: Scroller {
            color: colors.on_surface,
            border: border::rounded(400),
        },
        border: Border::default(),
    };

    let disabled = Rail {
        background: Some(
            Color {
                a: DISABLED_CONTAINER_OPACITY,
                ..colors.on_surface
            }
            .into(),
        ),
        scroller: Scroller {
            color: Color {
                a: DISABLED_TEXT_OPACITY,
                ..colors.on_surface
            },
            border: border::rounded(400),
        },
        ..active
    };

    let style = Style {
        container: surface_container(theme),
        vertical_rail: active,
        horizontal_rail: active,
        gap: None,
    };

    match status {
        Status::Active {
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
        } => Style {
            horizontal_rail: if is_horizontal_scrollbar_disabled {
                disabled
            } else {
                active
            },
            vertical_rail: if is_vertical_scrollbar_disabled {
                disabled
            } else {
                active
            },
            ..style
        },
        Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
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
                ..active
            };

            Style {
                horizontal_rail: if is_horizontal_scrollbar_disabled {
                    disabled
                } else if is_horizontal_scrollbar_hovered {
                    hovered_rail
                } else {
                    active
                },
                vertical_rail: if is_vertical_scrollbar_disabled {
                    disabled
                } else if is_vertical_scrollbar_hovered {
                    hovered_rail
                } else {
                    active
                },
                ..style
            }
        }
        Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
            is_horizontal_scrollbar_disabled,
            is_vertical_scrollbar_disabled,
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
                ..active
            };

            Style {
                horizontal_rail: if is_horizontal_scrollbar_disabled {
                    disabled
                } else if is_horizontal_scrollbar_dragged {
                    dragged_rail
                } else {
                    active
                },
                vertical_rail: if is_vertical_scrollbar_disabled {
                    disabled
                } else if is_vertical_scrollbar_dragged {
                    dragged_rail
                } else {
                    active
                },
                ..style
            }
        }
    }
}
