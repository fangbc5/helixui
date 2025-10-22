//! Overlay 系统
//!
//! 提供配置与通用类型

pub mod components;
pub mod config;
pub mod core;
mod utils;

pub use config::{MessagePosition, MessageType};

// 便捷再导出：让网站侧可通过 `helixui::overlay::*` 调用消息 API（来自 overlay 自身实现）
pub use components::message::{
    show_message, show_message_with_duration, show_message_with_position,
};
