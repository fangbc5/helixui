use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{DemoBox, GradientDirection, GradientText, Table};

#[component]
pub fn GradientTextPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "direction".to_string(),
            title: "方向".to_string(),
            level: 2,
        },
        TocItem {
            id: "colors".to_string(),
            title: "自定义颜色".to_string(),
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

            div { class: "component-doc",
                // 标题
                div { class: "mb-8",
                    h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-2", "渐变文字 GradientText" }
                    p { class: "text-gray-600 dark:text-gray-300", "使用 CSS 线性渐变与背景裁剪实现的渐变文字。" }
                }

                // 基础用法
                section { id: "basic", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "基础用法" }
                    DemoBox {
                        title: "基础用法",
                        description: "默认从绿色到蓝色，方向向右。",
                        code: r#"
                            GradientText {{ "渐变文字" }}
                        "#,
                        children: rsx! {
                            div { class: "text-4xl font-bold",
                                GradientText { "渐变文字" }
                            }
                        }
                    }
                }

                // 方向
                section { id: "direction", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "方向" }
                    DemoBox {
                        title: "方向",
                        description: "支持向右、向左、向下、向上以及自定义角度。",
                        code: r#"
                            GradientText {{ direction: GradientDirection::ToRight, "→ 向右" }}
                            GradientText {{ direction: GradientDirection::ToLeft, "← 向左" }}
                            GradientText {{ direction: GradientDirection::ToBottom, "↓ 向下" }}
                            GradientText {{ direction: GradientDirection::ToTop, "↑ 向上" }}
                            GradientText {{ direction: GradientDirection::Custom("45deg".to_string()), "45°" }}
                        "#,
                        children: rsx! {
                            div { class: "flex gap-6 text-2xl font-semibold",
                                GradientText { direction: GradientDirection::ToRight, "→ 向右" }
                                GradientText { direction: GradientDirection::ToLeft, "← 向左" }
                                GradientText { direction: GradientDirection::ToBottom, "↓ 向下" }
                                GradientText { direction: GradientDirection::ToTop, "↑ 向上" }
                                GradientText { direction: GradientDirection::Custom("45deg".to_string()), "45°" }
                            }
                        }
                    }
                }

                // 自定义颜色
                section { id: "colors", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "自定义颜色" }
                    DemoBox {
                        title: "自定义颜色",
                        description: "通过 from 与 to 设置渐变颜色。",
                        code: r##"
                            GradientText {{ from: "#ef4444".to_string(), to: "#f59e0b".to_string(), "红到橙" }}
                            GradientText {{ from: "#22c55e".to_string(), to: "#06b6d4".to_string(), "绿到青" }}
                            GradientText {{ from: "#a855f7".to_string(), to: "#3b82f6".to_string(), "紫到蓝" }}
                        "##,
                        children: rsx! {
                            div { class: "flex gap-8 text-3xl font-bold",
                                GradientText { from: "#ef4444".to_string(), to: "#f59e0b".to_string(), "红到橙" }
                                GradientText { from: "#22c55e".to_string(), to: "#06b6d4".to_string(), "绿到青" }
                                GradientText { from: "#a855f7".to_string(), to: "#3b82f6".to_string(), "紫到蓝" }
                            }
                        }
                    }
                }

                // API
                section { id: "api", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4", "API" }
                    div { class: "mt-2",
                        Table {
                            headers: Some(vec!["属性".to_string(), "类型".to_string(), "默认值".to_string(), "说明".to_string()]),
                            data: vec![
                                vec!["from".to_string(), "String".to_string(), "#10b981".to_string(), "渐变起始颜色".to_string()],
                                vec!["to".to_string(), "String".to_string(), "#3b82f6".to_string(), "渐变结束颜色".to_string()],
                                vec!["direction".to_string(), "GradientDirection".to_string(), "ToRight".to_string(), "渐变方向或角度".to_string()],
                                vec!["class".to_string(), "Option<String>".to_string(), "None".to_string(), "自定义类名".to_string()],
                                vec!["style".to_string(), "Option<String>".to_string(), "None".to_string(), "自定义样式（会拼接到渐变样式之后）".to_string()],
                            ],
                        }
                    }
                }
            }
        }
    }
}
