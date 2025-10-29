use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{
    DemoBox, Dropdown, DropdownContent, DropdownItem, DropdownTrigger, Table,
};

/// Dropdown 演示页面
#[component]
pub fn DropdownPage() -> Element {
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
            id: "disabled".to_string(),
            title: "禁用状态".to_string(),
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
                        "下拉菜单 Dropdown"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "Dropdown 组件用于创建下拉菜单，支持键盘导航和可访问性。"
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
                        description: "创建一个简单的下拉菜单，支持点击和键盘导航。".to_string(),
                        code: r#"use helixui::components::{Dropdown, DropdownTrigger, DropdownContent, DropdownItem};

#[component]
fn DropdownDemo() -> Element {
    rsx! {
        Dropdown {
            DropdownTrigger {
                button {
                    class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                    "打开菜单"
                }
            }
            DropdownContent {
                DropdownItem::<String> {
                    value: use_signal(|| "edit".to_string()),
                    index: use_signal(|| 0usize),
                    on_select: move |v| {
                        tracing::info!("Selected: {}", v);
                    },
                    "编辑"
                }
                DropdownItem::<String> {
                    value: use_signal(|| "delete".to_string()),
                    index: use_signal(|| 1usize),
                    on_select: move |v| {
                        tracing::info!("Selected: {}", v);
                    },
                    "删除"
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            BasicDropdownDemo {}
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
                        description: "通过受控模式，你可以完全控制下拉菜单的展开和折叠状态。".to_string(),
                        code: r#"use helixui::components::{Dropdown, DropdownTrigger, DropdownContent, DropdownItem};

#[component]
fn ControlledDropdownDemo() -> Element {
    let mut open: Signal<Option<bool>> = use_signal(|| Some(false));
    
    rsx! {
        Dropdown {
            open,
            on_open_change: move |v| open.set(Some(v)),
            DropdownTrigger {
                button {
                    class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                    if open().unwrap_or(false) { "菜单打开" } else { "点击打开" }
                }
            }
            DropdownContent {
                DropdownItem::<String> {
                    value: use_signal(|| "item1".to_string()),
                    index: use_signal(|| 0usize),
                    on_select: move |v| tracing::info!("Selected: {}", v),
                    "项目 1"
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            ControlledDropdownDemo {}
                        }
                    }
                }

                // 禁用状态
                section {
                    id: "disabled",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "禁用状态"
                    }

                    DemoBox {
                        title: "禁用状态".to_string(),
                        description: "可以禁用整个下拉菜单，或禁用单个菜单项。".to_string(),
                        code: r#"use helixui::components::{Dropdown, DropdownTrigger, DropdownContent, DropdownItem};

#[component]
fn DisabledDropdownDemo() -> Element {
    rsx! {
        div {
            class: "space-y-4",
            // 禁用整个菜单
            Dropdown {
                disabled: use_signal(|| true),
                DropdownTrigger {
                    button {
                        class: "px-4 py-2 bg-gray-400 text-white rounded-lg cursor-not-allowed",
                        "禁用菜单"
                    }
                }
                DropdownContent {
                    DropdownItem::<String> {
                        value: use_signal(|| "item".to_string()),
                        index: use_signal(|| 0usize),
                        on_select: move |_| {},
                        "项目"
                    }
                }
            }
            
            // 禁用单个项
            Dropdown {
                DropdownTrigger {
                    button {
                        class: "px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700",
                        "部分禁用的菜单"
                    }
                }
                DropdownContent {
                    DropdownItem::<String> {
                        value: use_signal(|| "enabled".to_string()),
                        index: use_signal(|| 0usize),
                        disabled: use_signal(|| false),
                        on_select: move |v| tracing::info!("Selected: {}", v),
                        "启用项"
                    }
                    DropdownItem::<String> {
                        value: use_signal(|| "disabled".to_string()),
                        index: use_signal(|| 1usize),
                        disabled: use_signal(|| true),
                        on_select: move |_| {},
                        "禁用项"
                    }
                }
            }
        }
    }
}"#.to_string(),

                        children: rsx! {
                            DisabledDropdownDemo {}
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

                        // Dropdown Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "Dropdown Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["default_open".to_string(), "bool".to_string(), "false".to_string(), "默认是否展开".to_string()],
                                    vec!["open".to_string(), "ReadSignal<Option<bool>>".to_string(), "None".to_string(), "控制是否展开（受控模式）".to_string()],
                                    vec!["on_open_change".to_string(), "Callback<bool>".to_string(), "-".to_string(), "展开状态改变时的回调".to_string()],
                                    vec!["disabled".to_string(), "ReadSignal<bool>".to_string(), "false".to_string(), "是否禁用".to_string()],
                                    vec!["roving_loop".to_string(), "ReadSignal<bool>".to_string(), "true".to_string(), "键盘导航是否循环".to_string()],
                                    vec!["children".to_string(), "Element".to_string(), "-".to_string(), "子元素".to_string()],
                                ],
                                bordered: true,
                                striped: true,
                            }
                        }

                        // DropdownTrigger Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4 mt-8",
                                "DropdownTrigger Props"
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

                        // DropdownContent Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4 mt-8",
                                "DropdownContent Props"
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

                        // DropdownItem Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4 mt-8",
                                "DropdownItem Props"
                            }
                            Table {
                                headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                                data: vec![
                                    vec!["value".to_string(), "ReadSignal<T>".to_string(), "-".to_string(), "项的值".to_string()],
                                    vec!["index".to_string(), "ReadSignal<usize>".to_string(), "-".to_string(), "项的索引".to_string()],
                                    vec!["disabled".to_string(), "ReadSignal<bool>".to_string(), "false".to_string(), "是否禁用".to_string()],
                                    vec!["on_select".to_string(), "Callback<T>".to_string(), "-".to_string(), "选中时的回调".to_string()],
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
fn BasicDropdownDemo() -> Element {
    let id = use_signal(|| None::<String>);
    rsx! {
        Dropdown {
            DropdownTrigger {
                "打开菜单"
            }
            DropdownContent {
                id,
                DropdownItem::<String> {
                    value: use_signal(|| "edit".to_string()),
                    index: use_signal(|| 0usize),
                    on_select: move |v| {
                        tracing::info!("Selected: {}", v);
                    },
                    "编辑"
                }
                DropdownItem::<String> {
                    value: use_signal(|| "delete".to_string()),
                    index: use_signal(|| 1usize),
                    on_select: move |v| {
                        tracing::info!("Selected: {}", v);
                    },
                    "删除"
                }
                DropdownItem::<String> {
                    value: use_signal(|| "share".to_string()),
                    index: use_signal(|| 2usize),
                    on_select: move |v| {
                        tracing::info!("Selected: {}", v);
                    },
                    "分享"
                }
            }
        }
    }
}

#[component]
fn ControlledDropdownDemo() -> Element {
    let mut open_opt: Signal<Option<bool>> = use_signal(|| Some(false));
    let id = use_signal(|| None::<String>);

    rsx! {
        Dropdown {
            open: open_opt,
            on_open_change: move |v| open_opt.set(Some(v)),
            DropdownTrigger {
                if open_opt().unwrap_or(false) { "菜单已打开" } else { "点击打开菜单" }
            }
            DropdownContent {
                id,
                DropdownItem::<String> {
                    value: use_signal(|| "item1".to_string()),
                    index: use_signal(|| 0usize),
                    on_select: move |v| {
                        tracing::info!("Selected: {}", v);
                    },
                    "项目 1"
                }
                DropdownItem::<String> {
                    value: use_signal(|| "item2".to_string()),
                    index: use_signal(|| 1usize),
                    on_select: move |v| {
                        tracing::info!("Selected: {}", v);
                    },
                    "项目 2"
                }
            }
        }
    }
}

#[component]
fn DisabledDropdownDemo() -> Element {
    let id1 = use_signal(|| None::<String>);
    let id2 = use_signal(|| None::<String>);

    rsx! {
        div {
            class: "space-y-4",
            // 禁用整个菜单
            Dropdown {
                disabled: use_signal(|| true),
                DropdownTrigger {
                    "禁用菜单"
                }
                DropdownContent {
                    id: id1,
                    DropdownItem::<String> {
                        value: use_signal(|| "item".to_string()),
                        index: use_signal(|| 0usize),
                        on_select: move |_| {},
                        "项目"
                    }
                }
            }

            // 禁用单个项
            Dropdown {
                DropdownTrigger {
                    "部分禁用的菜单"
                }
                DropdownContent {
                    id: id2,
                    DropdownItem::<String> {
                        value: use_signal(|| "enabled".to_string()),
                        index: use_signal(|| 0usize),
                        disabled: use_signal(|| false),
                        on_select: move |v| tracing::info!("Selected: {}", v),
                        "启用项"
                    }
                    DropdownItem::<String> {
                        value: use_signal(|| "disabled".to_string()),
                        index: use_signal(|| 1usize),
                        disabled: use_signal(|| true),
                        on_select: move |_| {},
                        "禁用项"
                    }
                }
            }
        }
    }
}
