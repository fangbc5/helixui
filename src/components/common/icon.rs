use dioxus::prelude::*;

/// 图标类型枚举
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IconType {
    /// 检查/对勾图标
    Check,
    /// 关闭/X 图标
    Close,
    /// 向下箭头
    ChevronDown,
    /// 向上箭头
    ChevronUp,
    /// 向左箭头
    ChevronLeft,
    /// 向右箭头
    ChevronRight,
    /// 信息图标
    Info,
    /// 警告图标
    Warning,
    /// 错误图标
    Error,
    /// 成功图标
    Success,
    /// 搜索图标
    Search,
    /// 设置图标
    Settings,
    /// 用户图标
    User,
    /// 主页图标
    Home,
    /// 菜单图标
    Menu,
    /// 月亮（深色模式）
    Moon,
    /// 太阳（浅色模式）
    Sun,
    /// GitHub 图标
    GitHub,
    /// 加载中
    Loading,
}

impl IconType {
    /// 获取图标的 SVG 路径数据
    pub fn path(&self) -> &'static str {
        match self {
            IconType::Check => "M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z",
            IconType::Close => "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z",
            IconType::ChevronDown => "M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z",
            IconType::ChevronUp => "M7.41 15.41L12 10.83l4.59 4.58L18 14l-6-6-6 6z",
            IconType::ChevronLeft => "M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z",
            IconType::ChevronRight => "M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z",
            IconType::Info => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z",
            IconType::Warning => "M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z",
            IconType::Error => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z",
            IconType::Success => "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z",
            IconType::Search => "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z",
            IconType::Settings => "M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94L14.4 2.81c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z",
            IconType::User => "M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z",
            IconType::Home => "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z",
            IconType::Menu => "M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z",
            IconType::Moon => "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z",
            IconType::Sun => "M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1zM5.99 4.58c-.39-.39-1.03-.39-1.41 0-.39.39-.39 1.03 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41L5.99 4.58zm12.37 12.37c-.39-.39-1.03-.39-1.41 0-.39.39-.39 1.03 0 1.41l1.06 1.06c.39.39 1.03.39 1.41 0 .39-.39.39-1.03 0-1.41l-1.06-1.06zm1.06-10.96c.39-.39.39-1.03 0-1.41-.39-.39-1.03-.39-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06zM7.05 18.36c.39-.39.39-1.03 0-1.41-.39-.39-1.03-.39-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06z",
            IconType::GitHub => "M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.17 6.839 9.49.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.463-1.11-1.463-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.578 9.578 0 0112 6.836c.85.004 1.705.114 2.504.336 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.138 20.167 22 16.418 22 12c0-5.523-4.477-10-10-10z",
            IconType::Loading => "M12 6v3l4-4-4-4v3c-4.42 0-8 3.58-8 8 0 1.57.46 3.03 1.24 4.26L6.7 14.8c-.45-.83-.7-1.79-.7-2.8 0-3.31 2.69-6 6-6zm6.76 1.74L17.3 9.2c.44.84.7 1.79.7 2.8 0 3.31-2.69 6-6 6v-3l-4 4 4 4v-3c4.42 0 8-3.58 8-8 0-1.57-.46-3.03-1.24-4.26z",
        }
    }

    /// 获取图标的视图框（viewBox）
    pub fn viewbox(&self) -> &'static str {
        "0 0 24 24"
    }
}

/// 图标尺寸
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IconSize {
    Small,  // 16px
    Medium, // 20px
    Large,  // 24px
    XLarge, // 32px
}

impl IconSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            IconSize::Small => "w-4 h-4",
            IconSize::Medium => "w-5 h-5",
            IconSize::Large => "w-6 h-6",
            IconSize::XLarge => "w-8 h-8",
        }
    }
    #[allow(dead_code)]
    pub fn to_px(&self) -> &'static str {
        match self {
            IconSize::Small => "16",
            IconSize::Medium => "20",
            IconSize::Large => "24",
            IconSize::XLarge => "32",
        }
    }
}

/// Icon 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    /// 图标类型
    pub icon: IconType,
    /// 图标尺寸
    #[props(default = IconSize::Medium)]
    pub size: IconSize,
    /// 自定义颜色类
    #[props(default = "currentColor".to_string())]
    pub color: String,
    /// 自定义类名
    #[props(default = String::new())]
    pub class: String,
    /// 是否旋转动画（用于 Loading 图标）
    #[props(default = false)]
    pub spin: bool,
}

/// Icon 组件
#[component]
pub fn Icon(props: IconProps) -> Element {
    let size_class = props.size.to_class();
    let spin_class = if props.spin { "animate-spin" } else { "" };
    let custom_class = &props.class;

    // 如果没有自定义类，默认使用深色模式适配的颜色
    let default_color_class = if custom_class.is_empty() {
        "text-gray-700 dark:text-gray-300"
    } else {
        ""
    };

    rsx! {
        svg {
            class: "inline-block {size_class} {default_color_class} {spin_class} {custom_class} transition-colors",
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "{props.icon.viewbox()}",
            fill: "currentColor",
            path {
                d: "{props.icon.path()}"
            }
        }
    }
}
