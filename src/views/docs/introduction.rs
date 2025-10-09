use crate::{
    i18n,
    views::{DocPage, DocsSidebar},
};
use dioxus::prelude::*;

/// 介绍页面
#[component]
pub fn Introduction() -> Element {
    rsx! {
        DocPage {
            sidebar: rsx! { DocsSidebar {} },

            div {
                class: "doc-page",

            h1 {
                class: "text-4xl font-bold text-gray-900 dark:text-white mb-4",
                "{i18n::t(\"intro.title\")}"
            }

            p {
                class: "text-lg text-gray-600 dark:text-gray-300 mb-8",
                "{i18n::t(\"intro.desc\")}"
            }

            section {
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "{i18n::t(\"intro.features\")}"
                }

                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-4",

                    FeatureCard { title: i18n::t("intro.features.modern"), description: "Inspired by Naive UI design philosophy".to_string() }
                    FeatureCard { title: i18n::t("intro.features.rust"), description: "Built on Dioxus framework with Rust performance".to_string() }
                    FeatureCard { title: i18n::t("intro.features.customizable"), description: "Simple API with clear documentation".to_string() }
                    FeatureCard { title: "Cross Platform".to_string(), description: "Supports Web and Desktop platforms".to_string() }
                }
            }

            section {
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "{i18n::t(\"version.title\")}"
                }
                div {
                    class: "bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-6",
                    p { class: "mb-2 text-gray-900 dark:text-white", strong { "{i18n::t(\"version.current\")}: " } "0.1.0" }
                    p { class: "mb-2 text-gray-900 dark:text-white", strong { "Dioxus: " } "0.6.0" }
                    p { class: "text-gray-900 dark:text-white", strong { "Status: " } "Alpha (In Development)" }
                }
            }
            }
        }
    }
}

#[component]
fn FeatureCard(title: String, description: String) -> Element {
    rsx! {
        div {
            class: "p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg transition-colors",
            h3 { class: "font-semibold text-gray-900 dark:text-white mb-2", "{title}" }
            p { class: "text-sm text-gray-600 dark:text-gray-300", "{description}" }
        }
    }
}
