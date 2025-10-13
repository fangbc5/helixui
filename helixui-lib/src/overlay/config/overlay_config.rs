use crate::overlay::core::{AnimationConfig, PlatformCapabilities, ThemeConfig};

/// Overlay 主配置
#[derive(Debug, Clone)]
pub struct OverlayConfig {
    pub z_index: ZIndexConfig,
    pub animation: AnimationConfig,
    pub theme: ThemeConfig,
    pub responsive: ResponsiveConfig,
    pub platform: PlatformConfig,
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self {
            z_index: ZIndexConfig::default(),
            animation: AnimationConfig::default(),
            theme: ThemeConfig::default(),
            responsive: ResponsiveConfig::default(),
            platform: PlatformConfig::default(),
        }
    }
}

impl OverlayConfig {
    /// 创建新的配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 z-index 配置
    pub fn with_z_index(mut self, z_index: ZIndexConfig) -> Self {
        self.z_index = z_index;
        self
    }

    /// 设置动画配置
    pub fn with_animation(mut self, animation: AnimationConfig) -> Self {
        self.animation = animation;
        self
    }

    /// 设置主题配置
    pub fn with_theme(mut self, theme: ThemeConfig) -> Self {
        self.theme = theme;
        self
    }

    /// 设置响应式配置
    pub fn with_responsive(mut self, responsive: ResponsiveConfig) -> Self {
        self.responsive = responsive;
        self
    }

    /// 设置平台配置
    pub fn with_platform(mut self, platform: PlatformConfig) -> Self {
        self.platform = platform;
        self
    }
}

/// 平台配置
#[derive(Debug, Clone)]
pub struct PlatformConfig {
    pub auto_detect: bool,
    pub force_platform: Option<String>,
    pub capabilities: PlatformCapabilities,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            auto_detect: true,
            force_platform: None,
            capabilities: PlatformCapabilities {
                supports_gestures: true,
                supports_haptic_feedback: false,
                supports_animations: true,
                supports_touch: true,
                max_z_index: 10000,
                animation_duration: 200,
            },
        }
    }
}

// 导入其他配置类型
use super::responsive_config::ResponsiveConfig;
use super::z_index_config::ZIndexConfig;
