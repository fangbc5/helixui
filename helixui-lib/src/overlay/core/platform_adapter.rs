
/// 平台类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Web,
    Desktop,
    Mobile,
    Tablet,
    Unknown,
}

/// 平台能力配置
#[derive(Debug, Clone)]
pub struct PlatformCapabilities {
    pub supports_gestures: bool,
    pub supports_haptic_feedback: bool,
    pub supports_animations: bool,
    pub supports_touch: bool,
    pub max_z_index: u32,
    pub animation_duration: u32,
}

/// 平台适配器
#[derive(Clone)]
pub struct PlatformAdapter {
    platform: Platform,
    capabilities: PlatformCapabilities,
}

impl PlatformAdapter {
    pub fn new() -> Self {
        let platform = Self::detect_platform();
        let capabilities = Self::get_platform_capabilities(&platform);

        Self {
            platform,
            capabilities,
        }
    }

    /// 检测平台类型
    fn detect_platform() -> Platform {
        if cfg!(target_arch = "wasm32") {
            // 在 Web 环境中进一步检测
            Self::detect_web_platform()
        } else {
            Platform::Desktop
        }
    }

    /// 检测 Web 平台的具体类型
    fn detect_web_platform() -> Platform {
        // 在实际应用中，这里应该通过 JavaScript 检测屏幕尺寸
        // 这里使用简化的实现
        Platform::Web
    }

    /// 获取平台能力配置
    fn get_platform_capabilities(platform: &Platform) -> PlatformCapabilities {
        match platform {
            Platform::Web => PlatformCapabilities {
                supports_gestures: true,
                supports_haptic_feedback: false,
                supports_animations: true,
                supports_touch: true,
                max_z_index: 2147483647, // 2^31 - 1
                animation_duration: 200,
            },
            Platform::Desktop => PlatformCapabilities {
                supports_gestures: false,
                supports_haptic_feedback: false,
                supports_animations: true,
                supports_touch: false,
                max_z_index: 10000,
                animation_duration: 150,
            },
            Platform::Mobile => PlatformCapabilities {
                supports_gestures: true,
                supports_haptic_feedback: true,
                supports_animations: true,
                supports_touch: true,
                max_z_index: 2000,
                animation_duration: 250,
            },
            Platform::Tablet => PlatformCapabilities {
                supports_gestures: true,
                supports_haptic_feedback: true,
                supports_animations: true,
                supports_touch: true,
                max_z_index: 5000,
                animation_duration: 200,
            },
            Platform::Unknown => PlatformCapabilities {
                supports_gestures: false,
                supports_haptic_feedback: false,
                supports_animations: false,
                supports_touch: false,
                max_z_index: 1000,
                animation_duration: 200,
            },
        }
    }

    /// 获取平台类型
    pub fn get_platform(&self) -> &Platform {
        &self.platform
    }

    /// 获取平台能力
    pub fn get_capabilities(&self) -> &PlatformCapabilities {
        &self.capabilities
    }

    /// 获取基础 z-index
    pub fn get_base_z_index(&self) -> u32 {
        match self.platform {
            Platform::Web => 1000,
            Platform::Desktop => 10000,
            Platform::Mobile => 2000,
            Platform::Tablet => 5000,
            Platform::Unknown => 1000,
        }
    }

    /// 获取动画持续时间
    pub fn get_animation_duration(&self) -> u32 {
        self.capabilities.animation_duration
    }

    /// 获取遮罩层样式类
    pub fn get_mask_class(&self) -> String {
        match self.platform {
            Platform::Web => "bg-black bg-opacity-50".to_string(),
            Platform::Desktop => "bg-black bg-opacity-30".to_string(),
            Platform::Mobile => "bg-black bg-opacity-60".to_string(),
            Platform::Tablet => "bg-black bg-opacity-50".to_string(),
            Platform::Unknown => "bg-black bg-opacity-50".to_string(),
        }
    }

    /// 是否支持手势
    pub fn supports_gestures(&self) -> bool {
        self.capabilities.supports_gestures
    }

    /// 是否支持触觉反馈
    pub fn supports_haptic_feedback(&self) -> bool {
        self.capabilities.supports_haptic_feedback
    }

    /// 是否支持动画
    pub fn supports_animations(&self) -> bool {
        self.capabilities.supports_animations
    }

    /// 是否支持触摸
    pub fn supports_touch(&self) -> bool {
        self.capabilities.supports_touch
    }

    /// 获取最大 z-index
    pub fn get_max_z_index(&self) -> u32 {
        self.capabilities.max_z_index
    }
}

impl Default for PlatformAdapter {
    fn default() -> Self {
        Self::new()
    }
}
