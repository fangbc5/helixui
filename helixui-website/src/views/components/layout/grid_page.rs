use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::layout::{Breakpoint, Grid, GridItem};
use helixui::components::DemoBox;
use std::collections::HashMap;

#[component]
pub fn GridPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "grid-basic".to_string(),
            title: "基础栅格".to_string(),
            level: 1,
        },
        TocItem {
            id: "grid-responsive".to_string(),
            title: "响应式栅格".to_string(),
            level: 1,
        },
        TocItem {
            id: "grid-offset".to_string(),
            title: "偏移与组合".to_string(),
            level: 1,
        },
        TocItem {
            id: "api".to_string(),
            title: "API".to_string(),
            level: 1,
        },
    ];

    // 预置一个响应式列数映射用于展示代码片段
    let mut responsive_cols = HashMap::new();
    responsive_cols.insert(Breakpoint::Sm, 12);
    responsive_cols.insert(Breakpoint::Md, 16);
    responsive_cols.insert(Breakpoint::Lg, 24);

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items: toc_items,

            // 基础栅格
            section { id: "grid-basic", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "基础栅格" }
                p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "基于 24 列的栅格系统，使用 span 指定占用列数。" }

                DemoBox {
                    title: "24 列基础".to_string(),
                    description: "Grid + GridItem(span)".to_string(),
                    code: r#"use helixui::components::layout::{Grid, GridItem};

rsx! {
    Grid { cols: 24, x_gap: Some(16), y_gap: Some(16),
        GridItem { span: Some(6), class: Some("bg-blue-100 p-4 rounded".to_string()), "6 列" }
        GridItem { span: Some(12), class: Some("bg-green-100 p-4 rounded".to_string()), "12 列" }
        GridItem { span: Some(6), class: Some("bg-yellow-100 p-4 rounded".to_string()), "6 列" }
    }
}"#.to_string(),
                    div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                        Grid { cols: 24, x_gap: Some(16), y_gap: Some(16),
                            GridItem { span: Some(6), class: Some("bg-blue-100 p-4 rounded text-center".to_string()), "6 列" }
                            GridItem { span: Some(12), class: Some("bg-green-100 p-4 rounded text-center".to_string()), "12 列" }
                            GridItem { span: Some(6), class: Some("bg-yellow-100 p-4 rounded text-center".to_string()), "6 列" }
                        }
                    }
                }
            }

            // 响应式栅格
            section { id: "grid-responsive", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "响应式栅格" }
                p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "不同断点可设置不同列数，合理分配内容布局。" }

                DemoBox {
                    title: "按断点调整列数".to_string(),
                    description: "Sm=12, Md=16, Lg=24".to_string(),
                    code: r#"use helixui::components::layout::{Grid, GridItem, Breakpoint};
use std::collections::HashMap;

let mut responsive_cols = HashMap::new();
responsive_cols.insert(Breakpoint::Sm, 12);
responsive_cols.insert(Breakpoint::Md, 16);
responsive_cols.insert(Breakpoint::Lg, 24);

rsx! {
    Grid { cols: 24, responsive_cols: Some(responsive_cols), x_gap: Some(12), y_gap: Some(12),
        for i in 1..=6 {
            GridItem { span: Some(4), class: Some("bg-gray-100 p-3 rounded text-center".to_string()), "项目 {i}" }
        }
    }
}"#.to_string(),
                    div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                        Grid { cols: 24, x_gap: Some(12), y_gap: Some(12),
                            for i in 1..=6 {
                                GridItem { span: Some(4), class: Some("bg-gray-100 p-3 rounded text-center".to_string()), "项目 {i}" }
                            }
                        }
                    }
                }
            }

            // 偏移与组合
            section { id: "grid-offset", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "偏移与组合" }
                p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "通过 offset 可以实现内容居中或留白的编排效果。" }

                DemoBox {
                    title: "偏移示例".to_string(),
                    description: "居中两列".to_string(),
                    code: r#"use helixui::components::layout::{Grid, GridItem};

rsx! {
    Grid { cols: 24, x_gap: Some(16), y_gap: Some(12),
        GridItem { span: Some(6), offset: Some(4), class: Some("bg-blue-100 p-4 rounded".to_string()), "6 列(偏移 4)" }
        GridItem { span: Some(6), class: Some("bg-green-100 p-4 rounded".to_string()), "6 列" }
    }
}"#.to_string(),
                    div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                        Grid { cols: 24, x_gap: Some(16), y_gap: Some(12),
                            GridItem { span: Some(6), offset: Some(4), class: Some("bg-blue-100 p-4 rounded text-center".to_string()), "6 列(偏移 4)" }
                            GridItem { span: Some(6), class: Some("bg-green-100 p-4 rounded text-center".to_string()), "6 列" }
                        }
                    }
                }
            }

            // API 文档
            section { id: "api", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "API" }
                div { class: "space-y-6",
                    // Grid Props
                    div {
                        h3 { class: "text-lg font-semibold text-gray-900 dark:text-white mb-2", "Grid Props" }
                        div { class: "overflow-x-auto",
                            table { class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                                thead { class: "bg-gray-50 dark:bg-gray-800",
                                    tr {
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "属性" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "类型" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "默认值" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "说明" }
                                    }
                                }
                                tbody { class: "bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700",
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white", "cols" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "u16" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "24" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "总列数（建议 24）" }
                                    }
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white", "x_gap" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "Option<i32>" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "None" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "列间横向间距（px）" }
                                    }
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white", "y_gap" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "Option<i32>" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "None" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "行间纵向间距（px）" }
                                    }
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white", "responsive_cols" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "Option<HashMap<Breakpoint, u16>>" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "None" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "按断点设置不同列数" }
                                    }
                                }
                            }
                        }
                    }

                    // GridItem Props
                    div {
                        h3 { class: "text-lg font-semibold text-gray-900 dark:text-white mb-2", "GridItem Props" }
                        div { class: "overflow-x-auto",
                            table { class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                                thead { class: "bg-gray-50 dark:bg-gray-800",
                                    tr {
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "属性" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "类型" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "默认值" }
                                        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider", "说明" }
                                    }
                                }
                                tbody { class: "bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700",
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white", "span" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "Option<u16>" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "Some(1)" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "占用列数" }
                                    }
                                    tr {
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900 dark:text-white", "offset" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "Option<u16>" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "None" }
                                        td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-300", "左侧偏移列数" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
