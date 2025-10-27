use crate::views::layout::ComponentsSidebar;
use crate::views::layout::TocItem;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::form::checkbox::CheckboxState;
use helixui::components::{Checkbox, DemoBox};

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
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "复选框的状态" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<Option<CheckboxState>>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "default_checked" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "默认状态" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "CheckboxState" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "Unchecked" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "disabled" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "是否禁用" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<bool>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "on_checked_change" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "状态变化时的回调" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "Callback<CheckboxState>" }
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
