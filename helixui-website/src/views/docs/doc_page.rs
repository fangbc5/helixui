use crate::views::layout::{PageToc, TocItem};
use dioxus::prelude::*;
use helixui::components::{Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant};

/// Section 结构体，用于滚动高亮
#[derive(PartialEq, Clone)]
struct Section {
    id: String,
    title: String,
    offset_top: f64,
    height: f64,
}

/// 文档页面布局包装器
///
/// 用法：
/// ```rust
/// rsx! {
///     DocPage {
///         sidebar: rsx! { div { "侧边栏" } },
///         toc_items: vec![TocItem { id: "intro".to_string(), title: "介绍".to_string() }],
///         children: rsx! {
///             p { "内容" }
///         }
///     }
/// }
/// ```
#[component]
pub fn DocPage(sidebar: Element, toc_items: Vec<TocItem>, children: Element) -> Element {
    let mut sidebar_collapsed = use_signal(|| false);

    // 跨平台滚动高亮：基于 Section 结构体
    let sections_data = toc_items
        .iter()
        .enumerate()
        .map(|(i, item)| Section {
            id: item.id.clone(),
            title: item.title.clone(),
            offset_top: (i as f64) * 600.0, // 预估位置，实际会被动态更新
            height: 600.0,
        })
        .collect::<Vec<_>>();

    let sections = use_signal(|| sections_data);

    let mut active_index = use_signal(|| 0usize);
    let scroll_counter = use_signal(|| 0u64);

    // 简化的滚动处理函数（不使用时间节流）
    let mut on_scroll = {
        let sections = sections.clone();
        let mut active_index = active_index.clone();
        let mut scroll_counter = scroll_counter.clone();
        move |scroll_top: f64| {
            // 简单的计数器节流
            scroll_counter.set(scroll_counter() + 1);
            if scroll_counter() % 3 != 0 {
                return;
            }

            // 找出当前活跃 section
            let mut idx = 0;
            for (i, section) in sections.read().iter().enumerate() {
                if scroll_top >= section.offset_top {
                    idx = i;
                } else {
                    break;
                }
            }

            if active_index() != idx {
                active_index.set(idx);
            }
        }
    };

    rsx! {
        div {
            class: "min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors",
            style: "scroll-behavior: smooth;",

            div {
                class: "flex",

                // 左侧边栏
                if !*sidebar_collapsed.read() {
                    aside {
                        class: "w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 h-[calc(100vh-4rem)] overflow-y-auto transition-colors z-40",
                        {sidebar}
                    }
                }

                // 主内容区域
                main {
                    class: if *sidebar_collapsed.read() {
                        "flex-1 transition-all duration-300 ease-in-out mr-64"
                    } else {
                        "flex-1 transition-all duration-300 ease-in-out mr-64"
                    },

                    // 滚动容器（仅此容器滚动，便于监听）
                    div {
                        class: "h-[calc(100vh-4rem)] overflow-y-auto",
                        onscroll: move |_e| {
                            // 临时方案：使用简化的滚动位置计算
                            // TODO: 需要根据实际的 Dioxus 0.6 API 调整
                            let scroll_top = 0.0; // 临时值，实际应该从事件中获取
                            on_scroll(scroll_top);
                        },

                        div {
                            class: "max-w-4xl mx-auto px-8 py-8",
                            {children}
                        }
                    }
                }

                // 右侧目录
                aside {
                    class: "w-64 bg-white dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700 fixed right-0 h-[calc(100vh-4rem)] overflow-y-auto transition-colors",
                    PageToc {
                        items: toc_items.clone(),
                        active_id: {
                            let current_idx = active_index().min(sections.read().len().saturating_sub(1));
                            sections.read().get(current_idx).map(|s| s.id.clone())
                        },
                        on_navigate: move |id: String| {
                            // 点击目录时滚动到对应 section
                            if let Some((idx, _)) = sections.read().iter().enumerate().find(|(_, s)| s.id == id) {
                                // 这里可以添加滚动到指定位置的逻辑
                                // 由于我们移除了 web-sys 依赖，这里使用简化的处理
                                active_index.set(idx);
                            }
                        }
                    }
                }

                // 侧边栏切换按钮
                div {
                    class: if *sidebar_collapsed.read() {
                        "fixed left-4 top-1/2 transform -translate-y-1/2 z-50"
                    } else {
                        "fixed left-60 top-1/2 transform -translate-y-1/2 z-50"
                    },
                    Button {
                        button_type: ButtonType::Default,
                        size: ButtonSize::Small,
                        shape: ButtonShape::Circle,
                        variant: ButtonVariant::Icon,
                        onclick: move |_| {
                            sidebar_collapsed.set(!sidebar_collapsed());
                        },
                        "☰"
                    }
                }
            }
        }
    }
}
