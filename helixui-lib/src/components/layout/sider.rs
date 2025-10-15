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

/// Sider 收起模式
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SiderCollapseMode {
    /// 改变宽度（默认）
    Width,
    /// 使用 transform 变换
    Transform,
}

/// Sider 触发器显示方式
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SiderShowTrigger {
    /// 不显示触发器
    None,
    /// 显示为条状触发器
    Bar,
    /// 显示为箭头触发器
    Arrow,
}

/// Sider 触发器位置
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SiderTriggerPlacement {
    /// 顶部
    Top,
    /// 底部
    Bottom,
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

    /// 是否收起（受控）
    #[props(optional)]
    pub collapsed: Option<bool>,

    /// 默认收起状态（非受控）
    #[props(optional)]
    pub default_collapsed: Option<bool>,

    /// 是否可收起
    #[props(optional)]
    pub collapsible: Option<bool>,

    /// 收起模式
    #[props(default = SiderCollapseMode::Width)]
    pub collapse_mode: SiderCollapseMode,

    /// 触发器显示方式
    #[props(default = SiderShowTrigger::None)]
    pub show_trigger: SiderShowTrigger,

    /// 触发器位置
    #[props(default = SiderTriggerPlacement::Bottom)]
    pub trigger_placement: SiderTriggerPlacement,

    /// 收起状态变化回调
    #[props(optional)]
    pub on_update_collapsed: Option<EventHandler<bool>>,

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
    let position = props.position.unwrap_or(SiderPosition::Left);
    let class = props.class.unwrap_or_default();
    let style = props.style.unwrap_or_default();

    // 受控/非受控状态管理
    let collapsed = props
        .collapsed
        .unwrap_or(props.default_collapsed.unwrap_or(false));

    // 计算实际宽度和样式
    let (actual_width, collapse_style) = match props.collapse_mode {
        SiderCollapseMode::Width => {
            let width = if collapsed { collapsed_width } else { width };
            (width, String::new())
        }
        SiderCollapseMode::Transform => {
            let width = width;
            let transform = if collapsed {
                format!("transform: translateX({}px);", -(width - collapsed_width))
            } else {
                "transform: translateX(0);".to_string()
            };
            (width, transform)
        }
    };

    // 构建样式
    let mut sider_style = format!("width:{}px;height:100%;{}", actual_width, collapse_style);

    // 添加位置样式
    match position {
        SiderPosition::Left => {
            sider_style.push_str("order:-1;");
        }
        SiderPosition::Right => {
            sider_style.push_str("order:1;");
        }
    }

    // 构建类名
    let mut class_list = vec!["hx-sider".to_string()];

    // 添加主题样式
    let theme_class = match props.theme {
        Some(SiderTheme::Dark) => "bg-gray-900 text-white",
        Some(SiderTheme::Light) => "bg-white text-gray-900 border-r border-gray-200",
        None => "bg-gray-900 text-white", // 默认使用 dark 主题
    };
    class_list.push(theme_class.to_string());

    if props.responsive == Some(true) {
        class_list.push("hx-sider-responsive".to_string());
    }

    if collapsed {
        class_list.push("hx-sider-collapsed".to_string());
    }

    match props.collapse_mode {
        SiderCollapseMode::Width => class_list.push("hx-sider-collapse-width".to_string()),
        SiderCollapseMode::Transform => class_list.push("hx-sider-collapse-transform".to_string()),
    }

    if !class.is_empty() {
        class_list.push(class);
    }

    let final_class = class_list.join(" ");

    if !style.is_empty() {
        sider_style.push_str(&style);
    }

    rsx! {
        aside {
            class: final_class,
            style: sider_style,
            "data-position": position.to_string(),
            "data-collapsed": collapsed.to_string(),
            "data-collapse-mode": format!("{:?}", props.collapse_mode),
            if props.trigger_placement == SiderTriggerPlacement::Top && props.show_trigger != SiderShowTrigger::None && props.collapsible == Some(true) {
                div {
                    class: format!("hx-sider-trigger hx-sider-trigger-top flex items-center justify-center cursor-pointer"),
                    onclick: move |_| {
                        if let Some(callback) = &props.on_update_collapsed {
                            callback.call(!collapsed);
                        }
                    },
                    if props.show_trigger == SiderShowTrigger::Arrow {
                        if collapsed { "→" } else { "←" }
                    } else {
                        "⋮"
                    }
                }
            }
            {props.children}
            if props.trigger_placement == SiderTriggerPlacement::Bottom && props.show_trigger != SiderShowTrigger::None && props.collapsible == Some(true) {
                div {
                    class: format!("hx-sider-trigger hx-sider-trigger-bottom flex items-center justify-center cursor-pointer"),
                    onclick: move |_| {
                        if let Some(callback) = &props.on_update_collapsed {
                            callback.call(!collapsed);
                        }
                    },
                    if props.show_trigger == SiderShowTrigger::Arrow {
                        if collapsed { "→" } else { "←" }
                    } else {
                        "⋮"
                    }
                }
            }
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
