use crate::i18n;
use crate::views::layout::{DefaultFooter, TopNavbar};
use dioxus::prelude::*;
use helixui::components::common::{Card, CardSize};
use helixui::components::layout::{Breakpoint, Content, Footer, Grid, GridItem, Header, Layout};
use std::collections::BTreeMap;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // 自研 Grid 响应式列配置：移动端 1 列，md 及以上 3 列
    let mut feature_grid_cols = BTreeMap::new();
    feature_grid_cols.insert(Breakpoint::Md, 3);
    rsx! {
        Layout {
            Header { height: Some(64),
                TopNavbar{}
            }
            Content {
                Grid { cols: 1, y_gap: Some(48), class: Some("min-h-screen bg-gradient-to-b from-green-50 to-white dark:from-gray-900 dark:to-gray-950 transition-colors".to_string()),
                    // Hero 区域
                    GridItem {
                        div {
                            class: "container mx-auto px-4 pt-24 pb-0 text-center",

                            h1 { class: "text-6xl font-bold text-gray-900 dark:text-white mb-4", "{i18n::t(\"home.title\")}" }
                            p { class: "text-2xl text-gray-600 dark:text-gray-300 mb-8", "{i18n::t(\"home.subtitle\")}" }
                            p { class: "text-lg text-gray-500 dark:text-gray-400 mb-12 max-w-2xl mx-auto", "{i18n::t(\"home.description\")}" }

                            // 按钮组（保持简单 div 包裹，若需要也可换 Flex）
                            div {
                                class: "flex gap-4 justify-center",
                                Link { to: crate::Route::Introduction {}, class: "px-8 py-3 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors text-lg font-medium", "{i18n::t(\"home.get_started\")}" }
                            }
                        }
                    }

                    // 特性展示（使用自研 Grid）
                    GridItem {
                        div { class: "container mx-auto px-4",
                            Grid { cols: 3, class: Some("gap-8".to_string()),
                                // 特性 1
                                GridItem {
                                    Card { size: CardSize::Large, bordered: true, class: Some("h-full".to_string()),
                                        div { class: "text-4xl mb-4", "🎨" }
                                        h3 { class: "text-xl font-semibold mb-2 text-gray-900 dark:text-white", "{i18n::t(\"feature.elegant.title\")}" }
                                        p { class: "text-gray-600 dark:text-gray-300", "{i18n::t(\"feature.elegant.desc\")}" }
                                    }
                                }
                                // 特性 2
                                GridItem {
                                    Card { size: CardSize::Large, bordered: true, class: Some("h-full".to_string()),
                                        div { class: "text-4xl mb-4", "⚡" }
                                        h3 { class: "text-xl font-semibold mb-2 text-gray-900 dark:text-white", "{i18n::t(\"feature.performance.title\")}" }
                                        p { class: "text-gray-600 dark:text-gray-300", "{i18n::t(\"feature.performance.desc\")}" }
                                    }
                                }
                                // 特性 3
                                GridItem {
                                    Card { size: CardSize::Large, bordered: true, class: Some("h-full".to_string()),
                                        div { class: "text-4xl mb-4", "🔧" }
                                        h3 { class: "text-xl font-semibold mb-2 text-gray-900 dark:text-white", "{i18n::t(\"feature.easy.title\")}" }
                                        p { class: "text-gray-600 dark:text-gray-300", "{i18n::t(\"feature.easy.desc\")}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Footer {
                DefaultFooter{}
            }
        }

    }
}
