use iced::widget::{Space, button, container, pane_grid, row, text, themer};
use iced::{Alignment, Element, Length};

use super::style;
use crate::types::{DesignerPane, Message, RenderedElement};

pub fn view<'a>(
    element_tree: Option<&RenderedElement>,
    designer_theme: iced::Theme,
    is_focused: bool,
) -> pane_grid::Content<'a, Message> {
    let el_tree: Element<'a, Message> = match element_tree {
        Some(tree) => tree.clone().into(),
        None => text("Open a project or begin creating one").into(),
    };
    let content = container(themer(designer_theme, el_tree))
        .id(iced::widget::container::Id::new("drop_zone"))
        .height(Length::Fill)
        .width(Length::Fill);
    let title = row![
        text("Designer"),
        Space::with_width(Length::Fill),
        button("Switch to Code view")
            .on_press(Message::SwitchPage(DesignerPane::CodeView)),
    ]
    .align_y(Alignment::Center);
    let title_bar = pane_grid::TitleBar::new(title)
        .padding(10)
        .style(style::title_bar);
    pane_grid::Content::new(content)
        .title_bar(title_bar)
        .style(if is_focused {
            style::pane_focused
        } else {
            style::pane_active
        })
}
