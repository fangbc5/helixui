use crate::{i18n, theme};
use dioxus::prelude::*;

const VERSION: &str = "0.1.0";

/// 统一的顶部导航栏组件
#[component]
pub fn TopNavbar() -> Element {
    let mut language = i18n::LANGUAGE.signal();
    let mut theme_state = theme::THEME.signal();

    // 根据主题状态，在 document 根元素添加或移除 dark 类
    use_effect(move || {
        let current_theme = *theme_state.read();
        let _is_dark = current_theme == theme::Theme::Dark;
        // 主题切换功能暂时禁用，避免 js-sys 依赖问题
        // TODO: 实现跨平台的主题切换功能
    });

    rsx! {
        div {
            // 固定导航栏
            header {
                class: "fixed top-0 left-0 right-0 h-16 bg-white dark:bg-gray-950 border-b border-gray-200 dark:border-gray-800 z-50 transition-colors",
                nav {
                    class: "container mx-auto px-4 h-full flex items-center justify-between",

                // Logo
                div {
                    class: "flex items-center space-x-8",
                    Link {
                        to: crate::Route::Home {},
                        class: "text-xl font-bold text-gray-900 dark:text-white hover:text-green-600 dark:hover:text-green-400 transition-colors",
                        "Helix UI"
                    }

                    // 主导航
                    div {
                        class: "flex items-center space-x-1",
                        NavLink { to: crate::Route::Home {}, label: i18n::t("nav.home") }
                        NavLink { to: crate::Route::Introduction {}, label: i18n::t("nav.docs") }
                        NavLink { to: crate::Route::ComponentsPage {}, label: i18n::t("nav.components") }
                    }
                }

                // 右侧工具栏
                div {
                    class: "flex items-center space-x-4",

                    // 版本号
                    div {
                        class: "text-sm text-gray-500 dark:text-gray-400",
                        "{VERSION}"
                    }

                    // 语言切换
                    button {
                        class: "px-3 py-1 text-sm text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-800 rounded transition-colors",
                        onclick: move |_| {
                            let current = *language.read();
                            *language.write() = current.toggle();
                        },
                        "{(*language.read()).label()}"
                    }

                    // 主题切换
                    button {
                        class: "px-3 py-1 text-sm text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-800 rounded transition-colors",
                        onclick: move |_| {
                            let current = *theme_state.read();
                            *theme_state.write() = current.toggle();
                        },
                        "{(*theme_state.read()).icon()}"
                    }

                    // GitHub 链接
                    a {
                        href: "https://github.com/fangbc5/helixui",
                        target: "_blank",
                        class: "text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors",
                        "GitHub"
                    }
                }
            }
        }

            // 内容容器，为固定导航栏留出空间
            div {
                class: "pt-16 min-h-screen",
                Outlet::<crate::Route> {}
            }
        }
    }
}

#[component]
fn NavLink(to: crate::Route, label: String) -> Element {
    rsx! {
        Link {
            to: to,
            class: "px-4 py-2 text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md transition-colors",
            active_class: "text-gray-900 dark:text-white bg-gray-100 dark:bg-gray-800",
            "{label}"
        }
    }
}
