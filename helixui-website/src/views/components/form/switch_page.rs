use crate::views::layout::ComponentsSidebar;
use crate::views::layout::TocItem;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{DemoBox, Switch};

/// Switch 演示页面
#[component]
pub fn SwitchPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic-usage".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "controlled".to_string(),
            title: "受控开关".to_string(),
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
                    code: r#"Switch {{
    checked: ReadSignal::new(Signal::new(None)),
    default_checked: false,
    "开关"
}}"#,
                    children: rsx! {
                        BasicSwitchDemo {}
                    }
                }

                DemoBox {
                    title: "受控开关",
                    description: "通过状态管理来控制开关状态。",
                    code: r#"let checked = use_signal(|| Some(false));
Switch {{
    checked: checked.read(),
    default_checked: false,
    on_checked_change: Callback::new(move |state| checked.set(Some(state))),
    "开关"
}}"#,
                    children: rsx! {
                        ControlledSwitchDemo {}
                    }
                }

                DemoBox {
                    title: "状态",
                    description: "开关的禁用状态。",
                    code: r#"Switch {{
    checked: ReadSignal::new(Signal::new(Some(true))),
    default_checked: false,
    disabled: ReadSignal::new(Signal::new(true)),
    "禁用开关"
}}"#,
                    children: rsx! {
                        SwitchStatesDemo {}
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
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "checked" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "开关的状态" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<Option<bool>>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "default_checked" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "默认状态" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "bool" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "false" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "disabled" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "是否禁用" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<bool>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "false" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "on_checked_change" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "状态变化时的回调" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "Callback<bool>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
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
fn BasicSwitchDemo() -> Element {
    rsx! {
        div {
            class: "space-y-4",
            Switch {
                checked: ReadSignal::new(Signal::new(None)),
                default_checked: true,
                span { "已启用" }
            }
            Switch {
                checked: ReadSignal::new(Signal::new(None)),
                default_checked: false,
                span { "未启用" }
            }
        }
    }
}

#[component]
fn ControlledSwitchDemo() -> Element {
    let mut checked = use_signal(|| Some(false));

    rsx! {
        div {
            div {
                class: "mb-4",
                Switch {
                    checked: ReadSignal::new(checked),
                    default_checked: false,
                    on_checked_change: Callback::new(move |state| checked.set(Some(state))),
                    span { "开关" }
                }
            }
            div {
                class: "p-3 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg text-sm text-blue-900 dark:text-blue-100",
                if checked().unwrap_or(false) {
                    "当前状态: 开启"
                } else {
                    "当前状态: 关闭"
                }
            }
        }
    }
}

#[component]
fn SwitchStatesDemo() -> Element {
    rsx! {
        div {
            class: "space-y-4",
            Switch {
                checked: ReadSignal::new(Signal::new(Some(true))),
                default_checked: false,
                disabled: ReadSignal::new(Signal::new(false)),
                span { "正常开关" }
            }
            Switch {
                checked: ReadSignal::new(Signal::new(Some(true))),
                default_checked: false,
                disabled: ReadSignal::new(Signal::new(true)),
                span { "禁用开关" }
            }
        }
    }
}
