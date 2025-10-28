use crate::views::layout::ComponentsSidebar;
use crate::views::layout::TocItem;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::data_display::scroll_area::{ScrollArea, ScrollDirection, ScrollType};
use helixui::components::DemoBox;

/// ScrollArea 演示页面
#[component]
pub fn ScrollAreaPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic-usage".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "directions".to_string(),
            title: "滚动方向".to_string(),
            level: 2,
        },
        TocItem {
            id: "scroll-types".to_string(),
            title: "滚动类型".to_string(),
            level: 2,
        },
        TocItem {
            id: "api".to_string(),
            title: "API".to_string(),
            level: 2,
        },
    ];

    rsx! {
        DocPage {
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items,
            div {
                DemoBox {
                    title: "基础用法",
                    description: "创建一个基本的滚动区域。",
                    code: "ScrollArea {{ \n    direction: ReadSignal::new(Signal::new(ScrollDirection::Vertical)), \n    div {{ \n        // 滚动内容 \n    }} \n}}",
                    children: rsx! {
                        BasicScrollAreaDemo {}
                    }
                }

                DemoBox {
                    title: "滚动方向",
                    description: "垂直滚动、水平滚动或双向滚动。",
                    code: "ScrollArea {{ \n    direction: ReadSignal::new(Signal::new(ScrollDirection::Horizontal)), \n    // 内容... \n}}",
                    children: rsx! {
                        ScrollDirectionDemo {}
                    }
                }

                DemoBox {
                    title: "滚动类型",
                    description: "三种滚动条显示模式：Auto（自动显示）、Always（始终显示）、Hidden（隐藏但可滚动）。",
                    code: "ScrollArea {{ \n    direction: ReadSignal::new(Signal::new(ScrollDirection::Vertical)),\n    scroll_type: ReadSignal::new(Signal::new(ScrollType::Always)),\n    // Auto: 需要时显示，Always: 始终显示，Hidden: 隐藏\n}}",
                    children: rsx! {
                        ScrollTypeDemo {}
                    }
                }

                div {
                    class: "mt-8",
                    h2 {
                        id: "api",
                        class: "text-2xl font-bold text-gray-900 dark:text-white mb-4",
                        "API"
                    }

                    h3 {
                        class: "text-xl font-bold text-gray-900 dark:text-white mb-4",
                        "ScrollArea"
                    }

                    div {
                        class: "overflow-x-auto mb-6",
                        table {
                            class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                            thead {
                                class: "bg-gray-50 dark:bg-gray-800",
                                tr {
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "参数"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "说明"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "类型"
                                    }
                                    th {
                                        class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider",
                                        "默认值"
                                    }
                                }
                            }
                            tbody {
                                class: "bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700",
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "direction" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "滚动方向" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<ScrollDirection>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "Both" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "scroll_type" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "滚动条显示类型" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<ScrollType>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "Auto" }
                                }
                                tr {
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100", "always_show_scrollbars" }
                                    td { class: "px-6 py-4 text-sm text-gray-600 dark:text-gray-400", "是否始终显示滚动条" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm font-mono text-gray-900 dark:text-gray-100", "ReadSignal<bool>" }
                                    td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400", "false" }
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
fn BasicScrollAreaDemo() -> Element {
    rsx! {
        div {
            class: "h-64 w-full border border-gray-300 dark:border-gray-700 rounded-lg",
            ScrollArea {
                direction: ReadSignal::new(Signal::new(ScrollDirection::Vertical)),
                div {
                    class: "p-4",
                    for i in 1..=30 {
                        div {
                            class: "mb-2 text-sm text-gray-700 dark:text-gray-300",
                            "这是第 {i} 行内容。你可以向下滚动查看更多内容。"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ScrollDirectionDemo() -> Element {
    rsx! {
        div {
            class: "space-y-4",
            div {
                h4 {
                    class: "text-sm font-medium text-gray-900 dark:text-white mb-2",
                    "垂直滚动"
                }
                div {
                    class: "h-32 w-full border border-gray-300 dark:border-gray-700 rounded-lg",
                    ScrollArea {
                        direction: ReadSignal::new(Signal::new(ScrollDirection::Vertical)),
                        div {
                            class: "p-4",
                            for i in 1..=10 {
                                div {
                                    class: "mb-2 text-sm text-gray-700 dark:text-gray-300",
                                    "垂直滚动项 {i}"
                                }
                            }
                        }
                    }
                }
            }
            div {
                h4 {
                    class: "text-sm font-medium text-gray-900 dark:text-white mb-2",
                    "水平滚动"
                }
                div {
                    class: "h-32 w-full border border-gray-300 dark:border-gray-700 rounded-lg",
                    ScrollArea {
                        direction: ReadSignal::new(Signal::new(ScrollDirection::Horizontal)),
                        div {
                            class: "p-4",
                            div {
                                class: "flex space-x-4",
                                for i in 1..=10 {
                                    div {
                                        class: "flex-shrink-0 w-32 h-24 bg-blue-100 dark:bg-blue-900 rounded flex items-center justify-center text-sm text-gray-700 dark:text-gray-300",
                                        "水平项 {i}"
                                    }
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
fn ScrollTypeDemo() -> Element {
    rsx! {
        div {
            class: "space-y-4",
            div {
                h4 {
                    class: "text-sm font-medium text-gray-900 dark:text-white mb-2",
                    "自动滚动条"
                }
                div {
                    class: "h-32 w-full border border-gray-300 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-800",
                    ScrollArea {
                        direction: ReadSignal::new(Signal::new(ScrollDirection::Vertical)),
                        scroll_type: ReadSignal::new(Signal::new(ScrollType::Auto)),
                        div {
                            class: "p-4 min-h-[200px]",
                            for i in 1..=10 {
                                div {
                                    class: "mb-2 text-sm text-gray-700 dark:text-gray-300",
                                    "自动滚动条项 {i}"
                                }
                            }
                        }
                    }
                }
            }
            div {
                h4 {
                    class: "text-sm font-medium text-gray-900 dark:text-white mb-2",
                    "始终显示滚动条"
                }
                div {
                    class: "h-32 w-full border border-gray-300 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-800",
                    ScrollArea {
                        direction: ReadSignal::new(Signal::new(ScrollDirection::Vertical)),
                        scroll_type: ReadSignal::new(Signal::new(ScrollType::Always)),
                        div {
                            class: "p-4 min-h-[200px]",
                            for i in 1..=10 {
                                div {
                                    class: "mb-2 text-sm text-gray-700 dark:text-gray-300",
                                    "始终显示滚动条项 {i}"
                                }
                            }
                        }
                    }
                }
            }
            div {
                h4 {
                    class: "text-sm font-medium text-gray-900 dark:text-white mb-2",
                    "隐藏滚动条"
                }
                div {
                    class: "h-32 w-full border border-gray-300 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-800",
                    ScrollArea {
                        direction: ReadSignal::new(Signal::new(ScrollDirection::Vertical)),
                        scroll_type: ReadSignal::new(Signal::new(ScrollType::Hidden)),
                        div {
                            class: "p-4 min-h-[200px]",
                            for i in 1..=10 {
                                div {
                                    class: "mb-2 text-sm text-gray-700 dark:text-gray-300",
                                    "隐藏滚动条项 {i} (但仍可滚动)"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
