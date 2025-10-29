use crate::views::layout::ComponentsSidebar;
use crate::views::layout::TocItem;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{DemoBox, Switch, Table};

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

                    Table {
                        headers: Some(vec![
                            "参数".to_string(),
                            "说明".to_string(),
                            "类型".to_string(),
                            "默认值".to_string(),
                        ]),
                        data: vec![
                            vec!["checked".to_string(), "开关的状态".to_string(), "ReadSignal<Option<bool>>".to_string(), "-".to_string()],
                            vec!["default_checked".to_string(), "默认状态".to_string(), "bool".to_string(), "false".to_string()],
                            vec!["disabled".to_string(), "是否禁用".to_string(), "ReadSignal<bool>".to_string(), "false".to_string()],
                            vec!["on_checked_change".to_string(), "状态变化时的回调".to_string(), "Callback<bool>".to_string(), "-".to_string()],
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
