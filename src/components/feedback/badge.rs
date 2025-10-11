use dioxus::prelude::*;

/// Badge 类型
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BadgeType {
    Default,
    Success,
    Error,
    Warning,
    Info,
}

impl BadgeType {
    /// 获取对应的颜色类
    pub fn color_class(&self) -> &'static str {
        match self {
            BadgeType::Default => "bg-gray-500",
            BadgeType::Success => "bg-green-500",
            BadgeType::Error => "bg-red-500",
            BadgeType::Warning => "bg-orange-500",
            BadgeType::Info => "bg-blue-500",
        }
    }

    /// 获取对应的文字颜色类
    pub fn text_color_class(&self) -> &'static str {
        match self {
            BadgeType::Default => "text-white",
            BadgeType::Success => "text-white",
            BadgeType::Error => "text-white",
            BadgeType::Warning => "text-white",
            BadgeType::Info => "text-white",
        }
    }
}

/// Badge 组件
#[component]
pub fn Badge(
    /// 标记数量
    #[props(default)]
    value: Option<String>,
    /// 标记显示类型
    #[props(default = BadgeType::Default)]
    badge_type: BadgeType,
    /// 标记是否显示为点
    #[props(default = false)]
    dot: bool,
    /// 标记最大数来处理溢出情况
    #[props(default)]
    max: Option<u32>,
    /// 标记为0时是否显示
    #[props(default = false)]
    show_zero: bool,
    /// 标记受控显示
    #[props(default = true)]
    show: bool,
    /// 标记显示进度
    #[props(default = false)]
    processing: bool,
    /// 标记的颜色
    #[props(default)]
    color: Option<String>,
    /// 距默认位置左侧、上方的偏移量
    #[props(default)]
    offset: Option<(String, String)>,
    /// 自定义类名
    #[props(default)]
    class: Option<String>,
    /// 子元素
    children: Element,
) -> Element {
    // 直接计算显示的值，不使用 use_memo
    let display_value = if !show {
        None
    } else if let Some(val) = value.as_ref() {
        // 尝试解析为数字
        if let Ok(num) = val.parse::<u32>() {
            if num == 0 && !show_zero {
                None
            } else if let Some(max_val) = max {
                if num > max_val {
                    Some(format!("{}+", max_val))
                } else {
                    Some(val.clone())
                }
            } else {
                Some(val.clone())
            }
        } else {
            // 非数字值直接显示
            Some(val.clone())
        }
    } else {
        None
    };

    // 确定是否显示徽章
    let should_show = show && (display_value.is_some() || dot);

    let text_class = if color.is_some() {
        "text-white" // 自定义颜色时使用白色文字
    } else {
        badge_type.text_color_class()
    };

    // 获取颜色样式
    let color_style = use_memo(move || {
        if let Some(custom_color) = color.as_ref() {
            format!("background-color: {};", custom_color)
        } else {
            match badge_type {
                BadgeType::Default => "background-color: #6b7280;".to_string(),
                BadgeType::Success => "background-color: #10b981;".to_string(),
                BadgeType::Error => "background-color: #ef4444;".to_string(),
                BadgeType::Warning => "background-color: #f59e0b;".to_string(),
                BadgeType::Info => "background-color: #3b82f6;".to_string(),
            }
        }
    });

    // 获取偏移样式
    let offset_style = use_memo(move || {
        if let Some((left, top)) = &offset {
            format!("left: {}; top: {};", left, top)
        } else {
            // dot 类型使用更小的偏移距离
            if dot {
                "right: -4px; top: -4px;".to_string()
            } else {
                "right: -8px; top: -8px;".to_string()
            }
        }
    });

    // 基础样式类
    let base_class = if dot {
        "absolute w-2 h-2 rounded-full border-2 border-white"
    } else {
        "absolute min-w-[18px] h-[18px] px-1 rounded-full text-xs font-medium flex items-center justify-center border-2 border-white"
    };

    let final_class = if let Some(custom_class) = class {
        format!("{} {} {}", base_class, text_class, custom_class)
    } else {
        format!("{} {}", base_class, text_class)
    };

    rsx! {
        div {
            class: "relative inline-block",

            // 子元素
            {children}

            // 徽章
            if should_show {
                div {
                    class: final_class,
                    style: format!("{} {}", color_style.read(), offset_style.read()),

                    // 处理中动画
                    if processing && !dot {
                        div {
                            class: "absolute inset-0 rounded-full animate-spin",
                            style: "background: conic-gradient(from 0deg, transparent, rgba(255,255,255,0.8), rgba(255,255,255,0.4), transparent);",
                        }
                        div {
                            class: "absolute inset-0 rounded-full animate-pulse",
                            style: "background: radial-gradient(circle, rgba(255,255,255,0.3) 0%, transparent 70%);",
                        }
                    }

                    // 显示内容
                    if !dot {
                        if let Some(val) = &display_value {
                            "{val}"
                        }
                    }
                }
            }
        }
    }
}

/// 便捷的徽章显示函数
pub fn create_badge(
    value: Option<String>,
    badge_type: BadgeType,
    dot: bool,
) -> impl Fn(Element) -> Element {
    move |children| {
        rsx! {
            Badge {
                value: value.clone(),
                badge_type,
                dot,
                children
            }
        }
    }
}

/// 带数字的徽章
pub fn create_number_badge(
    value: u32,
    badge_type: BadgeType,
    max: Option<u32>,
    show_zero: bool,
) -> impl Fn(Element) -> Element {
    move |children| {
        rsx! {
            Badge {
                value: Some(value.to_string()),
                badge_type,
                max,
                show_zero,
                children
            }
        }
    }
}

/// 点状徽章
pub fn create_dot_badge(badge_type: BadgeType, processing: bool) -> impl Fn(Element) -> Element {
    move |children| {
        rsx! {
            Badge {
                badge_type,
                dot: true,
                processing,
                children
            }
        }
    }
}

/// 自定义内容徽章
pub fn create_custom_badge(
    content: String,
    badge_type: BadgeType,
    color: Option<String>,
) -> impl Fn(Element) -> Element {
    move |children| {
        rsx! {
            Badge {
                value: Some(content.clone()),
                badge_type,
                color: color.clone(),
                children
            }
        }
    }
}

/// 受控徽章
pub fn create_controlled_badge(
    value: u32,
    show: bool,
    badge_type: BadgeType,
) -> impl Fn(Element) -> Element {
    move |children| {
        rsx! {
            Badge {
                value: Some(value.to_string()),
                show,
                badge_type,
                children
            }
        }
    }
}
