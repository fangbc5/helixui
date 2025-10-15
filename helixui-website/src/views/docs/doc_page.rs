use crate::views::layout::{PageToc, TocItem};
use dioxus::prelude::*;
use helixui::components::{Button, ButtonShape, ButtonSize, ButtonType, Icon, IconType};

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
    // Desktop 等无法使用锚点时，用占位高度完成“跳转”
    let mut jump_spacer_px = use_signal(|| 0.0f64);

    rsx! {
        div {
            class: "h-screen bg-gray-50 dark:bg-gray-900 transition-colors overflow-hidden",
            style: "scroll-behavior: smooth; overscroll-behavior: none;",

            div {
                class: "flex h-full",

                // 左侧边栏（固定在视口，独立滚动）
                if !*sidebar_collapsed.read() {
                    aside {
                        class: "fixed left-0 top-16 w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 h-[calc(100vh-4rem)] overflow-y-auto transition-colors z-40",
                        {sidebar}
                    }
                }

                // 主内容区域（页面滚动，不再内部滚动）
                main {
                    class: if *sidebar_collapsed.read() {
                        "flex-1 transition-all duration-300 ease-in-out mr-64 h-full overflow-y-auto"
                    } else {
                        "flex-1 transition-all duration-300 ease-in-out ml-64 mr-64 h-full overflow-y-auto"
                    },
                    style: "overscroll-behavior: contain;",
                    div {
                        class: "max-w-4xl mx-auto px-8 py-8 space-y-12 [&_*]:scroll-mt-16 min-h-full",
                        // 用一个可控的占位区在顶部，作为非 Web 平台的"滚动跳转"方案
                        div {
                            style: format!("height: {}px;", jump_spacer_px()),
                            class: "transition-all duration-300 ease-in-out"
                        }
                        {children}
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
                                // 计算需要的占位高度（使用预估 offset_top，并考虑 Topbar 高度）
                                if let Some(sec) = sections.read().get(idx) {
                                    let topbar_px: f64 = 64.0; // 4rem
                                    let mut target = (sec.offset_top - topbar_px).max(0.0);
                                    // 将占位高度限制在内容总高度范围内，避免出现“无限向上滚动”的大空白
                                    let total_height: f64 = sections.read().iter().map(|s| s.height).sum();
                                    if total_height > 0.0 {
                                        let max_spacer = (total_height - topbar_px).max(0.0);
                                        if target > max_spacer { target = max_spacer; }
                                    }
                                    jump_spacer_px.set(target);
                                }
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
                        onclick: move |_| {
                            sidebar_collapsed.set(!sidebar_collapsed());
                        },
                        if *sidebar_collapsed.read() { Icon { icon: IconType::ChevronRight } } else { Icon { icon: IconType::ChevronLeft } }
                    }
                }
            }
        }
    }
}
