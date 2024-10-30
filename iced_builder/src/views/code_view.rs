use super::style;
use crate::types::{DesignerPage, Message};
use iced::{
    highlighter,
    widget::{button, container, pane_grid, row, text, text_editor, tooltip, Space},
    Alignment, Font, Length,
};

pub fn view<'a>(
    editor_content: &'a text_editor::Content,
    dark_theme: bool,
    is_focused: bool,
) -> pane_grid::Content<'a, Message> {
    let title = row![
        text("Generated Code"),
        Space::with_width(Length::Fill),
        tooltip(
            button(container(text('\u{0e801}').font(Font::with_name("editor-icons"))).center_x(30))
                .on_press(Message::CopyCode),
            "Copy code to clipboard",
            tooltip::Position::FollowCursor
        ),
        Space::with_width(20),
        button("Switch to Designer view").on_press(Message::SwitchPage(DesignerPage::Designer))
    ]
    .align_y(Alignment::Center);
    let title_bar = pane_grid::TitleBar::new(title)
        .padding(10)
        .style(style::title_bar);
    pane_grid::Content::new(
        text_editor(editor_content)
            .on_action(Message::EditorAction)
            .highlight(
                "rs",
                if dark_theme {
                    highlighter::Theme::SolarizedDark
                } else {
                    highlighter::Theme::InspiredGitHub
                },
            )
            .height(Length::Fill)
            .padding(20),
    )
    .title_bar(title_bar)
    .style(if is_focused {
        style::pane_focused
    } else {
        style::pane_active
    })
}
