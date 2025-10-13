/// 响应式配置
#[derive(Debug, Clone)]
pub struct ResponsiveConfig {
    pub breakpoints: BreakpointConfig,
    pub layouts: LayoutConfig,
}

impl Default for ResponsiveConfig {
    fn default() -> Self {
        Self {
            breakpoints: BreakpointConfig::default(),
            layouts: LayoutConfig::default(),
        }
    }
}

/// 断点配置
#[derive(Debug, Clone)]
pub struct BreakpointConfig {
    pub mobile: u32,  // < 768px
    pub tablet: u32,  // 768px - 1024px
    pub desktop: u32, // > 1024px
}

impl Default for BreakpointConfig {
    fn default() -> Self {
        Self {
            mobile: 768,
            tablet: 1024,
            desktop: 1024,
        }
    }
}

/// 布局配置
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub mobile: LayoutStyle,
    pub tablet: LayoutStyle,
    pub desktop: LayoutStyle,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            mobile: LayoutStyle::default(),
            tablet: LayoutStyle::default(),
            desktop: LayoutStyle::default(),
        }
    }
}

/// 布局样式
#[derive(Debug, Clone)]
pub struct LayoutStyle {
    pub width: String,
    pub height: String,
    pub padding: String,
    pub margin: String,
    pub border_radius: String,
    pub shadow: String,
}

impl Default for LayoutStyle {
    fn default() -> Self {
        Self {
            width: "auto".to_string(),
            height: "auto".to_string(),
            padding: "1rem".to_string(),
            margin: "0.5rem".to_string(),
            border_radius: "0.5rem".to_string(),
            shadow: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)"
                .to_string(),
        }
    }
}

impl ResponsiveConfig {
    /// 创建新的响应式配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置断点配置
    pub fn with_breakpoints(mut self, breakpoints: BreakpointConfig) -> Self {
        self.breakpoints = breakpoints;
        self
    }

    /// 设置布局配置
    pub fn with_layouts(mut self, layouts: LayoutConfig) -> Self {
        self.layouts = layouts;
        self
    }

    /// 获取指定断点的布局样式
    pub fn get_layout_for_width(&self, width: u32) -> &LayoutStyle {
        if width < self.breakpoints.mobile {
            &self.layouts.mobile
        } else if width < self.breakpoints.tablet {
            &self.layouts.tablet
        } else {
            &self.layouts.desktop
        }
    }

    /// 获取断点类型
    pub fn get_breakpoint_type(&self, width: u32) -> BreakpointType {
        if width < self.breakpoints.mobile {
            BreakpointType::Mobile
        } else if width < self.breakpoints.tablet {
            BreakpointType::Tablet
        } else {
            BreakpointType::Desktop
        }
    }
}

/// 断点类型
#[derive(Debug, Clone, PartialEq)]
pub enum BreakpointType {
    Mobile,
    Tablet,
    Desktop,
}

impl BreakpointType {
    /// 是否为移动端
    pub fn is_mobile(&self) -> bool {
        matches!(self, BreakpointType::Mobile)
    }

    /// 是否为平板
    pub fn is_tablet(&self) -> bool {
        matches!(self, BreakpointType::Tablet)
    }

    /// 是否为桌面端
    pub fn is_desktop(&self) -> bool {
        matches!(self, BreakpointType::Desktop)
    }
}
