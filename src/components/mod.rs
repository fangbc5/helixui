//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

pub mod common;
mod copyright;
mod demo_box;
pub mod feedback;
mod footer;
mod logo;
mod navbar;

pub use common::{
    Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant, Icon, IconSize, IconType,
};
pub use demo_box::DemoBox;
pub use feedback::{
    close_all_dialogs, close_dialog, show_confirm_dialog, show_error_dialog, show_info_dialog,
    show_message, show_message_with_duration, show_message_with_position, show_success_dialog,
    show_warning_dialog, Dialog, DialogData, DialogManager, DialogType, GlobalDialogContainer,
    GlobalMessageContainer, Message, MessagePosition, MessageType,
};
