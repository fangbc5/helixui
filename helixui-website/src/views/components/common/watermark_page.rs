use crate::views::layout::{ComponentsSidebar, TocItem};
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{DemoBox, Table, Watermark};

#[component]
pub fn WatermarkPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 2,
        },
        TocItem {
            id: "text".to_string(),
            title: "文字水印".to_string(),
            level: 2,
        },
        TocItem {
            id: "image".to_string(),
            title: "图片水印".to_string(),
            level: 2,
        },
        TocItem {
            id: "custom".to_string(),
            title: "自定义样式".to_string(),
            level: 2,
        },
        TocItem {
            id: "multi-line".to_string(),
            title: "多行文字".to_string(),
            level: 2,
        },
        TocItem {
            id: "dense".to_string(),
            title: "密集水印".to_string(),
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
                        "水印 Watermark"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "用于在内容上显示水印，支持文字和图片两种类型。"
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
                        description: "最简单的文字水印用法。",
                        code: r#"
                            Watermark {{
                                content: "Helix UI".to_string(),
                                div {{
                                    class: "h-64 p-8",
                                    h2 {{ "这是内容区域" }}
                                    p {{ "水印会作为背景显示在这个区域。" }}
                                }}
                            }}
                        "#,
                        children: rsx! {
                            Watermark {
                                content: "Helix UI".to_string(),
                                div {
                                    class: "h-64 p-8 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800",
                                    h2 { class: "text-xl font-semibold mb-4 text-gray-900 dark:text-white", "这是内容区域" }
                                    p { class: "text-gray-700 dark:text-gray-300", "水印会作为背景显示在这个区域。水印默认以一定角度旋转，并且会重复显示。" }
                                }
                            }
                        }
                    }
                }

                // 文字水印
                section {
                    id: "text",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "文字水印"
                    }
                    DemoBox {
                        title: "文字水印",
                        description: "可以自定义文字颜色、大小、角度等。",
                        code: r#"
                            Watermark {{
                                content: "保密".to_string(),
                                font_size: 20.0,
                                color: "rgba(255,0,0,0.2)".to_string(),
                                rotate: -25.0,
                                gap_x: 150.0,
                                gap_y: 150.0,
                                div {{
                                    class: "h-64 p-8",
                                    "内容区域"
                                }}
                            }}
                        "#,
                        children: rsx! {
                            Watermark {
                                content: "保密".to_string(),
                                font_size: 20.0,
                                color: "rgba(255,0,0,0.2)".to_string(),
                                rotate: -25.0,
                                gap_x: 150.0,
                                gap_y: 150.0,
                                div {
                                    class: "h-64 p-8 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                    "内容区域"
                                }
                            }
                        }
                    }
                }

                // 图片水印
                section {
                    id: "image",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "图片水印"
                    }
                    DemoBox {
                        title: "图片水印",
                        description: "可以使用图片作为水印。",
                        code: r#"
                            Watermark {{
                                content: "".to_string(),
                                image: Some("https://example.com/logo.png".to_string()),
                                div {{
                                    class: "h-64 p-8",
                                    "内容区域"
                                }}
                            }}
                        "#,
                        children: rsx! {
                            Watermark {
                                content: "".to_string(),
                                image: Some("https://dummyimage.com/120x60/4F46E5/FFFFFF&text=LOGO".to_string()),
                                div {
                                    class: "h-64 p-8 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                    "内容区域"
                                }
                            }
                        }
                    }
                }

                // 自定义样式
                section {
                    id: "custom",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "自定义样式"
                    }
                    DemoBox {
                        title: "自定义样式",
                        description: "可以自定义水印的各种参数。",
                        code: r#"
                            Watermark {{
                                content: "自定义".to_string(),
                                font_size: 24.0,
                                color: "rgba(0,100,200,0.15)".to_string(),
                                rotate: -30.0,
                                gap_x: 200.0,
                                gap_y: 100.0,
                                width: 200.0,
                                height: 80.0,
                                font_family: "serif".to_string(),
                                div {{
                                    class: "h-64 p-8",
                                    "内容区域"
                                }}
                            }}
                        "#,
                        children: rsx! {
                            Watermark {
                                content: "自定义".to_string(),
                                font_size: 24.0,
                                color: "rgba(0,100,200,0.15)".to_string(),
                                rotate: -30.0,
                                gap_x: 200.0,
                                gap_y: 100.0,
                                width: 200.0,
                                height: 80.0,
                                font_family: "serif".to_string(),
                                div {
                                    class: "h-64 p-8 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                    "内容区域"
                                }
                            }
                        }
                    }
                }

                // 多行文字
                section {
                    id: "multi-line",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "多行文字"
                    }
                    DemoBox {
                        title: "多行文字",
                        description: "支持多行文字水印，使用 \\n 分隔。",
                        code: r#"
                            Watermark {{
                                content: "Helix UI\n水印组件".to_string(),
                                font_size: 16.0,
                                gap_x: 120.0,
                                gap_y: 80.0,
                                div {{
                                    class: "h-64 p-8",
                                    "内容区域"
                                }}
                            }}
                        "#,
                        children: rsx! {
                            Watermark {
                                content: "Helix UI\n水印组件".to_string(),
                                font_size: 16.0,
                                gap_x: 120.0,
                                gap_y: 80.0,
                                div {
                                    class: "h-64 p-8 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                    "内容区域"
                                }
                            }
                        }
                    }
                }

                // 密集水印
                section {
                    id: "dense",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "密集水印"
                    }
                    DemoBox {
                        title: "密集水印",
                        description: "通过减小间距参数可以创建更密集的水印效果。",
                        code: r#"
                            Watermark {{
                                content: "保密 Confidential".to_string(),
                                font_size: 14.0,
                                gap_x: 50.0,
                                gap_y: 50.0,
                                width: 100.0,
                                height: 50.0,
                                color: "rgba(0,0,0,0.12)".to_string(),
                                div {{
                                    class: "h-64 p-8",
                                    "内容区域"
                                }}
                            }}
                        "#,
                        children: rsx! {
                            Watermark {
                                content: "保密 Confidential".to_string(),
                                font_size: 14.0,
                                gap_x: 50.0,
                                gap_y: 50.0,
                                width: 100.0,
                                height: 50.0,
                                color: "rgba(0,0,0,0.12)".to_string(),
                                div {
                                    class: "h-64 p-8 border border-gray-200 dark:border-gray-700 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white",
                                    "内容区域"
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
                                vec!["content".to_string(), "String".to_string(), "—".to_string(), "水印文本内容".to_string()],
                                vec!["image".to_string(), "Option<String>".to_string(), "None".to_string(), "水印图片 URL，如果提供则优先使用图片而不是文本".to_string()],
                                vec!["font_size".to_string(), "f64".to_string(), "16.0".to_string(), "字体大小（像素）".to_string()],
                                vec!["color".to_string(), "String".to_string(), "\"rgba(0,0,0,0.15)\"".to_string(), "字体颜色（CSS 颜色值）".to_string()],
                                vec!["rotate".to_string(), "f64".to_string(), "-22.0".to_string(), "旋转角度（度）".to_string()],
                                vec!["gap_x".to_string(), "f64".to_string(), "100.0".to_string(), "水印之间的水平间隔（像素）".to_string()],
                                vec!["gap_y".to_string(), "f64".to_string(), "100.0".to_string(), "水印之间的垂直间隔（像素）".to_string()],
                                vec!["width".to_string(), "f64".to_string(), "120.0".to_string(), "水印块的宽度（像素）".to_string()],
                                vec!["height".to_string(), "f64".to_string(), "64.0".to_string(), "水印块的高度（像素）".to_string()],
                                vec!["font_family".to_string(), "String".to_string(), "\"Arial\"".to_string(), "字体族".to_string()],
                                vec!["enabled".to_string(), "bool".to_string(), "true".to_string(), "是否启用水印".to_string()],
                                vec!["class".to_string(), "Option<String>".to_string(), "None".to_string(), "自定义类名".to_string()],
                                vec!["children".to_string(), "Element".to_string(), "—".to_string(), "需要显示水印的内容".to_string()],
                            ],
                        }
                    }
                }
            }
        }
    }
}
