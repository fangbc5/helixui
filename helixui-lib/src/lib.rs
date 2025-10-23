//! # HelixUI
//!
//! A modern UI component library for Dioxus applications.
//!
//! ## Features
//!
//! - 🎨 Modern and beautiful components
//! - 🌙 Dark mode support
//! - 📱 Responsive design
//! - 🚀 Built with Dioxus
//! - 🎯 Type-safe and performant
//!
//! ## Quick Start
//!
//! ```rust
//! use helixui::components::{Button, ButtonType};
//! use dioxus::prelude::*;
//!
//! fn App() -> Element {
//!     rsx! {
//!         Button {
//!             button_type: ButtonType::Primary,
//!             "Hello HelixUI!"
//!         }
//!     }
//! }
//! ```

pub mod components;

// Re-export commonly used components
pub use components::*;
