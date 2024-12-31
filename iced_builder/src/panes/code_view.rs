use iced::widget::{button, pane_grid, row, text, text_editor, Space};
use iced::{Alignment, Font, Length, Theme};
use super::style;
use crate::icon::copy;
use crate::types::{DesignerPage, Message};
use crate::widget::tip;

pub fn view(
    editor_content: &text_editor::Content,
    theme: Theme,
    is_focused: bool,
) -> pane_grid::Content<'_, Message> {
    let title = row![
        text("Generated Code"),
        Space::with_width(Length::Fill),
        tip(
            button(copy()).on_press(Message::CopyCode),
            "Copy code to clipboard",
            tip::Position::FollowCursor
        ),
        Space::with_width(20),
        button("Switch to Designer view")
            .on_press(Message::SwitchPage(DesignerPage::DesignerView))
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
                if theme.to_string().contains("Dark") {
                    highlighter::Theme::SolarizedDark
                } else {
                    highlighter::Theme::InspiredGitHub
                },
            .font(Font::MONOSPACE)
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
