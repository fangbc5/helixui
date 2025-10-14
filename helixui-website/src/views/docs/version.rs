use crate::i18n;
use crate::views::DocPage;
use crate::views::layout::DocsSidebar;
use dioxus::prelude::*;

/// 版本页面
#[component]
pub fn Version() -> Element {
    rsx! {
        DocPage {
            sidebar: rsx! { DocsSidebar {} },
            toc_items: vec![],

            div {
                class: "doc-page",

                h1 {
                    class: "text-4xl font-bold text-gray-900 dark:text-white mb-4 transition-colors",
                    "{i18n::t(\"version.title\")}"
                }

                p {
                    class: "text-lg text-gray-600 dark:text-gray-300 mb-8 transition-colors",
                    "View Helix UI version history and change log."
                }

                section {
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "{i18n::t(\"version.current\")}"
                    }

                    div {
                        class: "bg-gradient-to-r from-green-50 to-blue-50 dark:from-green-900/20 dark:to-blue-900/20 border border-green-200 dark:border-green-800 rounded-lg p-8 transition-colors",
                        div {
                            class: "flex items-center justify-between mb-4",
                            h3 {
                                class: "text-3xl font-bold text-gray-900 dark:text-white",
                                "v0.1.0"
                            }
                            span {
                                class: "px-3 py-1 bg-green-600 text-white text-sm font-medium rounded-full",
                                "Alpha"
                            }
                        }

                        p {
                            class: "text-gray-700 dark:text-gray-300 mb-2",
                            "Release Date: 2025-01-10"
                        }

                        p {
                            class: "text-gray-700 dark:text-gray-300",
                            "Dioxus Version: 0.6.0"
                        }
                    }
                }

                section {
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "Change Log"
                    }

                    div {
                        class: "space-y-6",

                        VersionItem {
                            version: "0.1.0".to_string(),
                            date: "2025-01-10".to_string(),
                            changes: vec![
                                "🎉 Initial release".to_string(),
                                "✨ Added Button component".to_string(),
                                "🌙 Dark mode support".to_string(),
                                "🌍 i18n internationalization support (Chinese/English)".to_string(),
                                "📱 Responsive design".to_string(),
                            ]
                        }
                    }
                }

                section {
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "Roadmap"
                    }

                    div {
                        class: "bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-6 transition-colors",
                        ul {
                            class: "space-y-2 text-gray-700 dark:text-gray-300",
                            li { "🔲 v0.2.0 - More basic components (Input, Select, etc.)" }
                            li { "🔲 v0.3.0 - Form components and validation" }
                            li { "🔲 v0.4.0 - Data display components (Table, List, etc.)" }
                            li { "🔲 v1.0.0 - Stable version release" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn VersionItem(version: String, date: String, changes: Vec<String>) -> Element {
    rsx! {
        div {
            class: "border-l-4 border-green-500 dark:border-green-600 pl-6 py-2 transition-colors",
            div {
                class: "flex items-center gap-3 mb-2",
                h3 {
                    class: "text-xl font-semibold text-gray-900 dark:text-white",
                    "v{version}"
                }
                span {
                    class: "text-sm text-gray-500 dark:text-gray-400",
                    "{date}"
                }
            }

            ul {
                class: "space-y-1 text-gray-700 dark:text-gray-300",
                for change in changes {
                    li { "{change}" }
                }
            }
        }
    }
}
