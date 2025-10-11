use super::{PageToc, TopNavbar};
use helixui::components::{Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant, IconType};
use crate::views::layout::TocItem;
use dioxus::prelude::*;

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
                    div {
                        class: "max-w-4xl mx-auto px-8 py-8",
                        {children}
                    }
                }

                // 右侧目录
                aside {
                    class: "w-64 bg-white dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700 fixed right-0 h-[calc(100vh-4rem)] overflow-y-auto transition-colors",
                    PageToc { items: toc_items }
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
