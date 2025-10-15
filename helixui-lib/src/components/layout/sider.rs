use super::tokens::Breakpoint;
use dioxus::prelude::*;

/// Sider 位置
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SiderPosition {
    Left,
    Right,
}

impl std::fmt::Display for SiderPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SiderPosition::Left => write!(f, "left"),
            SiderPosition::Right => write!(f, "right"),
        }
    }
}

/// Sider 主题
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SiderTheme {
    Light,
    Dark,
}

/// Sider 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SiderProps {
    /// 宽度
    #[props(optional)]
    pub width: Option<i32>,

    /// 收起时的宽度
    #[props(optional)]
    pub collapsed_width: Option<i32>,

    /// 是否收起
    #[props(optional)]
    pub collapsed: Option<bool>,

    /// 是否可收起
    #[props(optional)]
    pub collapsible: Option<bool>,

    /// 位置
    #[props(optional)]
    pub position: Option<SiderPosition>,

    /// 主题
    #[props(optional)]
    pub theme: Option<SiderTheme>,

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

/// Sider 组件
#[allow(non_snake_case)]
pub fn Sider(props: SiderProps) -> Element {
    let width = props.width.unwrap_or(200);
    let collapsed_width = props.collapsed_width.unwrap_or(80);
    let collapsed = props.collapsed.unwrap_or(false);
    let position = props.position.unwrap_or(SiderPosition::Left);
    let class = props.class.unwrap_or_default();
    let style = props.style.unwrap_or_default();

    // 计算实际宽度
    let actual_width = if collapsed { collapsed_width } else { width };

    // 构建样式
    let mut sider_style = format!("width:{}px;height:100%;", actual_width);

    // 添加位置样式
    match position {
        SiderPosition::Left => {
            sider_style.push_str("order:-1;");
        }
        SiderPosition::Right => {
            sider_style.push_str("order:1;");
        }
    }

    // 添加主题样式
    let theme_class = match props.theme {
        Some(SiderTheme::Dark) => "bg-gray-900 text-white",
        Some(SiderTheme::Light) => "bg-white text-gray-900 border-r border-gray-200",
        None => "bg-gray-900 text-white", // 默认使用 dark 主题
    };

    // 添加响应式样式
    let responsive_class = if props.responsive == Some(true) {
        "hx-sider-responsive"
    } else {
        ""
    };

    // 添加收起状态样式
    let collapsed_class = if collapsed { "hx-sider-collapsed" } else { "" };

    if !style.is_empty() {
        sider_style.push_str(&style);
    }

    rsx! {
        aside {
            class: format!("flex flex-col transition-all duration-200 overflow-hidden {} {} {} {}", theme_class, responsive_class, collapsed_class, class),
            style: sider_style,
            "data-position": position.to_string(),
            "data-collapsed": collapsed.to_string(),
            {props.children}
        }
    }
}

/// Sider 触发器组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SiderTriggerProps {
    /// 是否收起
    #[props(optional)]
    pub collapsed: Option<bool>,

    /// 点击回调
    #[props(optional)]
    pub on_toggle: Option<EventHandler<bool>>,

    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,
}

/// Sider 触发器组件
#[allow(non_snake_case)]
pub fn SiderTrigger(props: SiderTriggerProps) -> Element {
    let collapsed = props.collapsed.unwrap_or(false);
    let class = props.class.unwrap_or_default();
    let style = props.style.unwrap_or_default();

    rsx! {
        div {
            class: format!("flex items-center justify-center h-12 bg-gray-800 hover:bg-gray-700 border-t border-gray-700 text-white text-sm cursor-pointer transition-colors {}", class),
            style: style,
            onclick: move |_| {
                if let Some(callback) = &props.on_toggle {
                    callback.call(!collapsed);
                }
            },
            if collapsed {
                "展开"
            } else {
                "收起"
            }
        }
    }
}
