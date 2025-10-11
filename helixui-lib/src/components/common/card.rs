use dioxus::prelude::*;

/// 卡片尺寸枚举
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CardSize {
    Small,
    Medium,
    Large,
}

impl CardSize {
    pub fn to_class(&self) -> &str {
        match self {
            CardSize::Small => "p-3",
            CardSize::Medium => "p-4",
            CardSize::Large => "p-6",
        }
    }
}

/// 卡片阴影级别
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CardShadow {
    Never,
    Hover,
    Always,
}

impl CardShadow {
    pub fn to_class(&self) -> &str {
        match self {
            CardShadow::Never => "",
            CardShadow::Hover => "hover:shadow-lg",
            CardShadow::Always => "shadow-lg",
        }
    }
}

/// 卡片组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(default)]
    pub title: Option<String>,
    #[props(default)]
    pub subtitle: Option<String>,
    #[props(default = CardSize::Medium)]
    pub size: CardSize,
    #[props(default = true)]
    pub bordered: bool,
    #[props(default = CardShadow::Hover)]
    pub shadow: CardShadow,
    #[props(default = false)]
    pub collapsible: bool,
    #[props(default = true)]
    pub default_expanded: bool,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

/// 卡片组件
#[component]
pub fn Card(props: CardProps) -> Element {
    let mut is_expanded = use_signal(|| props.default_expanded);

    let final_class = format!(
        "bg-white dark:bg-gray-800 rounded-lg transition-all duration-200 {} {} {} {}",
        props.size.to_class(),
        props.shadow.to_class(),
        if props.bordered {
            "border border-gray-200 dark:border-gray-700"
        } else {
            ""
        },
        props.class.as_deref().unwrap_or("")
    );

    let arrow_class = if is_expanded() {
        "w-4 h-4 transition-transform rotate-180"
    } else {
        "w-4 h-4 transition-transform"
    };

    rsx! {
        div {
            class: final_class,

            if props.title.is_some() || props.subtitle.is_some() {
                div {
                    class: "flex items-center justify-between mb-4",
                    div {
                        class: "flex-1",
                        if let Some(ref t) = props.title {
                            h3 {
                                class: "text-lg font-semibold text-gray-900 dark:text-white mb-1",
                                "{t}"
                            }
                        }
                        if let Some(ref s) = props.subtitle {
                            p {
                                class: "text-sm text-gray-500 dark:text-gray-400",
                                "{s}"
                            }
                        }
                    }
                    if props.collapsible {
                        button {
                            class: "p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors",
                            onclick: move |_| {
                                is_expanded.set(!is_expanded());
                            },
                            svg {
                                class: arrow_class,
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path {
                                    d: "M6 9l6 6 6-6"
                                }
                            }
                        }
                    }
                }
            }

            if !props.collapsible || is_expanded() {
                div {
                    class: "text-gray-700 dark:text-gray-300",
                    {props.children}
                }
            }
        }
    }
}

/// 卡片内容组件
#[component]
pub fn CardContent(#[props(default)] class: Option<String>, children: Element) -> Element {
    let final_class = format!(
        "text-gray-700 dark:text-gray-300 {}",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            class: final_class,
            {children}
        }
    }
}

/// 卡片页脚组件
#[component]
pub fn CardFooter(#[props(default)] class: Option<String>, children: Element) -> Element {
    let final_class = format!(
        "mt-4 pt-4 border-t border-gray-200 dark:border-gray-700 {}",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            class: final_class,
            {children}
        }
    }
}

/// 卡片头部组件
#[component]
pub fn CardHeader(
    #[props(default)] title: Option<String>,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] extra: Option<Element>,
    #[props(default)] class: Option<String>,
    #[props(default)] children: Option<Element>,
) -> Element {
    let final_class = format!(
        "flex items-center justify-between mb-4 {}",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            class: final_class,
            div {
                class: "flex-1",
                if let Some(ref t) = title {
                    h3 {
                        class: "text-lg font-semibold text-gray-900 dark:text-white mb-1",
                        "{t}"
                    }
                }
                if let Some(ref s) = subtitle {
                    p {
                        class: "text-sm text-gray-500 dark:text-gray-400",
                        "{s}"
                    }
                }
                if let Some(c) = children {
                    {c}
                }
            }
            if let Some(e) = extra {
                div {
                    class: "ml-4",
                    {e}
                }
            }
        }
    }
}

/// 卡片组组件
#[component]
pub fn CardGroup(
    #[props(default = "gap-4".to_string())] gap: String,
    #[props(default)] class: Option<String>,
    children: Element,
) -> Element {
    let final_class = format!("flex flex-col {} {}", gap, class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: final_class,
            {children}
        }
    }
}

/// 卡片网格组件
#[component]
pub fn CardGrid(
    #[props(default = 3)] columns: u32,
    #[props(default = "gap-4".to_string())] gap: String,
    #[props(default)] class: Option<String>,
    children: Element,
) -> Element {
    let grid_cols = match columns {
        1 => "grid-cols-1",
        2 => "grid-cols-1 md:grid-cols-2",
        3 => "grid-cols-1 md:grid-cols-2 lg:grid-cols-3",
        4 => "grid-cols-1 md:grid-cols-2 lg:grid-cols-4",
        _ => "grid-cols-1 md:grid-cols-2 lg:grid-cols-3",
    };

    let final_class = format!(
        "grid {} {} {}",
        grid_cols,
        gap,
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            class: final_class,
            {children}
        }
    }
}
