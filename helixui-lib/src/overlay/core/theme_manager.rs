use dioxus::prelude::*;

/// 主题模式
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    Auto,
}

/// 主题配置
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    pub mode: ThemeMode,
    pub primary_color: String,
    pub secondary_color: String,
    pub background_color: String,
    pub text_color: String,
    pub border_color: String,
    pub shadow_color: String,
    pub custom_css: Option<String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Auto,
            primary_color: "#3b82f6".to_string(),
            secondary_color: "#6b7280".to_string(),
            background_color: "#ffffff".to_string(),
            text_color: "#111827".to_string(),
            border_color: "#e5e7eb".to_string(),
            shadow_color: "#000000".to_string(),
            custom_css: None,
        }
    }
}

/// 主题管理器
#[derive(Clone)]
pub struct ThemeManager {
    config: ThemeConfig,
    current_theme: Signal<ThemeMode>,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            config: ThemeConfig::default(),
            current_theme: use_signal(|| ThemeMode::Auto),
        }
    }

    /// 设置主题配置
    pub fn set_config(&mut self, config: ThemeConfig) {
        self.config = config;
    }

    /// 获取主题配置
    pub fn get_config(&self) -> &ThemeConfig {
        &self.config
    }

    /// 设置主题模式
    pub fn set_theme_mode(&mut self, mode: ThemeMode) {
        self.current_theme.set(mode);
    }

    /// 获取当前主题模式
    pub fn get_current_theme(&self) -> ThemeMode {
        self.current_theme.read().clone()
    }

    /// 获取主题类名
    pub fn get_theme_class(&self) -> String {
        match self.get_current_theme() {
            ThemeMode::Light => "theme-light".to_string(),
            ThemeMode::Dark => "theme-dark".to_string(),
            ThemeMode::Auto => "theme-auto".to_string(),
        }
    }

    /// 获取颜色变量
    pub fn get_color_variables(&self) -> String {
        format!(
            "--primary-color: {}; --secondary-color: {}; --background-color: {}; --text-color: {}; --border-color: {}; --shadow-color: {};",
            self.config.primary_color,
            self.config.secondary_color,
            self.config.background_color,
            self.config.text_color,
            self.config.border_color,
            self.config.shadow_color
        )
    }

    /// 获取基础样式类
    pub fn get_base_class(&self) -> String {
        match self.get_current_theme() {
            ThemeMode::Light => "bg-white text-gray-900 border-gray-200".to_string(),
            ThemeMode::Dark => "bg-gray-800 text-gray-100 border-gray-700".to_string(),
            ThemeMode::Auto => "bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 border-gray-200 dark:border-gray-700".to_string(),
        }
    }

    /// 获取消息样式类
    pub fn get_message_class(&self, message_type: &str) -> String {
        let base_class = self.get_base_class();

        match message_type {
            "success" => {
                match self.get_current_theme() {
                    ThemeMode::Light => format!("{} bg-green-50 border-green-200 text-green-800", base_class),
                    ThemeMode::Dark => format!("{} bg-green-900 border-green-700 text-green-200", base_class),
                    ThemeMode::Auto => format!("{} bg-green-50 dark:bg-green-900 border-green-200 dark:border-green-700 text-green-800 dark:text-green-200", base_class),
                }
            },
            "warning" => {
                match self.get_current_theme() {
                    ThemeMode::Light => format!("{} bg-yellow-50 border-yellow-200 text-yellow-800", base_class),
                    ThemeMode::Dark => format!("{} bg-yellow-900 border-yellow-700 text-yellow-200", base_class),
                    ThemeMode::Auto => format!("{} bg-yellow-50 dark:bg-yellow-900 border-yellow-200 dark:border-yellow-700 text-yellow-800 dark:text-yellow-200", base_class),
                }
            },
            "error" => {
                match self.get_current_theme() {
                    ThemeMode::Light => format!("{} bg-red-50 border-red-200 text-red-800", base_class),
                    ThemeMode::Dark => format!("{} bg-red-900 border-red-700 text-red-200", base_class),
                    ThemeMode::Auto => format!("{} bg-red-50 dark:bg-red-900 border-red-200 dark:border-red-700 text-red-800 dark:text-red-200", base_class),
                }
            },
            "info" | "loading" => {
                match self.get_current_theme() {
                    ThemeMode::Light => format!("{} bg-blue-50 border-blue-200 text-blue-800", base_class),
                    ThemeMode::Dark => format!("{} bg-blue-900 border-blue-700 text-blue-200", base_class),
                    ThemeMode::Auto => format!("{} bg-blue-50 dark:bg-blue-900 border-blue-200 dark:border-blue-700 text-blue-800 dark:text-blue-200", base_class),
                }
            },
            _ => base_class,
        }
    }

    /// 获取遮罩层样式类
    pub fn get_mask_class(&self) -> String {
        match self.get_current_theme() {
            ThemeMode::Light => "bg-black bg-opacity-50".to_string(),
            ThemeMode::Dark => "bg-black bg-opacity-60".to_string(),
            ThemeMode::Auto => "bg-black bg-opacity-50 dark:bg-opacity-60".to_string(),
        }
    }

    /// 获取阴影样式类
    pub fn get_shadow_class(&self) -> String {
        match self.get_current_theme() {
            ThemeMode::Light => "shadow-lg".to_string(),
            ThemeMode::Dark => "shadow-2xl".to_string(),
            ThemeMode::Auto => "shadow-lg dark:shadow-2xl".to_string(),
        }
    }

    /// 应用自定义 CSS
    pub fn apply_custom_css(&self) -> Option<String> {
        self.config.custom_css.clone()
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
