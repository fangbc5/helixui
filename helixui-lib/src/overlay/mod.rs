//! Overlay 系统
//!
//! 提供配置与通用类型

pub mod config;
pub mod core;

pub use config::{SimpleMessagePosition, SimpleMessageType};

// 便捷再导出：让网站侧可通过 `helixui::overlay::*` 调用消息 API
pub use crate::components::feedback::{
    show_message, show_message_with_duration, show_message_with_position,
};
