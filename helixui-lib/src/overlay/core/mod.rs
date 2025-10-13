//! Overlay 核心层
//!
//! 包含状态管理、事件系统、平台适配等核心功能

pub mod animation_manager;
pub mod event_system;
pub mod platform_adapter;
pub mod state_manager;
pub mod theme_manager;

// 重新导出
pub use animation_manager::{AnimationConfig, AnimationManager, AnimationType, EasingType};
pub use event_system::{Event, EventBus, EventListener, EventType};
pub use platform_adapter::{Platform, PlatformAdapter, PlatformCapabilities};
pub use state_manager::{GlobalStateManager, OverlayState, OverlayType};
pub use theme_manager::{ThemeConfig, ThemeManager, ThemeMode};
