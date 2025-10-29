use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{Collapsible, CollapsibleContent, CollapsibleTrigger, DemoBox, Table};

/// Collapsible 演示页面
#[component]
pub fn CollapsiblePage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 1,
        },
        TocItem {
            id: "controlled".to_string(),
            title: "受控模式".to_string(),
            level: 1,
        },
        TocItem {
            id: "default-open".to_string(),
            title: "默认展开".to_string(),
            level: 1,
        },
        TocItem {
            id: "keep-mounted".to_string(),
            title: "保持挂载".to_string(),
            level: 1,
        },
        TocItem {
            id: "api".to_string(),
            title: "API".to_string(),
            level: 1,
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
                        "折叠面板 Collapsible"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "Collapsible 组件用于创建可以展开和折叠的内容面板，支持受控和非受控模式。"
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
                        title: "基础用法".to_string(),
                        description: "最简单的折叠面板用法，点击按钮展开或折叠内容。".to_string(),
                        code: r#"use helixui::components::{Collapsible, CollapsibleTrigger, CollapsibleContent};

#[component]
fn CollapsibleDemo() -> Element {
    rsx! {
        Collapsible {
            CollapsibleTrigger {
                button {
                    class: "w-full flex items-center justify-between p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg text-left",
                    span { class: "font-medium", "点击展开内容" }
                    span { "▼" }
                }
            }
            CollapsibleContent {
                div {
                    class: "p-4 border border-gray-200 dark:border-gray-700 border-t-0 rounded-b-lg bg-gray-50 dark:bg-gray-900",
                    "这里是折叠面板的内容。你可以在这里放置任何内容。"
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            BasicCollapsibleDemo {}
                        }
                    }
                }

                // 受控模式
                section {
                    id: "controlled",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "受控模式"
                    }

                    DemoBox {
                        title: "受控模式".to_string(),
                        description: "通过受控模式，你可以完全控制折叠面板的展开和折叠状态。".to_string(),
                        code: r#"use helixui::components::{Collapsible, CollapsibleTrigger, CollapsibleContent};

#[component]
fn ControlledCollapsibleDemo() -> Element {
    let mut open_opt: Signal<Option<bool>> = use_signal(|| Some(false));
    
    rsx! {
        div {
            class: "space-y-4",
            Collapsible {
                open: open.map(|v| Some(*v)),
                on_open_change: move |v| open.set(v),
                CollapsibleTrigger {
                    button {
                        class: "w-full flex items-center justify-between p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg text-left",
                        span { class: "font-medium", if open() { "收起" } else { "展开" } }
                        span { if open() { "▲" } else { "▼" } }
                    }
                }
                CollapsibleContent {
                    div {
                        class: "p-4 border border-gray-200 dark:border-gray-700 border-t-0 rounded-b-lg bg-gray-50 dark:bg-gray-900",
                        "受控模式下的折叠面板内容。"
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            ControlledCollapsibleDemo {}
                        }
                    }
                }

                // 默认展开
                section {
                    id: "default-open",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "默认展开"
                    }

                    DemoBox {
                        title: "默认展开".to_string(),
                        description: "使用 default_open 属性让折叠面板默认处于展开状态。".to_string(),
                        code: r#"use helixui::components::{Collapsible, CollapsibleTrigger, CollapsibleContent};

#[component]
fn DefaultOpenCollapsibleDemo() -> Element {
    rsx! {
        Collapsible {
            default_open: true,
            CollapsibleTrigger {
                button {
                    class: "w-full flex items-center justify-between p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg text-left",
                    span { class: "font-medium", "默认展开的面板" }
                    span { "▼" }
                }
            }
            CollapsibleContent {
                div {
                    class: "p-4 border border-gray-200 dark:border-gray-700 border-t-0 rounded-b-lg bg-gray-50 dark:bg-gray-900",
                    "这个折叠面板默认是展开的。"
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            DefaultOpenCollapsibleDemo {}
                        }
                    }
                }

                // 保持挂载
                section {
                    id: "keep-mounted",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "保持挂载"
                    }

                    DemoBox {
                        title: "保持挂载".to_string(),
                        description: "使用 keep_mounted 属性让折叠面板关闭时内容仍然保留在 DOM 中。".to_string(),
                        code: r#"use helixui::components::{Collapsible, CollapsibleTrigger, CollapsibleContent};

#[component]
fn KeepMountedCollapsibleDemo() -> Element {
    rsx! {
        Collapsible {
            keep_mounted: use_signal(|| true),
            CollapsibleTrigger {
                button {
                    class: "w-full flex items-center justify-between p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg text-left",
                    span { class: "font-medium", "保持挂载的面板" }
                    span { "▼" }
                }
            }
            CollapsibleContent {
                div {
                    class: "p-4 border border-gray-200 dark:border-gray-700 border-t-0 rounded-b-lg bg-gray-50 dark:bg-gray-900",
                    "这个面板关闭时，内容仍然保留在 DOM 中。"
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            KeepMountedCollapsibleDemo {}
                        }
                    }
                }

                // API 文档
                section {
                    id: "api",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "API"
                    }

                    div {
                        class: "space-y-8",

                        // Collapsible Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "Collapsible Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["default_open".to_string(), "bool".to_string(), "false".to_string(), "默认是否展开".to_string()],
                                    vec!["open".to_string(), "ReadSignal<Option<bool>>".to_string(), "None".to_string(), "控制是否展开（受控模式）".to_string()],
                                    vec!["on_open_change".to_string(), "Callback<bool>".to_string(), "-".to_string(), "展开状态改变时的回调".to_string()],
                                    vec!["disabled".to_string(), "ReadSignal<bool>".to_string(), "false".to_string(), "是否禁用".to_string()],
                                    vec!["keep_mounted".to_string(), "ReadSignal<bool>".to_string(), "false".to_string(), "关闭时是否保留在 DOM".to_string()],
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "子元素".to_string()],
                                ],
                                bordered: true,
                                striped: true,
                            }
                        }

                        // CollapsibleTrigger Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4 mt-8",
                                "CollapsibleTrigger Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "触发元素".to_string()],
                                ],
                                bordered: true,
                                striped: true,
                            }
                        }

                        // CollapsibleContent Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4 mt-8",
                                "CollapsibleContent Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["id".to_string(), "ReadSignal<Option<String>>".to_string(), "None".to_string(), "内容元素 ID".to_string()],
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "内容".to_string()],
                                ],
                                bordered: true,
                                striped: true,
                            }
                        }
                    }
                }
            }
        }
    }
}

// 演示组件
#[component]
fn BasicCollapsibleDemo() -> Element {
    let content_id = use_signal(|| None::<String>);
    rsx! {
        Collapsible {
            CollapsibleTrigger {
                button {
                    class: "w-full flex items-center justify-between p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg text-left hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors",
                    span { class: "font-medium text-gray-900 dark:text-white", "点击展开内容" }
                    span { class: "text-gray-500 dark:text-gray-400", "▼" }
                }
            }
            CollapsibleContent {
                id: content_id,
                div {
                    class: "p-4 border border-gray-200 dark:border-gray-700 border-t-0 rounded-b-lg bg-gray-50 dark:bg-gray-900 text-gray-700 dark:text-gray-300",
                    "这里是折叠面板的内容。你可以在这里放置任何内容。"
                }
            }
        }
    }
}

#[component]
fn ControlledCollapsibleDemo() -> Element {
    let mut open_opt: Signal<Option<bool>> = use_signal(|| Some(false));
    let content_id = use_signal(|| None::<String>);

    rsx! {
        div {
            class: "space-y-4",
            Collapsible {
                open: open_opt,
                on_open_change: move |v| open_opt.set(Some(v)),
                    CollapsibleTrigger {
                        button {
                            class: "w-full flex items-center justify-between p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg text-left hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors",
                        span { class: "font-medium text-gray-900 dark:text-white", if open_opt().unwrap_or(false) { "收起" } else { "展开" } }
                        span { class: "text-gray-500 dark:text-gray-400", if open_opt().unwrap_or(false) { "▲" } else { "▼" } }
                    }
                }
                CollapsibleContent {
                    id: content_id,
                    div {
                        class: "p-4 border border-gray-200 dark:border-gray-700 border-t-0 rounded-b-lg bg-gray-50 dark:bg-gray-900 text-gray-700 dark:text-gray-300",
                        "受控模式下的折叠面板内容。你可以完全控制展开和折叠状态。"
                    }
                }
            }
        }
    }
}

#[component]
fn DefaultOpenCollapsibleDemo() -> Element {
    let content_id = use_signal(|| None::<String>);
    rsx! {
        Collapsible {
            default_open: true,
            CollapsibleTrigger {
                button {
                    class: "w-full flex items-center justify-between p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg text-left hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors",
                    span { class: "font-medium text-gray-900 dark:text-white", "默认展开的面板" }
                    span { class: "text-gray-500 dark:text-gray-400", "▼" }
                }
            }
            CollapsibleContent {
                id: content_id,
                div {
                    class: "p-4 border border-gray-200 dark:border-gray-700 border-t-0 rounded-b-lg bg-gray-50 dark:bg-gray-900 text-gray-700 dark:text-gray-300",
                    "这个折叠面板默认是展开的。"
                }
            }
        }
    }
}

#[component]
fn KeepMountedCollapsibleDemo() -> Element {
    let content_id = use_signal(|| None::<String>);
    rsx! {
        Collapsible {
            keep_mounted: use_signal(|| true),
            CollapsibleTrigger {
                button {
                    class: "w-full flex items-center justify-between p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg text-left hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors",
                    span { class: "font-medium text-gray-900 dark:text-white", "保持挂载的面板" }
                    span { class: "text-gray-500 dark:text-gray-400", "▼" }
                }
            }
            CollapsibleContent {
                id: content_id,
                div {
                    class: "p-4 border border-gray-200 dark:border-gray-700 border-t-0 rounded-b-lg bg-gray-50 dark:bg-gray-900 text-gray-700 dark:text-gray-300",
                    "这个面板关闭时，内容仍然保留在 DOM 中。"
                }
            }
        }
    }
}
