use crate::components::{DemoBox, Icon, IconSize, IconType};
use crate::views::layout::TocItem;
use crate::views::{ComponentsSidebar, DocPage};
use dioxus::prelude::*;

/// Icon 组件文档页面
#[component]
pub fn IconPage() -> Element {
    // 定义目录项
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "演示".to_string(),
            level: 1,
        },
        TocItem {
            id: "size".to_string(),
            title: "尺寸".to_string(),
            level: 1,
        },
        TocItem {
            id: "color".to_string(),
            title: "颜色".to_string(),
            level: 1,
        },
        TocItem {
            id: "status".to_string(),
            title: "状态".to_string(),
            level: 1,
        },
        TocItem {
            id: "loading".to_string(),
            title: "加载".to_string(),
            level: 1,
        },
        TocItem {
            id: "all".to_string(),
            title: "所有图标".to_string(),
            level: 1,
        },
        TocItem {
            id: "api".to_string(),
            title: "API".to_string(),
            level: 1,
        },
    ];

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items,

            div {
                class: "component-doc",

                // 标题
                div {
                    class: "mb-8",
                    h1 {
                        class: "text-4xl font-bold text-gray-900 dark:text-white mb-2",
                        "图标 Icon"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "语义化的矢量图形，提供丰富的图标库。"
                    }
                }

                // 基础用法
                section {
                    id: "basic",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "基础用法"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "通过 icon 属性指定图标类型。"
                    }

                    DemoBox {
                        title: "基础图标".to_string(),
                        description: "最基本的图标用法".to_string(),
                        code: r#"use helixui::components::{Icon, IconType};

rsx! {
    Icon { icon: IconType::Home }
    Icon { icon: IconType::User }
    Icon { icon: IconType::Settings }
    Icon { icon: IconType::Search }
    Icon { icon: IconType::Menu }
}"#.to_string(),

                        div {
                            class: "flex items-center gap-4 flex-wrap",
                            Icon { icon: IconType::Home }
                            Icon { icon: IconType::User }
                            Icon { icon: IconType::Settings }
                            Icon { icon: IconType::Search }
                            Icon { icon: IconType::Menu }
                        }
                    }
                }

                // 图标尺寸
                section {
                    id: "size",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "图标尺寸"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "通过 size 属性设置图标大小。"
                    }

                    DemoBox {
                        title: "不同尺寸".to_string(),
                        description: "支持 Small、Medium、Large、XLarge 四种尺寸".to_string(),
                        code: r#"use helixui::components::{Icon, IconSize, IconType};

rsx! {
    Icon { icon: IconType::Home, size: IconSize::Small }
    Icon { icon: IconType::Home, size: IconSize::Medium }
    Icon { icon: IconType::Home, size: IconSize::Large }
    Icon { icon: IconType::Home, size: IconSize::XLarge }
}"#.to_string(),

                        div {
                            class: "flex items-center gap-6",
                            div {
                                class: "flex flex-col items-center gap-2",
                                Icon { icon: IconType::Home, size: IconSize::Small }
                                span { class: "text-xs text-gray-500 dark:text-gray-400", "Small" }
                            }
                            div {
                                class: "flex flex-col items-center gap-2",
                                Icon { icon: IconType::Home, size: IconSize::Medium }
                                span { class: "text-xs text-gray-500 dark:text-gray-400", "Medium" }
                            }
                            div {
                                class: "flex flex-col items-center gap-2",
                                Icon { icon: IconType::Home, size: IconSize::Large }
                                span { class: "text-xs text-gray-500 dark:text-gray-400", "Large" }
                            }
                            div {
                                class: "flex flex-col items-center gap-2",
                                Icon { icon: IconType::Home, size: IconSize::XLarge }
                                span { class: "text-xs text-gray-500 dark:text-gray-400", "XLarge" }
                            }
                        }
                    }
                }

                // 图标颜色
                section {
                    id: "color",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "图标颜色"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "使用 Tailwind CSS 的 text-* 类名自定义颜色。"
                    }

                    DemoBox {
                        title: "彩色图标".to_string(),
                        description: "通过 class 属性设置图标颜色".to_string(),
                        code: r#"use helixui::components::{Icon, IconType};

rsx! {
    Icon { icon: IconType::Home, class: "text-blue-500".to_string() }
    Icon { icon: IconType::User, class: "text-green-500".to_string() }
    Icon { icon: IconType::Settings, class: "text-purple-500".to_string() }
    Icon { icon: IconType::Search, class: "text-orange-500".to_string() }
    Icon { icon: IconType::Menu, class: "text-red-500".to_string() }
}"#.to_string(),

                        div {
                            class: "flex items-center gap-4",
                            Icon { icon: IconType::Home, class: "text-blue-500".to_string() }
                            Icon { icon: IconType::User, class: "text-green-500".to_string() }
                            Icon { icon: IconType::Settings, class: "text-purple-500".to_string() }
                            Icon { icon: IconType::Search, class: "text-orange-500".to_string() }
                            Icon { icon: IconType::Menu, class: "text-red-500".to_string() }
                        }
                    }
                }

                // 状态图标
                section {
                    id: "status",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "状态图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "常用的状态指示图标。"
                    }

                    div {
                        class: "p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700",
                        div {
                            class: "grid grid-cols-2 md:grid-cols-4 gap-4",

                            div {
                                class: "flex items-center gap-2",
                                Icon { icon: IconType::Success, class: "text-green-500".to_string() }
                                span { class: "text-gray-700 dark:text-gray-300", "Success" }
                            }

                            div {
                                class: "flex items-center gap-2",
                                Icon { icon: IconType::Info, class: "text-blue-500".to_string() }
                                span { class: "text-gray-700 dark:text-gray-300", "Info" }
                            }

                            div {
                                class: "flex items-center gap-2",
                                Icon { icon: IconType::Warning, class: "text-yellow-500".to_string() }
                                span { class: "text-gray-700 dark:text-gray-300", "Warning" }
                            }

                            div {
                                class: "flex items-center gap-2",
                                Icon { icon: IconType::Error, class: "text-red-500".to_string() }
                                span { class: "text-gray-700 dark:text-gray-300", "Error" }
                            }
                        }
                    }
                }

                // 加载动画
                section {
                    id: "loading",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "加载动画"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "设置 spin 属性使图标旋转。"
                    }

                    DemoBox {
                        title: "旋转动画".to_string(),
                        description: "任何图标都可以设置 spin 属性实现旋转效果".to_string(),
                        code: r#"use helixui::components::{Icon, IconSize, IconType};

rsx! {
    Icon { 
        icon: IconType::Loading, 
        spin: true, 
        class: "text-blue-500".to_string() 
    }
    Icon { 
        icon: IconType::Loading, 
        spin: true, 
        size: IconSize::Large, 
        class: "text-green-500".to_string() 
    }
    Icon { 
        icon: IconType::Settings, 
        spin: true, 
        class: "text-purple-500".to_string() 
    }
}"#.to_string(),

                        div {
                            class: "flex items-center gap-4",
                            Icon { icon: IconType::Loading, spin: true, class: "text-blue-500".to_string() }
                            Icon { icon: IconType::Loading, spin: true, size: IconSize::Large, class: "text-green-500".to_string() }
                            Icon { icon: IconType::Settings, spin: true, class: "text-purple-500".to_string() }
                        }
                    }
                }

                // 所有图标
                section {
                    id: "all",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "所有图标"
                    }

                    div {
                        class: "p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700",
                        div {
                            class: "grid grid-cols-3 md:grid-cols-6 lg:grid-cols-8 gap-4",

                            IconItem { icon: IconType::Check, name: "Check" }
                            IconItem { icon: IconType::Close, name: "Close" }
                            IconItem { icon: IconType::ChevronDown, name: "Down" }
                            IconItem { icon: IconType::ChevronUp, name: "Up" }
                            IconItem { icon: IconType::ChevronLeft, name: "Left" }
                            IconItem { icon: IconType::ChevronRight, name: "Right" }
                            IconItem { icon: IconType::Info, name: "Info" }
                            IconItem { icon: IconType::Warning, name: "Warning" }
                            IconItem { icon: IconType::Error, name: "Error" }
                            IconItem { icon: IconType::Success, name: "Success" }
                            IconItem { icon: IconType::Search, name: "Search" }
                            IconItem { icon: IconType::Settings, name: "Settings" }
                            IconItem { icon: IconType::User, name: "User" }
                            IconItem { icon: IconType::Home, name: "Home" }
                            IconItem { icon: IconType::Menu, name: "Menu" }
                            IconItem { icon: IconType::Moon, name: "Moon" }
                            IconItem { icon: IconType::Sun, name: "Sun" }
                            IconItem { icon: IconType::GitHub, name: "GitHub" }
                            IconItem { icon: IconType::Loading, name: "Loading" }
                        }
                    }
                }

                // API
                section {
                    id: "api",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "API"
                    }

                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "w-full text-left border-collapse",
                            thead {
                                tr {
                                    class: "border-b border-gray-200 dark:border-gray-700",
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "属性" }
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "说明" }
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "类型" }
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "默认值" }
                                }
                            }
                            tbody {
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "icon" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "图标类型" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "IconType" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "-" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "size" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "图标尺寸" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "IconSize" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "Medium" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "color" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "图标颜色" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "String" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "currentColor" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "class" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "自定义类名" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "String" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "\"\""  }
                                }
                                tr {
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "spin" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否旋转" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn IconItem(icon: IconType, name: &'static str) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center p-4 hover:bg-gray-50 dark:hover:bg-gray-700 rounded-lg cursor-pointer transition-colors",
            Icon { icon: icon, size: IconSize::Large }
            span { class: "text-xs text-gray-600 dark:text-gray-400 mt-2 text-center", "{name}" }
        }
    }
}
