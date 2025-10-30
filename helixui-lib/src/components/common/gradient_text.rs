use dioxus::prelude::*;

/// 渐变文字方向
#[derive(Clone, PartialEq)]
pub enum GradientDirection {
    ToRight,
    ToLeft,
    ToBottom,
    ToTop,
    Custom(String), // 例如 "45deg"
}

impl Default for GradientDirection {
    fn default() -> Self {
        GradientDirection::ToRight
    }
}

impl GradientDirection {
    fn to_css(&self) -> String {
        match self {
            GradientDirection::ToRight => "to right".to_string(),
            GradientDirection::ToLeft => "to left".to_string(),
            GradientDirection::ToBottom => "to bottom".to_string(),
            GradientDirection::ToTop => "to top".to_string(),
            GradientDirection::Custom(s) => s.clone(),
        }
    }
}

/// 渐变文字组件属性
#[derive(Props, Clone, PartialEq)]
pub struct GradientTextProps {
    /// 渐变起始颜色（CSS 颜色值）
    #[props(default = String::from("#10b981"))] // emerald-500
    pub from: String,
    /// 渐变结束颜色（CSS 颜色值）
    #[props(default = String::from("#3b82f6"))] // blue-500
    pub to: String,
    /// 渐变方向
    #[props(default = GradientDirection::ToRight)]
    pub direction: GradientDirection,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 子节点（展示的文字）
    pub children: Element,
}

/// 渐变文字组件
#[component]
pub fn GradientText(props: GradientTextProps) -> Element {
    let base_class = "bg-clip-text text-transparent inline-block";
    let user_class = props.class.as_deref().unwrap_or("");
    let final_class = if user_class.is_empty() {
        base_class.to_string()
    } else {
        format!("{} {}", base_class, user_class)
    };

    // 组合背景渐变样式
    let gradient_style = format!(
        "background-image: linear-gradient({}, {}, {});",
        props.direction.to_css(),
        props.from,
        props.to
    );

    let final_style = if let Some(user_style) = props.style.as_deref() {
        if user_style.is_empty() {
            gradient_style
        } else {
            format!("{} {}", gradient_style, user_style)
        }
    } else {
        gradient_style
    };

    rsx! {
        span {
            class: "{final_class}",
            style: final_style,
            {props.children}
        }
    }
}
