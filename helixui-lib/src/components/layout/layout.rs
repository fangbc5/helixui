use super::tokens::Breakpoint;
use dioxus::prelude::*;

/// 布局方向
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LayoutDirection {
    Row,    // 水平布局：Header + (Sider + Content) + Footer
    Column, // 垂直布局：Header + Content + Footer
}

impl std::fmt::Display for LayoutDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutDirection::Row => write!(f, "row"),
            LayoutDirection::Column => write!(f, "column"),
        }
    }
}

/// Layout 容器组件属性
#[derive(Props, PartialEq, Clone)]
pub struct LayoutProps {
    /// 布局方向
    #[props(default = LayoutDirection::Column)]
    pub direction: LayoutDirection,

    /// 侧边栏宽度
    #[props(optional)]
    pub sider_width: Option<i32>,

    /// 侧边栏收起时的宽度
    #[props(optional)]
    pub sider_collapsed_width: Option<i32>,

    /// 侧边栏是否收起
    #[props(optional)]
    pub sider_collapsed: Option<bool>,

    /// 侧边栏是否可收起
    #[props(optional)]
    pub sider_collapsible: Option<bool>,

    /// 头部高度
    #[props(optional)]
    pub header_height: Option<i32>,

    /// 头部是否固定
    #[props(optional)]
    pub header_fixed: Option<bool>,

    /// 底部高度
    #[props(optional)]
    pub footer_height: Option<i32>,

    /// 底部是否固定
    #[props(optional)]
    pub footer_fixed: Option<bool>,

    /// 是否启用响应式
    #[props(optional)]
    pub responsive: Option<bool>,

    /// 响应式断点
    #[props(optional)]
    pub breakpoint: Option<Breakpoint>,

    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,

    children: Element,
}

/// Layout 容器组件
#[allow(non_snake_case)]
pub fn Layout(props: LayoutProps) -> Element {
    let direction = props.direction.to_string();
    let class = props.class.unwrap_or_default();
    let style = props.style.unwrap_or_default();

    // 构建样式
    let mut layout_style = format!("display:flex;flex-direction:{};", direction);

    // 添加响应式样式
    if props.responsive == Some(true) {
        layout_style.push_str("width:100%;height:100vh;");
    }

    // 添加自定义样式
    if !style.is_empty() {
        layout_style.push_str(&style);
    }

    rsx! {
        div {
            class: format!("hx-layout {}", class),
            style: layout_style,
            {props.children}
        }
    }
}

/// Header 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct HeaderProps {
    /// 高度
    #[props(optional)]
    pub height: Option<i32>,

    /// 是否固定定位
    #[props(optional)]
    pub fixed: Option<bool>,

    /// z-index
    #[props(optional)]
    pub z_index: Option<i32>,

    /// 主题
    #[props(optional)]
    pub theme: Option<HeaderTheme>,

    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,

    children: Element,
}

/// Header 主题
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HeaderTheme {
    Light,
    Dark,
}

/// Header 组件
#[allow(non_snake_case)]
pub fn Header(props: HeaderProps) -> Element {
    let height = props.height.unwrap_or(64);
    let z_index = props.z_index.unwrap_or(1000);
    let class = props.class.unwrap_or_default();
    let style = props.style.unwrap_or_default();

    // 构建样式
    let mut header_style = format!("height:{}px;", height);

    if props.fixed == Some(true) {
        header_style.push_str(&format!(
            "position:fixed;top:0;left:0;right:0;z-index:{};",
            z_index
        ));
    }

    // 添加主题样式
    let theme_class = match props.theme {
        Some(HeaderTheme::Dark) => "bg-gray-900 text-white border-gray-700",
        Some(HeaderTheme::Light) => "bg-white text-gray-900 border-gray-200",
        None => "bg-white text-gray-900 border-gray-200", // 默认使用 light 主题
    };

    if !style.is_empty() {
        header_style.push_str(&style);
    }

    rsx! {
        header {
            class: format!("flex items-center px-6 border-b shadow-sm {} {}", theme_class, class),
            style: header_style,
            {props.children}
        }
    }
}

/// Content 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct ContentProps {
    /// 内边距
    #[props(optional)]
    pub padding: Option<i32>,

    /// 背景色
    #[props(optional)]
    pub background: Option<String>,

    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,

    children: Element,
}

/// Content 组件
#[allow(non_snake_case)]
pub fn Content(props: ContentProps) -> Element {
    let padding = props.padding.unwrap_or(24);
    let class = props.class.unwrap_or_default();
    let style = props.style.unwrap_or_default();

    // 构建样式
    let mut content_style = format!("padding:{}px;flex:1;", padding);

    if let Some(bg) = &props.background {
        content_style.push_str(&format!("background-color:{};", bg));
    }

    if !style.is_empty() {
        content_style.push_str(&style);
    }

    rsx! {
        main {
            class: format!("flex-1 bg-gray-50 overflow-auto {}", class),
            style: content_style,
            {props.children}
        }
    }
}

/// Footer 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct FooterProps {
    /// 高度
    #[props(optional)]
    pub height: Option<i32>,

    /// 是否固定定位
    #[props(optional)]
    pub fixed: Option<bool>,

    /// z-index
    #[props(optional)]
    pub z_index: Option<i32>,

    /// 主题
    #[props(optional)]
    pub theme: Option<FooterTheme>,

    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,

    children: Element,
}

/// Footer 主题
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FooterTheme {
    Light,
    Dark,
}

/// Footer 组件
#[allow(non_snake_case)]
pub fn Footer(props: FooterProps) -> Element {
    let height = props.height.unwrap_or(48);
    let z_index = props.z_index.unwrap_or(1000);
    let class = props.class.unwrap_or_default();
    let style = props.style.unwrap_or_default();

    // 构建样式
    let mut footer_style = format!("height:{}px;", height);

    if props.fixed == Some(true) {
        footer_style.push_str(&format!(
            "position:fixed;bottom:0;left:0;right:0;z-index:{};",
            z_index
        ));
    }

    // 添加主题样式
    let theme_class = match props.theme {
        Some(FooterTheme::Dark) => "bg-gray-900 text-white border-gray-700",
        Some(FooterTheme::Light) => "bg-white text-gray-900 border-gray-200",
        None => "bg-white text-gray-900 border-gray-200", // 默认使用 light 主题
    };

    if !style.is_empty() {
        footer_style.push_str(&style);
    }

    rsx! {
        footer {
            class: format!("flex items-center justify-center px-6 border-t shadow-sm {} {}", theme_class, class),
            style: footer_style,
            {props.children}
        }
    }
}
