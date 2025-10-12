use dioxus::prelude::*;

/// 分割线方向
#[derive(Clone, PartialEq)]
pub enum DividerDirection {
    Horizontal,
    Vertical,
}

impl DividerDirection {
    pub fn to_class(&self) -> &str {
        match self {
            DividerDirection::Horizontal => "w-full h-px",
            DividerDirection::Vertical => "h-full w-px",
        }
    }
}

/// 分割线位置
#[derive(Clone, PartialEq)]
pub enum DividerPosition {
    Left,
    Right,
    Top,
    Bottom,
}

impl DividerPosition {
    pub fn to_class(&self) -> &str {
        match self {
            DividerPosition::Left => "border-l",
            DividerPosition::Right => "border-r",
            DividerPosition::Top => "border-t",
            DividerPosition::Bottom => "border-b",
        }
    }
}

/// 分割线组件属性
#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    /// 分割线方向
    #[props(default = DividerDirection::Horizontal)]
    pub direction: DividerDirection,

    /// 分割线位置
    #[props(default)]
    pub position: Option<DividerPosition>,

    /// 是否为虚线
    #[props(default = false)]
    pub dashed: bool,

    /// 分割线标题
    #[props(default)]
    pub title: Option<String>,

    /// 标题位置
    #[props(default = "center".to_string())]
    pub title_placement: String,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
}

/// 分割线组件
#[component]
pub fn Divider(props: DividerProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");
    let custom_style = props.style.as_deref().unwrap_or("");

    // 如果有标题，渲染带标题的分割线
    if let Some(title) = &props.title {
        rsx! {
            div {
                class: "flex items-center w-full",
                style: custom_style,

                // 左侧分割线
                if props.title_placement != "left" {
                    div {
                        class: format!("flex-1 h-px border-t border-gray-200 dark:border-gray-700 {}", custom_class),
                        style: if props.dashed { "border-style: dashed;" } else { "" },
                    }
                }

                // 标题
                div {
                    class: "px-3 text-sm text-gray-500 dark:text-gray-400 whitespace-nowrap",
                    "{title}"
                }

                // 右侧分割线
                if props.title_placement != "right" {
                    div {
                        class: format!("flex-1 h-px border-t border-gray-200 dark:border-gray-700 {}", custom_class),
                        style: if props.dashed { "border-style: dashed;" } else { "" },
                    }
                }
            }
        }
    } else {
        // 普通分割线
        let direction_class = props.direction.to_class();
        let position_class = props.position.as_ref().map(|p| p.to_class()).unwrap_or("");
        let dashed_class = if props.dashed {
            "border-dashed"
        } else {
            "border-solid"
        };

        // 根据方向设置边框
        let border_class = match props.direction {
            DividerDirection::Horizontal => "border-t",
            DividerDirection::Vertical => "border-l",
        };

        let final_class = format!(
            "{} border-gray-200 dark:border-gray-700 {} {} {} {}",
            direction_class, border_class, position_class, dashed_class, custom_class
        );

        rsx! {
            div {
                class: final_class,
                style: custom_style,
            }
        }
    }
}

/// 垂直分割线组件
#[component]
pub fn DividerVertical(
    #[props(default)] dashed: bool,
    #[props(default)] class: Option<String>,
    #[props(default)] style: Option<String>,
) -> Element {
    rsx! {
        Divider {
            direction: DividerDirection::Vertical,
            dashed,
            class,
            style,
        }
    }
}

/// 水平分割线组件
#[component]
pub fn DividerHorizontal(
    #[props(default)] dashed: bool,
    #[props(default)] class: Option<String>,
    #[props(default)] style: Option<String>,
) -> Element {
    rsx! {
        Divider {
            direction: DividerDirection::Horizontal,
            dashed,
            class,
            style,
        }
    }
}
