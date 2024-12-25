use iced::widget::{column, container, pane_grid, text, Column};
use iced::{Alignment, Element, Length};
use iced_drop::droppable;

use super::style;
use crate::types::{ElementName, Message};

fn items_list_view<'a>(items: &'a [ElementName]) -> Element<'a, Message> {
    let mut column = Column::new()
        .spacing(20)
        .align_x(Alignment::Center)
        .width(Length::Fill);

    for item in items {
        column =
            column.push(droppable(text(item.clone().to_string())).on_drop(
                move |point, rect| {
                    Message::DropNewElement(item.clone(), point, rect)
                },
            ));
    }

    container(column)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn view<'a>(
    element_list: &'a [ElementName],
    is_focused: bool,
) -> pane_grid::Content<'a, Message> {
    let items_list = items_list_view(element_list);
    let content = column![items_list]
        .align_x(Alignment::Center)
        .height(Length::Fill)
        .width(Length::Fill);
    let title = text("Element List");
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
