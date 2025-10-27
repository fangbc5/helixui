use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{DemoBox, TextArea};

/// TextArea 演示页面
#[component]
pub fn TextAreaPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic-usage".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "rows".to_string(),
            title: "不同行数".to_string(),
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
                    description: "文本域的基本用法。",
                    code: r#"TextArea {{ placeholder: Some("请输入多行文本".to_string()), rows: 3 }}"#,
                    children: rsx! {
                        TextAreaDemo {}
                    }
                }
                DemoBox {
                    title: "不同行数",
                    description: "通过 rows 属性控制行数。",
                    code: r#"TextArea {{ rows: 5 }}"#,
                    children: rsx! {
                        div {
                            class: "space-y-4",
                            div {
                                TextArea {
                                    placeholder: Some("3行".to_string()),
                                    rows: 3,
                                }
                            }
                            div {
                                TextArea {
                                    placeholder: Some("5行".to_string()),
                                    rows: 5,
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
fn TextAreaDemo() -> Element {
    let mut value = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "space-y-4",
            TextArea {
                placeholder: Some("请输入多行文本".to_string()),
                value: Some(value()),
                rows: 3,
                on_change: move |new_value| {
                    value.set(new_value);
                },
            }
        }
    }
}
