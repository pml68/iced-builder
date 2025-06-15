use iced::widget::{
    button, center, container, pane_grid, responsive, row, text, themer,
};
use iced::{Alignment, Length};
use material_theme::Theme;

use super::style;
use crate::icon;
use crate::types::{DesignerPane, Message, RenderedElement};

pub fn view<'a>(
    element_tree: Option<&'a RenderedElement>,
    designer_theme: iced::Theme,
    is_focused: bool,
) -> pane_grid::Content<'a, Message, Theme> {
    let el_tree: iced::Element<'a, Message> = match element_tree {
        Some(tree) => responsive(|size| {
            center(
                container(tree.clone())
                    .style(|theme| {
                        container::background(theme.palette().background)
                    })
                    .height(size.height * 0.5)
                    .width(size.height * 0.8),
            )
            .into()
        })
        .into(),
        None => center("Open a project or begin creating one").into(),
    };

    let content = container(themer(designer_theme, el_tree))
        .id(iced::widget::container::Id::new("drop_zone"))
        .height(Length::Fill)
        .width(Length::Fill);

    let title_bar = pane_grid::TitleBar::new(text("Designer").center())
        .controls(pane_grid::Controls::dynamic(
            row![
                button("Switch to Code view")
                    .on_press(DesignerPane::CodeView.into(),)
            ]
            .align_y(Alignment::Center),
            row![
                button(icon::switch()).on_press(DesignerPane::CodeView.into(),)
            ]
            .align_y(Alignment::Center),
        ))
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
