//! Overlay 组件层
//!
//! 重新整理后的清晰结构：
//!
//! ## 核心组件
//! - `base.rs` - 通用 BaseOverlay 组件
//! - `factory.rs` - Overlay 工厂模式
//! - `host.rs` - 全局宿主组件
//!
//! ## 具体实现
//! - `message.rs` - 消息组件
//! - `dialog.rs` - 对话框组件
//! - `modal.rs` - 模态框组件
//! - `notice.rs` - 通知组件
//!
//! ## 数据定义
//! - `types.rs` - 共享的数据结构

// 核心组件
pub mod base;
pub mod factory;
pub mod host;

// 具体实现
pub mod dialog;
pub mod message;
pub mod modal;
pub mod notice;

// 数据定义
pub mod types;

// 重新导出核心 API
pub use base::{BaseOverlay, BaseOverlayProps, OverlayPosition};
pub use factory::OverlayFactory;
pub use host::GlobalOverlayHost;

// 重新导出具体组件（向后兼容）
pub use dialog::DialogOverlay;
pub use message::MessageOverlay;
pub use modal::ModalOverlay;
pub use notice::NoticeOverlay;
