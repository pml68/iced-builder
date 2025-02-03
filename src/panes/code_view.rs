use iced::widget::{button, pane_grid, row, text, text_editor, Space};
use iced::{Alignment, Background, Border, Font, Length, Theme};
use iced_custom_highlighter::{Highlight, Highlighter, Settings};

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
            .font(Font::MONOSPACE)
            .highlight_with::<Highlighter>(
                Settings::new(vec![], Highlight::default_style, theme, "rs"),
                Highlight::to_format,
            )
            .style(|theme, _| {
                let palette = theme.extended_palette();
                text_editor::Style {
                    background: Background::Color(
                        palette.background.base.color,
                    ),
                    border: Border {
                        radius: 2.0.into(),
                        width: 1.0,
                        color: palette.background.strong.color,
                    },
                    icon: palette.background.weak.text,
                    placeholder: palette.background.strong.color,
                    value: palette.background.base.text,
                    selection: palette.primary.weak.color,
                }
            })
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
