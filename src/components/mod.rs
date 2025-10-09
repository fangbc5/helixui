//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

pub mod common;
mod copyright;
mod demo_box;
mod footer;
mod logo;
mod navbar;

pub use common::{Button, ButtonSize, ButtonType, Icon, IconSize, IconType};
pub use demo_box::DemoBox;
