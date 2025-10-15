use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::layout::{Breakpoint, ResponsiveSize, Space, SpaceDirection};
use helixui::components::DemoBox;
use helixui::tokens::SpacingToken;

#[component]
pub fn SpacePage() -> Element {
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
            id: "spacing-token".to_string(),
            title: "使用 Spacing Token".to_string(),
            level: 1,
        },
        TocItem {
            id: "responsive".to_string(),
            title: "响应式尺寸".to_string(),
            level: 1,
        },
        TocItem {
            id: "wrap".to_string(),
            title: "换行".to_string(),
            level: 1,
        },
        TocItem {
            id: "split".to_string(),
            title: "分隔符".to_string(),
            level: 1,
        },
    ];

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items,
            div { class: "doc-page",
                h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-4 transition-colors", "Space 间距" }
                p { class: "text-lg text-gray-600 dark:text-gray-300 mb-8 transition-colors", "控制一组子元素间的间距、对齐与换行，支持分隔符。" }

                // 基础：横向
                section { id: "basic-horizontal", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "基础 · 横向" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "默认横向排列，通过 size 控制间距。" }
                    DemoBox {
                        title: "横向间距".to_string(),
                        description: "size: 12px".to_string(),
                        code: r#"use helixui::components::layout::{Space, SpaceDirection};

rsx! {
    Space { direction: SpaceDirection::Horizontal, size: Some(12),
        div { "A" }
        div { "B" }
        div { "C" }
    }
}"#.to_string(),
                        div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                            Space { direction: SpaceDirection::Horizontal, size: Some(12),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "A" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "B" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "C" }
                            }
                        }
                    }
                }

                // 基础：纵向
                section { id: "basic-vertical", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "基础 · 纵向" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "纵向堆叠，通过 size 控制间距。" }
                    DemoBox {
                        title: "纵向间距".to_string(),
                        description: "size: 8px".to_string(),
                        code: r#"use helixui::components::layout::{Space, SpaceDirection};

rsx! {
    Space { direction: SpaceDirection::Vertical, size: Some(8),
        div { "1" }
        div { "2" }
        div { "3" }
    }
}"#.to_string(),
                        div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                            Space { direction: SpaceDirection::Vertical, size: Some(8),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "1" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "2" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "3" }
                            }
                        }
                    }
                }

                // 使用 Spacing Token
                section { id: "spacing-token", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "使用 Spacing Token" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "使用设计令牌中的间距值。" }
                    DemoBox {
                        title: "Spacing Token".to_string(),
                        description: "spacing_token: Lg".to_string(),
                        code: r#"use helixui::components::layout::{Space, SpaceDirection, SpacingToken};

rsx! {
    Space { direction: SpaceDirection::Horizontal, spacing_token: Some(SpacingToken::Lg),
        div { "A" }
        div { "B" }
        div { "C" }
    }
}"#.to_string(),
                        div {
                            Space { direction: SpaceDirection::Horizontal, spacing_token: Some(SpacingToken::Lg),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "A" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "B" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "C" }
                            }
                        }
                    }
                }

                // 响应式尺寸
                section { id: "responsive", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "响应式尺寸" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "根据当前断点自动调整间距。" }
                    DemoBox {
                        title: "响应式间距".to_string(),
                        description: "sm: 4px, md: 8px, lg: 16px".to_string(),
                        code: r#"use helixui::components::layout::{Space, SpaceDirection, ResponsiveSize};

rsx! {
    Space { 
        direction: SpaceDirection::Horizontal, 
        responsive_size: Some(ResponsiveSize { sm: Some(4), md: Some(8), lg: Some(16), xl: None, xxl: None }),
        div { "A" }
        div { "B" }
        div { "C" }
    }
}"#.to_string(),
                        div {
                            Space {
                                direction: SpaceDirection::Horizontal,
                                responsive_size: Some({
                                    let mut resp = ResponsiveSize::new();
                                    resp.insert(Breakpoint::Sm, 4);
                                    resp.insert(Breakpoint::Md, 8);
                                    resp.insert(Breakpoint::Lg, 16);
                                    resp
                                }),
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "A" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "B" }
                                div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "C" }
                            }
                        }
                    }
                }

                // 换行
                section { id: "wrap", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "换行" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "开启 wrap 后，子元素在容器宽度不足时会换行。" }
                    DemoBox {
                        title: "横向换行".to_string(),
                        description: "wrap: true".to_string(),
                        code: r#"use helixui::components::layout::{Space, SpaceDirection};

rsx! {
    Space { direction: SpaceDirection::Horizontal, wrap: true, size: Some(8),
        for i in 0..8 {
            div { "Item {i}" }
        }
    }
}"#.to_string(),
                        div { class: "space-y-4",
                            div { class: "border border-gray-200 dark:border-gray-700 rounded p-4 max-w-xs",
                                Space { direction: SpaceDirection::Horizontal, wrap: true, size: Some(8),
                                    for i in 0..8 {
                                        div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300 whitespace-nowrap", "Item {i}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // 分隔符
                section { id: "split", class: "mb-12",
                    h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors", "分隔符" }
                    p { class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors", "在子元素间插入分隔符。" }
                    DemoBox {
                        title: "带分隔符".to_string(),
                        description: "split: 分隔符文本".to_string(),
                        code: r#"use helixui::components::layout::{Space, SpaceDirection};

rsx! {
    Space { 
        direction: SpaceDirection::Horizontal, 
        split: Some("|".to_string()),
        div { "A" }
        div { "B" }
        div { "C" }
    }
}"#.to_string(),
                        div { class: "space-y-4",
                            div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                                h4 { class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "横向分隔符" }
                                Space {
                                    direction: SpaceDirection::Horizontal,
                                    split: Some("|".to_string()),
                                    div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "A" }
                                    div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "B" }
                                    div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "C" }
                                }
                            }
                            div { class: "border border-gray-200 dark:border-gray-700 rounded p-4",
                                h4 { class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-2", "纵向分隔符" }
                                Space {
                                    direction: SpaceDirection::Vertical,
                                    split: Some("-".to_string()),
                                    div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "1" }
                                    div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "2" }
                                    div { class: "px-3 py-2 bg-gray-100 dark:bg-gray-800 rounded text-gray-700 dark:text-gray-300", "3" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
