use crate::i18n;
use crate::views::layout::{DocPage, DocsSidebar};
use dioxus::prelude::*;

/// 指南页面
#[component]
pub fn Guide() -> Element {
    rsx! {
        DocPage {
            sidebar: rsx! { DocsSidebar {} },
            toc_items: vec![],

            div {
                class: "doc-page",

                h1 {
                    class: "text-4xl font-bold text-gray-900 dark:text-white mb-4 transition-colors",
                    "{i18n::t(\"guide.title\")}"
                }

                p {
                    class: "text-lg text-gray-600 dark:text-gray-300 mb-8 transition-colors",
                    "This guide will help you better use Helix UI to develop applications."
                }

                section {
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "Component Usage"
                    }
                    p { class: "text-gray-700 dark:text-gray-300 mb-4 transition-colors", "Pass configuration and data to components via props:" }
                    pre {
                        class: "bg-gray-900 dark:bg-gray-950 text-white p-4 rounded-lg overflow-x-auto text-sm transition-colors",
                        "Button {{\n    button_type: ButtonType::Primary,\n    size: ButtonSize::Large,\n    \"Submit\"\n}}"
                    }
                }

                section {
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "{i18n::t(\"guide.theming\")}"
                    }
                    p { class: "text-gray-700 dark:text-gray-300 mb-4 transition-colors", "Helix UI provides built-in light and dark theme support. Toggle themes using the theme button in the top navigation bar." }
                    div {
                        class: "bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-6 transition-colors",
                        p { class: "text-gray-900 dark:text-white", "💡 Tip: The theme state is managed globally and all components will automatically respond to theme changes." }
                    }
                }

                section {
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "FAQ"
                    }

                    div {
                        class: "space-y-4",

                        FAQItem {
                            question: "How to customize component styles?".to_string(),
                            answer: "Use Tailwind CSS classes or custom CSS to override default styles.".to_string()
                        }

                        FAQItem {
                            question: "Does it support server-side rendering?".to_string(),
                            answer: "Yes, Dioxus framework supports SSR, Helix UI components are SSR compatible.".to_string()
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FAQItem(question: String, answer: String) -> Element {
    rsx! {
        div {
            class: "p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg transition-colors",
            h3 { class: "font-semibold text-gray-900 dark:text-white mb-2", "Q: {question}" }
            p { class: "text-gray-600 dark:text-gray-300", "A: {answer}" }
        }
    }
}
