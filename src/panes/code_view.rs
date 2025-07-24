use iced::advanced::text::highlighter::Format;
use iced::border::Radius;
use iced::widget::{button, hover, pane_grid, right, text, text_editor};
use iced::{Border, Font, Length};
use iced_custom_highlighter::{Highlight, Highlighter, Scope, Settings};
use iced_material::Theme;

use super::style;
use crate::icon;
use crate::types::{DesignerPane, Message};

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
            button("Switch to Designer view")
                .on_press(DesignerPane::DesignerView.into()),
            button(icon::switch()).on_press(DesignerPane::DesignerView.into()),
        ))
        .padding(10)
        .style(style::title_bar);

    let editor = text_editor(editor_content)
        .on_action(Message::EditorAction)
        .font(Font::MONOSPACE)
        .highlight_with::<Highlighter<Theme>>(
            Settings::new(vec![], highlight_style, "rs"),
            Highlight::to_format,
        )
        .style(|theme, _| {
            let style = iced_material::text_editor::default(
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
        .padding(20);

    let copy = button(icon::copy().size(22).line_height(1.0).center())
        .on_press(Message::CopyCode)
        .padding(3)
        .width(28)
        .style(|theme, status| {
            let style = iced_material::button::text(theme, status);

            button::Style {
                border: style.border.rounded(4),
                ..style
            }
        });

    pane_grid::Content::new(hover(editor, right(copy).padding(16.0 * 0.875)))
        .title_bar(title_bar)
        .style(if is_focused {
            style::pane_focused
        } else {
            style::pane_active
        })
}
