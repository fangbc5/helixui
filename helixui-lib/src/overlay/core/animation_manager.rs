use super::platform_adapter::PlatformAdapter;

/// 动画类型
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationType {
    None,
    FadeIn,
    FadeOut,
    SlideIn,
    SlideOut,
    ScaleIn,
    ScaleOut,
    BounceIn,
    BounceOut,
    Custom(String),
}

/// 缓动类型
#[derive(Debug, Clone, PartialEq)]
pub enum EasingType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    Custom(String),
}

/// 动画配置
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationConfig {
    pub duration: u32,
    pub delay: u32,
    pub easing: EasingType,
    pub enter: AnimationType,
    pub exit: AnimationType,
    pub fill_mode: FillMode,
    pub iteration_count: u32,
}

/// 填充模式
#[derive(Debug, Clone, PartialEq)]
pub enum FillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration: 200,
            delay: 0,
            easing: EasingType::EaseOut,
            enter: AnimationType::FadeIn,
            exit: AnimationType::FadeOut,
            fill_mode: FillMode::Both,
            iteration_count: 1,
        }
    }
}

/// 动画管理器
#[derive(Clone)]
pub struct AnimationManager {
    config: AnimationConfig,
    platform_adapter: PlatformAdapter,
}

impl AnimationManager {
    pub fn new(platform_adapter: PlatformAdapter) -> Self {
        Self {
            config: AnimationConfig::default(),
            platform_adapter,
        }
    }

    /// 设置动画配置
    pub fn set_config(&mut self, config: AnimationConfig) {
        self.config = config;
    }

    /// 获取动画配置
    pub fn get_config(&self) -> &AnimationConfig {
        &self.config
    }

    /// 生成动画类名
    pub fn get_animation_class(&self, animation_type: &AnimationType, is_entering: bool) -> String {
        if !self.platform_adapter.supports_animations() {
            return String::new();
        }

        let duration = self.config.duration;
        let easing = self.get_easing_class(&self.config.easing);

        match animation_type {
            AnimationType::None => String::new(),
            AnimationType::FadeIn => {
                if is_entering {
                    format!("animate-fade-in duration-{} {}", duration, easing)
                } else {
                    format!("animate-fade-out duration-{} {}", duration, easing)
                }
            }
            AnimationType::FadeOut => {
                if is_entering {
                    format!("animate-fade-in duration-{} {}", duration, easing)
                } else {
                    format!("animate-fade-out duration-{} {}", duration, easing)
                }
            }
            AnimationType::SlideIn => {
                if is_entering {
                    format!("animate-slide-in duration-{} {}", duration, easing)
                } else {
                    format!("animate-slide-out duration-{} {}", duration, easing)
                }
            }
            AnimationType::SlideOut => {
                if is_entering {
                    format!("animate-slide-in duration-{} {}", duration, easing)
                } else {
                    format!("animate-slide-out duration-{} {}", duration, easing)
                }
            }
            AnimationType::ScaleIn => {
                if is_entering {
                    format!("animate-scale-in duration-{} {}", duration, easing)
                } else {
                    format!("animate-scale-out duration-{} {}", duration, easing)
                }
            }
            AnimationType::ScaleOut => {
                if is_entering {
                    format!("animate-scale-in duration-{} {}", duration, easing)
                } else {
                    format!("animate-scale-out duration-{} {}", duration, easing)
                }
            }
            AnimationType::BounceIn => {
                if is_entering {
                    format!("animate-bounce-in duration-{} {}", duration, easing)
                } else {
                    format!("animate-bounce-out duration-{} {}", duration, easing)
                }
            }
            AnimationType::BounceOut => {
                if is_entering {
                    format!("animate-bounce-in duration-{} {}", duration, easing)
                } else {
                    format!("animate-bounce-out duration-{} {}", duration, easing)
                }
            }
            AnimationType::Custom(name) => {
                format!("animate-{} duration-{} {}", name, duration, easing)
            }
        }
    }

    /// 获取缓动类名
    fn get_easing_class(&self, easing: &EasingType) -> String {
        match easing {
            EasingType::Linear => "ease-linear",
            EasingType::EaseIn => "ease-in",
            EasingType::EaseOut => "ease-out",
            EasingType::EaseInOut => "ease-in-out",
            EasingType::EaseInQuad => "ease-in-quad",
            EasingType::EaseOutQuad => "ease-out-quad",
            EasingType::EaseInOutQuad => "ease-in-out-quad",
            EasingType::EaseInCubic => "ease-in-cubic",
            EasingType::EaseOutCubic => "ease-out-cubic",
            EasingType::EaseInOutCubic => "ease-in-out-cubic",
            EasingType::Custom(name) => name,
        }
        .to_string()
    }

    /// 创建进入动画
    pub fn create_enter_animation(&self) -> String {
        self.get_animation_class(&self.config.enter, true)
    }

    /// 创建退出动画
    pub fn create_exit_animation(&self) -> String {
        self.get_animation_class(&self.config.exit, false)
    }

    /// 检查是否支持动画
    pub fn supports_animations(&self) -> bool {
        self.platform_adapter.supports_animations()
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new(PlatformAdapter::new())
    }
}
