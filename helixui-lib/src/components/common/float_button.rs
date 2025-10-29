use super::Icon;
use super::IconSize;
use super::IconType;
use crate::components::Badge;
use crate::components::BadgeType;
use dioxus::prelude::*;

/// 浮动按钮位置
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FloatButtonPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl FloatButtonPosition {
    pub fn to_class(&self) -> &'static str {
        match self {
            FloatButtonPosition::TopLeft => "top-4 left-4",
            FloatButtonPosition::TopRight => "top-4 right-4",
            FloatButtonPosition::BottomLeft => "bottom-4 left-4",
            FloatButtonPosition::BottomRight => "bottom-4 right-4",
        }
    }
}

/// 浮动按钮尺寸
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FloatButtonSize {
    Small,
    Medium,
    Large,
}

impl FloatButtonSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            FloatButtonSize::Small => "w-10 h-10",
            FloatButtonSize::Medium => "w-12 h-12",
            FloatButtonSize::Large => "w-14 h-14",
        }
    }

    pub fn icon_size(&self) -> IconSize {
        match self {
            FloatButtonSize::Small => IconSize::Small,
            FloatButtonSize::Medium => IconSize::Medium,
            FloatButtonSize::Large => IconSize::Large,
        }
    }
}

/// Badge 配置（用于浮动按钮）
#[derive(Clone, PartialEq, Debug)]
pub struct FloatButtonBadge {
    pub value: Option<String>,
    pub dot: bool,
    pub max: Option<u32>,
    pub show_zero: bool,
    pub show: bool,
}

/// 浮动按钮属性
#[derive(Props, Clone, PartialEq)]
pub struct FloatButtonProps {
    /// 图标类型
    #[props(default = IconType::ArrowUp)]
    pub icon: IconType,
    /// 按钮位置
    #[props(default = FloatButtonPosition::BottomRight)]
    pub position: FloatButtonPosition,
    /// 按钮尺寸
    #[props(default = FloatButtonSize::Medium)]
    pub size: FloatButtonSize,
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    /// 点击事件处理
    #[props(default)]
    pub onclick: Option<EventHandler<()>>,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// Badge 配置（可选）
    #[props(default)]
    pub badge: Option<FloatButtonBadge>,
    /// 提示文本（tooltip）
    #[props(default)]
    pub tooltip: Option<String>,
}

/// 浮动按钮组件
#[component]
pub fn FloatButton(props: FloatButtonProps) -> Element {
    // 使用 absolute 定位，相对于最近的 relative 父容器
    // 如果需要相对于视窗，可以通过 class 传入 fixed
    let base_class = "absolute z-50 flex items-center justify-center rounded-full shadow-lg transition-all duration-200 cursor-pointer bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 hover:shadow-xl active:scale-95 group";
    let position_class = props.position.to_class();
    let size_class = props.size.to_class();
    let disabled_class = if props.disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        "hover:bg-gray-50 dark:hover:bg-gray-700"
    };
    let user_class = props.class.as_deref().unwrap_or("");

    // 计算徽章的 offset，让徽章更精确地位于按钮右上角
    // Badge 的 offset 使用 (left, top)，相对于其父容器（Icon 的容器）
    // 按钮尺寸：Small(40px), Medium(48px), Large(56px)
    // Icon 居中，为了让徽章位于按钮右上角边缘，需要从中心向右上角偏移
    let badge_offset = props.badge.as_ref().map(|badge| {
        match (props.size, badge.dot) {
            // Small: 按钮 40px，半径 20px
            // 对于圆点：位置更靠外，视觉上更平衡
            (FloatButtonSize::Small, true) => ("calc(50% + 15px)".to_string(), "-2px".to_string()),
            // 对于数字徽章：需要稍微靠内一点，避免过度溢出
            (FloatButtonSize::Small, false) => ("calc(50% + 16px)".to_string(), "-4px".to_string()),
            // Medium: 按钮 48px，半径 24px（默认尺寸，最常见）
            (FloatButtonSize::Medium, true) => ("calc(50% + 18px)".to_string(), "-2px".to_string()),
            (FloatButtonSize::Medium, false) => {
                ("calc(50% + 19px)".to_string(), "-4px".to_string())
            }
            // Large: 按钮 56px，半径 28px
            (FloatButtonSize::Large, true) => ("calc(50% + 21px)".to_string(), "-2px".to_string()),
            (FloatButtonSize::Large, false) => ("calc(50% + 22px)".to_string(), "-4px".to_string()),
        }
    });

    rsx! {
        button {
            class: "{base_class} {position_class} {size_class} {disabled_class} {user_class}",
            disabled: props.disabled,
            onclick: move |_| {
                if !props.disabled {
                    if let Some(handler) = &props.onclick {
                        handler.call(());
                    }
                }
            },
            if let Some(badge) = &props.badge {
                Badge {
                    value: badge.value.clone(),
                    dot: badge.dot,
                    max: badge.max,
                    show_zero: badge.show_zero,
                    show: badge.show,
                    badge_type: BadgeType::Error,
                    // offset: badge_offset.clone(),
                    Icon {
                        icon: props.icon,
                        size: props.size.icon_size(),
                        class: "text-gray-700 dark:text-gray-200".to_string(),
                    }
                }
            } else {
                Icon {
                    icon: props.icon,
                    size: props.size.icon_size(),
                    class: "text-gray-700 dark:text-gray-200".to_string(),
                }
            }
            if let Some(tooltip_text) = &props.tooltip {
                div {
                    class: "absolute pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity bg-gray-900 dark:bg-gray-700 text-white text-xs py-1 px-2 rounded whitespace-nowrap",
                    style: match props.position {
                        FloatButtonPosition::TopLeft | FloatButtonPosition::TopRight => "bottom: calc(100% + 8px); left: 50%; transform: translateX(-50%);",
                        _ => "top: calc(100% + 8px); left: 50%; transform: translateX(-50%);",
                    },
                    {tooltip_text.clone()}
                }
            }
        }
    }
}
