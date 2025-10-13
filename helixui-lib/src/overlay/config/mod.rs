//! Overlay 配置系统
//!
//! 包含各种配置结构和默认值

pub mod overlay_config;
pub mod responsive_config;
pub mod simple_types;
pub mod z_index_config;

// 重新导出
pub use overlay_config::OverlayConfig;
pub use responsive_config::{BreakpointConfig, LayoutConfig, LayoutStyle, ResponsiveConfig};
pub use simple_types::{SimpleMessagePosition, SimpleMessageType};
pub use z_index_config::ZIndexConfig;
