use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{DemoBox, Input, Table};

/// Input 演示页面
#[component]
pub fn InputPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic-usage".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "input-types".to_string(),
            title: "输入类型".to_string(),
            level: 2,
        },
        TocItem {
            id: "controlled".to_string(),
            title: "受控输入".to_string(),
            level: 2,
        },
        TocItem {
            id: "states".to_string(),
            title: "状态".to_string(),
            level: 2,
        },
        TocItem {
            id: "api".to_string(),
            title: "API".to_string(),
            level: 2,
        },
    ];

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items,
            div {
                DemoBox {
                    title: "基础用法",
                    description: "最简单的用法。",
                    code: r#"Input {{
    placeholder: Some("请输入文本".to_string()),
}}"#,
                    children: rsx! {
                        BasicInputDemo {}
                    }
                }

                DemoBox {
                    title: "输入类型",
                    description: "支持不同类型的输入框。",
                    code: r#"Input {{
    input_type: "text".to_string(),
    placeholder: Some("文本输入框".to_string()),
}}
Input {{
    input_type: "password".to_string(),
    placeholder: Some("密码输入框".to_string()),
}}
Input {{
    input_type: "number".to_string(),
    placeholder: Some("数字输入框".to_string()),
}}"#,
                    children: rsx! {
                        InputTypesDemo {}
                    }
                }

                DemoBox {
                    title: "受控输入",
                    description: "通过状态管理来控制输入框的值。",
                    code: r#"let mut value = use_signal(|| "".to_string());
Input {{
    value: Some(value()),
    on_change: move |new_value| value.set(new_value),
}}"#,
                    children: rsx! {
                        ControlledInputDemo {}
                    }
                }

                DemoBox {
                    title: "状态",
                    description: "输入框的禁用、只读等状态。",
                    code: r#"Input {{
    value: Some("已禁用的输入框".to_string()),
    disabled: true,
}}
Input {{
    value: Some("只读输入框".to_string()),
    readonly: true,
}}"#,
                    children: rsx! {
                        InputStatesDemo {}
                    }
                }

                div {
                    class: "mt-8",
                    h2 {
                        id: "api",
                        class: "text-2xl font-bold text-gray-900 dark:text-white mb-4",
                        "API"
                    }

                    Table {
                        headers: Some(vec![
                            "参数".to_string(),
                            "说明".to_string(),
                            "类型".to_string(),
                            "默认值".to_string(),
                        ]),
                        data: vec![
                            vec!["input_type".to_string(), "输入框类型，如 'text', 'password', 'number' 等".to_string(), "String".to_string(), "'text'".to_string()],
                            vec!["value".to_string(), "输入框的值".to_string(), "Option<String>".to_string(), "None".to_string()],
                            vec!["placeholder".to_string(), "占位符文本".to_string(), "Option<String>".to_string(), "None".to_string()],
                            vec!["disabled".to_string(), "是否禁用".to_string(), "bool".to_string(), "false".to_string()],
                            vec!["readonly".to_string(), "是否只读".to_string(), "bool".to_string(), "false".to_string()],
                            vec!["on_change".to_string(), "值变化时的回调".to_string(), "Option<EventHandler<String>>".to_string(), "None".to_string()],
                            vec!["class".to_string(), "自定义类名".to_string(), "Option<String>".to_string(), "None".to_string()],
                        ],
                        bordered: true,
                        striped: true,
                    }
                }
            }
        }
    }
}

#[component]
fn BasicInputDemo() -> Element {
    rsx! {
        div {
            class: "space-y-4",
            Input {
                placeholder: Some("请输入文本".to_string()),
            }
        }
    }
}

#[component]
fn InputTypesDemo() -> Element {
    rsx! {
        div {
            class: "space-y-4",
            Input {
                input_type: "text".to_string(),
                placeholder: Some("文本输入框".to_string()),
            }
            Input {
                input_type: "password".to_string(),
                placeholder: Some("密码输入框".to_string()),
            }
            Input {
                input_type: "email".to_string(),
                placeholder: Some("邮箱输入框".to_string()),
            }
            Input {
                input_type: "number".to_string(),
                placeholder: Some("数字输入框".to_string()),
            }
        }
    }
}

#[component]
fn ControlledInputDemo() -> Element {
    let mut value = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "space-y-4",
            Input {
                value: Some(value()),
                placeholder: Some("请输入内容".to_string()),
                on_change: move |new_value| value.set(new_value),
            }
            if !value().is_empty() {
                p {
                    class: "text-sm text-gray-600 dark:text-gray-400",
                    "当前值: {value()}"
                }
            }
        }
    }
}

#[component]
fn InputStatesDemo() -> Element {
    rsx! {
        div {
            class: "space-y-4",
            Input {
                value: Some("已禁用的输入框".to_string()),
                disabled: true,
            }
            Input {
                value: Some("只读输入框".to_string()),
                readonly: true,
            }
        }
    }
}
