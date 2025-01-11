use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

pub fn error_dialog(description: impl Into<String>) {
    let _ = MessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_buttons(MessageButtons::Ok)
        .set_title("Oops! Something went wrong.")
        .set_description(description)
        .show();
}

pub fn warning_dialog(description: impl Into<String>) {
    let _ = MessageDialog::new()
        .set_level(MessageLevel::Warning)
        .set_buttons(MessageButtons::Ok)
        .set_title("Heads up!")
        .set_description(description)
        .show();
}

pub fn unsaved_changes_dialog(description: impl Into<String>) -> bool {
    let result = MessageDialog::new()
        .set_level(MessageLevel::Warning)
        .set_buttons(MessageButtons::OkCancel)
        .set_title("Unsaved changes")
        .set_description(description)
        .show();

    matches!(result, MessageDialogResult::Ok)
}
