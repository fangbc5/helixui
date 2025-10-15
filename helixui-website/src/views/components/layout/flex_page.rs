use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::layout::{
    AlignItems, Flex, FlexDirection, Justify, Space, SpaceDirection, SpaceSize,
};
use helixui::components::DemoBox;

#[component]
pub fn FlexPage() -> Element {
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
            id: "wrap".to_string(),
            title: "换行".to_string(),
            level: 1,
        },
        TocItem {
            id: "with-space".to_string(),
            title: "配合 Space".to_string(),
            level: 1,
        },
        TocItem {
            id: "justify-center".to_string(),
            title: "主轴居中".to_string(),
            level: 1,
        },
        TocItem {
            id: "justify-end".to_string(),
            title: "从尾部对齐".to_string(),
            level: 1,
        },
        TocItem {
            id: "justify-around".to_string(),
            title: "空间环绕".to_string(),
            level: 1,
        },
        TocItem {
            id: "justify-between".to_string(),
            title: "两端对齐（space-between）".to_string(),
            level: 1,
        },
        TocItem {
            id: "justify-evenly".to_string(),
            title: "均匀分布（space-evenly）".to_string(),
            level: 1,
        },
        TocItem {
            id: "align-items".to_string(),
            title: "交叉轴对齐（align-items）".to_string(),
            level: 1,
        },
    ];

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items,
            div { class: "doc-page",
                h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-4 transition-colors", "Flex 弹性布局" }
                p { class: "text-lg text-gray-600 dark:text-gray-300 mb-8 transition-colors", "基于 CSS Flex 的一维布局容器，支持横向/纵向与换行。" }

                section { id: "basic-horizontal", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "基础 · 横向" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "默认横向排列，可通过内联样式演示 gap（后续将接入 tokens）。" }
                    DemoBox {
                        title: "横向排列".to_string(),
                        description: "Row 方向，设置 8px 间距".to_string(),
                        code: r#"use helixui::components::layout::{Flex, FlexDirection};

rsx! {
    Flex { direction: FlexDirection::Row, style: Some("gap: 8px;".to_string()),
        div { "A" }
        div { "B" }
        div { "C" }
    }
}"#.to_string(),
                        div { class: "",
                            Flex { direction: FlexDirection::Row, style: Some("gap: 8px;".to_string()),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "A" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "B" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "C" }
                            }
                        }
                    }
                }

                section { id: "basic-vertical", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "基础 · 纵向" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "Column 方向纵向堆叠。" }
                    DemoBox {
                        title: "纵向堆叠".to_string(),
                        description: "Column 方向，6px 间距".to_string(),
                        code: r#"use helixui::components::layout::{Flex, FlexDirection};

rsx! {
    Flex { direction: FlexDirection::Column, style: Some("gap: 6px;".to_string()),
        div { "1" }
        div { "2" }
        div { "3" }
    }
}"#.to_string(),
                        div {
                            Flex { direction: FlexDirection::Column, style: Some("gap: 6px;".to_string()),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "1" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "2" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "3" }
                            }
                        }
                    }
                }

                section { id: "wrap", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "换行" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "开启 wrap 后，子项在容器宽度不足时会换行。" }
                    DemoBox {
                        title: "横向换行".to_string(),
                        description: "Row + wrap".to_string(),
                        code: r#"use helixui::components::layout::{Flex, FlexDirection};

rsx! {
    Flex { direction: FlexDirection::Row, wrap: true, gap: Some(8),
        for i in 0..8 {
            div { "Item {i}" }
        }
    }
}"#.to_string(),
                        div { class: "space-y-4",
                            div { class: "border border-gray-200 dark:border-gray-700 rounded p-4 max-w-xs",
                                Flex { direction: FlexDirection::Row, wrap: true, gap: Some(8),
                                    for i in 0..8 {
                                        div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300 whitespace-nowrap", "Item {i}" }
                                    }
                                }
                            }
                        }
                    }
                }

                section { id: "with-space", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "配合 Space" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "使用 Space 控制一致的间距。" }
                    DemoBox {
                        title: "Space 控制间距".to_string(),
                        description: "Horizontal 间距 12px".to_string(),
                        code: r#"use helixui::components::layout::{Space, SpaceDirection};

rsx! {
    Space { direction: SpaceDirection::Horizontal, size: Some(12),
        div { "1" }
        div { "2" }
        div { "3" }
    }
}"#.to_string(),
                        div {
                            Space { direction: SpaceDirection::Horizontal, size: Some(SpaceSize::Single(12)),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "1" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "2" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "3" }
                            }
                        }
                    }
                }

                // 对齐：主轴居中
                section { id: "justify-center", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "主轴居中" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "通过设置 justify-content: center 实现主轴居中（后续提供 props 封装）。" }
                    DemoBox {
                        title: "居中".to_string(),
                        description: "justify-content: center".to_string(),
                        code: r#"use helixui::components::layout::{Flex, FlexDirection, Justify};

rsx! {
    Flex { direction: FlexDirection::Row, justify: Some(Justify::Center), gap: Some(8),
        div { "A" }
        div { "B" }
        div { "C" }
    }
}"#.to_string(),
                        div {
                            Flex { direction: FlexDirection::Row, justify: Some(Justify::Center), gap: Some(8),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "A" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "B" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "C" }
                            }
                        }
                    }
                }

                // 对齐：从尾部
                section { id: "justify-end", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "从尾部对齐" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "通过设置 justify-content: flex-end 实现。" }
                    DemoBox {
                        title: "尾部对齐".to_string(),
                        description: "justify-content: flex-end".to_string(),
                        code: r#"use helixui::components::layout::{Flex, FlexDirection, Justify};

rsx! {
    Flex { direction: FlexDirection::Row, justify: Some(Justify::End), gap: Some(8),
        div { "A" }
        div { "B" }
        div { "C" }
    }
}"#.to_string(),
                        div {
                            Flex { direction: FlexDirection::Row, justify: Some(Justify::End), gap: Some(8),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "A" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "B" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "C" }
                            }
                        }
                    }
                }

                // 对齐：空间环绕
                section { id: "justify-around", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "空间环绕" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "通过设置 justify-content: space-around 实现。" }
                    DemoBox {
                        title: "空间环绕".to_string(),
                        description: "justify-content: space-around".to_string(),
                        code: r#"use helixui::components::layout::{Flex, FlexDirection, Justify};

rsx! {
    Flex { direction: FlexDirection::Row, justify: Some(Justify::SpaceAround),
        div { "A" }
        div { "B" }
        div { "C" }
    }
}"#.to_string(),
                        div {
                            Flex { direction: FlexDirection::Row, justify: Some(Justify::SpaceAround),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "A" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "B" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "C" }
                            }
                        }
                    }
                }

                // 对齐：两端对齐（space-between）
                section { id: "justify-between", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "两端对齐（space-between）" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "通过设置 justify-content: space-between 实现。" }
                    DemoBox {
                        title: "space-between".to_string(),
                        description: "justify-content: space-between".to_string(),
                        code: r#"use helixui::components::layout::{Flex, FlexDirection, Justify};

rsx! {
    Flex { direction: FlexDirection::Row, justify: Some(Justify::SpaceBetween),
        div { "A" }
        div { "B" }
        div { "C" }
    }
}"#.to_string(),
                        div {
                            Flex { direction: FlexDirection::Row, justify: Some(Justify::SpaceBetween),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "A" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "B" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "C" }
                            }
                        }
                    }
                }

                // 对齐：均匀分布（space-evenly）
                section { id: "justify-evenly", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "均匀分布（space-evenly）" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "通过设置 justify-content: space-evenly 实现。" }
                    DemoBox {
                        title: "space-evenly".to_string(),
                        description: "justify-content: space-evenly".to_string(),
                        code: r#"use helixui::components::layout::{Flex, FlexDirection, Justify};

rsx! {
    Flex { direction: FlexDirection::Row, justify: Some(Justify::SpaceEvenly),
        div { "A" }
        div { "B" }
        div { "C" }
    }
}"#.to_string(),
                        div {
                            Flex { direction: FlexDirection::Row, justify: Some(Justify::SpaceEvenly),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "A" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "B" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "C" }
                            }
                        }
                    }
                }

                // 交叉轴对齐（align-items）
                section { id: "align-items", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "交叉轴对齐（align-items）" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "通过设置 align-items 控制交叉轴对齐方式。" }
                    DemoBox {
                        title: "align-items: center".to_string(),
                        description: "垂直居中对齐".to_string(),
                        code: r#"use helixui::components::layout::{Flex, FlexDirection, AlignItems};

rsx! {
    Flex { direction: FlexDirection::Row, align: Some(AlignItems::Center), style: Some("height: 80px; gap: 8px;".to_string()),
        div { "A" }
        div { "B" }
        div { "C" }
    }
}"#.to_string(),
                        div {
                            Flex { direction: FlexDirection::Row, align: Some(AlignItems::Center), style: Some("height: 80px; gap: 8px;".to_string()),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "A" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "B" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "C" }
                            }
                        }
                    }
                }
            }
        }
    }
}
