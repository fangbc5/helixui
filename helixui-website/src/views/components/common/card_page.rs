use crate::views::layout::ComponentsSidebar;
use crate::views::layout::TocItem;
use crate::views::DocPage;
use dioxus::prelude::*;
use helixui::components::{
    Button, ButtonType, Card, CardContent, CardFooter, CardGrid, CardGroup, CardShadow, CardSize,
    DemoBox,
};

/// Card 组件文档页面
#[component]
pub fn CardPage() -> Element {
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
            id: "title".to_string(),
            title: "标题和操作".to_string(),
            level: 1,
        },
        TocItem {
            id: "content".to_string(),
            title: "内容区域".to_string(),
            level: 1,
        },
        TocItem {
            id: "footer".to_string(),
            title: "页脚".to_string(),
            level: 1,
        },
        TocItem {
            id: "collapsible".to_string(),
            title: "可折叠".to_string(),
            level: 1,
        },
        TocItem {
            id: "bordered".to_string(),
            title: "边框".to_string(),
            level: 1,
        },
        TocItem {
            id: "shadow".to_string(),
            title: "阴影".to_string(),
            level: 1,
        },
        TocItem {
            id: "group".to_string(),
            title: "卡片组".to_string(),
            level: 1,
        },
        TocItem {
            id: "grid".to_string(),
            title: "卡片网格".to_string(),
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
                        "卡片 Card"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "卡片组件，用于展示信息内容。"
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
                        "最简单的卡片用法。"
                    }

                    DemoBox {
                        title: "基础卡片".to_string(),
                        description: "最基本的卡片容器".to_string(),
                        code: r#"use helixui::components::{Card, CardContent};

rsx! {
    Card {
        CardContent {
            "这是一张基础卡片的内容。"
        }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            Card {
                                CardContent {
                                    "这是一张基础卡片的内容。"
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
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "卡片提供三种尺寸：小、中、大。"
                    }

                    DemoBox {
                        title: "不同尺寸".to_string(),
                        description: "小、中、大三种尺寸的卡片".to_string(),
                        code: r#"use helixui::components::{Card, CardContent, CardSize};

rsx! {
    div {
        class: "space-y-4",
        Card {
            size: CardSize::Small,
            CardContent {
                "小尺寸卡片"
            }
        }
        Card {
            size: CardSize::Medium,
            CardContent {
                "中等尺寸卡片"
            }
        }
        Card {
            size: CardSize::Large,
            CardContent {
                "大尺寸卡片"
            }
        }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            Card {
                                size: CardSize::Small,
                                CardContent {
                                    "小尺寸卡片"
                                }
                            }
                            Card {
                                size: CardSize::Medium,
                                CardContent {
                                    "中等尺寸卡片"
                                }
                            }
                            Card {
                                size: CardSize::Large,
                                CardContent {
                                    "大尺寸卡片"
                                }
                            }
                        }
                    }
                }

                // 标题和操作
                section {
                    id: "title",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "标题和操作"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "卡片可以包含标题、副标题和操作按钮。"
                    }

                    DemoBox {
                        title: "带标题的卡片".to_string(),
                        description: "包含标题、副标题和操作按钮的卡片".to_string(),
                        code: r#"use helixui::components::{Card, CardContent, CardHeader, Button, ButtonType};

rsx! {
    Card {
        title: Some("卡片标题".to_string()),
        subtitle: Some("这是卡片的副标题".to_string()),
        CardContent {
            "卡片的主要内容区域。"
        }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            Card {
                                title: Some("卡片标题".to_string()),
                                subtitle: Some("这是卡片的副标题".to_string()),
                                CardContent {
                                    "卡片的主要内容区域。"
                                }
                            }
                        }
                    }
                }

                // 内容区域
                section {
                    id: "content",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "内容区域"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "卡片内容区域可以包含任意内容。"
                    }

                    DemoBox {
                        title: "丰富内容".to_string(),
                        description: "包含多种内容的卡片".to_string(),
                        code: r#"use helixui::components::{Card, CardContent, CardHeader, Button, ButtonType};

rsx! {
    Card {
        title: Some("项目统计".to_string()),
        CardContent {
            div {
                class: "space-y-4",
                div {
                    class: "flex justify-between items-center",
                    span { "总项目数" }
                    span { class: "font-semibold", "24" }
                }
                div {
                    class: "flex justify-between items-center",
                    span { "已完成" }
                    span { class: "font-semibold text-green-600", "18" }
                }
                div {
                    class: "flex justify-between items-center",
                    span { "进行中" }
                    span { class: "font-semibold text-blue-600", "6" }
                }
            }
        }
    }
}"#.to_string(),

                        Card {
                            title: Some("项目统计".to_string()),
                            CardContent {
                                div {
                                    class: "space-y-4",
                                    div {
                                        class: "flex justify-between items-center",
                                        span { "总项目数" }
                                        span { class: "font-semibold", "24" }
                                    }
                                    div {
                                        class: "flex justify-between items-center",
                                        span { "已完成" }
                                        span { class: "font-semibold text-green-600", "18" }
                                    }
                                    div {
                                        class: "flex justify-between items-center",
                                        span { "进行中" }
                                        span { class: "font-semibold text-blue-600", "6" }
                                    }
                                }
                            }
                        }
                    }
                }

                // 页脚
                section {
                    id: "footer",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "页脚"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "卡片可以包含页脚区域。"
                    }

                    DemoBox {
                        title: "带页脚的卡片".to_string(),
                        description: "包含页脚区域的卡片".to_string(),
                        code: r#"use helixui::components::{Card, CardContent, CardFooter, Button, ButtonType};

rsx! {
    Card {
        title: Some("用户信息".to_string()),
        CardContent {
            "这是用户的基本信息内容。"
        }
        CardFooter {
            div {
                class: "flex justify-end space-x-2",
                Button {
                    button_type: ButtonType::Secondary,
                    "取消"
                }
                Button {
                    button_type: ButtonType::Primary,
                    "确定"
                }
            }
        }
    }
}"#.to_string(),

                        Card {
                            title: Some("用户信息".to_string()),
                            CardContent {
                                "这是用户的基本信息内容。"
                            }
                            CardFooter {
                                div {
                                    class: "flex justify-end space-x-2",
                                    Button {
                                        button_type: ButtonType::Default,
                                        "取消"
                                    }
                                    Button {
                                        button_type: ButtonType::Primary,
                                        "确定"
                                    }
                                }
                            }
                        }
                    }
                }

                // 可折叠
                section {
                    id: "collapsible",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "可折叠"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "卡片可以设置为可折叠，支持展开和收起。"
                    }

                    DemoBox {
                        title: "可折叠卡片".to_string(),
                        description: "支持展开和收起的卡片".to_string(),
                        code: r#"use helixui::components::{Card, CardContent};

rsx! {
    Card {
        title: Some("可折叠卡片".to_string()),
        collapsible: true,
        default_expanded: true,
        CardContent {
            "这是可折叠卡片的内容。点击标题右侧的箭头可以展开或收起内容。"
        }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            Card {
                                title: Some("可折叠卡片".to_string()),
                                collapsible: true,
                                default_expanded: true,
                                CardContent {
                                    "这是可折叠卡片的内容。点击标题右侧的箭头可以展开或收起内容。"
                                }
                            }
                            Card {
                                title: Some("默认收起".to_string()),
                                collapsible: true,
                                default_expanded: false,
                                CardContent {
                                    "这个卡片默认是收起状态。"
                                }
                            }
                        }
                    }
                }

                // 边框
                section {
                    id: "bordered",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "边框"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "卡片可以控制是否显示边框。"
                    }

                    DemoBox {
                        title: "边框控制".to_string(),
                        description: "有边框和无边框的卡片对比".to_string(),
                        code: r#"use helixui::components::{Card, CardContent};

rsx! {
    div {
        class: "space-y-4",
        Card {
            title: Some("有边框".to_string()),
            bordered: true,
            CardContent {
                "这个卡片有边框。"
            }
        }
        Card {
            title: Some("无边框".to_string()),
            bordered: false,
            CardContent {
                "这个卡片没有边框。"
            }
        }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            Card {
                                title: Some("有边框".to_string()),
                                bordered: true,
                                CardContent {
                                    "这个卡片有边框。"
                                }
                            }
                            Card {
                                title: Some("无边框".to_string()),
                                bordered: false,
                                CardContent {
                                    "这个卡片没有边框。"
                                }
                            }
                        }
                    }
                }

                // 阴影
                section {
                    id: "shadow",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "阴影"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "卡片提供三种阴影级别：从不、悬停时、总是。"
                    }

                    DemoBox {
                        title: "阴影级别".to_string(),
                        description: "不同阴影级别的卡片".to_string(),
                        code: r#"use helixui::components::{Card, CardContent, CardShadow};

rsx! {
    div {
        class: "space-y-4",
        Card {
            title: Some("无阴影".to_string()),
            shadow: CardShadow::Never,
            CardContent {
                "这个卡片没有阴影。"
            }
        }
        Card {
            title: Some("悬停阴影".to_string()),
            shadow: CardShadow::Hover,
            CardContent {
                "这个卡片在悬停时显示阴影。"
            }
        }
        Card {
            title: Some("总是阴影".to_string()),
            shadow: CardShadow::Always,
            CardContent {
                "这个卡片总是显示阴影。"
            }
        }
    }
}"#.to_string(),

                        div {
                            class: "space-y-4",
                            Card {
                                title: Some("无阴影".to_string()),
                                shadow: CardShadow::Never,
                                CardContent {
                                    "这个卡片没有阴影。"
                                }
                            }
                            Card {
                                title: Some("悬停阴影".to_string()),
                                shadow: CardShadow::Hover,
                                CardContent {
                                    "这个卡片在悬停时显示阴影。"
                                }
                            }
                            Card {
                                title: Some("总是阴影".to_string()),
                                shadow: CardShadow::Always,
                                CardContent {
                                    "这个卡片总是显示阴影。"
                                }
                            }
                        }
                    }
                }

                // 卡片组
                section {
                    id: "group",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "卡片组"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "多个卡片可以组成卡片组。"
                    }

                    DemoBox {
                        title: "卡片组".to_string(),
                        description: "垂直排列的卡片组".to_string(),
                        code: r#"use helixui::components::{Card, CardContent, CardGroup};

rsx! {
    CardGroup {
        Card {
            title: Some("卡片 1".to_string()),
            CardContent {
                "第一个卡片的内容。"
            }
        }
        Card {
            title: Some("卡片 2".to_string()),
            CardContent {
                "第二个卡片的内容。"
            }
        }
        Card {
            title: Some("卡片 3".to_string()),
            CardContent {
                "第三个卡片的内容。"
            }
        }
    }
}"#.to_string(),

                        CardGroup {
                            Card {
                                title: Some("卡片 1".to_string()),
                                CardContent {
                                    "第一个卡片的内容。"
                                }
                            }
                            Card {
                                title: Some("卡片 2".to_string()),
                                CardContent {
                                    "第二个卡片的内容。"
                                }
                            }
                            Card {
                                title: Some("卡片 3".to_string()),
                                CardContent {
                                    "第三个卡片的内容。"
                                }
                            }
                        }
                    }
                }

                // 卡片网格
                section {
                    id: "grid",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "卡片网格"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "卡片可以以网格形式排列。"
                    }

                    DemoBox {
                        title: "卡片网格".to_string(),
                        description: "网格布局的卡片".to_string(),
                        code: r#"use helixui::components::{Card, CardContent, CardGrid};

rsx! {
    CardGrid {
        columns: 3,
        Card {
            title: Some("项目 A".to_string()),
            CardContent {
                "项目 A 的描述信息。"
            }
        }
        Card {
            title: Some("项目 B".to_string()),
            CardContent {
                "项目 B 的描述信息。"
            }
        }
        Card {
            title: Some("项目 C".to_string()),
            CardContent {
                "项目 C 的描述信息。"
            }
        }
    }
}"#.to_string(),

                        CardGrid {
                            columns: 3,
                            Card {
                                title: Some("项目 A".to_string()),
                                CardContent {
                                    "项目 A 的描述信息。"
                                }
                            }
                            Card {
                                title: Some("项目 B".to_string()),
                                CardContent {
                                    "项目 B 的描述信息。"
                                }
                            }
                            Card {
                                title: Some("项目 C".to_string()),
                                CardContent {
                                    "项目 C 的描述信息。"
                                }
                            }
                        }
                    }
                }

                // API 文档
                section {
                    id: "api",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "API"
                    }

                    div {
                        class: "space-y-8",

                        // Card Props
                        div {
                            h3 {
                                class: "text-xl font-semibold text-gray-900 dark:text-white mb-4",
                                "Card Props"
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
                                                "title"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "Option<String>"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "None"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "卡片标题"
                                            }
                                        }
                                        tr {
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                                "subtitle"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "Option<String>"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "None"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "卡片副标题"
                                            }
                                        }
                                        tr {
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                                "size"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "CardSize"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "Medium"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "卡片尺寸"
                                            }
                                        }
                                        tr {
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                                "bordered"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "bool"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "true"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "是否显示边框"
                                            }
                                        }
                                        tr {
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                                "shadow"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "CardShadow"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "Hover"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "阴影级别"
                                            }
                                        }
                                        tr {
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                                "collapsible"
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
                                                "是否可折叠"
                                            }
                                        }
                                        tr {
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                                "default_expanded"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "bool"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "true"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "是否默认展开（仅在可折叠时有效）"
                                            }
                                        }
                                        tr {
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 font-mono text-sm text-gray-900 dark:text-white",
                                                "class"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "String"
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "\"\""
                                            }
                                            td {
                                                class: "border border-gray-200 dark:border-gray-700 px-4 py-2 text-gray-900 dark:text-white",
                                                "自定义类名"
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
    }
}
