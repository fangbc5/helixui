use dioxus::prelude::*;

/// 文本省略的行数类型
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EllipsisLines {
    Single,
    Multiple(usize),
}

/// 省略文字组件属性
#[derive(Props, Clone, PartialEq)]
pub struct EllipsisProps {
    /// 文本内容
    pub text: String,
    /// 省略行数：Single 表示单行省略，Multiple(n) 表示多行省略
    #[props(default = EllipsisLines::Single)]
    pub lines: EllipsisLines,
    /// 展开时的文本（可选，如果有则显示展开/收起按钮）
    #[props(default)]
    pub expand_text: Option<String>,
    /// 收起时的文本（可选）
    #[props(default)]
    pub collapse_text: Option<String>,
    /// 是否默认展开（仅在提供 expand_text 时有效）
    #[props(default = false)]
    pub default_expanded: bool,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,
    /// 展开/收起状态变化回调
    #[props(default)]
    pub on_expand_change: Option<EventHandler<bool>>,
}

/// 省略文字组件
#[component]
pub fn Ellipsis(props: EllipsisProps) -> Element {
    let mut expanded = use_signal(|| props.default_expanded);

    let (base_class, line_clamp_style) = match props.lines {
        EllipsisLines::Single => ("truncate".to_string(), String::new()),
        EllipsisLines::Multiple(n) => {
            // 多行省略需要使用 -webkit-line-clamp
            let style = format!("display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: {}; overflow: hidden;", n);
            ("block w-full".to_string(), style)
        }
    };

    let user_class = props.class.as_deref().unwrap_or("");
    let final_class = if user_class.is_empty() {
        base_class
    } else {
        format!("{} {}", base_class, user_class)
    };

    // 合并样式
    let final_style = if line_clamp_style.is_empty() {
        props.style.as_deref().unwrap_or("").to_string()
    } else {
        let user_style = props.style.as_deref().unwrap_or("");
        if user_style.is_empty() {
            line_clamp_style
        } else {
            format!("{}; {}", line_clamp_style, user_style)
        }
    };

    let has_expand_button = props.expand_text.is_some();
    let is_expanded = *expanded.peek();

    rsx! {
        span {
            class: "block w-full",
            if has_expand_button && !is_expanded {
                span {
                    class: "flex items-baseline gap-1 flex-wrap",
                    span {
                        class: "{final_class} block flex-1 min-w-0",
                        style: final_style,
                        {props.text.clone()}
                    }
                    button {
                        class: "text-green-600 dark:text-green-400 hover:text-green-700 dark:hover:text-green-300 cursor-pointer text-sm whitespace-nowrap flex-shrink-0",
                        onclick: move |_| {
                            *expanded.write() = true;
                            if let Some(handler) = &props.on_expand_change {
                                handler.call(true);
                            }
                        },
                        {props.expand_text.as_ref().map(|s| s.clone()).unwrap_or_else(|| "展开".to_string())}
                    }
                }
            } else if has_expand_button && is_expanded {
                span {
                    class: "flex items-baseline gap-1 flex-wrap",
                    span {
                        class: "{user_class} block flex-1 min-w-0",
                        style: final_style,
                        {props.text.clone()}
                    }
                    button {
                        class: "text-green-600 dark:text-green-400 hover:text-green-700 dark:hover:text-green-300 cursor-pointer text-sm whitespace-nowrap flex-shrink-0",
                        onclick: move |_| {
                            *expanded.write() = false;
                            if let Some(handler) = &props.on_expand_change {
                                handler.call(false);
                            }
                        },
                        {props.collapse_text.as_ref().map(|s| s.clone()).unwrap_or_else(|| "收起".to_string())}
                    }
                }
            } else {
                span {
                    class: "{final_class} block w-full",
                    style: final_style,
                    {props.text.clone()}
                }
            }
        }
    }
}
