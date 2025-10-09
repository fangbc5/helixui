use super::{PageToc, TopNavbar};
use dioxus::prelude::*;

/// 文档页面布局包装器
///
/// 用法：
/// ```rust
/// rsx! {
///     DocPage {
///         sidebar: rsx! { ComponentsSidebar {} },
///         // 页面内容
///         h1 { "标题" }
///         p { "内容" }
///     }
/// }
/// ```
#[component]
pub fn DocPage(sidebar: Element, children: Element) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors",

            TopNavbar {}

            div {
                class: "flex pt-16",

                // 左侧边栏
                aside {
                    class: "w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 fixed h-[calc(100vh-4rem)] overflow-y-auto transition-colors",
                    {sidebar}
                }

                // 主内容区
                main {
                    class: "flex-1 ml-64 mr-64",
                    div {
                        class: "max-w-4xl mx-auto px-8 py-8",
                        {children}
                    }
                }

                // 右侧目录
                aside {
                    class: "w-64 bg-white dark:bg-gray-800 border-l border-gray-200 dark:border-gray-700 fixed right-0 h-[calc(100vh-4rem)] overflow-y-auto transition-colors",
                    PageToc {}
                }
            }
        }
    }
}
