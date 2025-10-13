//! Overlay 插件系统
//!
//! 支持动画、主题、无障碍等插件

pub mod accessibility_plugin;
pub mod animation_plugin;
pub mod plugin_manager;
pub mod theme_plugin;

// 重新导出
pub use accessibility_plugin::AccessibilityPlugin;
pub use animation_plugin::AnimationPlugin;
pub use plugin_manager::{OverlayContext, OverlayPlugin, PluginManager};
pub use theme_plugin::ThemePlugin;
