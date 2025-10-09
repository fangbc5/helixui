use dioxus::prelude::*;

/// 右侧页面目录
#[component]
pub fn PageToc() -> Element {
    rsx! {
        div {
            class: "p-4",
            h3 {
                class: "px-2 mb-4 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider",
                "目录"
            }
            nav {
                class: "space-y-2",
                a { href: "#basic", class: "block px-2 py-1 text-sm text-gray-600 dark:text-gray-300 hover:text-green-600 dark:hover:text-green-400 transition-colors", "基础" }
                a { href: "#secondary", class: "block px-2 py-1 text-sm text-gray-600 dark:text-gray-300 hover:text-green-600 dark:hover:text-green-400 transition-colors", "次要按钮" }
                a { href: "#size", class: "block px-2 py-1 text-sm text-gray-600 dark:text-gray-300 hover:text-green-600 dark:hover:text-green-400 transition-colors", "尺寸" }
                a { href: "#disabled", class: "block px-2 py-1 text-sm text-gray-600 dark:text-gray-300 hover:text-green-600 dark:hover:text-green-400 transition-colors", "禁用" }
                a { href: "#api", class: "block px-2 py-1 text-sm text-gray-600 dark:text-gray-300 hover:text-green-600 dark:hover:text-green-400 transition-colors", "API" }
            }
        }
    }
}
