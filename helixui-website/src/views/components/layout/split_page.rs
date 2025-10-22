use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::layout::{Split, SplitDirection};
use helixui::components::DemoBox;

#[component]
pub fn SplitPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic-horizontal".to_string(),
            title: "基础 · 横向".to_string(),
            level: 1,
        },
        TocItem {
            id: "basic-vertical".to_string(),
            title: "基础 · 纵向".to_string(),
            level: 1,
        },
        TocItem {
            id: "min-sizes".to_string(),
            title: "最小尺寸".to_string(),
            level: 1,
        },
        TocItem {
            id: "disabled".to_string(),
            title: "禁用拖拽".to_string(),
            level: 1,
        },
        TocItem {
            id: "custom-sizes".to_string(),
            title: "自定义比例".to_string(),
            level: 1,
        },
    ];

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items,
            div { class: "doc-page",
                h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-4 transition-colors", "Split 分割面板" }
                p { class: "text-lg text-gray-600 dark:text-gray-300 mb-8 transition-colors", "将空间拆分为可调整的面板区域，支持水平/垂直方向、初始尺寸与最小尺寸。" }

                // 基础：横向
                section { id: "basic-horizontal", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "基础 · 横向" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "水平分割，两个面板按比例排列。" }
                    DemoBox {
                        title: "水平分割".to_string(),
                        description: "direction: Row, initial_sizes: [300, 300]".to_string(),
                        code: r#"use helixui::components::layout::{Split, SplitDirection};

rsx! {
    Split { direction: SplitDirection::Row, initial_sizes: vec![300.0, 300.0],
        panel1: rsx! { div { "左侧面板" } },
        panel2: rsx! { div { "右侧面板" } },
    }
}"#.to_string(),
                        div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                            div { class: "w-full h-24",
                                Split { direction: SplitDirection::Row, initial_sizes: vec![300.0, 300.0],
                                    panel1: rsx! { div { class: "px-3 py-6 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300 text-center h-full flex items-center justify-center", "左侧面板" } },
                                    panel2: rsx! { div { class: "px-3 py-6 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300 text-center h-full flex items-center justify-center", "右侧面板" } },
                                }
                            }
                        }
                    }
                }

                // 基础：纵向
                section { id: "basic-vertical", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "基础 · 纵向" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "垂直分割，上下两个面板按比例排列。" }
                    DemoBox {
                        title: "垂直分割".to_string(),
                        description: "direction: Column, initial_sizes: [40, 56]".to_string(),
                        code: r#"use helixui::components::layout::{Split, SplitDirection};

rsx! {
    Split { direction: SplitDirection::Column, initial_sizes: vec![40.0, 56.0],
        panel1: rsx! { div { "上方面板" } },
        panel2: rsx! { div { "下方面板" } },
    }
}"#.to_string(),
                        div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                            div { class: "w-full h-24",
                                Split { direction: SplitDirection::Column, initial_sizes: vec![40.0, 56.0],
                                    panel1: rsx! { div { class: "px-3 py-6 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300 text-center h-full flex items-center justify-center", "上方面板" } },
                                    panel2: rsx! { div { class: "px-3 py-6 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300 text-center h-full flex items-center justify-center", "下方面板" } },
                                }
                            }
                        }
                    }
                }
                // 最小尺寸
                section { id: "min-sizes", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "最小尺寸" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "为每个面板设置最小像素尺寸，防止被拖至过小。" }
                    DemoBox {
                        title: "设置最小尺寸".to_string(),
                        description: "min_sizes: [120, 120]".to_string(),
                        code: r#"use helixui::components::layout::{Split, SplitDirection};

rsx! {
    Split { direction: SplitDirection::Row, initial_sizes: vec![360.0, 240.0], min_sizes: vec![120.0, 120.0],
        panel1: rsx! { div { "左侧面板" } },
        panel2: rsx! { div { "右侧面板" } },
    }
}"#.to_string(),
                        div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                            div { class: "w-full h-24",
                                Split { direction: SplitDirection::Row, initial_sizes: vec![360.0, 240.0], min_sizes: vec![120.0, 120.0],
                                    panel1: rsx! { div { class: "px-3 py-6 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300 text-center h-full flex items-center justify-center", "左侧面板" } },
                                    panel2: rsx! { div { class: "px-3 py-6 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300 text-center h-full flex items-center justify-center", "右侧面板" } },
                                }
                            }
                        }
                    }
                }

                // 禁用拖拽
                section { id: "disabled", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "禁用拖拽" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "在某些场景下可暂时禁用拖拽交互。" }
                    DemoBox {
                        title: "禁用状态".to_string(),
                        description: "disabled: true".to_string(),
                        code: r#"use helixui::components::layout::{Split, SplitDirection};

rsx! {
    Split { direction: SplitDirection::Row, initial_sizes: vec![240.0, 560.0], disabled: true,
        panel1: rsx! { div { "左侧" } },
        panel2: rsx! { div { "右侧" } },
    }
}"#.to_string(),
                        div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                            div { class: "w-full h-24",
                                Split { direction: SplitDirection::Row, initial_sizes: vec![300.0, 300.0], disabled: true,
                                    panel1: rsx! { div { class: "bg-gray-50 dark:bg-gray-800/50 text-gray-500 dark:text-gray-400 text-center flex items-center justify-center text-sm font-medium", "左侧" } },
                                    panel2: rsx! { div { class: "bg-gray-50 dark:bg-gray-800/50 text-gray-500 dark:text-gray-400 text-center flex items-center justify-center text-sm font-medium", "右侧" } },
                                }
                            }
                        }
                    }
                }

                // 自定义比例
                section { id: "custom-sizes", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "自定义比例" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "通过 initial_sizes 指定任意比例。" }
                    DemoBox {
                        title: "30% / 70%".to_string(),
                        description: "initial_sizes: [300, 300]".to_string(),
                        code: r#"use helixui::components::layout::{Split, SplitDirection};

rsx! {
    Split { direction: SplitDirection::Row, initial_sizes: vec![300.0, 300.0],
        panel1: rsx! { div { "30%" } },
        panel2: rsx! { div { "70%" } },
    }
}"#.to_string(),
                        div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                            div { class: "w-full h-24",
                                Split { direction: SplitDirection::Row, initial_sizes: vec![300.0, 300.0],
                                    panel1: rsx! { div { class: "px-3 py-6 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300 text-center h-full flex items-center justify-center", "30%" } },
                                    panel2: rsx! { div { class: "px-3 py-6 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300 text-center h-full flex items-center justify-center", "70%" } },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
