use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{
    DemoBox, FloatButton, FloatButtonBadge, FloatButtonPosition, FloatButtonSize, IconType, Table,
};

#[component]
pub fn FloatButtonPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "position".to_string(),
            title: "位置".to_string(),
            level: 2,
        },
        TocItem {
            id: "size".to_string(),
            title: "尺寸".to_string(),
            level: 2,
        },
        TocItem {
            id: "badge".to_string(),
            title: "带徽章".to_string(),
            level: 2,
        },
        TocItem {
            id: "tooltip".to_string(),
            title: "提示信息".to_string(),
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
                class: "component-doc",

                // 标题
                div {
                    class: "mb-8",
                    h1 {
                        class: "text-4xl font-bold text-gray-900 dark:text-white mb-2",
                        "浮动按钮 FloatButton"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "浮动按钮，固定在页面某个位置，常用于返回顶部、快捷操作等场景。"
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
                    DemoBox {
                        title: "基础用法",
                        description: "最简单的浮动按钮用法。",
                        code: r#"
                            FloatButton {{
                                icon: IconType::ArrowUp,
                                onclick: |_| {{
                                    // 返回顶部
                                }},
                            }}
                        "#,
                        children: rsx! {
                            div {
                                class: "relative h-64 border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900",
                                FloatButton {
                                    icon: IconType::ArrowUp,
                                    position: FloatButtonPosition::BottomRight,
                                    onclick: |_| {},
                                }
                            }
                        }
                    }
                }

                // 位置
                section {
                    id: "position",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "位置"
                    }
                    DemoBox {
                        title: "位置",
                        description: "可以设置浮动按钮在四个角落的位置。",
                        code: r#"
                            FloatButton {{ position: FloatButtonPosition::TopLeft, icon: IconType::Settings }}
                            FloatButton {{ position: FloatButtonPosition::TopRight, icon: IconType::Bell }}
                            FloatButton {{ position: FloatButtonPosition::BottomLeft, icon: IconType::Help }}
                            FloatButton {{ position: FloatButtonPosition::BottomRight, icon: IconType::ArrowUp }}
                        "#,
                        children: rsx! {
                            div {
                                class: "relative h-64 border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900",
                                FloatButton {
                                    position: FloatButtonPosition::TopLeft,
                                    icon: IconType::Settings,
                                    onclick: |_| {},
                                }
                                FloatButton {
                                    position: FloatButtonPosition::TopRight,
                                    icon: IconType::Bell,
                                    onclick: |_| {},
                                }
                                FloatButton {
                                    position: FloatButtonPosition::BottomLeft,
                                    icon: IconType::Info,
                                    onclick: |_| {},
                                }
                                FloatButton {
                                    position: FloatButtonPosition::BottomRight,
                                    icon: IconType::ArrowUp,
                                    onclick: |_| {},
                                }
                            }
                        }
                    }
                }

                // 尺寸
                section {
                    id: "size",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "尺寸"
                    }
                    DemoBox {
                        title: "尺寸",
                        description: "小/中/大三种尺寸。",
                        code: r#"
                            FloatButton {{ size: FloatButtonSize::Small, icon: IconType::Plus }}
                            FloatButton {{ size: FloatButtonSize::Medium, icon: IconType::Plus }}
                            FloatButton {{ size: FloatButtonSize::Large, icon: IconType::Plus }}
                        "#,
                        children: rsx! {
                            div {
                                class: "relative h-64 border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900 flex items-center justify-center gap-4",
                                FloatButton {
                                    size: FloatButtonSize::Small,
                                    icon: IconType::Plus,
                                    position: FloatButtonPosition::BottomLeft,
                                    onclick: |_| {},
                                }
                                FloatButton {
                                    size: FloatButtonSize::Medium,
                                    icon: IconType::Plus,
                                    position: FloatButtonPosition::BottomRight,
                                    onclick: |_| {},
                                }
                                FloatButton {
                                    size: FloatButtonSize::Large,
                                    icon: IconType::Plus,
                                    position: FloatButtonPosition::TopRight,
                                    onclick: |_| {},
                                }
                            }
                        }
                    }
                }

                // 带徽章
                section {
                    id: "badge",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "带徽章"
                    }
                    DemoBox {
                        title: "带徽章",
                        description: "可以在浮动按钮上显示徽章，常用于显示未读消息数等。",
                        code: r#"
                            FloatButton {{
                                icon: IconType::Bell,
                                badge: Some(FloatButtonBadge {{
                                    value: Some("5".to_string()),
                                    dot: false,
                                    max: None,
                                    show_zero: false,
                                    show: true,
                                }}),
                            }}
                            FloatButton {{
                                icon: IconType::Message,
                                badge: Some(FloatButtonBadge {{
                                    value: Some("99+".to_string()),
                                    dot: false,
                                    max: Some(99),
                                    show_zero: false,
                                    show: true,
                                }}),
                            }}
                            FloatButton {{
                                icon: IconType::Mail,
                                badge: Some(FloatButtonBadge {{
                                    value: None,
                                    dot: true,
                                    show: true,
                                }}),
                            }}
                        "#,
                        children: rsx! {
                            div {
                                class: "relative h-64 border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900",
                                FloatButton {
                                    icon: IconType::Bell,
                                    position: FloatButtonPosition::BottomRight,
                                    badge: Some(FloatButtonBadge {
                                        value: Some("5".to_string()),
                                        dot: false,
                                        max: None,
                                        show_zero: false,
                                        show: true,
                                    }),
                                    onclick: |_| {},
                                }
                                FloatButton {
                                    icon: IconType::Message,
                                    position: FloatButtonPosition::BottomLeft,
                                    badge: Some(FloatButtonBadge {
                                        value: Some("150".to_string()),
                                        dot: false,
                                        max: Some(99),
                                        show_zero: false,
                                        show: true,
                                    }),
                                    onclick: |_| {},
                                }
                                FloatButton {
                                    icon: IconType::Mail,
                                    position: FloatButtonPosition::TopRight,
                                    badge: Some(FloatButtonBadge {
                                        value: None,
                                        dot: true,
                                        max: None,
                                        show_zero: false,
                                        show: true,
                                    }),
                                    onclick: |_| {},
                                }
                            }
                        }
                    }
                }

                // 提示信息
                section {
                    id: "tooltip",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "提示信息"
                    }
                    DemoBox {
                        title: "提示信息",
                        description: "可以添加提示文本，鼠标悬停时显示。",
                        code: r#"
                            FloatButton {{
                                icon: IconType::ArrowUp,
                                tooltip: Some("返回顶部".to_string()),
                            }}
                            FloatButton {{
                                icon: IconType::Settings,
                                tooltip: Some("设置".to_string()),
                            }}
                        "#,
                        children: rsx! {
                            div {
                                class: "relative h-64 border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-900",
                                FloatButton {
                                    icon: IconType::ArrowUp,
                                    position: FloatButtonPosition::BottomRight,
                                    tooltip: Some("返回顶部".to_string()),
                                    onclick: |_| {},
                                }
                                FloatButton {
                                    icon: IconType::Settings,
                                    position: FloatButtonPosition::TopRight,
                                    tooltip: Some("设置".to_string()),
                                    onclick: |_| {},
                                }
                            }
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
                    div { class: "mt-2",
                        Table {
                            headers: Some(vec![
                                "属性".to_string(),
                                "类型".to_string(),
                                "默认值".to_string(),
                                "说明".to_string(),
                            ]),
                            data: vec![
                                vec!["icon".to_string(), "IconType".to_string(), "ArrowUp".to_string(), "按钮图标".to_string()],
                                vec!["position".to_string(), "FloatButtonPosition".to_string(), "BottomRight".to_string(), "按钮位置：TopLeft/TopRight/BottomLeft/BottomRight".to_string()],
                                vec!["size".to_string(), "FloatButtonSize".to_string(), "Medium".to_string(), "按钮尺寸：Small/Medium/Large".to_string()],
                                vec!["disabled".to_string(), "bool".to_string(), "false".to_string(), "是否禁用".to_string()],
                                vec!["onclick".to_string(), "Option<EventHandler<()>>".to_string(), "None".to_string(), "点击事件处理".to_string()],
                                vec!["badge".to_string(), "Option<FloatButtonBadge>".to_string(), "None".to_string(), "徽章配置".to_string()],
                                vec!["tooltip".to_string(), "Option<String>".to_string(), "None".to_string(), "提示文本".to_string()],
                                vec!["class".to_string(), "Option<String>".to_string(), "None".to_string(), "自定义类名".to_string()],
                            ],
                        }
                    }
                }
            }
        }
    }
}
