use iced::{Element, Task};
use iced_dialog::button;

use crate::Message;
use crate::types::{DialogAction, DialogButtons};

pub const UNSAVED_CHANGES_TITLE: &str = "Unsaved changes";
pub const WARNING_TITLE: &str = "Heads up!";
pub const ERROR_TITLE: &str = "Oops! Something went wrong.";

pub fn ok_button<'a>() -> Element<'a, Message> {
    button("Ok").on_press(Message::DialogOk).into()
}

pub fn cancel_button<'a>() -> Element<'a, Message> {
    button("Cancel").on_press(Message::DialogCancel).into()
}

pub fn error_dialog(description: impl Into<String>) -> Task<Message> {
    Task::done(Message::OpenDialog(
        ERROR_TITLE,
        description.into(),
        DialogButtons::Ok,
        DialogAction::None,
    ))
}

pub fn warning_dialog(description: impl Into<String>) -> Task<Message> {
    Task::done(Message::OpenDialog(
        WARNING_TITLE,
        description.into(),
        DialogButtons::Ok,
        DialogAction::None,
    ))
}

pub fn unsaved_changes_dialog(
    description: impl Into<String>,
    action: DialogAction,
) -> Task<Message> {
    Task::done(Message::OpenDialog(
        UNSAVED_CHANGES_TITLE,
        description.into(),
        DialogButtons::OkCancel,
        action,
    ))
}
