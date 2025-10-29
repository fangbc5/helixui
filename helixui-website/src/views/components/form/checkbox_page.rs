use crate::views::layout::ComponentsSidebar;
use crate::views::layout::TocItem;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::form::checkbox::CheckboxState;
use helixui::components::{Checkbox, DemoBox, Table};

/// Checkbox 演示页面
#[component]
pub fn CheckboxPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic-usage".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "controlled".to_string(),
            title: "受控复选框".to_string(),
            level: 2,
        },
        TocItem {
            id: "states".to_string(),
            title: "状态".to_string(),
            level: 2,
        },
        TocItem {
            id: "indeterminate".to_string(),
            title: "不确定状态".to_string(),
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
                    code: r#"Checkbox {{
    checked: ReadSignal::new(Signal::new(None)),
    default_checked: CheckboxState::Unchecked,
    "选项1"
}}"#,
                    children: rsx! {
                        BasicCheckboxDemo {}
                    }
                }

                DemoBox {
                    title: "受控复选框",
                    description: "通过状态管理来控制选中。",
                    code: r#"let checked = use_signal(|| Some(CheckboxState::Checked));
Checkbox {{
    checked: checked.read(),
    default_checked: CheckboxState::Unchecked,
    on_checked_change: Callback::new(move |state| checked.set(Some(state))),
    "选项"
}}"#,
                    children: rsx! {
                        ControlledCheckboxDemo {}
                    }
                }

                DemoBox {
                    title: "状态",
                    description: "复选框的禁用、只读等状态。",
                    code: r#"Checkbox {{
    checked: ReadSignal::new(Signal::new(Some(CheckboxState::Checked))),
    default_checked: CheckboxState::Unchecked,
    disabled: ReadSignal::new(Signal::new(true)),
    "禁用选项"
}}"#,
                    children: rsx! {
                        CheckboxStatesDemo {}
                    }
                }

                DemoBox {
                    title: "不确定状态",
                    description: "复选框的不确定状态。",
                    code: r#"Checkbox {{
    checked: ReadSignal::new(Signal::new(Some(CheckboxState::Indeterminate))),
    default_checked: CheckboxState::Unchecked,
    "不确定选项"
}}"#,
                    children: rsx! {
                        IndeterminateCheckboxDemo {}
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
                            vec!["checked".to_string(), "复选框的状态".to_string(), "ReadSignal<Option<CheckboxState>>".to_string(), "-".to_string()],
                            vec!["default_checked".to_string(), "默认状态".to_string(), "CheckboxState".to_string(), "Unchecked".to_string()],
                            vec!["disabled".to_string(), "是否禁用".to_string(), "ReadSignal<bool>".to_string(), "-".to_string()],
                            vec!["on_checked_change".to_string(), "状态变化时的回调".to_string(), "Callback<CheckboxState>".to_string(), "-".to_string()],
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
fn BasicCheckboxDemo() -> Element {
    rsx! {
        div {
            class: "space-y-2",
            Checkbox {
                checked: ReadSignal::new(Signal::new(None)),
                default_checked: CheckboxState::Checked,
                span { "苹果" }
            }
            Checkbox {
                checked: ReadSignal::new(Signal::new(None)),
                default_checked: CheckboxState::Unchecked,
                span { "香蕉" }
            }
            Checkbox {
                checked: ReadSignal::new(Signal::new(None)),
                default_checked: CheckboxState::Unchecked,
                span { "橙子" }
            }
        }
    }
}

#[component]
fn ControlledCheckboxDemo() -> Element {
    let mut checked1 = use_signal(|| Some(CheckboxState::Checked));
    let mut checked2 = use_signal(|| Some(CheckboxState::Unchecked));
    let mut checked3 = use_signal(|| Some(CheckboxState::Unchecked));

    rsx! {
        div {
            class: "space-y-2",
            Checkbox {
                checked: ReadSignal::new(checked1),
                default_checked: CheckboxState::Unchecked,
                on_checked_change: Callback::new(move |state| checked1.set(Some(state))),
                span { "苹果" }
            }
            Checkbox {
                checked: ReadSignal::new(checked2),
                default_checked: CheckboxState::Unchecked,
                on_checked_change: Callback::new(move |state| checked2.set(Some(state))),
                span { "香蕉" }
            }
            Checkbox {
                checked: ReadSignal::new(checked3),
                default_checked: CheckboxState::Unchecked,
                on_checked_change: Callback::new(move |state| checked3.set(Some(state))),
                span { "橙子" }
            }
        }
        div {
            class: "mt-4 p-3 bg-gray-50 dark:bg-gray-800 rounded text-sm",
            "状态: 苹果={checked1():?}, 香蕉={checked2():?}, 橙子={checked3():?}"
        }
    }
}

#[component]
fn CheckboxStatesDemo() -> Element {
    rsx! {
        div {
            class: "space-y-2",
            Checkbox {
                checked: ReadSignal::new(Signal::new(Some(CheckboxState::Checked))),
                default_checked: CheckboxState::Unchecked,
                disabled: ReadSignal::new(Signal::new(false)),
                span { "正常选项" }
            }
            Checkbox {
                checked: ReadSignal::new(Signal::new(Some(CheckboxState::Checked))),
                default_checked: CheckboxState::Unchecked,
                disabled: ReadSignal::new(Signal::new(true)),
                span { "禁用选项" }
            }
        }
    }
}

#[component]
fn IndeterminateCheckboxDemo() -> Element {
    rsx! {
        div {
            class: "space-y-2",
            Checkbox {
                checked: ReadSignal::new(Signal::new(Some(CheckboxState::Checked))),
                default_checked: CheckboxState::Unchecked,
                span { "已选中" }
            }
            Checkbox {
                checked: ReadSignal::new(Signal::new(Some(CheckboxState::Indeterminate))),
                default_checked: CheckboxState::Unchecked,
                span { "不确定" }
            }
            Checkbox {
                checked: ReadSignal::new(Signal::new(Some(CheckboxState::Unchecked))),
                default_checked: CheckboxState::Unchecked,
                span { "未选中" }
            }
        }
    }
}
