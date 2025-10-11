use dioxus::prelude::*;

/// Copyright 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct CopyrightProps {
    #[props(default)]
    pub text: Option<String>,
    #[props(default)]
    pub class: Option<String>,
}

/// Copyright 组件
#[component]
pub fn Copyright(props: CopyrightProps) -> Element {
    let text = props
        .text
        .as_deref()
        .unwrap_or("Copyright © 2025 HelixUI. All rights reserved.");
    let _custom_class = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            id: "copyright",
            class: "text-center text-sm text-gray-500 dark:text-gray-400 py-4",
            "{text}"
        }
    }
}
