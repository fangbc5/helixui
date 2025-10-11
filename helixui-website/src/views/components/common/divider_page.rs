use crate::views::layout::{ComponentsSidebar, DocPage, TocItem};
use dioxus::prelude::*;
use helixui::components::{DemoBox, Divider, DividerHorizontal, DividerVertical};

/// Divider 组件演示页面
#[component]
pub fn DividerPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 1,
        },
        TocItem {
            id: "direction".to_string(),
            title: "方向".to_string(),
            level: 1,
        },
        TocItem {
            id: "dashed".to_string(),
            title: "虚线".to_string(),
            level: 1,
        },
        TocItem {
            id: "title".to_string(),
            title: "标题".to_string(),
            level: 1,
        },
        TocItem {
            id: "title-placement".to_string(),
            title: "标题位置".to_string(),
            level: 1,
        },
        TocItem {
            id: "custom-style".to_string(),
            title: "自定义样式".to_string(),
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
                class: "doc-page",

                h1 {
                    class: "text-4xl font-bold text-gray-900 dark:text-white mb-4 transition-colors",
                    "Divider 分割线"
                }

                p {
                    class: "text-lg text-gray-600 dark:text-gray-300 mb-8 transition-colors",
                    "用于分隔内容的分割线组件。"
                }

                // 基础用法
                section {
                    id: "basic",
                    class: "mb-12",

                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "基础用法"
                    }

                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                        "基础的分割线。"
                    }

                    DemoBox {
                        title: "基础分割线".to_string(),
                        description: "用于分隔内容的分割线组件".to_string(),
                        code: r#"use helixui::components::DividerHorizontal;

rsx! {
    div {
        class: "space-y-4",
        div { "文本内容" }
        DividerHorizontal {}
        div { "更多内容" }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "文本内容"
                            }

                            DividerHorizontal {}

                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "更多内容"
                            }
                        }
                    }
                }

                // 方向
                section {
                    id: "direction",
                    class: "mb-12",

                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "方向"
                    }

                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                        "支持水平和垂直两种方向。"
                    }

                    DemoBox {
                        title: "水平分割线".to_string(),
                        description: "水平方向的分割线".to_string(),
                        code: r#"use helixui::components::DividerHorizontal;

rsx! {
    div {
        class: "space-y-4",
        div { "水平分割线" }
        DividerHorizontal {}
        div { "更多内容" }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "水平分割线"
                            }

                            DividerHorizontal {}

                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "更多内容"
                            }
                        }
                    }

                    DemoBox {
                        title: "垂直分割线".to_string(),
                        description: "垂直方向的分割线".to_string(),
                        code: r#"use helixui::components::DividerVertical;

rsx! {
    div {
        class: "flex items-center space-x-4 h-20",
        span { "左侧" }
        DividerVertical {}
        span { "右侧" }
    }
}"#.to_string(),

                        div {
                            class: "flex items-center space-x-4 h-20",
                            span {
                                class: "text-gray-700 dark:text-gray-300",
                                "左侧"
                            }
                            DividerVertical {}
                            span {
                                class: "text-gray-700 dark:text-gray-300",
                                "右侧"
                            }
                        }
                    }
                }

                // 虚线
                section {
                    id: "dashed",
                    class: "mb-12",

                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "虚线"
                    }

                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                        "可以设置为虚线样式。"
                    }

                    DemoBox {
                        title: "虚线分割线".to_string(),
                        description: "虚线样式的分割线".to_string(),
                        code: r#"use helixui::components::DividerHorizontal;

rsx! {
    div {
        class: "space-y-4",
        div { "虚线分割线" }
        DividerHorizontal { dashed: true }
        div { "更多内容" }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "虚线分割线"
                            }

                            DividerHorizontal { dashed: true }

                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "更多内容"
                            }
                        }
                    }
                }

                // 标题
                section {
                    id: "title",
                    class: "mb-12",

                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "标题"
                    }

                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                        "可以设置分割线的标题。"
                    }

                    DemoBox {
                        title: "带标题的分割线".to_string(),
                        description: "在分割线中间显示标题".to_string(),
                        code: r#"use helixui::components::Divider;

rsx! {
    div {
        class: "space-y-4",
        div { "文本内容" }
        Divider {
            title: Some("分割线标题".to_string()),
        }
        div { "更多内容" }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "文本内容"
                            }

                            Divider {
                                title: Some("分割线标题".to_string()),
                            }

                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "更多内容"
                            }
                        }
                    }
                }

                // 标题位置
                section {
                    id: "title-placement",
                    class: "mb-12",

                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "标题位置"
                    }

                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                        "可以设置标题的位置，支持左对齐、居中、右对齐。"
                    }

                    DemoBox {
                        title: "左对齐标题".to_string(),
                        description: "标题左对齐的分割线".to_string(),
                        code: r#"use helixui::components::Divider;

rsx! {
    div {
        class: "space-y-4",
        div { "左对齐标题" }
        Divider {
            title: Some("左对齐".to_string()),
            title_placement: "left".to_string(),
        }
        div { "更多内容" }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "左对齐标题"
                            }

                            Divider {
                                title: Some("左对齐".to_string()),
                                title_placement: "left".to_string(),
                            }

                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "更多内容"
                            }
                        }
                    }

                    DemoBox {
                        title: "居中对齐标题".to_string(),
                        description: "标题居中对齐的分割线".to_string(),
                        code: r#"use helixui::components::Divider;

rsx! {
    div {
        class: "space-y-4",
        div { "居中对齐标题" }
        Divider {
            title: Some("居中".to_string()),
            title_placement: "center".to_string(),
        }
        div { "更多内容" }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "居中对齐标题"
                            }

                            Divider {
                                title: Some("居中".to_string()),
                                title_placement: "center".to_string(),
                            }

                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "更多内容"
                            }
                        }
                    }

                    DemoBox {
                        title: "右对齐标题".to_string(),
                        description: "标题右对齐的分割线".to_string(),
                        code: r#"use helixui::components::Divider;

rsx! {
    div {
        class: "space-y-4",
        div { "右对齐标题" }
        Divider {
            title: Some("右对齐".to_string()),
            title_placement: "right".to_string(),
        }
        div { "更多内容" }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "右对齐标题"
                            }

                            Divider {
                                title: Some("右对齐".to_string()),
                                title_placement: "right".to_string(),
                            }

                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "更多内容"
                            }
                        }
                    }
                }

                // 自定义样式
                section {
                    id: "custom-style",
                    class: "mb-12",

                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "自定义样式"
                    }

                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                        "可以通过 class 和 style 属性来自定义分割线的样式。"
                    }

                    DemoBox {
                        title: "自定义颜色".to_string(),
                        description: "使用 Tailwind CSS 类自定义分割线颜色".to_string(),
                        code: r#"use helixui::components::DividerHorizontal;

rsx! {
    div {
        class: "space-y-4",
        div { "自定义颜色" }
        DividerHorizontal {
            class: Some("border-red-500".to_string()),
        }
        div { "更多内容" }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "自定义颜色"
                            }

                            DividerHorizontal {
                                class: Some("border-red-500".to_string()),
                            }

                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "更多内容"
                            }
                        }
                    }

                    DemoBox {
                        title: "自定义粗细".to_string(),
                        description: "使用 Tailwind CSS 类自定义分割线粗细".to_string(),
                        code: r#"use helixui::components::DividerHorizontal;

rsx! {
    div {
        class: "space-y-4",
        div { "自定义粗细" }
        DividerHorizontal {
            class: Some("border-2".to_string()),
        }
        div { "更多内容" }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "自定义粗细"
                            }

                            DividerHorizontal {
                                class: Some("border-2".to_string()),
                            }

                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "更多内容"
                            }
                        }
                    }

                    DemoBox {
                        title: "自定义样式".to_string(),
                        description: "使用内联样式自定义分割线外观".to_string(),
                        code: r#"use helixui::components::DividerHorizontal;

rsx! {
    div {
        class: "space-y-4",
        div { "自定义样式" }
        DividerHorizontal {
            style: Some("border-color: #10b981; border-width: 3px;".to_string()),
        }
        div { "更多内容" }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "自定义样式"
                            }

                            DividerHorizontal {
                                style: Some("border-color: #10b981; border-width: 3px;".to_string()),
                            }

                            div {
                                class: "text-gray-700 dark:text-gray-300",
                                "更多内容"
                            }
                        }
                    }
                }

                // API
                section {
                    id: "api",
                    class: "mb-12",

                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                        "API"
                    }

                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                        "Divider 组件的属性说明。"
                    }

                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "w-full border-collapse border border-gray-200 dark:border-gray-700",

                            thead {
                                tr {
                                    class: "bg-gray-50 dark:bg-gray-800",
                                    th {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-left font-semibold text-gray-900 dark:text-white",
                                        "属性"
                                    }
                                    th {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-left font-semibold text-gray-900 dark:text-white",
                                        "类型"
                                    }
                                    th {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-left font-semibold text-gray-900 dark:text-white",
                                        "默认值"
                                    }
                                    th {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-left font-semibold text-gray-900 dark:text-white",
                                        "说明"
                                    }
                                }
                            }

                            tbody {
                                tr {
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                        "direction"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "DividerDirection"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "Horizontal"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "分割线方向，支持水平和垂直"
                                    }
                                }

                                tr {
                                    class: "bg-gray-50 dark:bg-gray-800",
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                        "dashed"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "bool"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "false"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "是否为虚线"
                                    }
                                }

                                tr {
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                        "title"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "String?"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "-"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "分割线标题"
                                    }
                                }

                                tr {
                                    class: "bg-gray-50 dark:bg-gray-800",
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                        "title_placement"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "String"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "\"center\""
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "标题位置，支持 left、center、right"
                                    }
                                }

                                tr {
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                        "class"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "String?"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "-"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "自定义 CSS 类名"
                                    }
                                }

                                tr {
                                    class: "bg-gray-50 dark:bg-gray-800",
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                        "style"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "String?"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "-"
                                    }
                                    td {
                                        class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                        "自定义内联样式"
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
