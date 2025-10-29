use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{DemoBox, Ellipsis, EllipsisLines, Table};

#[component]
pub fn EllipsisPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "single-line".to_string(),
            title: "单行省略".to_string(),
            level: 2,
        },
        TocItem {
            id: "multi-line".to_string(),
            title: "多行省略".to_string(),
            level: 2,
        },
        TocItem {
            id: "expand".to_string(),
            title: "展开收起".to_string(),
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
                class: "component-doc",

                // 标题
                div {
                    class: "mb-8",
                    h1 {
                        class: "text-4xl font-bold text-gray-900 dark:text-white mb-2",
                        "省略文字 Ellipsis"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "用于文本过长时的省略显示，支持单行和多行省略，可展开收起。"
                    }
                }

                // 基础用法
                section {
                    id: "basic",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "基础用法"
                    }
                    DemoBox {
                        title: "基础用法",
                        description: "最简单的省略文字用法，单行省略。",
                        code: r#"
                            Ellipsis {{
                                text: "这是一段很长很长的文本，当文本内容超过容器宽度时会被省略显示。".to_string(),
                            }}
                        "#,
                        children: rsx! {
                            div {
                                class: "w-64 border border-gray-200 dark:border-gray-700 rounded-lg p-4",
                                Ellipsis {
                                    text: "这是一段很长很长的文本，当文本内容超过容器宽度时会被省略显示。".to_string(),
                                }
                            }
                        }
                    }
                }

                // 单行省略
                section {
                    id: "single-line",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "单行省略"
                    }
                    DemoBox {
                        title: "单行省略",
                        description: "使用 lines: EllipsisLines::Single 来设置单行省略（默认）。",
                        code: r#"
                            Ellipsis {{
                                text: "单行省略示例：这是一段很长很长的文本内容。".to_string(),
                                lines: EllipsisLines::Single,
                            }}
                        "#,
                        children: rsx! {
                            div {
                                class: "w-64 border border-gray-200 dark:border-gray-700 rounded-lg p-4",
                                Ellipsis {
                                    text: "单行省略示例：这是一段很长很长的文本内容，当文本超过容器宽度时会被截断并显示省略号。".to_string(),
                                    lines: EllipsisLines::Single,
                                }
                            }
                        }
                    }
                }

                // 多行省略
                section {
                    id: "multi-line",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "多行省略"
                    }
                    DemoBox {
                        title: "多行省略",
                        description: "使用 lines: EllipsisLines::Multiple(n) 来设置多行省略。",
                        code: r#"
                            Ellipsis {{
                                text: "多行省略示例...".to_string(),
                                lines: EllipsisLines::Multiple(2),
                            }}
                            Ellipsis {{
                                text: "多行省略示例...".to_string(),
                                lines: EllipsisLines::Multiple(3),
                            }}
                        "#,
                        children: rsx! {
                            div {
                                class: "space-y-4",
                                div {
                                    class: "w-64 border border-gray-200 dark:border-gray-700 rounded-lg p-4",
                                    Ellipsis {
                                        text: "多行省略示例：这是一段很长很长的文本内容，当文本超过指定行数时会被截断并显示省略号。支持自定义行数，可以设置为2行、3行或更多行。".to_string(),
                                        lines: EllipsisLines::Multiple(2),
                                    }
                                }
                                div {
                                    class: "w-64 border border-gray-200 dark:border-gray-700 rounded-lg p-4",
                                    Ellipsis {
                                        text: "多行省略示例：这是一段很长很长的文本内容，当文本超过指定行数时会被截断并显示省略号。支持自定义行数，可以设置为2行、3行或更多行。这里设置为3行省略。".to_string(),
                                        lines: EllipsisLines::Multiple(3),
                                    }
                                }
                            }
                        }
                    }
                }

                // 展开收起
                section {
                    id: "expand",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "展开收起"
                    }
                    DemoBox {
                        title: "展开收起",
                        description: "可以通过 expand_text 和 collapse_text 来添加展开/收起功能。",
                        code: r#"
                            Ellipsis {{
                                text: "这是一段很长很长的文本...".to_string(),
                                expand_text: Some("展开".to_string()),
                                collapse_text: Some("收起".to_string()),
                            }}
                            Ellipsis {{
                                text: "这是一段很长很长的文本...".to_string(),
                                lines: EllipsisLines::Multiple(2),
                                expand_text: Some("展开".to_string()),
                            }}
                        "#,
                        children: rsx! {
                            div {
                                class: "space-y-4",
                                div {
                                    class: "w-64 border border-gray-200 dark:border-gray-700 rounded-lg p-4",
                                    Ellipsis {
                                        text: "这是一段很长很长的文本内容，当文本超过容器宽度时会被省略显示。点击展开按钮可以查看完整内容，再次点击收起可以折叠文本。".to_string(),
                                        expand_text: Some("展开".to_string()),
                                        collapse_text: Some("收起".to_string()),
                                    }
                                }
                                div {
                                    class: "w-64 border border-gray-200 dark:border-gray-700 rounded-lg p-4",
                                    Ellipsis {
                                        text: "多行展开收起示例：这是一段很长很长的文本内容，当文本超过指定行数时会被截断并显示省略号。点击展开按钮可以查看完整内容，再次点击收起可以折叠文本。支持自定义展开和收起的文本。".to_string(),
                                        lines: EllipsisLines::Multiple(2),
                                        expand_text: Some("展开".to_string()),
                                        collapse_text: Some("收起".to_string()),
                                    }
                                }
                            }
                        }
                    }
                }

                // API
                section {
                    id: "api",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "API"
                    }
                    div { class: "mt-2",
                        Table {
                            headers: Some(vec![
                                "属性".to_string(),
                                "类型".to_string(),
                                "默认值".to_string(),
                                "说明".to_string(),
                            ]),
                            data: vec![
                                vec!["text".to_string(), "String".to_string(), "-".to_string(), "文本内容".to_string()],
                                vec!["lines".to_string(), "EllipsisLines".to_string(), "Single".to_string(), "省略行数：Single 表示单行省略，Multiple(n) 表示多行省略".to_string()],
                                vec!["expand_text".to_string(), "Option<String>".to_string(), "None".to_string(), "展开按钮的文本，如果提供则显示展开/收起功能".to_string()],
                                vec!["collapse_text".to_string(), "Option<String>".to_string(), "None".to_string(), "收起按钮的文本，如果不提供则使用展开文本".to_string()],
                                vec!["default_expanded".to_string(), "bool".to_string(), "false".to_string(), "是否默认展开（仅在提供 expand_text 时有效）".to_string()],
                                vec!["class".to_string(), "Option<String>".to_string(), "None".to_string(), "自定义类名".to_string()],
                                vec!["style".to_string(), "Option<String>".to_string(), "None".to_string(), "自定义样式".to_string()],
                                vec!["on_expand_change".to_string(), "Option<EventHandler<bool>>".to_string(), "None".to_string(), "展开/收起状态变化回调".to_string()],
                            ],
                        }
                    }
                }
            }
        }
    }
}
