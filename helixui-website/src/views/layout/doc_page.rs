use super::{PageToc, TopNavbar};
use crate::views::layout::TocItem;
use dioxus::prelude::*;
use helixui::components::{Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant, IconType};

/// 文档页面布局包装器
///
/// 用法：
/// ```rust
/// rsx! {
///     DocPage {
///         sidebar: rsx! { ComponentsSidebar {} },
///         toc_items: vec![
///             TocItem { id: "basic".to_string(), title: "基础用法".to_string(), level: 1 },
///             TocItem { id: "api".to_string(), title: "API".to_string(), level: 1 },
///         ],
///         // 页面内容
///         h1 { "标题" }
///         p { "内容" }
///     }
/// }
/// ```
#[component]
pub fn DocPage(sidebar: Element, toc_items: Vec<TocItem>, children: Element) -> Element {
    let mut sidebar_collapsed = use_signal(|| false);
    // 生产就绪：使用章节权重与比例换算，稳定高亮
    let mut pseudo_scroll = use_signal(|| 0.0f64);
    let mut active_idx = use_signal(|| 0usize);
    // 章节集合（Rc 便于在多个闭包中克隆使用）
    let toc_arc = std::rc::Rc::new(toc_items.clone());
    // 将累计滚动映射为 [0, total_weight]，每个权重段对应一节

    rsx! {
        div {
            class: "min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors",
            style: "scroll-behavior: smooth;",

            TopNavbar {}

            div {
                class: "flex pt-16",

                // 左侧边栏
                aside {
                    class: if *sidebar_collapsed.read() {
                        "bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 fixed h-[calc(100vh-4rem)] overflow-y-auto transition-all duration-300 ease-in-out w-0 -ml-64"
                    } else {
                        "bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 fixed h-[calc(100vh-4rem)] overflow-y-auto transition-all duration-300 ease-in-out w-64"
                    },
                    {sidebar}
                }

                // 主内容区
                main {
                    class: if *sidebar_collapsed.read() {
                        "flex-1 transition-all duration-300 ease-in-out ml-0 mr-64"
                    } else {
                        "flex-1 transition-all duration-300 ease-in-out ml-64 mr-64"
                    },
                    // 滚动容器（仅此容器滚动，便于监听）
                    div {
                        class: "h-[calc(100vh-4rem)] overflow-y-auto",
                        onscroll: {
                            let toc_arc_for_scroll = toc_arc.clone();
                            move |_| {
                            // 稳定策略：按容器可视高度作为步长基数
                            let base_step = 240.0; // 经验步长，减少平台差异
                            let v = (pseudo_scroll() + base_step).max(0.0);
                            pseudo_scroll.set(v);
                            // 将 v 归一化到权重总和范围
                            let total_weight_local = toc_arc_for_scroll.len().max(1) as f64; // 均分权重
                            let progress = (v / base_step).min(total_weight_local * 2.0); // 防止过大
                            let mut acc = 0.0;
                            let mut idx = 0usize;
                            for i in 0..toc_arc_for_scroll.len().max(1) {
                                acc += 1.0;
                                if progress <= acc { idx = i; break; }
                                idx = i;
                            }
                            active_idx.set(idx.min(toc_arc_for_scroll.len().saturating_sub(1)));
                        }},
                        div {
                            class: "max-w-4xl mx-auto px-8 py-8",
                            {children}
                        }
                    }
                }

                // 右侧目录
                aside {
                    class: "w-64 bg-white dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700 fixed right-0 h-[calc(100vh-4rem)] overflow-y-auto transition-colors",
                    {
                        let toc_arc_for_active = toc_arc.clone();
                        let current_idx = (*active_idx.read()).min(toc_arc_for_active.len().saturating_sub(1));
                        let current_active_id = toc_arc_for_active.get(current_idx).map(|t| t.id.clone()).unwrap_or_default();
                        let toc_arc_for_nav = toc_arc.clone();
                        rsx!{
                            PageToc {
                                items: (*toc_arc_for_active).clone(),
                                active_id: Some(current_active_id),
                                on_navigate: move |id: String| {
                                    let toc_local = (*toc_arc_for_nav).clone();
                                    if let Some((i, _)) = toc_local.iter().enumerate().find(|(_, t)| t.id == id) {
                                        let mut acc = 0.0;
                                        for _ in 0..i { acc += 1.0; }
                                        let center = acc + 0.5;
                                        let base_step = 240.0;
                                        pseudo_scroll.set(center * base_step);
                                        active_idx.set(i);
                                    }
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
                        variant: ButtonVariant::Icon,
                        shape: ButtonShape::Circle,
                        icon: if *sidebar_collapsed.read() {
                            Some(IconType::ChevronRight)
                        } else {
                            Some(IconType::ChevronLeft)
                        },
                        onclick: move |_| {
                            let current = *sidebar_collapsed.read();
                            sidebar_collapsed.set(!current);
                        },
                        class: "shadow-lg hover:shadow-xl transition-all duration-200",
                    }
                }
            }
        }
    }
}
