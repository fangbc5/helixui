use crate::i18n;
use crate::views::DocPage;
use crate::views::layout::DocsSidebar;
use dioxus::prelude::*;

/// 快速上手页面
#[component]
pub fn QuickStart() -> Element {
    rsx! {
        DocPage {
            sidebar: rsx! { DocsSidebar {} },
            toc_items: vec![],

            div {
                class: "doc-page",

            h1 {
                class: "text-4xl font-bold text-gray-900 dark:text-white mb-4",
                "{i18n::t(\"quick.title\")}"
            }

            p {
                class: "text-lg text-gray-600 dark:text-gray-300 mb-8",
                "This section will introduce how to use Helix UI in your project."
            }

            section {
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "{i18n::t(\"quick.install\")}"
                }
                p { class: "text-gray-700 dark:text-gray-300 mb-4", "Add dependency in Cargo.toml:" }
                pre {
                    class: "bg-gray-900 dark:bg-gray-950 text-white p-4 rounded-lg overflow-x-auto text-sm",
                    "[dependencies]\ndioxus = \"0.6.0\""
                }
            }

            section {
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "{i18n::t(\"quick.usage\")}"
                }
                p { class: "text-gray-700 dark:text-gray-300 mb-4", "A simple example:" }
                pre {
                    class: "bg-gray-900 dark:bg-gray-950 text-white p-4 rounded-lg overflow-x-auto text-sm",
                    "use dioxus::prelude::*;\nuse helixui::components::Button;\n\nfn App() -> Element {{\n    rsx! {{\n        Button {{ \"Click me\" }}\n    }}\n}}"
                }
            }

            section {
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "Next Steps"
                }
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Link {
                        to: crate::Route::ComponentsPage {},
                        class: "p-6 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-green-500 dark:hover:border-green-500 transition-colors",
                        h3 { class: "font-semibold text-gray-900 dark:text-white mb-2", "Browse Components" }
                        p { class: "text-sm text-gray-600 dark:text-gray-300", "View all available components" }
                    }
                    Link {
                        to: crate::Route::Guide {},
                        class: "p-6 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg hover:border-green-500 dark:hover:border-green-500 transition-colors",
                        h3 { class: "font-semibold text-gray-900 dark:text-white mb-2", "Development Guide" }
                        p { class: "text-sm text-gray-600 dark:text-gray-300", "Learn more development tips" }
                    }
                }
            }
            }
        }
    }
}
