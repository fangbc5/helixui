use crate::i18n;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-b from-green-50 to-white dark:from-gray-900 dark:to-gray-950 pt-16 transition-colors",

            // Hero 区域
            div {
                class: "container mx-auto px-4 py-32 text-center",

                h1 {
                    class: "text-6xl font-bold text-gray-900 dark:text-white mb-4",
                    "{i18n::t(\"home.title\")}"
                }

                p {
                    class: "text-2xl text-gray-600 dark:text-gray-300 mb-8",
                    "{i18n::t(\"home.subtitle\")}"
                }

                p {
                    class: "text-lg text-gray-500 dark:text-gray-400 mb-12 max-w-2xl mx-auto",
                    "{i18n::t(\"home.description\")}"
                }

                // 按钮组
                div {
                    class: "flex gap-4 justify-center",
                    Link {
                        to: crate::Route::Introduction {},
                        class: "px-8 py-3 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors text-lg font-medium",
                        "{i18n::t(\"home.get_started\")}"
                    }
                    a {
                        href: "https://github.com/fangbc5/helixui",
                        class: "px-8 py-3 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 border-2 border-gray-300 dark:border-gray-700 rounded-lg hover:border-gray-400 dark:hover:border-gray-600 transition-colors text-lg font-medium",
                        "GitHub"
                    }
                }
            }

            // 特性展示
            div {
                class: "container mx-auto px-4 py-16",
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-8",

                    // 特性 1
                    div {
                        class: "p-6 bg-white dark:bg-gray-800 rounded-lg shadow-sm transition-colors",
                        div {
                            class: "text-4xl mb-4",
                            "🎨"
                        }
                        h3 {
                            class: "text-xl font-semibold mb-2 text-gray-900 dark:text-white",
                            "{i18n::t(\"feature.elegant.title\")}"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-300",
                            "{i18n::t(\"feature.elegant.desc\")}"
                        }
                    }

                    // 特性 2
                    div {
                        class: "p-6 bg-white dark:bg-gray-800 rounded-lg shadow-sm transition-colors",
                        div {
                            class: "text-4xl mb-4",
                            "⚡"
                        }
                        h3 {
                            class: "text-xl font-semibold mb-2 text-gray-900 dark:text-white",
                            "{i18n::t(\"feature.performance.title\")}"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-300",
                            "{i18n::t(\"feature.performance.desc\")}"
                        }
                    }

                    // 特性 3
                    div {
                        class: "p-6 bg-white dark:bg-gray-800 rounded-lg shadow-sm transition-colors",
                        div {
                            class: "text-4xl mb-4",
                            "🔧"
                        }
                        h3 {
                            class: "text-xl font-semibold mb-2 text-gray-900 dark:text-white",
                            "{i18n::t(\"feature.easy.title\")}"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-300",
                            "{i18n::t(\"feature.easy.desc\")}"
                        }
                    }
                }
            }
        }
    }
}
