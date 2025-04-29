use iced::advanced::text::highlighter::Format;
use iced::border::Radius;
use iced::widget::{button, pane_grid, row, text, text_editor};
use iced::{Alignment, Border, Font, Length};
use iced_custom_highlighter::{Highlight, Highlighter, Scope, Settings};
use material_theme::Theme;

use super::style;
use crate::icon;
use crate::types::{DesignerPane, Message};
use crate::widget::tip;

// TODO: implement a highlight style for the material theme
fn highlight_style(theme: &Theme, scope: &Scope) -> Format<Font> {
    let theme = if theme.is_dark() {
        iced::Theme::SolarizedDark
    } else {
        iced::Theme::SolarizedLight
    };

    match scope {
        Scope::Custom { .. } | Scope::Other => Format {
            color: Some(theme.extended_palette().primary.strong.color),
            font: None,
        },
        _ => Highlight::default_style(&theme, scope),
    }
}

pub fn view(
    editor_content: &text_editor::Content,
    is_focused: bool,
) -> pane_grid::Content<'_, Message, Theme> {
    let title_bar = pane_grid::TitleBar::new(text("Generated Code").center())
        .controls(pane_grid::Controls::dynamic(
            row![
                tip(
                    button(icon::copy())
                        .on_press(Message::CopyCode)
                        .padding([2, 7])
                        .style(material_theme::button::text),
                    "Copy",
                    tip::Position::FollowCursor
                ),
                button("Switch to Designer view")
                    .on_press(Message::SwitchPage(DesignerPane::DesignerView))
            ]
            .spacing(20)
            .align_y(Alignment::Center),
            row![
                tip(
                    button(icon::copy())
                        .on_press(Message::CopyCode)
                        .padding([2, 7])
                        .style(material_theme::button::text),
                    "Copy",
                    tip::Position::FollowCursor
                ),
                button(icon::switch())
                    .on_press(Message::SwitchPage(DesignerPane::DesignerView))
            ]
            .spacing(20)
            .align_y(Alignment::Center),
        ))
        .padding(10)
        .style(style::title_bar);

    pane_grid::Content::new(
        text_editor(editor_content)
            .on_action(Message::EditorAction)
            .font(Font::MONOSPACE)
            .highlight_with::<Highlighter<Theme>>(
                Settings::new(vec![], highlight_style, "rs"),
                Highlight::to_format,
            )
            .style(|theme, _| {
                let style = material_theme::text_editor::default(
                    theme,
                    text_editor::Status::Active,
                );

                text_editor::Style {
                    border: Border {
                        radius: Radius::default(),
                        ..style.border
                    },
                    ..style
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
