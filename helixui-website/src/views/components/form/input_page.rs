use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{DemoBox, Input};

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

                    div {
                        class: "overflow-x-auto mb-6",
                        table {
                            class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                            thead {
                                class: "bg-gray-50 dark:bg-gray-800",
                                tr {
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "参数"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "说明"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "类型"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "默认值"
                                    }
                                }
                            }
                            tbody {
                                class: "bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700",
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "input_type" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "输入框类型，如 'text', 'password', 'number' 等" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "String" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "'text'" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "value" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "输入框的值" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "Option<String>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "None" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "placeholder" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "占位符文本" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "Option<String>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "None" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "disabled" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "是否禁用" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "bool" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "false" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "readonly" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "是否只读" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "bool" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "false" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "on_change" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "值变化时的回调" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "Option<EventHandler<String>>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "None" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "class" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "自定义类名" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "Option<String>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "None" }
                                }
                            }
                        }
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
