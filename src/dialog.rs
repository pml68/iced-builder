use std::borrow::Cow;

use iced::widget::text;
use iced_dialog::button;

use crate::Message;
use crate::types::Element;

pub const UNSAVED_CHANGES_TITLE: &str = "Hold on for a sec!";
pub const WARNING_TITLE: &str = "Heads up!";
pub const ERROR_TITLE: &str = "Oops! Something went wrong.";

#[derive(Debug, Clone, Copy, Default)]
pub enum Action {
    #[default]
    None,
    Close,
    UnsavedChanges(UnsavedChanges),
}

#[derive(Debug, Clone, Copy)]
pub enum UnsavedChanges {
    New,
    Open,
    Exit,
}

impl From<Action> for Vec<Element<'_, Message>> {
    fn from(action: Action) -> Self {
        match action {
            Action::None => vec![],
            Action::Close => vec![button("Close", Message::DialogYes).into()],
            Action::UnsavedChanges(_) => vec![
                button("Don't Save", Message::DialogNo).into(),
                button("Save", Message::DialogYes).into(),
                button("Cancel", Message::DialogCancel).into(),
            ],
        }
    }
}

#[derive(Debug, Default)]
pub struct Dialog {
    is_open: bool,
    title: Cow<'static, str>,
    content: Cow<'static, str>,
    action: Action,
}

impl Dialog {
    pub fn new(
        title: impl Into<Cow<'static, str>>,
        content: impl Into<Cow<'static, str>>,
        action: Action,
    ) -> Self {
        Self {
            is_open: true,
            title: title.into(),
            content: content.into(),
            action,
        }
    }

    pub fn error(content: impl Into<Cow<'static, str>>) -> Self {
        Self::new(ERROR_TITLE, content, Action::Close)
    }

    pub fn warning(content: impl Into<Cow<'static, str>>) -> Self {
        Self::new(WARNING_TITLE, content, Action::Close)
    }

    pub fn unsaved_changes(
        content: impl Into<Cow<'static, str>>,
        unsaved_changes: UnsavedChanges,
    ) -> Self {
        Self::new(
            UNSAVED_CHANGES_TITLE,
            content,
            Action::UnsavedChanges(unsaved_changes),
        )
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn action(&self) -> Action {
        self.action
    }

    pub fn as_iced_dialog<'a>(
        &'a self,
        base: impl Into<Element<'a, Message>>,
    ) -> iced_dialog::Dialog<'a, Message, material_theme::Theme> {
        iced_dialog::Dialog::with_buttons(
            self.is_open,
            base,
            text(&*self.content),
            self.action.into(),
        )
        .title(&*self.title)
    }
}
