use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{DemoBox, RadioGroup, RadioItem};

/// Radio 演示页面
#[component]
pub fn RadioPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic-usage".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "controlled".to_string(),
            title: "受控单选".to_string(),
            level: 2,
        },
        TocItem {
            id: "real-world".to_string(),
            title: "实际应用".to_string(),
            level: 2,
        },
        TocItem {
            id: "horizontal".to_string(),
            title: "横向布局".to_string(),
            level: 2,
        },
        TocItem {
            id: "disabled".to_string(),
            title: "禁用状态".to_string(),
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
                    description: "最简单的用法，可以选择水果。",
                    code: r#"RadioGroup {{
    value: ReadSignal::new(Signal::new(Some("苹果".to_string()))),
    default_value: "苹果".to_string(),
    RadioItem {{
        value: ReadSignal::new(Signal::new("苹果".to_string())),
        index: ReadSignal::new(Signal::new(0)),
        "苹果"
    }}
    RadioItem {{
        value: ReadSignal::new(Signal::new("香蕉".to_string())),
        index: ReadSignal::new(Signal::new(1)),
        "香蕉"
    }}
    RadioItem {{
        value: ReadSignal::new(Signal::new("橙子".to_string())),
        index: ReadSignal::new(Signal::new(2)),
        "橙子"
    }}
}}"#,
                    children: rsx! {
                        BasicRadioDemo {}
                    }
                }

                DemoBox {
                    title: "受控单选",
                    description: "显示当前选中值并支持动态更改。",
                    code: r#"let selected = use_signal(|| Some("option1".to_string()));
RadioGroup {{
    value: selected.read(),
    default_value: "option1".to_string(),
    on_value_change: Callback::new(move |v| selected.set(Some(v))),
    // ...
}}"#,
                    children: rsx! {
                        ControlledRadioDemo {}
                    }
                }

                DemoBox {
                    title: "实际应用",
                    description: "选择支付方式的真实场景。",
                    code: r#"let payment_method = use_signal(|| Some("credit".to_string()));
RadioGroup {{
    value: payment_method.read(),
    default_value: "credit".to_string(),
    on_value_change: Callback::new(move |v| payment_method.set(Some(v))),
    RadioItem {{ value: "credit", "信用卡" }}
    RadioItem {{ value: "alipay", "支付宝" }}
    RadioItem {{ value: "wechat", "微信支付" }}
}}"#,
                    children: rsx! {
                        PaymentMethodDemo {}
                    }
                }

                DemoBox {
                    title: "横向布局",
                    description: "单选框可以横向排列，常用于主题切换、布局选择等场景。",
                    code: r#"RadioGroup {{
    value: ReadSignal::new(Signal::new(Some("light".to_string()))),
    default_value: "light".to_string(),
    horizontal: ReadSignal::new(Signal::new(true)),
    RadioItem {{ value: "light", "浅色" }}
    RadioItem {{ value: "dark", "深色" }}
    RadioItem {{ value: "auto", "自动" }}
}}"#,
                    children: rsx! {
                        HorizontalRadioDemo {}
                    }
                }

                DemoBox {
                    title: "禁用状态",
                    description: "禁用某个选项，常见的业务场景。",
                    code: r#"RadioGroup {{
    value: ReadSignal::new(Signal::new(Some("normal".to_string()))),
    default_value: "normal".to_string(),
    RadioItem {{ value: "normal", "正常" }}
    RadioItem {{ 
        value: "disabled", 
        disabled: ReadSignal::new(Signal::new(true)),
        "暂不可用"
    }}
}}"#,
                    children: rsx! {
                        DisabledRadioDemo {}
                    }
                }

                div {
                    class: "mt-8",
                    h2 {
                        id: "api",
                        class: "text-2xl font-bold text-gray-900 dark:text-white mb-4",
                        "API"
                    }

                    h3 {
                        class: "text-xl font-bold text-gray-900 dark:text-white mb-4",
                        "RadioGroup"
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
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "value" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "当前选中的值" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<Option<String>>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "default_value" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "默认选中的值" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "String" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "on_value_change" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "值变化时的回调" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "Callback<String>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "disabled" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "是否禁用整个组" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<bool>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "horizontal" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "是否横向排列" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<bool>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "roving_loop" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "焦点是否循环" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<bool>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "true" }
                                }
                            }
                        }
                    }

                    h3 {
                        class: "text-xl font-bold text-gray-900 dark:text-white mb-4",
                        "RadioItem"
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
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "value" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "单选框的值" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<String>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "index" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "单选框的索引" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<usize>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "-" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "disabled" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "是否禁用" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<bool>" }
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
fn BasicRadioDemo() -> Element {
    let selected = use_signal(|| Some("苹果".to_string()));

    rsx! {
        div {
            class: "flex flex-col gap-3",
            RadioGroup {
                value: ReadSignal::new(selected),
                default_value: "苹果".to_string(),
                on_value_change: Callback::new({
                    let mut selected = selected;
                    move |v: String| selected.set(Some(v))
                }),
                RadioItem {
                    value: ReadSignal::new(Signal::new("苹果".to_string())),
                    index: ReadSignal::new(Signal::new(0)),
                    span { "苹果" }
                }
                RadioItem {
                    value: ReadSignal::new(Signal::new("香蕉".to_string())),
                    index: ReadSignal::new(Signal::new(1)),
                    span { "香蕉" }
                }
                RadioItem {
                    value: ReadSignal::new(Signal::new("橙子".to_string())),
                    index: ReadSignal::new(Signal::new(2)),
                    span { "橙子" }
                }
            }
        }
    }
}

#[component]
fn ControlledRadioDemo() -> Element {
    let selected = use_signal(|| Some("option1".to_string()));

    rsx! {
        div {
            class: "flex flex-col gap-3",
            RadioGroup {
                value: ReadSignal::new(selected),
                default_value: "option1".to_string(),
                on_value_change: Callback::new({
                    let mut selected = selected;
                    move |v: String| selected.set(Some(v))
                }),
                RadioItem {
                    value: ReadSignal::new(Signal::new("option1".to_string())),
                    index: ReadSignal::new(Signal::new(0)),
                    span { "选项 1" }
                }
                RadioItem {
                    value: ReadSignal::new(Signal::new("option2".to_string())),
                    index: ReadSignal::new(Signal::new(1)),
                    span { "选项 2" }
                }
                RadioItem {
                    value: ReadSignal::new(Signal::new("option3".to_string())),
                    index: ReadSignal::new(Signal::new(2)),
                    span { "选项 3" }
                }
            }
        }
        div {
            class: "mt-4 p-3 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg text-sm text-blue-900 dark:text-blue-100",
            "当前选中: "
            strong { "{selected().unwrap_or_default()}" }
        }
    }
}

#[component]
fn PaymentMethodDemo() -> Element {
    let payment_method = use_signal(|| Some("credit".to_string()));
    let payment_status = use_signal(|| "未选择".to_string());

    rsx! {
        div {
            div {
                class: "flex flex-col gap-3 mb-4",
                RadioGroup {
                    value: ReadSignal::new(payment_method),
                    default_value: "credit".to_string(),
                    on_value_change: Callback::new({
                        let mut payment_method = payment_method;
                        let mut payment_status = payment_status;
                        move |v: String| {
                            payment_method.set(Some(v.clone()));
                            let status = match v.as_str() {
                                "credit" => "您选择了信用卡支付",
                                "alipay" => "您选择了支付宝支付",
                                "wechat" => "您选择了微信支付",
                                _ => "",
                            };
                            payment_status.set(status.to_string());
                        }
                    }),
                    RadioItem {
                        value: ReadSignal::new(Signal::new("credit".to_string())),
                        index: ReadSignal::new(Signal::new(0)),
                        span { "信用卡" }
                    }
                    RadioItem {
                        value: ReadSignal::new(Signal::new("alipay".to_string())),
                        index: ReadSignal::new(Signal::new(1)),
                        span { "支付宝" }
                    }
                    RadioItem {
                        value: ReadSignal::new(Signal::new("wechat".to_string())),
                        index: ReadSignal::new(Signal::new(2)),
                        span { "微信支付" }
                    }
                }
            }
            div {
                class: "p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg",
                div {
                    class: "text-sm font-medium text-green-900 dark:text-green-100 mb-1",
                    "状态："
                }
                div {
                    class: "text-base text-green-800 dark:text-green-200",
                    {payment_status()}
                }
            }
        }
    }
}

#[component]
fn HorizontalRadioDemo() -> Element {
    let theme = use_signal(|| Some("light".to_string()));

    rsx! {
        RadioGroup {
            value: ReadSignal::new(theme),
            default_value: "light".to_string(),
            horizontal: ReadSignal::new(Signal::new(true)),
            on_value_change: Callback::new({
                let mut theme = theme;
                move |v: String| theme.set(Some(v))
            }),
            RadioItem {
                value: ReadSignal::new(Signal::new("light".to_string())),
                index: ReadSignal::new(Signal::new(0)),
                span { "浅色" }
            }
            RadioItem {
                value: ReadSignal::new(Signal::new("dark".to_string())),
                index: ReadSignal::new(Signal::new(1)),
                span { "深色" }
            }
            RadioItem {
                value: ReadSignal::new(Signal::new("auto".to_string())),
                index: ReadSignal::new(Signal::new(2)),
                span { "自动" }
            }
        }
    }
}

#[component]
fn DisabledRadioDemo() -> Element {
    let status = use_signal(|| Some("normal".to_string()));

    rsx! {
        div {
            class: "flex flex-col gap-3",
            RadioGroup {
                value: ReadSignal::new(status),
                default_value: "normal".to_string(),
                on_value_change: Callback::new({
                    let mut status = status;
                    move |v: String| status.set(Some(v))
                }),
                RadioItem {
                    value: ReadSignal::new(Signal::new("normal".to_string())),
                    index: ReadSignal::new(Signal::new(0)),
                    disabled: ReadSignal::new(Signal::new(false)),
                    span { "正常状态 - 可以点击" }
                }
                RadioItem {
                    value: ReadSignal::new(Signal::new("premium".to_string())),
                    index: ReadSignal::new(Signal::new(1)),
                    disabled: ReadSignal::new(Signal::new(false)),
                    span { "高级状态 - 可以点击" }
                }
                RadioItem {
                    value: ReadSignal::new(Signal::new("disabled".to_string())),
                    index: ReadSignal::new(Signal::new(2)),
                    disabled: ReadSignal::new(Signal::new(true)),
                    span { "暂不可用 - 已禁用" }
                }
            }
        }
    }
}
